// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    convert::TryInto,
    future::{ready, Ready},
    io::{Error, ErrorKind},
};

use crate::{
    models::{
        Course, CourseGroup, Module, ModuleCourse, ModuleMenu, ModuleMenuEntryModuleRef, UndoneUser,
    },
    tucan::Tucan,
    url::{
        parse_tucan_url, Coursedetails, Moduledetails, Mymodules, Persaddress, Registration,
        RootRegistration, TucanProgram, TucanUrl,
    },
};
use crate::{
    models::{RegistrationEnum, TucanSession, UserCourse, UserModule},
    url::Profcourses,
};
use actix_session::SessionExt;
use actix_web::{dev::Payload, error::ErrorUnauthorized, FromRequest, HttpRequest};
use chrono::{NaiveDateTime, Utc};
use ego_tree::NodeRef;
use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::HeaderValue;
use scraper::{ElementRef, Html};

use crate::schema::*;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;

use diesel::GroupedBy;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::{dsl::not, upsert::excluded};
use log::debug;

use scraper::Selector;

fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

impl FromRequest for TucanSession {
    type Error = actix_web::error::Error;
    type Future = Ready<Result<TucanSession, actix_web::error::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = SessionExt::get_session(req);
        match session.get::<TucanSession>("session").unwrap() {
            Some(session) => ready(Ok(session)),
            None => ready(Err(ErrorUnauthorized("Not logged in!"))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TucanUser {
    pub tucan: Tucan,
    pub session: TucanSession,
}

#[derive(Debug)]
pub enum CourseOrCourseGroup {
    Course(Course),
    CourseGroup(CourseGroup),
}

static NORMALIZED_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ /)(.]+").unwrap());

impl TucanUser {
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

    pub(crate) async fn fetch_document(&self, url: &TucanProgram) -> anyhow::Result<Html> {
        let cookie = format!("cnsc={}", self.session.session_id);

        let a = self
            .tucan
            .client
            .get(url.to_tucan_url(Some(self.session.session_nr.try_into().unwrap())));
        let mut b = a.build().unwrap();
        b.headers_mut()
            .insert("Cookie", HeaderValue::from_str(&cookie).unwrap());

        let permit = self.tucan.semaphore.clone().acquire_owned().await?;
        let resp = self.tucan.client.execute(b).await?.text().await?;
        drop(permit);

        let html_doc = Html::parse_document(&resp);

        if html_doc
            .select(&s("h1"))
            .any(|s| s.inner_html() == "Timeout!")
        {
            return Err(Error::new(ErrorKind::Other, "well we got a timeout here. relogin").into());
        }
        Ok(html_doc)
    }

    pub async fn module(&self, url: Moduledetails) -> anyhow::Result<(Module, Vec<Course>)> {
        use diesel_async::RunQueryDsl;

        let mut connection = self.tucan.pool.get().await?;

        let existing_module = modules_unfinished::table
            .filter(modules_unfinished::tucan_id.eq(&url.id))
            .filter(modules_unfinished::done)
            .select((
                modules_unfinished::tucan_id,
                modules_unfinished::tucan_last_checked,
                modules_unfinished::title,
                modules_unfinished::module_id,
                modules_unfinished::credits,
                modules_unfinished::content,
                modules_unfinished::done,
            ))
            .get_result::<Module>(&mut connection)
            .await
            .optional()?;

        if let Some(existing_module) = existing_module {
            debug!("[~] module {:?}", existing_module);

            let course_list = ModuleCourse::belonging_to(&existing_module)
                .inner_join(courses_unfinished::table)
                .select((
                    courses_unfinished::tucan_id,
                    courses_unfinished::tucan_last_checked,
                    courses_unfinished::title,
                    courses_unfinished::course_id,
                    courses_unfinished::sws,
                    courses_unfinished::content,
                    courses_unfinished::done,
                ))
                .load::<Course>(&mut connection)
                .await?;

            return Ok((existing_module, course_list));
        }

        drop(connection);

        let document = self.fetch_document(&url.clone().into()).await?;

        let name = element_by_selector(&document, "h1").unwrap();

        let text = name.inner_html();
        let mut fs = text.split("&nbsp;");
        let module_id = fs.next().unwrap().trim();

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

        // Hinweis: In Ihrer Prüfungsordnung können abweichende Credits festgelegt sein.
        let credits = credits
            .trim()
            .strip_suffix(",0")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);

        /* let responsible_person = document
        .select(&s("#dozenten"))
        .next()
        .unwrap()
        .inner_html();*/
        let content = document
            .select(&s("#contentlayoutleft tr.tbdata"))
            .next()
            .unwrap_or_else(|| panic!("{}", document.root_element().inner_html()))
            .inner_html();

        let courses = document
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
                    content: "".to_string(),
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let module = Module {
            tucan_id: url.id,
            tucan_last_checked: Utc::now().naive_utc(),
            title: module_name.unwrap().to_string(),
            credits: Some(credits),
            module_id: TucanUser::normalize(module_id),
            content,
            done: true,
        };

        debug!("[+] module {:?}", module);

        let mut connection = self.tucan.pool.get().await?;

        diesel::insert_into(modules_unfinished::table)
            .values(&module)
            .on_conflict(modules_unfinished::tucan_id)
            .do_update()
            .set(&module)
            .execute(&mut connection)
            .await?;

        diesel::insert_into(courses_unfinished::table)
            .values(&courses)
            .on_conflict(courses_unfinished::tucan_id)
            .do_nothing()
            .execute(&mut connection)
            .await?;

        diesel::insert_into(module_courses::table)
            .values(
                courses
                    .iter()
                    .map(|c| ModuleCourse {
                        course: c.tucan_id.clone(),
                        module: module.tucan_id.clone(),
                    })
                    .collect::<Vec<_>>(),
            )
            .on_conflict(module_courses::all_columns)
            .do_nothing()
            .execute(&mut connection)
            .await?;

        Ok((module, courses))
    }

    async fn course(&self, url: Coursedetails, document: Html) -> anyhow::Result<Course> {
        use diesel_async::RunQueryDsl;

        let name = element_by_selector(&document, "h1").unwrap();

        let text = name.inner_html();
        let mut fs = text.trim().split('\n');
        let course_id = fs.next().unwrap().trim();
        let course_name = fs.next().map(str::trim);

        let sws = document
            .select(&s(r#"#contentlayoutleft b"#))
            .find(|e| e.inner_html() == "Semesterwochenstunden: ")
            .map(|v| v.next_sibling().unwrap().value().as_text().unwrap());

        let sws = sws.and_then(|v| v.trim().parse::<i16>().ok()).unwrap_or(0);

        let content = document
            .select(&s("#contentlayoutleft td.tbdata"))
            .next()
            .unwrap_or_else(|| panic!("{}", document.root_element().inner_html()))
            .inner_html();

        let course = Course {
            tucan_id: url.id.clone(),
            tucan_last_checked: Utc::now().naive_utc(),
            title: course_name.unwrap().to_string(),
            sws,
            course_id: TucanUser::normalize(course_id),
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
                        .unwrap()
                        .value()
                        .attr("href")
                        .unwrap()
                ))
                .program
                .try_into()
                .unwrap();
                CourseGroup {
                    tucan_id: coursegroupdetails.id,
                    course: url.id.clone(),
                    title: e
                        .select(&s(".dl-ul-li-headline strong"))
                        .next()
                        .unwrap()
                        .inner_html(),
                    done: false,
                }
            })
            .collect();

        debug!("[+] course {:?}", course);

        let mut connection = self.tucan.pool.get().await?;

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

        Ok(course)
    }

