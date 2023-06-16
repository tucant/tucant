// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use diesel::prelude::RunQueryDsl;
use std::{collections::HashMap, convert::TryInto};

use crate::{
    models::{
        self, CompleteCourse, CompleteModule, CourseEvent, CourseExam, CourseGroup,
        CourseGroupEvent, Exam, MaybeCompleteCourse, MaybeCompleteModule, ModuleCourse, ModuleExam,
        ModuleExamType, ModuleMenu, ModuleMenuEntryModule, PartialCourse, PartialModule,
        UndoneUser, UserCourseGroup, UserExam, COURSES_UNFINISHED, MODULES_UNFINISHED,
    },
    schema::course_groups_unfinished,
    tucan::{s, Authenticated, Tucan, Unauthenticated},
    url::{
        parse_tucan_url, Coursedetails, Courseresults, Examdetails, Examresults, Moduledetails,
        Myexams, Mymodules, Persaddress, Registration, RootRegistration, TucanProgram, TucanUrl, Semester,
    },
};
use crate::{
    models::{UserCourse, UserModule},
    url::Profcourses,
};
use chrono::{NaiveDateTime, Utc};

use either::Either;
use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use once_cell::sync::Lazy;

use scraper::ElementRef;
use serde::{Deserialize, Serialize};
use tucant_derive::Typescriptable;

use crate::schema::{
    course_exams, courses_unfinished, exams_unfinished, module_courses, module_exams,
    module_menu_module, module_menu_unfinished, modules_unfinished, user_course_groups,
    user_courses, user_exams, user_modules, users_unfinished,
};
use diesel::ExpressionMethods;

use base64::prelude::{Engine, BASE64_URL_SAFE_NO_PAD};
use diesel::upsert::excluded;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use log::debug;

#[derive(Debug, Typescriptable, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum CourseOrCourseGroup {
    Course(
        (
            CompleteCourse,
            Vec<CourseGroup>,
            Vec<CourseEvent>,
            Vec<MaybeCompleteModule>,
        ),
    ),
    CourseGroup((CourseGroup, Vec<CourseGroupEvent>)),
}

static TUCANSCHEISS: Lazy<CompleteModule> = Lazy::new(|| CompleteModule {
    tucan_id: BASE64_URL_SAFE_NO_PAD.decode("TUCANSCHEISS").unwrap(),
    tucan_last_checked: Utc::now().naive_utc(),
    title: "TUCANSCHEISS".to_string(),
    module_id: "TUCANSCHEISS".to_string(),
    credits: 0,
    content: "TUCANSCHEISS".to_string(),
});

impl Tucan<Authenticated> {
    #[must_use]
    pub fn as_unauthenticated(&self) -> Tucan<Unauthenticated> {
        Tucan {
            pool: self.pool.clone(),
            client: self.client.clone(),
            semaphore: self.semaphore.clone(),
            state: Unauthenticated {},
        }
    }

    pub async fn root_registration(&self) -> anyhow::Result<ModuleMenu> {
        // TODO FIXME cache this

        let document = self.fetch_document(&RootRegistration {}.into()).await?;

        let document = Self::parse_document(&document);

        let url_element = document
            .select(&s("h2 a"))
            .filter(|e| e.inner_html() != "<!--$MG_DESCNAVI-->")
            .last()
            .unwrap();

        let url = parse_tucan_url(&format!(
            "https://www.tucan.tu-darmstadt.de{}",
            url_element.value().attr("href").unwrap()
        ));

        let TucanUrl {
            program: TucanProgram::Registration(url),
            ..
        } = url else { panic!() };

        Ok(ModuleMenu {
            tucan_id: url.path,
            tucan_last_checked: Utc::now().naive_utc(),
            name: url_element.inner_html(),
            done: false,
            parent: None,
        })
    }

