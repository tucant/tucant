// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use axum::http::HeaderValue;
use chrono::{NaiveDateTime, TimeZone, Utc};
use deadpool::managed::Pool;
use diesel::OptionalExtension;
use diesel::{upsert::excluded, QueryDsl};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use deadpool::managed::Object;

use diesel::ExpressionMethods;
use ego_tree::NodeRef;
use itertools::Itertools;
use log::debug;
use once_cell::sync::Lazy;
use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    OpenSearch,
};
use regex::Regex;
use reqwest::{Client, Url};
use scraper::{ElementRef, Html, Selector};
use tokio::sync::Semaphore;

use crate::{
    models::{
        Course, CourseEvent, CourseGroup, CourseGroupEvent, Module, TucanSession, UndoneUser,
        VVMenuCourses, VVMenuItem, COURSES_UNFINISHED, MODULES_UNFINISHED,
    },
    schema::{
        course_events, course_groups_events, course_groups_unfinished, courses_unfinished,
        module_courses, modules_unfinished, sessions, users_unfinished, vv_menu_courses,
        vv_menu_unfinished,
    },
    tucan_user::CourseOrCourseGroup,
    url::{
        parse_tucan_url, Action, Coursedetails, Externalpages, Moduledetails, TucanProgram,
        TucanUrl,
    },
};

use dotenvy::dotenv;

fn get_config() -> AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url)
}

fn create_pool() -> deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    let config = get_config();
    Pool::builder(config).build().unwrap()
}

pub trait GetTucanSession {
    fn session(&self) -> Option<&TucanSession>;
}

#[derive(Clone)]
pub struct Unauthenticated;

impl GetTucanSession for Unauthenticated {
    fn session(&self) -> Option<&TucanSession> {
        None
    }
}

#[derive(Clone)]
pub struct Authenticated {
    pub session: TucanSession,
}

impl GetTucanSession for Authenticated {
    fn session(&self) -> Option<&TucanSession> {
        Some(&self.session)
    }
}

#[derive(Clone)]
pub struct Tucan<State: GetTucanSession + Sync + Send = Unauthenticated> {
    pub(crate) client: Client,
    pub(crate) semaphore: Arc<Semaphore>,
    pub pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub opensearch: OpenSearch,
    pub state: State,
}

impl std::fmt::Debug for Tucan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tucan").finish()
    }
}

pub fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

static NORMALIZED_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ /)(.]+").unwrap());

pub fn normalize(string: &str) -> String {
    // maybe do in postgres as this is generated?
    // &amp; replace with -
    // replace , to -
    // remove consecutive -
    // remove [] to -
    // remove - at end and start
    NORMALIZED_NAME_REGEX
        .replace_all(string, "-")
        .trim_matches('-')
        .to_lowercase()
}

impl Tucan<Unauthenticated> {
    pub fn new() -> anyhow::Result<Self> {
        let pool = create_pool();

        let url = Url::parse("https://localhost:9200")?;
        let conn_pool = SingleNodeConnectionPool::new(url);
        let transport = TransportBuilder::new(conn_pool)
            .auth(Credentials::Basic("admin".to_string(), "admin".to_string()))
            .cert_validation(CertificateValidation::None)
            .build()?;
        let opensearch = OpenSearch::new(transport);

        Ok(Self {
            pool,
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Tucant/0.1.0 https://github.com/tucant/tucant")
                .build()?,
            semaphore: Arc::new(Semaphore::new(3)),
            opensearch,
            state: Unauthenticated,
        })
    }

