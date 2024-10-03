// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::pin::Pin;

use crate::Coursedetails;
use crate::Moduledetails;
use crate::Registration;
use crate::Tucan;
use crate::TucanSession;

use async_stream::stream;
use axum::body::Body;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use reqwest::header;
use tucant_core::MyError;

use axum::body::Bytes;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use tucant_core::models::MaybeCompleteCourse;
use tucant_core::models::MaybeCompleteModule;
use tucant_core::tucan::Authenticated;
use tucant_core::tucan::Unauthenticated;
use tucant_core::url::Action;
use tucant_core::url::TucanProgram;

#[derive(Clone, Copy)]
pub enum ModulesOrCourses {
    #[allow(dead_code)]
    Modules,
    #[allow(dead_code)]
    Courses,
    Both,
}

#[derive(Debug)]
enum ModuleOrCourse {
    #[allow(dead_code)]
    Module(MaybeCompleteModule),
    #[allow(dead_code)]
    Course(MaybeCompleteCourse),
}

pub async fn setup_vv(tucan: State<Tucan>, _input: Json<()>) -> Result<Response, MyError> {
    let stream = stream! {
        yield Ok::<_, std::io::Error>(Bytes::from("\nVV wird heruntergeladen..."));

        let root = tucan.vv_root().await.unwrap();

        let mut inner_stream = prefetch_vv(
            tucan.0,
            Action {
                magic: root.0.tucan_id,
            },
        );

        while let Some(value) = inner_stream.next().await {
            yield Ok(value);
        }

        yield Ok(Bytes::from("\nFertig!"));
    };

    let headers = [(header::CONTENT_TYPE, "text/plain")];

    Ok((headers, Body::from_stream(stream)).into_response())
}

fn prefetch_vv(
    tucan: Tucan<Unauthenticated>,
    action: Action,
) -> Pin<Box<dyn futures::Stream<Item = Bytes> + Send>> {
    Box::pin(async_stream::stream! {
        let (value0, value1, value2) = tucan.vv(action).await.unwrap();

        yield Bytes::from(format!("\nvv menu {}", value0.name));

        let mut result = futures::stream::iter(value1)
            .map(|submenu| {
                let t = tucan.clone();
                prefetch_vv(
                    t,
                    Action {
                        magic: submenu.tucan_id,
                    },
                )
            })
            .flatten_unordered(None);

        let mut course_result: FuturesUnordered<_> = value2.iter()
        .map(|course| {
            tucan.course(Coursedetails { id: course.tucan_id().clone() })
        }).collect();

        while let Some(value) = result.next().await {
            yield value;
        }

        while let Some(value) = course_result.next().await {
            yield Bytes::from(format!("\nvv course {}", value.unwrap().0.title));
        }
    })
}

fn fetch_registration(
    tucan: Tucan<Authenticated>,
    parent: Registration,
    modules_or_courses: ModulesOrCourses,
) -> Pin<Box<dyn futures::Stream<Item = Bytes> + Send>> {
    Box::pin(async_stream::stream! {
        let value = tucan.registration(parent.clone()).await.unwrap();

        yield Bytes::from(format!("\nmenu {}", value.0.name));

        let tucan_clone = tucan.clone();

        let mut result = futures::stream::iter(value.1.submenus)
            .map(|menu| {
                let t = tucan_clone.clone();
                fetch_registration(
                    t,
                    Registration {
                        path: menu.tucan_id,
                    },
                    modules_or_courses,
                )
            })
            .flatten_unordered(None);

        while let Some(value) = result.next().await {
            yield value;
        }

        for module in value.1.modules_and_courses {
            match modules_or_courses {
                ModulesOrCourses::Modules | ModulesOrCourses::Both => {
                    let module = tucan
                        .module(Moduledetails {
                            id: module.0.tucan_id().clone(),
                        })
                        .await
                        .unwrap();
                    yield Bytes::from(format!("\nmodule {:?}", module.0.title));
                }
                ModulesOrCourses::Courses => {}
            }
            match modules_or_courses {
                ModulesOrCourses::Courses | ModulesOrCourses::Both => {
                    // some history modules have multiple courses per module
                    // so we have to fetch all here

                    for course in module.1 {
                        match tucan
                            .course_or_course_group(Coursedetails {
                                id: course.tucan_id().clone(),
                            })
                            .await
                            .unwrap()
                        {
                            tucant_core::tucan_user::CourseOrCourseGroup::Course(course) => {
                                yield Bytes::from(format!(
                                        "\ncourse {:?}",
                                        course.0.title
                                    ));
                            }
                            tucant_core::tucan_user::CourseOrCourseGroup::CourseGroup(_) => {
                                panic!()
                            }
                        }
                    }
                }
                ModulesOrCourses::Modules => {}
            }
        }
    })
}

pub async fn setup(
    tucan: State<Tucan>,
    session: TucanSession,
    _input: Json<()>,
) -> Result<Response, MyError> {
    let stream = stream! {
        yield Ok::<_, anyhow::Error>(Bytes::from(
                "\nAlle Module und Veranstaltungen werden heruntergeladen...",
            ));

        let tucan = tucan.continue_session(session).await?;

        let root = tucan.root_registration().await.unwrap();

        let mut inner_stream = fetch_registration(
            tucan,
            Registration {
                path: root.tucan_id,
            },
            ModulesOrCourses::Both,
        );

        while let Some(value) = inner_stream.next().await {
            yield Ok(value);
        }

        yield Ok(Bytes::from("\nFertig!"));
    };

    let headers = [(header::CONTENT_TYPE, "text/plain")];

    Ok((headers, Body::from_stream(stream)).into_response())
}

fn fetch_module_urls(
    tucan: Tucan<Authenticated>,
    parent: Registration,
) -> Pin<Box<dyn futures::Stream<Item = Bytes> + Send>> {
    Box::pin(async_stream::stream! {
        let value = tucan.registration(parent.clone()).await.unwrap();

        let tucan_clone = tucan.clone();

        let mut result = futures::stream::iter(value.1.submenus)
            .map(|menu| {
                let t = tucan_clone.clone();
                fetch_module_urls(
                    t,
                    Registration {
                        path: menu.tucan_id,
                    },
                )
            })
            .flatten_unordered(None);

        while let Some(value) = result.next().await {
            yield value;
        }

        for module in value.1.modules_and_courses {
            let tucan_program: TucanProgram = Moduledetails {
                id: module.0.tucan_id().clone(),
            }
            .into();
            yield Bytes::from(format!(
                    "{}\n",
                    tucan_program.to_tucan_url(None)
                ));
        }
    })
}

pub async fn module_urls(
    tucan: State<Tucan>,
    session: TucanSession,
    _input: Json<()>,
) -> Result<Response, MyError> {
    let stream = stream! {
        let tucan = tucan.continue_session(session).await?;

        let root = tucan.root_registration().await.unwrap();

        let mut inner_stream = fetch_module_urls(
            tucan,
            Registration {
                path: root.tucan_id,
            },
        );

        while let Some(value) = inner_stream.next().await {
           yield Ok::<_, anyhow::Error>(value);
        }
    };

    let headers = [(header::CONTENT_TYPE, "text/plain")];

    Ok((headers, Body::from_stream(stream)).into_response())
}