    async fn course_group(
        &self,
        url: Coursedetails,
        document: Html,
    ) -> anyhow::Result<CourseGroup> {
        use diesel_async::RunQueryDsl;

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

        let course_group = CourseGroup {
            tucan_id: url.id,
            course: course_details.id,
            title: name,
            done: true,
        };

        debug!("[+] course group {:?}", course_group);

        let mut connection = self.tucan.pool.get().await?;

        diesel::insert_into(course_groups_unfinished::table)
            .values(&course_group)
            .on_conflict(course_groups_unfinished::tucan_id)
            .do_update()
            .set(&course_group)
            .execute(&mut connection)
            .await?;

        Ok(course_group)
    }

    pub async fn course_or_course_group(
        &self,
        url: Coursedetails,
    ) -> anyhow::Result<CourseOrCourseGroup> {
        use diesel_async::RunQueryDsl;

        let mut connection = self.tucan.pool.get().await?;

        let existing = courses_unfinished::table
            .filter(courses_unfinished::tucan_id.eq(&url.id))
            .filter(courses_unfinished::done)
            .select((
                courses_unfinished::tucan_id,
                courses_unfinished::tucan_last_checked,
                courses_unfinished::title,
                courses_unfinished::course_id,
                courses_unfinished::sws,
                courses_unfinished::content,
                courses_unfinished::done,
            ))
            .get_result::<Course>(&mut connection)
            .await
            .optional()?;

        if let Some(existing) = existing {
            debug!("[~] course {:?}", existing);
            return Ok(CourseOrCourseGroup::Course(existing));
        }

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
            return Ok(CourseOrCourseGroup::CourseGroup(existing));
        }