    pub async fn vv_root(&self) -> anyhow::Result<(VVMenuItem, Vec<VVMenuItem>, Vec<Course>)> {
        let document = self
            .fetch_document(&TucanProgram::Externalpages(Externalpages {
                id: 344,
                name: "welcome".to_string(),
            }))
            .await?;

        let vv_link = {
            let document = Self::parse_document(&document);

            document
                .select(&s("a"))
                .find(|e| e.inner_html() == "Vorlesungsverzeichnis (VV)")
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_owned()
        };

        let vv_action: Action =
            parse_tucan_url(&format!("https://www.tucan.tu-darmstadt.de{vv_link}"))
                .program
                .try_into()
                .unwrap();

        {
            use diesel_async::RunQueryDsl;

            let mut connection = self.pool.get().await?;
            diesel::insert_into(vv_menu_unfinished::table)
                .values(VVMenuItem {
                    tucan_id: vv_action.magic.clone(),
                    tucan_last_checked: Utc::now().naive_utc(),
                    name: "root".to_string(),
                    done: false,
                    parent: None,
                })
                .on_conflict_do_nothing() // TODO FIXME
                .execute(&mut connection)
                .await?;
        }

        self.vv(vv_action).await
    }

    async fn cached_vv(
        &self,
        url: Action,
    ) -> anyhow::Result<Option<(VVMenuItem, Vec<VVMenuItem>, Vec<Course>)>> {
        use diesel::prelude::{ExpressionMethods, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;

        let mut connection = self.pool.get().await?;

        let existing_vv_menu_already_fetched = vv_menu_unfinished::table
            .filter(vv_menu_unfinished::tucan_id.eq(&url.magic))
            .filter(vv_menu_unfinished::done)
            .get_result::<VVMenuItem>(&mut connection)
            .await
            .optional()?;

        if let Some(vv_menu) = existing_vv_menu_already_fetched {
            let submenus = vv_menu_unfinished::table
                .select(vv_menu_unfinished::all_columns)
                .filter(vv_menu_unfinished::parent.eq(&url.magic))
                .order(vv_menu_unfinished::name.asc())
                .load::<VVMenuItem>(&mut connection)
                .await?;

            let submodules: Vec<Course> = vv_menu_courses::table
                .inner_join(courses_unfinished::table)
                .select(COURSES_UNFINISHED)
                .filter(vv_menu_courses::vv_menu_id.eq(&url.magic))
                .order(courses_unfinished::title.asc())
                .load::<Course>(&mut connection)
                .await?;

            Ok(Some((vv_menu, submenus, submodules)))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unused_peekable)]
    pub async fn fetch_vv(&self, url: Action) -> anyhow::Result<()> {
        use diesel::prelude::ExpressionMethods;
        use diesel_async::RunQueryDsl;

        let document = self.fetch_document(&url.clone().into()).await?;

        let (registration_list, course_list) = {
            let document = Self::parse_document(&document);

            (
                document
                    .select(&s("#auditRegistration_list"))
                    .next()
                    .is_some(),
                document
                    .select(&s("div.tb div.tbhead"))
                    .next()
                    .map_or(false, |e| e.inner_html() == "Veranstaltungen / Module"),
            )
        };

        if registration_list {
            let vv_menus = {
                let document = Self::parse_document(&document);

                document
                    .select(&s("#auditRegistration_list li a.auditRegNodeLink"))
                    .map(|registration| VVMenuItem {
                        tucan_id: TryInto::<Action>::try_into(
                            parse_tucan_url(&format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                registration.value().attr("href").unwrap()
                            ))
                            .program,
                        )
                        .unwrap()
                        .magic,
                        tucan_last_checked: Utc::now().naive_utc(),
                        name: registration.inner_html(),
                        done: false,
                        parent: Some(url.magic.clone()),
                    })
                    .collect_vec()
            };

            {
                let mut connection = self.pool.get().await?;

                diesel::insert_into(vv_menu_unfinished::table)
                    .values(&vv_menus)
                    .on_conflict_do_nothing() // TODO FIXME
                    .execute(&mut connection)
                    .await?;
            }

            /* let results = vv_menus
                .iter()
                .map(|url| async {
                    self.vv(Action {
                        magic: url.tucan_id.clone(),
                    })
                    .await
                })
                .collect::<FuturesUnordered<_>>();

            let results: Vec<anyhow::Result<_>> = results.collect().await;

            let results: anyhow::Result<Vec<_>> = results.into_iter().collect();

            let _: Vec<_> = results?;
            */
        } else if course_list {
            let (courses, vv_courses) = {
                let document = Self::parse_document(&document);

                let courses = document
                .select(&s(r#"a[name="eventLink"]"#))
                .flat_map(|node| {
                    match parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        node.value().attr("href").unwrap()
                    ))
                    .program {
                        TucanProgram::Coursedetails(Coursedetails { id }) => {
                            let course_id_title = node.inner_html();
                            let Some((course_id, title)) = course_id_title.split_once(' ') else {panic!()};
                            vec![Course {
                                tucan_last_checked: Utc::now().naive_utc(),
                                course_id: course_id.to_string(),
                                title: title.to_string(),
                                tucan_id: id,
                                sws: 0,
                                content: String::new(),
                                done: false,
                            }]
                        }
                        TucanProgram::Moduledetails(Moduledetails { id: _ }) => {
                            // Don't handle as there is one in the whole thing
                            vec![]
                        }
                        _ => {
                            panic!();
                        }
                    }
                })
                .collect::<Vec<_>>();

                let vv_courses: Vec<_> = courses
                    .iter()
                    .map(|course| VVMenuCourses {
                        vv_menu_id: url.magic.clone(),
                        course_id: course.tucan_id.clone(),
                    })
                    .collect();

                (courses, vv_courses)
            };

            let mut connection = self.pool.get().await?;

            diesel::insert_into(courses_unfinished::table)
                .values(&courses)
                .on_conflict(courses_unfinished::tucan_id)
                .do_nothing()
                .execute(&mut connection)
                .await?;

            diesel::insert_into(vv_menu_courses::table)
                .values(&vv_courses)
                .on_conflict_do_nothing() // TODO FIXME
                .execute(&mut connection)
                .await?;
        } else {
            panic!(
                "unknown url {:?}",
                Into::<TucanProgram>::into(url).to_tucan_url(None)
            );
        }

        let mut connection = self.pool.get().await?;

        diesel::update(vv_menu_unfinished::table)
            .filter(vv_menu_unfinished::tucan_id.eq(url.magic.clone()))
            .set(vv_menu_unfinished::done.eq(true))
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    // caching is relatively useless as all urls when logged in are changing all the time. Only the vv links not logged in are static.
    #[async_recursion::async_recursion]
    pub async fn vv(
        &self,
        url: Action,
    ) -> anyhow::Result<(VVMenuItem, Vec<VVMenuItem>, Vec<Course>)> {
        if let Some(value) = self.cached_vv(url.clone()).await? {
            return Ok(value);
        }

        self.fetch_vv(url.clone()).await?;

        Ok(self.cached_vv(url.clone()).await?.unwrap())
    }
}

#[must_use]
pub fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

impl<State: GetTucanSession + Sync + Send> Tucan<State> {
    pub(crate) fn parse_courses(document: &Html) -> Vec<Course> {
        document
            .select(&s(r#"a[name="eventLink"]"#))
            .map(|e| e.parent().unwrap().parent().unwrap())
            .unique_by(NodeRef::id)
            .map(|node| {
                let element_ref = ElementRef::wrap(node).unwrap();
                let selector = &s("a");
                let mut links = element_ref.select(selector);
                Course {
                    tucan_last_checked: Utc::now().naive_utc(),
                    course_id: links.next().unwrap().inner_html(),
                    title: links.next().unwrap().inner_html(),
                    tucan_id: TryInto::<Coursedetails>::try_into(
                        parse_tucan_url(&format!(
                            "https://www.tucan.tu-darmstadt.de{}",
                            links.next().unwrap().value().attr("href").unwrap()
                        ))
                        .program,
                    )
                    .unwrap()
                    .id,
                    sws: 0,
                    content: String::new(),
                    done: false,
                }
            })
            .collect::<Vec<_>>()
    }

    pub(crate) async fn fetch_document(&self, url: &TucanProgram) -> anyhow::Result<String> {
        let url = url.to_tucan_url(
            self.state
                .session()
                .map(|session| session.session_nr.try_into().unwrap()),
        );
        let mut request = self.client.get(url).build().unwrap();

        if let Some(session) = self.state.session() {
            request.headers_mut().insert(
                "Cookie",
                HeaderValue::from_str(&format!("cnsc={}", session.session_id)).unwrap(),
            );
        }

        let permit = self.semaphore.clone().acquire_owned().await?;
        let resp = self.client.execute(request).await?.text().await?;
        drop(permit);

        if resp.contains("timeout.htm") {
            return Err(Error::new(ErrorKind::Other, "session timeout").into());
        }

        if resp.contains("access_denied.htm") {
            return Err(Error::new(ErrorKind::Other, "access denied").into());
        }

        Ok(resp)
    }

    pub(crate) fn parse_document(resp: &str) -> Html {
        Html::parse_document(resp)
    }

    #[must_use]
    pub fn continue_session(&self, session: TucanSession) -> Tucan<Authenticated> {
        Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            opensearch: self.opensearch.clone(),
            state: Authenticated { session },
        }
    }

    pub async fn tucan_session_from_session_data(
        &self,
        session_nr: i64,
        session_id: String,
    ) -> anyhow::Result<Tucan<Authenticated>> {
        let session = TucanSession {
            matriculation_number: -1, // TODO FIXME implement this more cleanly
            session_nr,
            session_id: session_id.clone(),
        };

        let tucan_user = Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            opensearch: self.opensearch.clone(),
            state: Authenticated { session },
        };

        let user = tucan_user.personal_data().await?;

        let session = TucanSession {
            matriculation_number: user.matriculation_number,
            session_nr,
            session_id,
        };

        Ok(Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            opensearch: self.opensearch.clone(),
            state: Authenticated { session },
        })
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Tucan<Authenticated>> {
        use diesel_async::RunQueryDsl;

        let params: [(&str, &str); 10] = [
            ("usrname", username),
            ("pass", password),
            ("APPNAME", "CampusNet"),
            ("PRGNAME", "LOGINCHECK"),
            (
                "ARGUMENTS",
                "clino,usrname,pass,menuno,menu_type,browser,platform",
            ),
            ("clino", "000000000000001"),
            ("menuno", "000344"),
            ("menu_type", "classic"),
            ("browser", ""),
            ("platform", ""),
        ];
        let res_headers = self
            .client
            .post("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll")
            .form(&params)
            .send()
            .await?;

        let refresh_header = res_headers.headers().get("refresh");

        if refresh_header.is_some() {
            let redirect_url = &format!(
                "https://www.tucan.tu-darmstadt.de{}",
                &refresh_header.unwrap().to_str()?[7..]
            );

            let url = parse_tucan_url(redirect_url);

            if let TucanUrl {
                session_nr: Some(nr),
                ..
            } = url
            {
                let session_cookie = res_headers.cookies().next().unwrap();
                let id = session_cookie.value().to_string();

                res_headers.text().await?;

                let session_nr = nr.try_into().unwrap();
                let session_id = id.to_string();

                let user = self
                    .tucan_session_from_session_data(session_nr, session_id.clone())
                    .await?;

                let mut connection = self.pool.get().await?;

                {
                    let user_session = user.state.session.clone();
                    connection
                        .build_transaction()
                        .run(|mut connection| {
                            Box::pin(async move {
                                diesel::insert_into(users_unfinished::table)
                                    .values(UndoneUser::new(
                                        user.state.session.matriculation_number,
                                    ))
                                    .on_conflict(users_unfinished::matriculation_number)
                                    .do_nothing()
                                    .execute(&mut connection)
                                    .await?;

                                diesel::insert_into(sessions::table)
                                    .values(user_session)
                                    .execute(&mut connection)
                                    .await?;

                                Ok::<(), diesel::result::Error>(())
                            })
                        })
                        .await?;
                }

                return Ok(user);
            }
            panic!("Failed to extract session_nr");
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())
    }