    async fn cached_registration(
        &self,
        url: Registration,
    ) -> anyhow::Result<Option<(ModuleMenu, crate::models::Registration)>> {
        // making this here 100% correct is probably not easy as you get different modules depending on when you registered for a module
        // also you can get multiple courses per module
        // you can also get no module but courses (I think we currently don't return these, NEVER FIX THIS BULLSHIT)
        // maybe return highest row for each course_id

        let mut connection = self.pool.get()?;

        let existing_registration_already_fetched = module_menu_unfinished::table
            .filter(module_menu_unfinished::tucan_id.eq(&url.path))
            .filter(module_menu_unfinished::done)
            .get_result::<ModuleMenu>(&mut connection)
            .optional()?;

        if let Some(module_menu) = existing_registration_already_fetched {
            debug!("[~] menu {:?}", module_menu);

            // existing submenus
            let submenus = module_menu_unfinished::table
                .select(module_menu_unfinished::all_columns)
                .filter(module_menu_unfinished::parent.eq(&url.path))
                .order(module_menu_unfinished::name)
                .load::<ModuleMenu>(&mut connection)?;

            // existing submodules
            let submodules: Vec<MaybeCompleteModule> = module_menu_module::table
                .inner_join(modules_unfinished::table)
                .select(MODULES_UNFINISHED)
                .order(modules_unfinished::title)
                .filter(module_menu_module::module_menu_id.eq(&url.path))
                .load::<MaybeCompleteModule>(&mut connection)?;

            // TODO FIXME maybe only return the latest course for courses with same course_id
            let module_courses: Vec<(ModuleCourse, MaybeCompleteCourse)> = module_courses::table
                .filter(
                    module_courses::module
                        .eq_any(submodules.iter().map(MaybeCompleteModule::tucan_id)),
                )
                .inner_join(courses_unfinished::table)
                .select((
                    (module_courses::module, module_courses::course),
                    COURSES_UNFINISHED,
                ))
                .order(courses_unfinished::title)
                .load::<(ModuleCourse, MaybeCompleteCourse)>(&mut connection)?;

            let id_indices: HashMap<_, _> = submodules
                .iter()
                .enumerate()
                .map(|(i, u)| (u.tucan_id(), i))
                .collect();
            let mut grouped_module_courses =
                submodules.iter().map(|_| Vec::new()).collect::<Vec<_>>();
            for child in module_courses {
                grouped_module_courses[id_indices[&child.0.module]].push(child);
            }

            let modules_and_courses: Vec<(MaybeCompleteModule, Vec<MaybeCompleteCourse>)> =
                submodules
                    .into_iter()
                    .zip(grouped_module_courses)
                    .map(|(m, r)| (m, r.into_iter().map(|r| r.1).collect::<Vec<_>>()))
                    .collect();

            Ok(Some((
                module_menu,
                crate::models::Registration {
                    submenus,
                    modules_and_courses,
                },
            )))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unused_peekable)]
    pub async fn fetch_registration(&self, url: Registration) -> anyhow::Result<()> {
        let document = self.fetch_document(&url.clone().into()).await?;
        let mut connection = self.pool.get()?;

        let (module_menu, submenus, modules) = {
            let document = Self::parse_document(&document);

            let (_name, module_menu) = {
                let url_element = document
                    .select(&s("h2 a"))
                    .filter(|e| e.inner_html() != "<!--$MG_DESCNAVI-->")
                    .last()
                    .unwrap();

                (
                    url_element.inner_html(),
                    ModuleMenu {
                        tucan_id: url.path.clone(),
                        tucan_last_checked: Utc::now().naive_utc(),
                        name: url_element.inner_html(),
                        done: false,
                        parent: None,
                    },
                )
            };

            debug!("[+] menu {:?}", module_menu);

            let selector = s("table.tbcoursestatus strong a[href]");

            let a = document.select(&selector).fuse().peekable();

            let d = a.batching(|f| {
                let title = if f.peek()?.value().attr("name") == Some("eventLink") {
                    None
                } else {
                    f.next()
                };
                let sub_elements: Vec<ElementRef> = f
                    .peeking_take_while(|e| e.value().attr("name") == Some("eventLink"))
                    .collect();

                Some((title, sub_elements))
            });

            let modules: Vec<(MaybeCompleteModule, Vec<MaybeCompleteCourse>)> = d
                .map(|e| {
                    let module = e.0.map_or_else(
                        || MaybeCompleteModule::Complete(TUCANSCHEISS.clone()),
                        |i| {
                            let mut text = i.text();
                            MaybeCompleteModule::Partial(PartialModule {
                                tucan_id: TryInto::<Moduledetails>::try_into(
                                    parse_tucan_url(&format!(
                                        "https://www.tucan.tu-darmstadt.de{}",
                                        i.value().attr("href").unwrap()
                                    ))
                                    .program,
                                )
                                .unwrap()
                                .id,
                                tucan_last_checked: Utc::now().naive_utc(),
                                module_id: text
                                    .next()
                                    .unwrap_or_else(|| panic!("{:?}", i.text().collect::<Vec<_>>()))
                                    .to_string(),
                                title: text
                                    .next()
                                    .unwrap_or_else(|| panic!("{:?}", i.text().collect::<Vec<_>>()))
                                    .to_string(),
                            })
                        },
                    );

                    let courses =
                        e.1.into_iter()
                            .map(|course| {
                                let mut text = course.text();

                                MaybeCompleteCourse::Partial(PartialCourse {
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
                                })
                            })
                            .collect::<Vec<_>>();

                    (module, courses)
                })
                .collect();

            let utc = Utc::now().naive_utc();
            let submenus: Vec<ModuleMenu> = document
                .select(&s("#contentSpacer_IE ul a[href]"))
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
                        done: false,
                        parent: Some(url.path.clone()),
                    }
                })
                .collect::<Vec<_>>();

            (module_menu, submenus, modules)
        };

        diesel::insert_into(module_menu_unfinished::table)
            .values(&module_menu)
            .on_conflict(module_menu_unfinished::tucan_id)
            .do_update()
            .set(&module_menu) // TODO FIXME this is broken now - treat_none_as_null is false so parent should't be overwritten
            .get_result::<ModuleMenu>(&mut connection)?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = modules
            .iter()
            .map(|m| &m.0)
            .map(|module| -> Result<_, _> {
                diesel::insert_into(modules_unfinished::table)
                    .values(module)
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = modules
            .iter()
            .map(|m| &m.0)
            .map(|m| ModuleMenuEntryModule {
                module_id: m.tucan_id().clone(),
                module_menu_id: url.path.clone(),
            })
            .map(|module| -> Result<_, _> {
                diesel::insert_into(module_menu_module::table)
                    .values(module)
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = modules
            .iter()
            .flat_map(|m| &m.1)
            .map(|module| -> Result<_, _> {
                diesel::insert_into(courses_unfinished::table)
                    .values(module)
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = modules
            .into_iter()
            .flat_map(|m| m.1.into_iter().map(move |e| (m.0.clone(), e)))
            .map(|m| ModuleCourse {
                module: m.0.tucan_id().clone(),
                course: m.1.tucan_id().clone(),
            })
            .map(|module| -> Result<_, _> {
                diesel::insert_into(module_courses::table)
                    .values(module)
                    .on_conflict_do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = submenus
            .iter()
            .map(|submenu| -> Result<_, _> {
                diesel::insert_into(module_menu_unfinished::table)
                    .values(submenu)
                    .on_conflict(module_menu_unfinished::tucan_id)
                    .do_update()
                    .set(
                        module_menu_unfinished::parent.eq(excluded(module_menu_unfinished::parent)),
                    )
                    .execute(&mut connection)
            })
            .collect();
        res?;

        diesel::update(module_menu_unfinished::table)
            .filter(module_menu_unfinished::tucan_id.eq(url.path.clone()))
            .set(module_menu_unfinished::done.eq(true))
            .execute(&mut connection)?;

        Ok(())
    }

    pub async fn registration(
        &self,
        url: Registration,
    ) -> anyhow::Result<(ModuleMenu, crate::models::Registration)> {
        if let Some(value) = self.cached_registration(url.clone()).await? {
            return Ok(value);
        }

        self.fetch_registration(url.clone()).await?;

        Ok(self.cached_registration(url.clone()).await?.unwrap())
    }

    async fn cached_my_modules(&self) -> anyhow::Result<Option<Vec<MaybeCompleteModule>>> {
        let mut connection = self.pool.get()?;
        let tu_id = self.state.session.matriculation_number;

        let modules = {
            let user_studies_already_fetched = users_unfinished::table
                .filter(users_unfinished::matriculation_number.eq(&tu_id))
                .select(users_unfinished::user_modules_last_checked)
                .get_result::<Option<NaiveDateTime>>(&mut connection)?;

            if user_studies_already_fetched.is_some() {
                Some(
                    user_modules::table
                        .filter(user_modules::user_id.eq(&tu_id))
                        .inner_join(modules_unfinished::table)
                        .select(MODULES_UNFINISHED)
                        .order(modules_unfinished::title)
                        .load::<MaybeCompleteModule>(&mut connection)?,
                )
            } else {
                None
            }
        };

        Ok(modules)
    }

    async fn fetch_my_modules(&self) -> anyhow::Result<()> {
        let document = self.fetch_document(&Mymodules.clone().into()).await?;
        let my_modules = {
            let document = Self::parse_document(&document);

            document
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
                // TODO FIXME insert partial module
                .map(|moduledetails| self.module(moduledetails))
                .collect::<FuturesUnordered<_>>()
        };

        #[allow(clippy::type_complexity)]
        let results: Vec<
            anyhow::Result<(
                CompleteModule,
                Vec<MaybeCompleteCourse>,
                Vec<ModuleExamType>,
            )>,
        > = my_modules.collect().await;

        #[allow(clippy::type_complexity)]
        let results: anyhow::Result<
            Vec<(
                CompleteModule,
                Vec<MaybeCompleteCourse>,
                Vec<ModuleExamType>,
            )>,
        > = results.into_iter().collect();

        let results: Vec<(
            CompleteModule,
            Vec<MaybeCompleteCourse>,
            Vec<ModuleExamType>,
        )> = results?;

        let my_user_studies = results
            .iter()
            .map(|(m, _cs, _)| UserModule {
                user_id: self.state.session.matriculation_number,
                module_id: m.tucan_id.clone(),
            })
            .collect::<Vec<_>>();

        let mut connection = self.pool.get()?;

        let matriculation_number = self.state.session.matriculation_number;
        {
            // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
            let res: Result<Vec<usize>, _> = my_user_studies
                .iter()
                .map(|my_user_study| -> Result<_, _> {
                    diesel::insert_into(user_modules::table)
                        .values(my_user_study)
                        .on_conflict((user_modules::user_id, user_modules::module_id))
                        .do_nothing()
                        .execute(&mut connection)
                })
                .collect();
            res?;

            diesel::update(users_unfinished::table)
                .filter(users_unfinished::matriculation_number.eq(matriculation_number))
                .set(users_unfinished::user_modules_last_checked.eq(Utc::now().naive_utc()))
                .execute(&mut connection)?;
        }

        Ok(())
    }

    pub async fn my_modules(&self) -> anyhow::Result<Vec<MaybeCompleteModule>> {
        if let Some(value) = self.cached_my_modules().await? {
            return Ok(value);
        }

        self.fetch_my_modules().await?;

        Ok(self.cached_my_modules().await?.unwrap())
    }

    async fn cached_my_courses(
        &self,
    ) -> anyhow::Result<Option<(Vec<MaybeCompleteCourse>, Vec<CourseGroup>)>> {
        let mut connection = self.pool.get()?;
        let matriculation_number = self.state.session.matriculation_number;

        Ok({
            let user_courses_already_fetched = users_unfinished::table
                .filter(users_unfinished::matriculation_number.eq(&matriculation_number))
                .select(users_unfinished::user_courses_last_checked)
                .get_result::<Option<NaiveDateTime>>(&mut connection)?;

            if user_courses_already_fetched.is_some() {
                Some((
                    user_courses::table
                        .filter(user_courses::user_id.eq(&matriculation_number))
                        .inner_join(courses_unfinished::table)
                        .select(COURSES_UNFINISHED)
                        .order(courses_unfinished::title)
                        .load(&mut connection)?,
                    user_course_groups::table
                        .filter(user_course_groups::user_id.eq(&matriculation_number))
                        .inner_join(course_groups_unfinished::table)
                        .select(course_groups_unfinished::all_columns)
                        .order(course_groups_unfinished::title)
                        .load(&mut connection)?,
                ))
            } else {
                None
            }
        })
    }

    pub async fn fetch_my_courses(&self) -> anyhow::Result<()> {
        let document = self.fetch_document(&Profcourses {
            semester: Semester::AllSemesters
        }.clone().into()).await?;
        let my_courses = {
            let document = Self::parse_document(&document);

            document
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
                // TODO FIXME make this lazy
                .map(|details| self.course_or_course_group(details))
                .collect::<FuturesUnordered<_>>()
        };

        let results: Vec<anyhow::Result<CourseOrCourseGroup>> = my_courses.collect().await;

        let results: anyhow::Result<Vec<CourseOrCourseGroup>> = results.into_iter().collect();

        let courses_or_course_groups = results?;

        let my_user_studies: (Vec<_>, Vec<_>) =
            courses_or_course_groups
                .iter()
                .partition_map(|value| match value {
                    CourseOrCourseGroup::Course(c) => Either::Left(UserCourse {
                        user_id: self.state.session.matriculation_number,
                        course_id: c.0.tucan_id.clone(),
                    }),
                    CourseOrCourseGroup::CourseGroup(cg) => Either::Right(UserCourseGroup {
                        user_id: self.state.session.matriculation_number,
                        course_group_id: cg.0.tucan_id.clone(),
                    }),
                });

        {
            let mut connection = self.pool.get()?;

            let tu_id = self.state.session.matriculation_number;
            {
                // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
                let res: Result<Vec<usize>, _> = my_user_studies
                    .0
                    .iter()
                    .map(|my_user_study| -> Result<_, _> {
                        diesel::insert_into(user_courses::table)
                            .values(my_user_study)
                            .on_conflict((user_courses::user_id, user_courses::course_id))
                            .do_nothing()
                            .execute(&mut connection)
                    })
                    .collect();
                res?;

                // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
                let res: Result<Vec<usize>, _> = my_user_studies
                    .1
                    .iter()
                    .map(|my_user_study| -> Result<_, _> {
                        diesel::insert_into(user_course_groups::table)
                            .values(my_user_study)
                            .on_conflict((
                                user_course_groups::user_id,
                                user_course_groups::course_group_id,
                            ))
                            .do_nothing()
                            .execute(&mut connection)
                    })
                    .collect();
                res?;

                diesel::update(users_unfinished::table)
                    .filter(users_unfinished::matriculation_number.eq(tu_id))
                    .set(users_unfinished::user_courses_last_checked.eq(Utc::now().naive_utc()))
                    .execute(&mut connection)?;
            }
        }

        Ok(())
    }

    pub async fn my_courses(&self) -> anyhow::Result<(Vec<MaybeCompleteCourse>, Vec<CourseGroup>)> {
        // TODO FIXME for integrated courses only the groups is returned but we should probably also return a course entry
        if let Some(value) = self.cached_my_courses().await? {
            return Ok(value);
        }

        self.fetch_my_courses().await?;

        Ok(self.cached_my_courses().await?.unwrap())
    }

    pub async fn root_module_results(&self) -> anyhow::Result<Vec<u64>> {
        let document = self
            .fetch_document(&Courseresults { semester: None }.clone().into())
            .await?;
        let document = Self::parse_document(&document);

        let semesters = document
            .select(&s("#semester option"))
            .map(|v| v.value().attr("value").unwrap().to_owned().parse().unwrap())
            .collect_vec();

        Ok(semesters)
    }

    pub async fn module_results(&self, semester: u64) -> anyhow::Result<()> {
        let modules = self.my_modules().await?;

        let document = self
            .fetch_document(
                &Courseresults {
                    semester: Some(semester),
                }
                .clone()
                .into(),
            )
            .await?;
        let document = Self::parse_document(&document);

        let rows_selector = s("table.nb.list tbody tr");
        let rows = document.select(&rows_selector);

        rows.map(|row| {
            let cols_selector = s("td");
            let mut cols = row.select(&cols_selector);
            let first_col = cols.next();
            first_col?;
            let nr = first_col.unwrap().inner_html().trim().to_owned();
            let module_name = cols.next().unwrap().inner_html().trim().to_owned();
            let grade = cols.next().unwrap().inner_html().trim().to_owned();
            let credits = cols.next().unwrap().inner_html().trim().to_owned();
            let status = cols.next().unwrap().inner_html().trim().to_owned();
            println!("{nr} {module_name} {grade} {credits} {status}");

            let module = modules.iter().find(|m| m.module_id() == &nr).unwrap();
            println!("{:?}", module.tucan_id());

            Some(())
        })
        .collect_vec();

        Ok(())
    }

    pub async fn course_results(&self) -> anyhow::Result<()> {
        let modules = self.my_modules().await?;
        let courses = self.my_courses().await?;

        let document = self
            .fetch_document(
                &Examresults {
                    semester: Some(999),
                }
                .clone()
                .into(),
            )
            .await?;
        let document = Self::parse_document(&document);

        let rows_selector = s("table.nb.list tbody tr");
        let rows = document.select(&rows_selector);

        rows.map(|row| {
            let cols_selector = s("td");
            let mut cols = row.select(&cols_selector);
            let mut name_parts = cols.next().unwrap().text();
            let a = name_parts.next().unwrap().trim().to_owned();
            let (module_id, text) = a.split_once(' ').unwrap();
            let module_id = module_id.trim();
            let text = text.trim();
            let b = name_parts.next().unwrap().trim().to_owned();
            let date = cols.next().unwrap().inner_html().trim().to_owned();
            let grade = cols.next().unwrap().inner_html().trim().to_owned();
            let grade_text = cols.next().unwrap().inner_html().trim().to_owned();
            println!("|{module_id}|{text}|{b}| {date} {grade} {grade_text}");

            let module = modules.iter().find(|m| m.module_id() == module_id);
            let course = courses.0.iter().find(|c| c.course_id() == module_id);
            //let course_group = courses.1.iter().find(|c| c.course_id == &module_id);
            println!("{:?}", module.map(models::MaybeCompleteModule::tucan_id));
            println!("{:?}", course.map(models::MaybeCompleteCourse::tucan_id));
        })
        .collect_vec();

        Ok(())
    }

    pub async fn personal_data(&self) -> anyhow::Result<UndoneUser> {
        let document = self.fetch_document(&Persaddress.clone().into()).await?;
        let document = Self::parse_document(&document);

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

    async fn cached_exam_details(
        &self,
        exam_details: Examdetails,
    ) -> anyhow::Result<Option<(Exam, Vec<MaybeCompleteModule>, Vec<MaybeCompleteCourse>)>> {
        let mut connection = self.pool.get()?;

        let existing = exams_unfinished::table
            .filter(exams_unfinished::tucan_id.eq(&exam_details.id))
            .filter(exams_unfinished::done)
            .get_result::<Exam>(&mut connection)
            .optional()?;

        if let Some(existing) = existing {
            let module_exams: Vec<MaybeCompleteModule> = module_exams::table
                .filter(module_exams::exam.eq(&exam_details.id))
                .inner_join(modules_unfinished::table)
                .select(MODULES_UNFINISHED)
                .order(modules_unfinished::title)
                .load(&mut connection)?;

            let course_exams: Vec<MaybeCompleteCourse> = course_exams::table
                .filter(course_exams::exam.eq(&exam_details.id))
                .inner_join(courses_unfinished::table)
                .select(COURSES_UNFINISHED)
                .order(courses_unfinished::title)
                .load(&mut connection)?;

            Ok(Some((existing, module_exams, course_exams)))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_lines)]
    pub async fn fetch_exam_details(&self, exam_details: Examdetails) -> anyhow::Result<()> {
        let exam = {
            let name_document = self.fetch_document(&exam_details.clone().into()).await?;
            let name_document = Self::parse_document(&name_document);

            let registration_range_element = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Anmeldezeitraum")
                .unwrap();
            let registration_range = registration_range_element
                .next_sibling()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .trim()
                .trim_start_matches(": ")
                .split_once(" - ")
                .unwrap();
            let unregistration_range_element = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Abmeldezeitraum")
                .unwrap();
            let unregistration_range = unregistration_range_element
                .next_sibling()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .trim()
                .trim_start_matches(": ")
                .split_once(" - ")
                .unwrap();

            let date_format = "%d.%m.%y %H:%M";
            let registration_start =
                NaiveDateTime::parse_from_str(registration_range.0, date_format)?;
            let registration_end =
                NaiveDateTime::parse_from_str(registration_range.1, date_format)?;
            let unregistration_start =
                NaiveDateTime::parse_from_str(unregistration_range.0, date_format)?;
            let unregistration_end =
                NaiveDateTime::parse_from_str(unregistration_range.1, date_format)?;

            let semester = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Semester")
                .unwrap()
                .next_sibling()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .trim()
                .trim_start_matches(": ")
                .to_string();

            let examinator = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Prüfer")
                .map(|examinator| {
                    examinator
                        .next_sibling()
                        .unwrap()
                        .value()
                        .as_text()
                        .unwrap()
                        .trim()
                        .trim_start_matches(": ")
                        .to_string()
                });

            let room = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Raum")
                .map(|room| {
                    room.next_siblings()
                        .find_map(ElementRef::wrap)
                        .unwrap()
                        .inner_html()
                });

            let exam_type = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Name")
                .unwrap()
                .next_sibling()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .trim()
                .trim_start_matches(": ")
                .to_string();

            let exam_time = name_document
                .select(&s("table td b"))
                .find(|e| e.inner_html() == "Termin")
                .map(|exam_time| {
                    Self::parse_datetime(
                        exam_time
                            .next_sibling()
                            .unwrap()
                            .value()
                            .as_text()
                            .unwrap()
                            .trim()
                            .trim_start_matches(": "),
                    )
                });

            Exam {
                tucan_id: exam_details.id,
                exam_type,
                semester,
                exam_time_start: exam_time.map(|v| v.1),
                exam_time_end: exam_time.map(|v| v.2),
                registration_start,
                registration_end,
                unregistration_start,
                unregistration_end,
                examinator,
                room,
                done: true,
            }
        };

        let mut connection = self.pool.get()?;

        diesel::insert_into(exams_unfinished::table)
            .values(&exam)
            .on_conflict(exams_unfinished::tucan_id)
            .do_update()
            .set(&exam)
            .execute(&mut connection)?;

        Ok(())
    }

    pub async fn exam_details(
        &self,
        exam_details: Examdetails,
    ) -> anyhow::Result<(Exam, Vec<MaybeCompleteModule>, Vec<MaybeCompleteCourse>)> {
        if let Some(value) = self.cached_exam_details(exam_details.clone()).await? {
            return Ok(value);
        }

        self.fetch_exam_details(exam_details.clone()).await?;

        Ok(self
            .cached_exam_details(exam_details.clone())
            .await?
            .unwrap())
    }

    pub async fn cached_my_exams(
        &self,
    ) -> anyhow::Result<
        Option<(
            Vec<(MaybeCompleteModule, Exam)>,
            Vec<(MaybeCompleteCourse, Exam)>,
        )>,
    > {
        let matriculation_number = self.state.session.matriculation_number;

        let mut connection = self.pool.get()?;

        let exams_already_fetched = users_unfinished::table
            .filter(users_unfinished::matriculation_number.eq(&matriculation_number))
            .select(users_unfinished::user_exams_last_checked)
            .get_result::<Option<NaiveDateTime>>(&mut connection)?;

        if exams_already_fetched.is_some() {
            let modules = user_exams::table
                .filter(user_exams::matriculation_number.eq(&matriculation_number))
                .inner_join(
                    exams_unfinished::table
                        .inner_join(module_exams::table.inner_join(modules_unfinished::table)),
                )
                .select((MODULES_UNFINISHED, exams_unfinished::all_columns))
                .order((modules_unfinished::title, exams_unfinished::exam_time_start))
                .load::<(MaybeCompleteModule, Exam)>(&mut connection)?;

            let courses = user_exams::table
                .filter(user_exams::matriculation_number.eq(&matriculation_number))
                .inner_join(
                    exams_unfinished::table
                        .inner_join(course_exams::table.inner_join(courses_unfinished::table)),
                )
                .select((COURSES_UNFINISHED, exams_unfinished::all_columns))
                .order((courses_unfinished::title, exams_unfinished::exam_time_start))
                .load::<(MaybeCompleteCourse, Exam)>(&mut connection)?;

            Ok(Some((modules, courses)))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn fetch_my_exams(&self) -> anyhow::Result<()> {
        type ModuleExams = Vec<(MaybeCompleteModule, Exam)>;
        type CourseExams = Vec<(MaybeCompleteCourse, Exam)>;

        let matriculation_number = self.state.session.matriculation_number;

        let exams = {
            let document = self.fetch_document(&Myexams.clone().into()).await?;
            let document = Self::parse_document(&document);

            document
                .select(&s("table tbody tr"))
                .map(|exam| {
                    let selector = s(r#"td"#);
                    let mut tds = exam.select(&selector);
                    let _nr_column = tds.next().unwrap();
                    let module_column = tds.next().unwrap();
                    let name_column = tds.next().unwrap();
                    let date_column = tds.next().unwrap();
                    let _registered = tds.next().unwrap();

                    let module_link = module_column.select(&s("a")).next().unwrap();
                    let name_link = name_column.select(&s("a")).next().unwrap();
                    let date_link = date_column.select(&s("a")).next();

                    let module_program = parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        module_link.value().attr("href").unwrap()
                    ))
                    .program;

                    let name_program = parse_tucan_url(&format!(
                        "https://www.tucan.tu-darmstadt.de{}",
                        name_link.value().attr("href").unwrap()
                    ))
                    .program;

                    let date = date_link.map(|date| Self::parse_datetime(&date.inner_html()));

                    let examdetails = TryInto::<Examdetails>::try_into(name_program).unwrap();

                    (
                        module_program,
                        Exam {
                            tucan_id: examdetails.id,
                            exam_type: name_link.inner_html(),
                            semester: String::new(),
                            exam_time_start: date.map(|d| d.1),
                            exam_time_end: date.map(|d| d.2),
                            registration_start: Utc::now().naive_utc(), // TODO FIXME remove
                            registration_end: Utc::now().naive_utc(),
                            unregistration_start: Utc::now().naive_utc(),
                            unregistration_end: Utc::now().naive_utc(),
                            examinator: None,
                            room: None,
                            done: false,
                        },
                        module_link.inner_html(),
                    )
                })
                .collect::<Vec<_>>()
        };

        let mut connection = self.pool.get()?;

        let res: Result<Vec<usize>, _> = exams
            .iter()
            .map(|e| &e.1)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|exam| -> Result<_, _> {
                diesel::insert_into(exams_unfinished::table)
                    .values(exam)
                    .on_conflict(exams_unfinished::tucan_id)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        let res: Result<Vec<usize>, _> = exams
            .iter()
            .map(|e| UserExam {
                matriculation_number,
                exam: e.1.tucan_id.clone(),
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|exam| -> Result<_, _> {
                diesel::insert_into(user_exams::table)
                    .values(exam)
                    .on_conflict(user_exams::all_columns)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        let (module_exams, course_exams): (ModuleExams, CourseExams) =
            exams.into_iter().partition_map(|v| match v.0 {
                TucanProgram::Moduledetails(moduledetails) => Either::Left((
                    MaybeCompleteModule::Partial(PartialModule {
                        tucan_id: moduledetails.id,
                        tucan_last_checked: Utc::now().naive_utc(),
                        module_id: String::new(),
                        title: v.2,
                    }),
                    v.1,
                )),
                TucanProgram::Coursedetails(coursedetails) => Either::Right((
                    MaybeCompleteCourse::Partial(PartialCourse {
                        tucan_id: coursedetails.id,
                        tucan_last_checked: Utc::now().naive_utc(),
                        course_id: String::new(),
                        title: v.2,
                    }),
                    v.1,
                )),
                _ => panic!(),
            });

        let res: Result<Vec<usize>, _> = module_exams
            .iter()
            .map(|v| &v.0)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|module_exam| -> Result<_, _> {
                diesel::insert_into(modules_unfinished::table)
                    .values(module_exam)
                    .on_conflict(modules_unfinished::tucan_id)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        let res: Result<Vec<usize>, _> = module_exams
            .iter()
            .map(|e| ModuleExam {
                module_id: e.0.tucan_id().clone(),
                exam: e.1.tucan_id.clone(),
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|module_exam| -> Result<_, _> {
                diesel::insert_into(module_exams::table)
                    .values(module_exam)
                    .on_conflict(module_exams::all_columns)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        // https://github.com/diesel-rs/diesel/discussions/3115#discussioncomment-2509647
        let res: Result<Vec<usize>, _> = course_exams
            .iter()
            .map(|v| &v.0)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|course_exam| -> Result<_, _> {
                diesel::insert_into(courses_unfinished::table)
                    .values(course_exam)
                    .on_conflict(courses_unfinished::tucan_id)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        let res: Result<Vec<usize>, _> = course_exams
            .iter()
            .map(|e| CourseExam {
                course_id: e.0.tucan_id().clone(),
                exam: e.1.tucan_id.clone(),
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|course_exam| -> Result<_, _> {
                diesel::insert_into(course_exams::table)
                    .values(course_exam)
                    .on_conflict(course_exams::all_columns)
                    .do_nothing()
                    .execute(&mut connection)
            })
            .collect();
        res?;

        diesel::update(users_unfinished::table)
            .filter(users_unfinished::matriculation_number.eq(matriculation_number))
            .set(users_unfinished::user_exams_last_checked.eq(Utc::now().naive_utc()))
            .execute(&mut connection)?;

        Ok(())
    }

    pub async fn my_exams(
        &self,
    ) -> anyhow::Result<(
        Vec<(MaybeCompleteModule, Exam)>,
        Vec<(MaybeCompleteCourse, Exam)>,
    )> {
        if let Some(value) = self.cached_my_exams().await? {
            return Ok(value);
        }

        self.fetch_my_exams().await?;

        Ok(self.cached_my_exams().await?.unwrap())
    }
}
