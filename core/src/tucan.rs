// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use chrono::{NaiveDateTime, TimeZone, Utc};
use diesel::{prelude::{Identifiable, Insertable, PgArrayExpressionMethods, PgJsonbExpressionMethods, PgNetExpressionMethods, PgRangeExpressionMethods, Queryable, QueryableByName, RunQueryDsl}, r2d2};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    OptionalExtension, SqliteConnection,
};
use diesel::{upsert::excluded, QueryDsl};

use diesel::ExpressionMethods;
use dotenvy::dotenv;
use ego_tree::NodeRef;
use itertools::Itertools;
use log::debug;
use regex::Regex;
use reqwest::{header::HeaderValue, Client};
use scraper::{ElementRef, Html, Selector};
use tokio::sync::Semaphore;

use crate::{
    models::{
        CompleteCourse, CompleteModule, CourseEvent, CourseGroup, CourseGroupEvent,
        MaybeCompleteCourse, MaybeCompleteModule, ModuleCourse, ModuleExamType, PartialCourse,
        TucanSession, UndoneUser, VVMenuCourses, VVMenuItem, COURSES_UNFINISHED,
        MODULES_UNFINISHED,
    },
    schema::{
        course_events, course_groups_events, course_groups_unfinished, courses_unfinished,
        module_courses, module_exam_types, modules_unfinished, sessions, users_unfinished,
        vv_menu_courses, vv_menu_unfinished,
    },
    tucan_user::CourseOrCourseGroup,
    url::{
        parse_tucan_url, Action, Coursedetails, Externalpages, Moduledetails, TucanProgram,
        TucanUrl,
    },
};

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

impl GetTucanSession for Box<dyn GetTucanSession + Sync + Send> {
    fn session(&self) -> Option<&TucanSession> {
        self.as_ref().session()
    }
}

#[derive(Clone)]
pub struct Tucan<State: GetTucanSession + Sync + Send = Unauthenticated> {
    pub(crate) client: Client,
    pub(crate) semaphore: Arc<Semaphore>,
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
    pub state: State,
}

impl std::fmt::Debug for Tucan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tucan").finish()
    }
}

#[must_use]
pub fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