    pub fn parse_datetime(date_string: &str) -> (bool, NaiveDateTime, NaiveDateTime) {
        let re = Regex::new(
            r"([[:alpha:]]{2}), (\d{1,2})\. ([[^ ]]{3,4}) (\d{4})(\*)? (\d{2}):(\d{2})-(\d{2}):(\d{2})",
        )
        .unwrap()
        .captures_iter(date_string)
        .next()
        .unwrap();
        let mut captures = re.iter();

        let _full_match = captures.next().unwrap().unwrap().as_str();
        let _weekday_name = captures.next().unwrap().unwrap().as_str();
        let day_of_month = captures.next().unwrap().unwrap().as_str().parse().unwrap();
        let month_name = captures.next().unwrap().unwrap().as_str();
        let month_id = [
            "Jan.", "Feb.", "Mär.", "Apr.", "Mai", "Jun.", "Jul.", "Aug.", "Sep.", "Okt.", "Nov.",
            "Dez.",
        ]
        .into_iter()
        .position(|v| v == month_name)
        .unwrap()
            + 1;
        let year = captures.next().unwrap().unwrap().as_str().parse().unwrap();
        let is_star_event = captures.next().unwrap();

        let start_hour = captures.next().unwrap().unwrap().as_str().parse().unwrap();
        let start_minute = captures.next().unwrap().unwrap().as_str().parse().unwrap();
        let mut end_hour = captures.next().unwrap().unwrap().as_str().parse().unwrap();
        let mut end_minute = captures.next().unwrap().unwrap().as_str().parse().unwrap();
        let start_datetime = Utc
            .with_ymd_and_hms(
                year,
                month_id.try_into().unwrap(),
                day_of_month,
                start_hour,
                start_minute,
                0,
            )
            .unwrap();
        if end_hour == 24 && end_minute == 0 {
            end_hour = 23;
            end_minute = 59;
        }
        let end_datetime = Utc
            .with_ymd_and_hms(
                year,
                month_id.try_into().unwrap(),
                day_of_month,
                end_hour,
                end_minute,
                0,
            )
            .unwrap();

        (
            is_star_event.is_some(),
            start_datetime.naive_utc(),
            end_datetime.naive_utc(),
        )
    }