        drop(connection);

        let document = self.fetch_document(&url.clone().into()).await?;

        let is_course_group = element_by_selector(&document, "form h1 + h2").is_some();

        println!("is_course_group {}", is_course_group);

        if is_course_group {
            Ok(CourseOrCourseGroup::CourseGroup(
                self.course_group(url, document).await?,
            ))
        } else {
            Ok(CourseOrCourseGroup::Course(
                self.course(url, document).await?,
            ))
        }
    }

    pub async fn root_registration(&self) -> anyhow::Result<ModuleMenu> {
        let document = self.fetch_document(&RootRegistration {}.into()).await?;

        let url_element = document
            .select(&s("h2 a"))
            .filter(|e| e.inner_html() != "<!--$MG_DESCNAVI-->")
            .last()
            .unwrap();

        let url = parse_tucan_url(&format!(
            "https://www.tucan.tu-darmstadt.de{}",
            url_element.value().attr("href").unwrap()
        ));

        let url = match url {
            TucanUrl {
                program: TucanProgram::Registration(r),
                ..
            } => r,
            _ => panic!(),
        };

        let name = url_element.inner_html();
        let _normalized_name = TucanUser::normalize(&name);

        Ok(ModuleMenu {
            tucan_id: url.path,
            tucan_last_checked: Utc::now().naive_utc(),
            name: url_element.inner_html(),
            child_type: 0,
            parent: None,
        })
    }

    pub async fn registration(
        &self,
        url: Registration,
    ) -> anyhow::Result<(ModuleMenu, RegistrationEnum)> {
        use diesel_async::RunQueryDsl;

        // making this here 100% correct is probably not easy as you get different modules depending on when you registered for a module
        // also you can get multiple courses per module
        // you can also get no module but courses (I think we currently don't return these, NEVER FIX THIS BULLSHIT)
        // maybe return highest row for each course_id

        let mut connection = self.tucan.pool.get().await?;

        let existing_registration_already_fetched = module_menu_unfinished::table
            .filter(module_menu_unfinished::tucan_id.eq(&url.path))
            .filter(not(module_menu_unfinished::child_type.eq(0)))
            .get_result::<ModuleMenu>(&mut connection)
            .await
            .optional()?;

        match existing_registration_already_fetched {
            Some(module_menu @ ModuleMenu { child_type: 1, .. }) => {
                debug!("[~] menu {:?}", module_menu);

                // existing submenus
                let submenus = module_menu_unfinished::table
                    .select(module_menu_unfinished::all_columns)
                    .filter(module_menu_unfinished::parent.eq(&url.path))
                    .load::<ModuleMenu>(&mut connection)
                    .await?;

                return Ok((module_menu, RegistrationEnum::Submenu(submenus)));
            }
            Some(module_menu @ ModuleMenu { child_type: 2, .. }) => {
                debug!("[~] menu {:?}", module_menu);

                // existing submodules
                let submodules: Vec<Module> = module_menu_module::table
                    .inner_join(modules_unfinished::table)
                    .select((
                        modules_unfinished::tucan_id,
                        modules_unfinished::tucan_last_checked,
                        modules_unfinished::title,
                        modules_unfinished::module_id,
                        modules_unfinished::credits,
                        modules_unfinished::content,
                        modules_unfinished::done,
                    ))
                    .filter(module_menu_module::module_menu_id.eq(&url.path))
                    .load::<Module>(&mut connection)
                    .await?;

                // TODO FIXME maybe only return the latest course for courses with same course_id
                let module_courses: Vec<(ModuleCourse, Course)> =
                    ModuleCourse::belonging_to(&submodules)
                        .inner_join(courses_unfinished::table)
                        .select((
                            (module_courses::module, module_courses::course),
                            (
                                courses_unfinished::tucan_id,
                                courses_unfinished::tucan_last_checked,
                                courses_unfinished::title,
                                courses_unfinished::course_id,
                                courses_unfinished::sws,
                                courses_unfinished::content,
                                courses_unfinished::done,
                            ),
                        ))
                        .load::<(ModuleCourse, Course)>(&mut connection)
                        .await?;
                let grouped_module_courses: Vec<Vec<(ModuleCourse, Course)>> =
                    module_courses.grouped_by(&submodules);
                let result: Vec<(Option<Module>, Vec<Course>)> = submodules
                    .into_iter()
                    .zip(grouped_module_courses)
                    .map(|(m, r)| (Some(m), r.into_iter().map(|r| r.1).collect_vec()))
                    .collect();

                return Ok((module_menu, RegistrationEnum::ModulesAndCourses(result)));
            }
            _ => {}
        }

        drop(connection);

        let document = self.fetch_document(&url.clone().into()).await?;

        // list of subcategories
        let submenu_list = element_by_selector(&document, "#contentSpacer_IE ul");

        // list of modules
        let modules_list = element_by_selector(&document, "table.tbcoursestatus");

        let url_element = document
            .select(&s("h2 a"))
            .filter(|e| e.inner_html() != "<!--$MG_DESCNAVI-->")
            .last()
            .unwrap();

        let name = url_element.inner_html();
        let _normalized_name = TucanUser::normalize(&name);

        let child_type = match (submenu_list, modules_list) {
            (_, Some(_)) => 2,
            (Some(_), None) => 1,
            _ => panic!(),
        };

        let module_menu = ModuleMenu {
            tucan_id: url.path.clone(),
            tucan_last_checked: Utc::now().naive_utc(),
            name: url_element.inner_html(),
            child_type,
            parent: None,
        };

        debug!("[+] menu {:?}", module_menu);

        let mut connection = self.tucan.pool.get().await?;

        let module_menu = diesel::insert_into(module_menu_unfinished::table)
            .values(&module_menu)
            .on_conflict(module_menu_unfinished::tucan_id)
            .do_update()
            .set(&module_menu) // treat_none_as_null is false so parent should't be overwritten
            // I think there is a bug here when using ModuleMenuChangeset in set() the types are wrong.
            .get_result::<ModuleMenu>(&mut connection)
            .await?;

        let return_value = match (submenu_list, modules_list) {
            (_, Some(list)) => {
                let selector = s(".tbcoursestatus strong a[href]");
                let a = list.select(&selector).fuse().peekable();

                let d = a.batching(|f| {
                    let title = if f.peek()?.value().attr("name") != Some("eventLink") {
                        f.next()
                    } else {
                        None
                    };
                    let sub_elements: Vec<ElementRef> = f
                        .peeking_take_while(|e| e.value().attr("name") == Some("eventLink"))
                        .collect();

                    Some((title, sub_elements))
                });

                let modules: Vec<(Option<Module>, Vec<Course>)> = d
                    .map(|e| {
                        let module = e.0.map(|i| {
                            let mut text = i.text();
                            Module {
                                tucan_id: TryInto::<Moduledetails>::try_into(
                                    parse_tucan_url(&format!(
                                        "https://www.tucan.tu-darmstadt.de{}",
                                        i.value().attr("href").unwrap()
                                    ))
                                    .program,
                                )
                                .unwrap()
                                .id,
                                //expect(&Into::<TucanProgram>::into(url.clone()).to_tucan_url(None))
                                tucan_last_checked: Utc::now().naive_utc(),
                                module_id: text
                                    .next()
                                    .unwrap_or_else(|| panic!("{:?}", i.text().collect::<Vec<_>>()))
                                    .to_string(),
                                title: text
                                    .next()
                                    .unwrap_or_else(|| panic!("{:?}", i.text().collect::<Vec<_>>()))
                                    .to_string(),
                                credits: None,
                                content: "".to_string(),
                                done: false,
                            }
                        });

                        let courses =
                            e.1.into_iter()
                                .map(|course| {
                                    let mut text = course.text();

                                    Course {
                                        tucan_id: TryInto::<Coursedetails>::try_into(
                                            parse_tucan_url(&format!(
                                                "https://www.tucan.tu-darmstadt.de{}",
                                                course.value().attr("href").unwrap()
                                            ))
                                            .program,
                                        )
                                        .unwrap()
                                        .id,
                                        tucan_last_checked: Utc::now().naive_utc(),
                                        course_id: text
                                            .next()
                                            .unwrap_or_else(|| {
                                                panic!("{:?}", course.text().collect::<Vec<_>>())
                                            })
                                            .to_string(),
                                        title: text
                                            .next()
                                            .unwrap_or_else(|| {
                                                panic!("{:?}", course.text().collect::<Vec<_>>())
                                            })
                                            .to_string(),
                                        sws: 0,
                                        content: "".to_string(),
                                        done: false,
                                    }
                                })
                                .collect_vec();

                        (module, courses)
                    })
                    .collect();

                diesel::insert_into(modules_unfinished::table)
                    .values(
                        modules
                            .iter()
                            .map(|m| &m.0)
                            .filter_map(|v| v.as_ref())
                            .collect_vec(),
                    )
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
                    .await?;

                diesel::insert_into(module_menu_module::table)
                    .values(
                        modules
                            .iter()
                            .map(|m| &m.0)
                            .filter_map(|v| v.as_ref())
                            .map(|m| ModuleMenuEntryModuleRef {
                                module_id: &m.tucan_id,
                                module_menu_id: &url.path,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
                    .await?;

                diesel::insert_into(courses_unfinished::table)
                    .values(modules.iter().flat_map(|m| &m.1).collect_vec())
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
                    .await?;

                diesel::insert_into(module_courses::table)
                    .values(
                        modules
                            .iter()
                            .flat_map(|m| m.1.iter().map(|e| (&m.0, e)))
                            .filter_map(|v| v.0.as_ref().map(|v0| (v0, v.1)))
                            .map(|m| ModuleCourse {
                                module: m.0.tucan_id.clone(),
                                course: m.1.tucan_id.clone(),
                            })
                            .collect::<Vec<_>>(),
                    )
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
                    .await?;

                RegistrationEnum::ModulesAndCourses(modules)
            }
            (Some(list), None) => {
                let utc = Utc::now().naive_utc();
                let submenus: Vec<ModuleMenu> = list
                    .select(&s("a[href]"))
                    .map(|e| {
                        let child = TryInto::<Registration>::try_into(
                            parse_tucan_url(&format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                e.value().attr("href").unwrap()
                            ))
                            .program,
                        )
                        .unwrap()
                        .path;

                        ModuleMenu {
                            tucan_id: child,
                            tucan_last_checked: utc,
                            name: e.inner_html().trim().to_string(),
                            child_type: 0,
                            parent: Some(url.path.clone()),
                        }
                    })
                    .collect::<Vec<_>>();

                diesel::insert_into(module_menu_unfinished::table)
                    .values(&submenus[..])
                    .on_conflict(module_menu_unfinished::tucan_id)
                    .do_update()
                    .set(
                        module_menu_unfinished::parent.eq(excluded(module_menu_unfinished::parent)),
                    )
                    .get_result::<ModuleMenu>(&mut connection)
                    .await?;

                RegistrationEnum::Submenu(submenus)
            }
            _ => {
                panic!(
                    "{:?} {} {}",
                    url.clone(),
                    Into::<TucanProgram>::into(url)
                        .to_tucan_url(Some(self.session.session_nr.try_into().unwrap())),
                    document.root_element().html()
                );
            }
        };

        Ok((module_menu, return_value))
    }

    pub async fn my_modules(&self) -> anyhow::Result<Vec<Module>> {
        {
            let mut connection = self.tucan.pool.get().await?;
            let tu_id = self.session.matriculation_number.clone();

            let modules = connection
                .build_transaction()
                .run(|mut connection| {
                    Box::pin(async move {
                        let user_studies_already_fetched = users_unfinished::table
                            .filter(users_unfinished::matriculation_number.eq(&tu_id))
                            .select(users_unfinished::user_modules_last_checked)
                            .get_result::<Option<NaiveDateTime>>(&mut connection)
                            .await?;

                        if user_studies_already_fetched.is_some() {
                            Ok::<Option<Vec<Module>>, diesel::result::Error>(Some(
                                user_modules::table
                                    .filter(user_modules::user_id.eq(&tu_id))
                                    .inner_join(modules_unfinished::table)
                                    .select((
                                        modules_unfinished::tucan_id,
                                        modules_unfinished::tucan_last_checked,
                                        modules_unfinished::title,
                                        modules_unfinished::module_id,
                                        modules_unfinished::credits,
                                        modules_unfinished::content,
                                        modules_unfinished::done,
                                    ))
                                    .load::<Module>(&mut connection)
                                    .await?,
                            ))
                        } else {
                            Ok(None)
                        }
                    })
                })
                .await?;

            if let Some(modules) = modules {
                return Ok(modules);
            }
        }

        let document = self.fetch_document(&Mymodules.clone().into()).await?;

        let my_modules = document
            .select(&s("tbody tr a"))
            .map(|link| {
                TryInto::<Moduledetails>::try_into(
                    parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        link.value().attr("href").unwrap()
                    ))
                    .program,
                )
                .unwrap()
            })
            .map(|moduledetails| self.module(moduledetails))
            .collect::<FuturesUnordered<_>>();

        let results: Vec<anyhow::Result<(Module, Vec<Course>)>> = my_modules.collect().await;

        let results: anyhow::Result<Vec<(Module, Vec<Course>)>> = results.into_iter().collect();

        let results: Vec<(Module, Vec<Course>)> = results?;

        let my_user_studies = results
            .iter()
            .map(|(m, _cs)| UserModule {
                user_id: self.session.matriculation_number.clone(),
                module_id: m.tucan_id.clone(),
            })
            .collect::<Vec<_>>();

        use diesel_async::RunQueryDsl;

        {
            let mut connection = self.tucan.pool.get().await?;

            let matriculation_number = self.session.matriculation_number.clone();
            connection
                .build_transaction()
                .run(|mut connection| {
                    Box::pin(async move {
                        diesel::insert_into(user_modules::table)
                            .values(my_user_studies)
                            .on_conflict((user_modules::user_id, user_modules::module_id))
                            .do_nothing()
                            .execute(&mut connection)
                            .await?;

                        diesel::update(users_unfinished::table)
                            .filter(users_unfinished::matriculation_number.eq(matriculation_number))
                            .set(
                                users_unfinished::user_modules_last_checked
                                    .eq(Utc::now().naive_utc()),
                            )
                            .execute(&mut connection)
                            .await?;

                        Ok::<(), diesel::result::Error>(())
                    })
                })
                .await?;
        }

        Ok(results.into_iter().map(|r| r.0).collect())
    }

    pub async fn my_courses(&self) -> anyhow::Result<Vec<Course>> {
        {
            let mut connection = self.tucan.pool.get().await?;
            let matriculation_number = self.session.matriculation_number.clone();

            let courses = connection
                .build_transaction()
                .run(|mut connection| {
                    Box::pin(async move {
                        let user_courses_already_fetched = users_unfinished::table
                            .filter(
                                users_unfinished::matriculation_number.eq(&matriculation_number),
                            )
                            .select(users_unfinished::user_courses_last_checked)
                            .get_result::<Option<NaiveDateTime>>(&mut connection)
                            .await?;

                        if user_courses_already_fetched.is_some() {
                            Ok::<Option<Vec<Course>>, diesel::result::Error>(Some(
                                user_courses::table
                                    .filter(user_courses::user_id.eq(&matriculation_number))
                                    .inner_join(courses_unfinished::table)
                                    .select((
                                        courses_unfinished::tucan_id,
                                        courses_unfinished::tucan_last_checked,
                                        courses_unfinished::title,
                                        courses_unfinished::course_id,
                                        courses_unfinished::sws,
                                        courses_unfinished::content,
                                        courses_unfinished::done,
                                    ))
                                    .load::<Course>(&mut connection)
                                    .await?,
                            ))
                        } else {
                            Ok(None)
                        }
                    })
                })
                .await?;

            if let Some(courses) = courses {
                return Ok(courses);
            }
        }

        let document = self.fetch_document(&Profcourses.clone().into()).await?;

        let my_courses = document
            .select(&s("tbody tr a"))
            .map(|link| {
                TryInto::<Coursedetails>::try_into(
                    parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        link.value().attr("href").unwrap()
                    ))
                    .program,
                )
                .unwrap()
            })
            .map(|details| self.course_or_course_group(details))
            .collect::<FuturesUnordered<_>>();

        let results: Vec<anyhow::Result<CourseOrCourseGroup>> = my_courses.collect().await;

        let results: anyhow::Result<Vec<CourseOrCourseGroup>> = results.into_iter().collect();

        let results: Vec<Course> = results?
            .into_iter()
            .filter_map(|v| match v {
                CourseOrCourseGroup::Course(course) => Some(course),
                CourseOrCourseGroup::CourseGroup(_) => None,
            })
            .collect_vec();

        let my_user_studies = results
            .iter()
            .map(|c| UserCourse {
                user_id: self.session.matriculation_number.clone(),
                course_id: c.tucan_id.clone(),
            })
            .collect::<Vec<_>>();

        use diesel_async::RunQueryDsl;

        {
            let mut connection = self.tucan.pool.get().await?;

            let tu_id = self.session.matriculation_number.clone();
            connection
                .build_transaction()
                .run(|mut connection| {
                    Box::pin(async move {
                        diesel::insert_into(user_courses::table)
                            .values(my_user_studies)
                            .on_conflict((user_courses::user_id, user_courses::course_id))
                            .do_nothing()
                            .execute(&mut connection)
                            .await?;

                        diesel::update(users_unfinished::table)
                            .filter(users_unfinished::matriculation_number.eq(tu_id))
                            .set(
                                users_unfinished::user_courses_last_checked
                                    .eq(Utc::now().naive_utc()),
                            )
                            .execute(&mut connection)
                            .await?;

                        Ok::<(), diesel::result::Error>(())
                    })
                })
                .await?;
        }

        Ok(results)
    }

    pub async fn personal_data(&self) -> anyhow::Result<UndoneUser> {
        let document = self.fetch_document(&Persaddress.clone().into()).await?;

        let matriculation_number: i32 = document
            .select(&s(r#"td[name="matriculationNumber"]"#))
            .next()
            .unwrap()
            .inner_html()
            .trim()
            .parse()
            .unwrap();

        Ok(UndoneUser::new(matriculation_number))
    }
}
