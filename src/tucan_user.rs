use std::{
    convert::TryInto,
    io::{Error, ErrorKind},
};

use chrono::Utc;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::HeaderValue;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::{
    element_by_selector,
    models::{
        Course, Module, ModuleCourse, ModuleMenu, ModuleMenuEntryModuleRef, ModuleMenuTreeEntry,
    },
    s,
    tucan::Tucan,
    url::{
        parse_tucan_url, Coursedetails, Moduledetails, Registration, RootRegistration,
        TucanProgram, TucanUrl,
    },
};

use crate::schema::*;
use diesel::dsl::not;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use log::{debug, error, trace};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TucanSession {
    pub nr: u64,
    pub id: String,
}

#[derive(Clone)]
pub struct TucanUser {
    pub tucan: Tucan,
    pub session: TucanSession,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum RegistrationEnum {
    Submenu(Vec<ModuleMenu>),
    Modules(Vec<Module>),
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
        let cookie = format!("cnsc={}", self.session.id);

        let a = self
            .tucan
            .client
            .get(url.to_tucan_url(Some(self.session.nr)));
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
            .select((modules_unfinished::tucan_id,modules_unfinished::tucan_last_checked,modules_unfinished::title,modules_unfinished::module_id,modules_unfinished::credits,modules_unfinished::content,modules_unfinished::done,))
            .get_result::<Module>(&mut connection)
            .await
            .optional()?;

        if let Some(existing_module) = existing_module {
            trace!("[~] module {:?}", existing_module);

            let course_list = ModuleCourse::belonging_to(&existing_module)
                .inner_join(courses_unfinished::table)
                .select((courses_unfinished::tucan_id, courses_unfinished::tucan_last_checked, courses_unfinished::title, courses_unfinished::course_id, courses_unfinished::sws, courses_unfinished::content, courses_unfinished::done))
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
            .expect(&document.root_element().inner_html())
            .inner_html();

        let courses = document
            .select(&s(r#"a[name="eventLink"]"#))
            .map(|c| Course {
                tucan_id: TryInto::<Coursedetails>::try_into(
                    parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        c.value().attr("href").unwrap()
                    ))
                    .program,
                )
                .unwrap()
                .id,
                tucan_last_checked: Utc::now().naive_utc(),
                title: "TODO".to_string(),
                course_id: "TODO".to_string(),
                sws: 0,
                content: "TODO".to_string(),
                done: false,
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

        trace!("[+] module {:?}", module);

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

    pub async fn course(&self, url: Coursedetails) -> anyhow::Result<Course> {
        use diesel_async::RunQueryDsl;

        let mut connection = self.tucan.pool.get().await?;

        let existing = courses_unfinished::table
            .filter(courses_unfinished::tucan_id.eq(&url.id))
            .filter(courses_unfinished::done)
            .select((courses_unfinished::tucan_id, courses_unfinished::tucan_last_checked, courses_unfinished::title, courses_unfinished::course_id, courses_unfinished::sws, courses_unfinished::content, courses_unfinished::done))
            .get_result::<Course>(&mut connection)
            .await
            .optional()?;

        drop(connection);

        if let Some(existing) = existing {
            debug!("[~] course {:?}", existing);
            return Ok(existing);
        }

        let document = self.fetch_document(&url.clone().into()).await?;

        let name = element_by_selector(&document, "h1").unwrap();

        let text = name.inner_html();
        let mut fs = text.trim().split("\n");
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
            .expect(&document.root_element().inner_html())
            .inner_html();

        let course = Course {
            tucan_id: url.id,
            tucan_last_checked: Utc::now().naive_utc(),
            title: course_name.unwrap().to_string(),
            sws,
            course_id: TucanUser::normalize(course_id),
            content,
            done: true,
        };

        debug!("[+] course {:?}", course);

        let mut connection = self.tucan.pool.get().await?;

        diesel::insert_into(courses_unfinished::table)
            .values(&course)
            .on_conflict(courses_unfinished::tucan_id)
            .do_update()
            .set(&course)
            .execute(&mut connection)
            .await?;

        Ok(course)
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
        let normalized_name = TucanUser::normalize(&name);

        Ok(ModuleMenu {
            tucan_id: url.path,
            tucan_last_checked: Utc::now().naive_utc(),
            name: url_element.inner_html(),
            normalized_name,
            child_type: 0,
        })
    }

    pub async fn registration(
        &self,
        url: Registration,
    ) -> anyhow::Result<(ModuleMenu, RegistrationEnum)> {
        use diesel_async::RunQueryDsl;

        let mut connection = self.tucan.pool.get().await?;

        let existing_registration_already_fetched = module_menu_unfinished::table
            .filter(module_menu_unfinished::tucan_id.eq(&url.path))
            .filter(not(module_menu_unfinished::child_type.eq(0)))
            .get_result::<ModuleMenu>(&mut connection)
            .await
            .optional()?;

        match existing_registration_already_fetched {
            Some(module_menu @ ModuleMenu { child_type: 1, .. }) => {
                trace!("[~] menu {:?}", module_menu);

                // existing submenus
                let submenus = module_menu_unfinished::table
                    .inner_join(
                        module_menu_tree::table
                            .on(module_menu_tree::child.eq(module_menu_unfinished::tucan_id)),
                    )
                    .select(module_menu_unfinished::all_columns)
                    .filter(module_menu_tree::parent.eq(&url.path))
                    .load::<ModuleMenu>(&mut connection)
                    .await?;

                return Ok((module_menu, RegistrationEnum::Submenu(submenus)));
            }
            Some(module_menu @ ModuleMenu { child_type: 2, .. }) => {
                trace!("[~] menu {:?}", module_menu);

                // existing submodules
                let submodules = module_menu_module::table
                    .inner_join(modules_unfinished::table)
                    .select((modules_unfinished::tucan_id,modules_unfinished::tucan_last_checked,modules_unfinished::title,modules_unfinished::module_id,modules_unfinished::credits,modules_unfinished::content,modules_unfinished::done,))
                    .filter(module_menu_module::module_menu_id.eq(&url.path))
                    .load::<Module>(&mut connection)
                    .await?;

                return Ok((module_menu, RegistrationEnum::Modules(submodules)));
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
        let normalized_name = TucanUser::normalize(&name);

        let child_type = match (submenu_list, modules_list) {
            (_, Some(_)) => 2,
            (Some(_), None) => 1,
            _ => panic!(),
        };

        // ModuleMenuRef?
        let module_menu = ModuleMenu {
            tucan_id: url.path.clone(),
            tucan_last_checked: Utc::now().naive_utc(),
            name: url_element.inner_html(),
            normalized_name,
            child_type,
        };

        trace!("[+] menu {:?}", module_menu);

        let mut connection = self.tucan.pool.get().await?;

        diesel::insert_into(module_menu_unfinished::table)
            .values(&module_menu)
            .on_conflict(module_menu_unfinished::tucan_id)
            .do_update()
            .set(&module_menu) // we don't override parent because it's set as optional and therefore not overwritten
            .get_result::<ModuleMenu>(&mut connection)
            .await?;

        let return_value = match (submenu_list, modules_list) {
            (_, Some(list)) => {
                let modules: Vec<Module> = list
                    .select(&s(r#"td.tbsubhead.dl-inner a[href]"#))
                    .map(|e| Module {
                        tucan_id: TryInto::<Moduledetails>::try_into(
                            parse_tucan_url(&format!(
                                "https://www.tucan.tu-darmstadt.de{}",
                                e.value().attr("href").unwrap()
                            ))
                            .program,
                        )
                        .unwrap()
                        .id,
                        tucan_last_checked: Utc::now().naive_utc(),
                        title: "TODO".to_string(),
                        module_id: "TODO".to_string(),
                        credits: None,
                        content: "TODO".to_string(),
                        done: false,
                    })
                    .collect();

                diesel::insert_into(modules_unfinished::table)
                    .values(&modules[..])
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
                    .await?;

                diesel::insert_into(module_menu_module::table)
                    .values(
                        modules
                            .iter()
                            .map(|m| ModuleMenuEntryModuleRef {
                                module_id: &m.tucan_id,
                                module_menu_id: &url.path,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
                    .await?;

                RegistrationEnum::Modules(modules)
            }
            (Some(list), None) => {
                let utc = Utc::now().naive_utc();
                let (submenus, menu_tree): (Vec<ModuleMenu>, Vec<ModuleMenuTreeEntry>) = list
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
                        (
                            ModuleMenu {
                                tucan_id: child.clone(),
                                tucan_last_checked: utc,
                                name: "TODO".to_string(),
                                normalized_name: "TODO".to_string(),
                                child_type: 0,
                            },
                            ModuleMenuTreeEntry {
                                parent: url.path.clone(),
                                child,
                            },
                        )
                    })
                    .unzip();

                diesel::insert_into(module_menu_unfinished::table)
                    .values(&submenus[..])
                    .on_conflict(module_menu_unfinished::tucan_id)
                    .do_nothing()
                    .get_result::<ModuleMenu>(&mut connection)
                    .await?;

                diesel::insert_into(module_menu_tree::table)
                    .values(&menu_tree[..])
                    .on_conflict((module_menu_tree::child, module_menu_tree::parent))
                    .do_nothing()
                    .get_result::<ModuleMenuTreeEntry>(&mut connection)
                    .await?;

                RegistrationEnum::Submenu(submenus)
            }
            _ => {
                panic!(
                    "{:?} {} {}",
                    url.clone(),
                    Into::<TucanProgram>::into(url).to_tucan_url(Some(self.session.nr)),
                    document.root_element().html()
                );
            }
        };

        Ok((module_menu, return_value))
    }
}