impl Tucan<Unauthenticated> {
    pub fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        /*
                let url = Url::parse("https://localhost:9200")?;

                let conn_pool = SingleNodeConnectionPool::new(url);

                let transport = TransportBuilder::new(conn_pool)
                    .auth(Credentials::Basic("admin".to_string(), "admin".to_string()))
                    .cert_validation(CertificateValidation::None)
                    .build()?;
                let opensearch = OpenSearch::new(transport);
        */
        Ok(Self {
            pool,
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Tucant/0.1.0 https://github.com/tucant/tucant")
                .build()?,
            semaphore: Arc::new(Semaphore::new(3)),
            state: Unauthenticated,
        })
    }

    pub async fn test(&self) -> anyhow::Result<()> {
        let mut connection = self.pool.get()?;

        let _test: Vec<_> = courses_unfinished::table
            .select(COURSES_UNFINISHED)
            .load::<MaybeCompleteCourse>(&mut connection)?;
        Ok(())
    }

    pub async fn vv_root(
        &self,
    ) -> anyhow::Result<(VVMenuItem, Vec<VVMenuItem>, Vec<MaybeCompleteCourse>)> {
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
            let mut connection = self.pool.get()?;
            diesel::insert_into(vv_menu_unfinished::table)
                .values(VVMenuItem {
                    tucan_id: vv_action.magic.clone(),
                    tucan_last_checked: Utc::now().naive_utc(),
                    name: "root".to_string(),
                    done: false,
                    parent: None,
                })
                .on_conflict_do_nothing() // TODO FIXME
                .execute(&mut connection)?;
        }

        self.vv(vv_action).await
    }

    async fn cached_vv(
        &self,
        url: Action,
    ) -> anyhow::Result<Option<(VVMenuItem, Vec<VVMenuItem>, Vec<MaybeCompleteCourse>)>> {
        let mut connection = self.pool.get()?;

        let existing_vv_menu_already_fetched = vv_menu_unfinished::table
            .filter(vv_menu_unfinished::tucan_id.eq(&url.magic))
            .filter(vv_menu_unfinished::done)
            .get_result::<VVMenuItem>(&mut connection)
            .optional()?;

        if let Some(vv_menu) = existing_vv_menu_already_fetched {
            let submenus = vv_menu_unfinished::table
                .select(vv_menu_unfinished::all_columns)
                .filter(vv_menu_unfinished::parent.eq(&url.magic))
                .order(vv_menu_unfinished::name)
                .load::<VVMenuItem>(&mut connection)?;

            let submodules: Vec<MaybeCompleteCourse> = vv_menu_courses::table
                .inner_join(courses_unfinished::table)
                .select(COURSES_UNFINISHED)
                .filter(vv_menu_courses::vv_menu_id.eq(&url.magic))
                .order(courses_unfinished::title)
                .load::<MaybeCompleteCourse>(&mut connection)?;

            Ok(Some((vv_menu, submenus, submodules)))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unused_peekable)]
    pub async fn fetch_vv(&self, url: Action) -> anyhow::Result<()> {
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
                let mut connection = self.pool.get()?;

                // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
                let res: Result<Vec<usize>, _> = vv_menus
                    .into_iter()
                    .map(|vv_menu| -> Result<_, _> {
                        diesel::insert_into(vv_menu_unfinished::table)
                            .values(vv_menu)
                            .on_conflict_do_nothing() // TODO FIXME
                            .execute(&mut connection)
                    })
                    .collect();
                res?;
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
                            vec![MaybeCompleteCourse::Partial(PartialCourse {
                                tucan_last_checked: Utc::now().naive_utc(),
                                course_id: course_id.to_string(),
                                title: title.to_string(),
                                tucan_id: id,
                            })]
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
                        course_id: course.tucan_id().clone(),
                    })
                    .collect();

                (courses, vv_courses)
            };

            let mut connection = self.pool.get()?;

            // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
            let res: Result<Vec<usize>, _> = courses
                .into_iter()
                .map(|course| -> Result<_, _> {
                    diesel::insert_into(courses_unfinished::table)
                        .values(course)
                        .on_conflict(courses_unfinished::tucan_id)
                        .do_nothing()
                        .execute(&mut connection)
                })
                .collect();
            res?;

            let res: Result<Vec<usize>, _> = vv_courses
                .into_iter()
                .map(|vv_course| -> Result<_, _> {
                    diesel::insert_into(vv_menu_courses::table)
                        .values(vv_course)
                        .on_conflict_do_nothing() // TODO FIXME
                        .execute(&mut connection)
                })
                .collect();
            res?;
        } else {
            panic!(
                "unknown url {:?}",
                Into::<TucanProgram>::into(url).to_tucan_url(None)
            );
        }

        let mut connection = self.pool.get()?;

        diesel::update(vv_menu_unfinished::table)
            .filter(vv_menu_unfinished::tucan_id.eq(url.magic.clone()))
            .set(vv_menu_unfinished::done.eq(true))
            .execute(&mut connection)?;

        Ok(())
    }

    // caching is relatively useless as all urls when logged in are changing all the time. Only the vv links not logged in are static.
    #[async_recursion::async_recursion]
    pub async fn vv(
        &self,
        url: Action,
    ) -> anyhow::Result<(VVMenuItem, Vec<VVMenuItem>, Vec<MaybeCompleteCourse>)> {
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

impl<State: GetTucanSession + Sync + Send + 'static> Tucan<State> {
    pub(crate) fn parse_courses(document: &Html) -> Vec<MaybeCompleteCourse> {
        document
            .select(&s(r#"a[name="eventLink"]"#))
            .map(|e| e.parent().unwrap().parent().unwrap())
            .unique_by(NodeRef::id)
            .map(|node| {
                let element_ref = ElementRef::wrap(node).unwrap();
                let selector = &s("a");
                let mut links = element_ref.select(selector);
                MaybeCompleteCourse::Partial(PartialCourse {
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
                })
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

    pub async fn continue_optional_session(
        &self,
        session: Option<TucanSession>,
    ) -> anyhow::Result<Tucan<Box<dyn GetTucanSession + Sync + Send + 'static>>> {
        Ok(match session {
            Some(session) => {
                let tucan = self.continue_session(session).await?;
                Tucan {
                    client: tucan.client,
                    semaphore: tucan.semaphore,
                    pool: tucan.pool,
                    state: Box::new(tucan.state),
                }
            }
            None => Tucan {
                client: self.client.clone(),
                semaphore: self.semaphore.clone(),
                pool: self.pool.clone(),
                state: Box::new(Unauthenticated),
            },
        })
    }

    pub async fn continue_session(
        &self,
        session: TucanSession,
    ) -> anyhow::Result<Tucan<Authenticated>> {
        let mut connection = self.pool.get()?;

        // TODO FIXME strg+f insert_into(users_unfinished and do this everywhere directly in the method so it can't be forgotten
        diesel::insert_into(users_unfinished::table)
            .values(UndoneUser::new(session.matriculation_number))
            .on_conflict(users_unfinished::matriculation_number)
            .do_nothing()
            .execute(&mut connection)?;

        diesel::insert_into(sessions::table)
            .values(session.clone())
            .on_conflict((
                sessions::matriculation_number,
                sessions::session_nr,
                sessions::session_id,
            ))
            .do_nothing()
            .execute(&mut connection)?;

        drop(connection);

        Ok(Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            state: Authenticated {
                session,
            },
        })
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
            state: Authenticated { session },
        })
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Tucan<Authenticated>> {
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

                let mut connection = self.pool.get()?;

                {
                    let user_session = user.state.session.clone();
                    diesel::insert_into(users_unfinished::table)
                        .values(UndoneUser::new(user.state.session.matriculation_number))
                        .on_conflict(users_unfinished::matriculation_number)
                        .do_nothing()
                        .execute(&mut connection)?;

                    diesel::insert_into(sessions::table)
                        .values(user_session)
                        .on_conflict((
                            sessions::matriculation_number,
                            sessions::session_nr,
                            sessions::session_id,
                        ))
                        .do_nothing()
                        .execute(&mut connection)?;
                }

                return Ok(user);
            }
            panic!("Failed to extract session_nr");
        }

        res_headers.text().await?;

        Err(Error::new(ErrorKind::Other, "Invalid username or password").into())
    }

    #[must_use]
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
                // TODO the link is optional, eg. Praktikum Visual Computing doesn't link a room
                let room = room_column.text().join("");
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
        connection: &mut SqliteConnection,
    ) -> anyhow::Result<()> {
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
            let course_id = fs
                .next()
                .unwrap_or_else(|| unwrap_handler())
                .trim()
                .to_owned();
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

            let course = MaybeCompleteCourse::Complete(CompleteCourse {
                tucan_id: url.id.clone(),
                tucan_last_checked: Utc::now().naive_utc(),
                title: course_name.unwrap_or_else(|| unwrap_handler()).to_string(),
                sws,
                course_id,
                content,
            });

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
            /*
                        // TODO FIXME only do this when logged in as otherwise this doesn't work
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
            */
            (course, course_groups, events)
        };

        debug!("[+] course {:?}", course);
        diesel::insert_into(courses_unfinished::table)
            .values(&course)
            .on_conflict(courses_unfinished::tucan_id)
            .do_update()
            .set(&course)
            .execute(connection)?;

        let res: Result<Vec<usize>, _> = course_groups
            .into_iter()
            .map(|course_group| -> Result<_, _> {
                diesel::insert_into(course_groups_unfinished::table)
                    .values(&course_group)
                    .on_conflict(course_groups_unfinished::tucan_id)
                    .do_nothing()
                    .execute(connection)
            })
            .collect();
        res?;

        let res: Result<Vec<usize>, _> = events
            .into_iter()
            .map(|event| -> Result<_, _> {
                diesel::insert_into(course_events::table)
                    .values(event)
                    .on_conflict((
                        course_events::course,
                        course_events::timestamp_start,
                        course_events::timestamp_end,
                        course_events::room,
                    ))
                    .do_update()
                    .set(course_events::teachers.eq(excluded(course_events::teachers)))
                    .execute(connection)
            })
            .collect();
        res?;

        Ok(())
    }

    async fn fetch_course_group(
        &self,
        url: Coursedetails,
        document: String,
        connection: &mut SqliteConnection,
    ) -> anyhow::Result<()> {
        let (course_group, events, h1) = {
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

            let h1 = document
                .select(&s("h1"))
                .next()
                .unwrap()
                .inner_html()
                .trim()
                .to_owned();

            (
                CourseGroup {
                    tucan_id: url.id,
                    course: course_details.id,
                    title: name,
                    done: true,
                },
                events,
                h1,
            )
        };

        debug!("[+] course group {:?}", course_group);

        let Some((course_id, title)) = h1.split_once('\n') else {panic!("{}", h1)};

        let course = MaybeCompleteCourse::Partial(PartialCourse {
            tucan_id: course_group.course.clone(),
            tucan_last_checked: Utc::now().naive_utc(),
            title: title.to_owned(),
            course_id: course_id.to_owned(),
        });

        diesel::insert_into(courses_unfinished::table)
            .values(&course)
            .on_conflict(courses_unfinished::tucan_id)
            .do_update()
            .set(&course)
            .execute(connection)?;

        diesel::insert_into(course_groups_unfinished::table)
            .values(&course_group)
            .on_conflict(course_groups_unfinished::tucan_id)
            .do_update()
            .set(&course_group)
            .execute(connection)?;

        let res: Result<Vec<usize>, _> = events
            .into_iter()
            .map(|event| -> Result<_, _> {
                diesel::insert_into(course_groups_events::table)
                    .values(event)
                    .on_conflict((
                        course_groups_events::course,
                        course_groups_events::timestamp_start,
                        course_groups_events::timestamp_end,
                        course_groups_events::room,
                    ))
                    .do_update()
                    .set(
                        course_groups_events::teachers.eq(excluded(course_groups_events::teachers)),
                    )
                    .execute(connection)
            })
            .collect();
        res?;

        Ok(())
    }

    async fn cached_course(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<
        Option<(
            CompleteCourse,
            Vec<CourseGroup>,
            Vec<CourseEvent>,
            Vec<MaybeCompleteModule>,
        )>,
    > {
        let mut connection = self.pool.get()?;

        let existing = courses_unfinished::table
            .filter(courses_unfinished::tucan_id.eq(&url.id))
            .filter(courses_unfinished::done)
            .select(COURSES_UNFINISHED)
            .get_result::<CompleteCourse>(&mut connection)
            .optional()?;

        if let Some(existing) = existing {
            debug!("[~] course {:?}", existing);

            let course_groups = courses_unfinished::table
                .filter(courses_unfinished::tucan_id.eq(&existing.tucan_id))
                .inner_join(course_groups_unfinished::table)
                .select(course_groups_unfinished::all_columns)
                .order(course_groups_unfinished::title)
                .load::<CourseGroup>(&mut connection)?;

            let course_events = courses_unfinished::table
                .filter(courses_unfinished::tucan_id.eq(&existing.tucan_id))
                .inner_join(course_events::table)
                .select(course_events::all_columns)
                .order(course_events::timestamp_start)
                .load::<CourseEvent>(&mut connection)?;

            let parent_modules = module_courses::table
                .filter(module_courses::course.eq(&existing.tucan_id))
                .inner_join(modules_unfinished::table)
                .select(MODULES_UNFINISHED)
                .order(modules_unfinished::title)
                .load::<MaybeCompleteModule>(&mut connection)?;

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
        let mut connection = self.pool.get()?;

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
            .optional()?;

        if let Some(existing) = existing {
            debug!("[~] coursegroup {:?}", existing);

            let course_group_events: Vec<CourseGroupEvent> = course_groups_events::table
                .filter(course_groups_events::course.eq(&existing.tucan_id))
                .select(course_groups_events::all_columns)
                .order(course_groups_events::timestamp_start)
                .load::<CourseGroupEvent>(&mut connection)?;

            return Ok(Some((existing, course_group_events)));
        }

        Ok(None)
    }

    pub async fn course(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<(
        CompleteCourse,
        Vec<CourseGroup>,
        Vec<CourseEvent>,
        Vec<MaybeCompleteModule>,
    )> {
        if let Some(value) = self.cached_course(url.clone()).await? {
            return Ok(value);
        }

        let document = self.fetch_document(&url.clone().into()).await?;
        let mut connection = self.pool.get()?;

        self.fetch_course(url.clone(), document, &mut connection)
            .await?;

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
        let mut connection = self.pool.get()?;

        self.fetch_course_group(url.clone(), document, &mut connection)
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
        let mut connection = self.pool.get()?;

        let is_course_group =
            element_by_selector(&Self::parse_document(&document), "form h1 + h2").is_some();

        if is_course_group {
            Ok(CourseOrCourseGroup::CourseGroup({
                self.fetch_course_group(url.clone(), document, &mut connection)
                    .await?;
                self.cached_course_group(url.clone()).await?.unwrap()
            }))
        } else {
            Ok(CourseOrCourseGroup::Course({
                self.fetch_course(url.clone(), document, &mut connection)
                    .await?;
                self.cached_course(url.clone()).await?.unwrap()
            }))
        }
    }

    async fn cached_module(
        &self,
        url: Moduledetails,
    ) -> anyhow::Result<
        Option<(
            CompleteModule,
            Vec<MaybeCompleteCourse>,
            Vec<ModuleExamType>,
        )>,
    > {
        let mut connection = self.pool.get()?;

        let existing_module = modules_unfinished::table
            .filter(modules_unfinished::tucan_id.eq(&url.id))
            .filter(modules_unfinished::done)
            .select(MODULES_UNFINISHED)
            .order(modules_unfinished::title)
            .get_result::<CompleteModule>(&mut connection)
            .optional()?;

        if let Some(existing_module) = existing_module {
            debug!("[~] module {:?}", existing_module);

            let course_list = module_courses::table
                .filter(module_courses::module.eq(&existing_module.tucan_id))
                .inner_join(courses_unfinished::table)
                .order(courses_unfinished::title)
                .select(COURSES_UNFINISHED)
                .load::<MaybeCompleteCourse>(&mut connection)?;

            let exam_types = module_exam_types::table
                .filter(module_exam_types::module_id.eq(&existing_module.tucan_id))
                .order(module_exam_types::exam_type)
                .load::<ModuleExamType>(&mut connection)?;

            Ok(Some((existing_module, course_list, exam_types)))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_lines)]
    pub async fn fetch_module(&self, url: Moduledetails) -> anyhow::Result<()> {
        let document = self.fetch_document(&url.clone().into()).await?;
        let mut connection = self.pool.get()?;

        let (module, courses, modul_exam_types) = {
            let document = Self::parse_document(&document);

            let name = element_by_selector(&document, "h1").unwrap();

            let text = name.inner_html();
            let mut fs = text.split("&nbsp;");
            let module_id = fs.next().unwrap().trim().to_owned();

            let module_name = fs.next().map(str::trim);

            let credits = document
                .select(&s(r#"#contentlayoutleft b"#))
                .find(|e| e.inner_html() == "Credits: ")
                .unwrap()
                .next_sibling()
                .unwrap()
                .value()
                .as_text()
                .unwrap();

            let credits = credits
                .trim()
                .strip_suffix(",0")
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(0);

            let content = document
                .select(&s("#contentlayoutleft tr.tbdata"))
                .next()
                .unwrap_or_else(|| panic!("{}", document.root_element().inner_html()))
                .inner_html();

            let courses = Self::parse_courses(&document);

            let leistungen = document
                .select(&s("table[summary=\"Leistungen\"] tbody"))
                .next()
                .unwrap();

            let modul_exam_types = if leistungen.select(&s("tr")).next().is_none() {
                Vec::new() // empty
            } else {
                let title = leistungen
                    .select(&s(".level02_color"))
                    .next()
                    .unwrap()
                    .inner_html();
                let re = Regex::new(r"\s+").unwrap();
                let title = re.replace_all(&title, " ");
                let _title = title.trim();

                let exam_types = if leistungen.select(&s("tr.tbdata")).next().is_some() {
                    leistungen
                        .select(&s("tr.tbdata"))
                        .map(|tr| {
                            let detail_reqachieve = tr
                                .select(&s(".rw-detail-reqachieve"))
                                .next()
                                .unwrap()
                                .inner_html();
                            let detail_reqachieve = detail_reqachieve.replace("&nbsp;", "");
                            let detail_reqachieve = detail_reqachieve.trim().to_owned();
                            let detail_compulsory = tr
                                .select(&s(".rw-detail-compulsory"))
                                .next()
                                .unwrap()
                                .inner_html();
                            let detail_compulsory = detail_compulsory.trim().to_owned();
                            let detail_weight = tr
                                .select(&s(".rw-detail-weight"))
                                .next()
                                .unwrap()
                                .inner_html();
                            let detail_weight = detail_weight.trim().to_owned();

                            println!(
                                "{}: {}|{}|{}",
                                module_name.unwrap(),
                                detail_reqachieve,
                                detail_compulsory,
                                detail_weight
                            );

                            (detail_reqachieve, detail_compulsory, detail_weight)
                        })
                        .collect_vec()
                } else {
                    leistungen
                        .select(&s("tr"))
                        .map(|tr| {
                            let detail_reqachieve = tr
                                .select(&s(".rw-detail-reqachieve"))
                                .next()
                                .unwrap()
                                .inner_html();
                            let detail_reqachieve = detail_reqachieve.replace("&nbsp;", "");
                            let detail_reqachieve = detail_reqachieve.trim().to_owned();
                            let detail_compulsory = tr
                                .select(&s(".rw-detail-compulsory"))
                                .next()
                                .unwrap()
                                .inner_html();
                            let detail_compulsory = detail_compulsory.trim().to_owned();
                            let detail_weight = tr
                                .select(&s(".rw-detail-weight"))
                                .next()
                                .unwrap()
                                .inner_html();
                            let detail_weight = detail_weight.trim().to_owned();

                            println!(
                                "{}: {}|{}|{}",
                                module_name.unwrap(),
                                detail_reqachieve,
                                detail_compulsory,
                                detail_weight
                            );
                            (detail_reqachieve, detail_compulsory, detail_weight)
                        })
                        .collect_vec()
                };

                exam_types
                    .into_iter()
                    .map(|(detail_reqachieve, detail_compulsory, detail_weight)| {
                        let detail_weight = detail_weight.split("<br>").next().unwrap().trim();
                        ModuleExamType {
                            module_id: url.id.clone(),
                            exam_type: detail_reqachieve,
                            required: match detail_compulsory.as_str() {
                                "Ja" => true,
                                "Nein" => false,
                                _ => panic!(),
                            },
                            weight: if detail_weight.ends_with('%') {
                                detail_weight.trim_end_matches('%').parse().unwrap()
                            } else {
                                detail_weight.parse().unwrap()
                            },
                        }
                    })
                    .collect()
            };
            /*
                        let modul_exam_types = document
                            .select(&s("table[summary=\"Modulabschlussprüfungen\"] tbody tr"))
                            .map(|module_exam_type| {
                                // this here even has a date for some reason
                                // maybe we can even ignore these here completely as they may be redundant information
                                let detail_exam = module_exam_type
                                    .select(&s(".rw-detail-exam"))
                                    .next()
                                    .unwrap()
                                    .inner_html();
                                let (nr, detail_exam) = detail_exam.trim().split_once("&nbsp;").unwrap();
                                let nr = nr.trim();
                                let detail_exam = detail_exam.trim();
                                let detail_date = module_exam_type
                                    .select(&s(".rw-detail-date"))
                                    .next()
                                    .unwrap()
                                    .inner_html()
                                    .trim()
                                    .to_owned();
                                let detail_instructors = module_exam_type
                                    .select(&s(".rw-detail-instructors"))
                                    .next()
                                    .unwrap()
                                    .inner_html()
                                    .trim()
                                    .to_owned();
                                let detail_compulsory = module_exam_type
                                    .select(&s(".rw-detail-compulsory"))
                                    .next()
                                    .unwrap()
                                    .inner_html()
                                    .trim()
                                    .to_owned();
                                println!(
                                    "{}: {}|{}|{}|{}|{}",
                                    module_name.unwrap(),
                                    nr,
                                    detail_exam,
                                    detail_date,
                                    detail_instructors,
                                    detail_compulsory
                                );
                            })
                            .collect_vec();
            */
            let module = CompleteModule {
                tucan_id: url.clone().id,
                tucan_last_checked: Utc::now().naive_utc(),
                title: module_name.unwrap().to_string(),
                credits,
                module_id,
                content,
            };

            (module, courses, modul_exam_types)
        };

        debug!("[+] module {:?}", module);

        diesel::insert_into(modules_unfinished::table)
            .values(&module)
            .on_conflict(modules_unfinished::tucan_id)
            .do_update()
            .set(&module)
            .execute(&mut connection)?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = courses
            .iter()
            .map(|course| -> Result<_, _> {
                diesel::insert_into(courses_unfinished::table)
                    .values(course)
                    .on_conflict(courses_unfinished::tucan_id)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = courses
            .iter()
            .map(|c| ModuleCourse {
                course: c.tucan_id().clone(),
                module: module.tucan_id.clone(),
            })
            .map(|course| -> Result<_, _> {
                diesel::insert_into(module_courses::table)
                    .values(course)
                    .on_conflict(module_courses::all_columns)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = modul_exam_types
            .into_iter()
            .map(|modul_exam_type| -> Result<_, _> {
                diesel::insert_into(module_exam_types::table)
                    .values(&modul_exam_type)
                    .on_conflict((module_exam_types::module_id, module_exam_types::exam_type))
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        Ok(())
    }

    pub async fn module(
        &self,
        url: Moduledetails,
    ) -> anyhow::Result<(
        CompleteModule,
        Vec<MaybeCompleteCourse>,
        Vec<ModuleExamType>,
    )> {
        if let Some(value) = self.cached_module(url.clone()).await? {
            return Ok(value);
        }

        self.fetch_module(url.clone()).await?;

        Ok(self.cached_module(url).await?.unwrap())
    }
}