    fn extract_events(&self, url: &Coursedetails, document: &Html) -> Vec<CourseEvent> {
        let unwrap_handler = || -> ! {
            panic!(
                "{}",
                Into::<TucanProgram>::into(url.clone()).to_tucan_url(
                    self.state
                        .session()
                        .map(|s| s.session_nr.try_into().unwrap())
                )
            );
        };

        let events_tbody = document
            .select(&s(r#"caption"#))
            .find(|e| e.inner_html() == "Termine")
            .unwrap_or_else(|| unwrap_handler())
            .next_siblings()
            .find_map(ElementRef::wrap)
            .unwrap_or_else(|| unwrap_handler());

        let selector = s("tr");
        let events = events_tbody
            .select(&selector)
            .filter(|e| !e.value().classes().contains(&"rw-hide"));

        events
            .filter_map(|event| {
                let selector = s(r#"td"#);
                let mut tds = event.select(&selector);
                let id_column = tds.next().unwrap_or_else(|| unwrap_handler());
                if id_column.inner_html() == "Es liegen keine Termine vor." {
                    return None;
                }
                let date_column = tds.next().unwrap_or_else(|| unwrap_handler()); // here
                let start_time_column = tds.next().unwrap_or_else(|| unwrap_handler());
                let end_time_column = tds.next().unwrap();
                let room_column = tds.next().unwrap();
                let lecturer_column = tds.next().unwrap();

                let val = format!(
                    "{} {}-{}",
                    date_column.inner_html(),
                    start_time_column.inner_html(),
                    end_time_column.inner_html()
                );
                let date = Self::parse_datetime(&val);
                let room = room_column
                    .select(&s("a"))
                    .next()
                    .unwrap_or_else(|| unwrap_handler())
                    .inner_html();
                let lecturers = lecturer_column.inner_html().trim().to_string();

                if date.0 {
                    None
                } else {
                    Some(CourseEvent {
                        course: url.id.clone(),
                        timestamp_start: date.1,
                        timestamp_end: date.2,
                        room,
                        teachers: lecturers,
                    })
                }
            })
            .collect::<Vec<_>>()
    }

    #[allow(clippy::too_many_lines)]
    async fn fetch_course(
        &self,
        url: Coursedetails,
        document: String,
        mut connection: Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> anyhow::Result<()> {
        use diesel_async::RunQueryDsl;

        let unwrap_handler = || -> ! {
            panic!(
                "{}",
                Into::<TucanProgram>::into(url.clone()).to_tucan_url(
                    self.state
                        .session()
                        .map(|s| s.session_nr.try_into().unwrap())
                )
            );
        };

        let (course, course_groups, events) = {
            let document = Self::parse_document(&document);

            let name = element_by_selector(&document, "h1").unwrap_or_else(|| unwrap_handler());

            let text = name.inner_html();
            let mut fs = text.trim().split('\n');
            let course_id = fs.next().unwrap_or_else(|| unwrap_handler()).trim();
            let course_name = fs.next().map(str::trim);

            let sws = document
                .select(&s(r#"#contentlayoutleft b"#))
                .find(|e| e.inner_html() == "Semesterwochenstunden: ")
                .map(|v| {
                    v.next_sibling()
                        .unwrap_or_else(|| unwrap_handler())
                        .value()
                        .as_text()
                        .unwrap_or_else(|| unwrap_handler())
                });

            let sws = sws.and_then(|v| v.trim().parse::<i16>().ok()).unwrap_or(0);

            let content = document
                .select(&s("#contentlayoutleft td.tbdata"))
                .next()
                .unwrap_or_else(|| panic!("{}", document.root_element().inner_html()))
                .inner_html();

            let events = self.extract_events(&url, &document);

            let course = Course {
                tucan_id: url.id.clone(),
                tucan_last_checked: Utc::now().naive_utc(),
                title: course_name.unwrap_or_else(|| unwrap_handler()).to_string(),
                sws,
                course_id: normalize(course_id),
                content,
                done: true,
            };

            let course_groups: Vec<CourseGroup> = document
                .select(&s(".dl-ul-listview .listelement"))
                .map(|e| {
                    let coursegroupdetails: Coursedetails = parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        e.select(&s(".img_arrowLeft"))
                            .next()
                            .unwrap_or_else(|| unwrap_handler())
                            .value()
                            .attr("href")
                            .unwrap_or_else(|| unwrap_handler())
                    ))
                    .program
                    .try_into()
                    .unwrap_or_else(|_| unwrap_handler());
                    CourseGroup {
                        tucan_id: coursegroupdetails.id,
                        course: url.id.clone(),
                        title: e
                            .select(&s(".dl-ul-li-headline strong"))
                            .next()
                            .unwrap_or_else(|| unwrap_handler())
                            .inner_html(),
                        done: false,
                    }
                })
                .collect();

            let contained_in_modules = document
                .select(&s(r#"caption"#))
                .find(|e| e.inner_html() == "Enthalten in Modulen")
                .unwrap_or_else(|| unwrap_handler())
                .next_siblings()
                .find_map(ElementRef::wrap)
                .unwrap_or_else(|| unwrap_handler());
            let selector = s("td.tbdata");
            let _contained_in_modules = contained_in_modules
                .select(&selector)
                .map(|module| module.inner_html().trim().to_owned())
                .collect_vec();

            (course, course_groups, events)
        };

        debug!("[+] course {:?}", course);
        diesel::insert_into(courses_unfinished::table)
            .values(&course)
            .on_conflict(courses_unfinished::tucan_id)
            .do_update()
            .set(&course)
            .execute(&mut connection)
            .await?;

        diesel::insert_into(course_groups_unfinished::table)
            .values(&course_groups)
            .on_conflict(course_groups_unfinished::tucan_id)
            .do_nothing()
            .execute(&mut connection)
            .await?;

        diesel::insert_into(course_events::table)
            .values(&events)
            .on_conflict((
                course_events::course,
                course_events::timestamp_start,
                course_events::timestamp_end,
                course_events::room,
            ))
            .do_update()
            .set(course_events::teachers.eq(excluded(course_events::teachers)))
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    async fn fetch_course_group(
        &self,
        url: Coursedetails,
        document: String,
        mut connection: Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> anyhow::Result<()> {
        use diesel_async::RunQueryDsl;

        let (course_group, events) = {
            let document = Self::parse_document(&document);

            let plenum_element = document
                .select(&s(".img_arrowLeft"))
                .find(|e| e.inner_html() == "Plenumsveranstaltung anzeigen")
                .unwrap();

            let plenum_url = parse_tucan_url(&format!(
                "https://www.tucan.tu-darmstadt.de{}",
                plenum_element.value().attr("href").unwrap()
            ));

            let course_details: Coursedetails = plenum_url.program.try_into().unwrap();

            let name = element_by_selector(
                &document,
                ".dl-ul-listview .tbsubhead .dl-ul-li-headline strong",
            )
            .unwrap()
            .inner_html();

            let events = self
                .extract_events(&url, &document)
                .into_iter()
                .map(|ce| CourseGroupEvent {
                    course: ce.course,
                    timestamp_start: ce.timestamp_start,
                    timestamp_end: ce.timestamp_end,
                    room: ce.room,
                    teachers: ce.teachers,
                })
                .collect::<Vec<_>>();

            (
                CourseGroup {
                    tucan_id: url.id,
                    course: course_details.id,
                    title: name,
                    done: true,
                },
                events,
            )
        };

        debug!("[+] course group {:?}", course_group);

        let course = Course {
            tucan_id: course_group.course.clone(),
            tucan_last_checked: Utc::now().naive_utc(),
            title: String::new(),
            sws: 0,
            course_id: String::new(),
            content: String::new(),
            done: false,
        };

        diesel::insert_into(courses_unfinished::table)
            .values(&course)
            .on_conflict(courses_unfinished::tucan_id)
            .do_update()
            .set(&course)
            .execute(&mut connection)
            .await?;

        diesel::insert_into(course_groups_unfinished::table)
            .values(&course_group)
            .on_conflict(course_groups_unfinished::tucan_id)
            .do_update()
            .set(&course_group)
            .execute(&mut connection)
            .await?;

        diesel::insert_into(course_groups_events::table)
            .values(&events)
            .on_conflict((
                course_groups_events::course,
                course_groups_events::timestamp_start,
                course_groups_events::timestamp_end,
                course_groups_events::room,
            ))
            .do_update()
            .set(course_groups_events::teachers.eq(excluded(course_groups_events::teachers)))
            .execute(&mut connection)
            .await?;

        Ok(())
    }

    async fn cached_course(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<Option<(Course, Vec<CourseGroup>, Vec<CourseEvent>, Vec<Module>)>> {
        use diesel_async::RunQueryDsl;

        let mut connection = self.pool.get().await?;

        let existing = courses_unfinished::table
            .filter(courses_unfinished::tucan_id.eq(&url.id))
            .filter(courses_unfinished::done)
            .select(COURSES_UNFINISHED)
            .get_result::<Course>(&mut connection)
            .await
            .optional()?;

        if let Some(existing) = existing {
            debug!("[~] course {:?}", existing);

            let course_groups = courses_unfinished::table
                .filter(courses_unfinished::tucan_id.eq(&existing.tucan_id))
                .inner_join(course_groups_unfinished::table)
                .select(course_groups_unfinished::all_columns)
                .load::<CourseGroup>(&mut connection)
                .await?;

            let course_events = courses_unfinished::table
                .filter(courses_unfinished::tucan_id.eq(&existing.tucan_id))
                .inner_join(course_events::table)
                .select(course_events::all_columns)
                .load::<CourseEvent>(&mut connection)
                .await?;

            let parent_modules = module_courses::table
                .filter(module_courses::course.eq(&existing.tucan_id))
                .inner_join(modules_unfinished::table)
                .select(MODULES_UNFINISHED)
                .load::<Module>(&mut connection)
                .await?;

            return Ok(Some((
                existing,
                course_groups,
                course_events,
                parent_modules,
            )));
        }

        Ok(None)
    }

    pub async fn cached_course_group(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<Option<(CourseGroup, Vec<CourseGroupEvent>)>> {
        use diesel_async::RunQueryDsl;

        let mut connection = self.pool.get().await?;

        let existing = course_groups_unfinished::table
            .filter(course_groups_unfinished::tucan_id.eq(&url.id))
            .filter(course_groups_unfinished::done)
            .select((
                course_groups_unfinished::tucan_id,
                course_groups_unfinished::course,
                course_groups_unfinished::title,
                course_groups_unfinished::done,
            ))
            .get_result::<CourseGroup>(&mut connection)
            .await
            .optional()?;

        if let Some(existing) = existing {
            debug!("[~] coursegroup {:?}", existing);

            let course_group_events: Vec<CourseGroupEvent> = course_groups_events::table
                .filter(course_groups_events::course.eq(&existing.tucan_id))
                .select(course_groups_events::all_columns)
                .load::<CourseGroupEvent>(&mut connection)
                .await?;

            return Ok(Some((existing, course_group_events)));
        }

        Ok(None)
    }

    pub async fn course(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<(Course, Vec<CourseGroup>, Vec<CourseEvent>, Vec<Module>)> {
        if let Some(value) = self.cached_course(url.clone()).await? {
            return Ok(value);
        }

        let document = self.fetch_document(&url.clone().into()).await?;
        let connection = self.pool.get().await?;

        self.fetch_course(url.clone(), document, connection).await?;

        Ok(self.cached_course(url).await?.unwrap())
    }

    pub async fn course_group(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<(CourseGroup, Vec<CourseGroupEvent>)> {
        if let Some(value) = self.cached_course_group(url.clone()).await? {
            return Ok(value);
        }

        let document = self.fetch_document(&url.clone().into()).await?;
        let connection = self.pool.get().await?;

        self.fetch_course_group(url.clone(), document, connection)
            .await?;

        Ok(self.cached_course_group(url).await?.unwrap())
    }

    pub async fn course_or_course_group(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<CourseOrCourseGroup> {
        if let Some(value) = self.cached_course(url.clone()).await? {
            return Ok(CourseOrCourseGroup::Course(value));
        }

        if let Some(value) = self.cached_course_group(url.clone()).await? {
            return Ok(CourseOrCourseGroup::CourseGroup(value));
        }

        let document = self.fetch_document(&url.clone().into()).await?;
        let connection = self.pool.get().await?;

        let is_course_group =
            element_by_selector(&Self::parse_document(&document), "form h1 + h2").is_some();

        if is_course_group {
            Ok(CourseOrCourseGroup::CourseGroup({
                self.fetch_course_group(url.clone(), document, connection)
                    .await?;
                self.cached_course_group(url.clone()).await?.unwrap()
            }))
        } else {
            Ok(CourseOrCourseGroup::Course({
                self.fetch_course(url.clone(), document, connection).await?;
                self.cached_course(url.clone()).await?.unwrap()
            }))
        }
    }
}
