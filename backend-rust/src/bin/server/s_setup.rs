// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::pin::Pin;

use crate::Coursedetails;
use crate::Moduledetails;
use crate::Registration;
use crate::Tucan;
use crate::TucanSession;

use axum::body::StreamBody;
use futures::StreamExt;
use reqwest::header;
use tucant::MyError;

use async_stream::try_stream;
use axum::body::Bytes;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use tucant::models::Course;
use tucant::models::Module;
use tucant::tucan::Authenticated;
use tucant::tucan::Unauthenticated;
use tucant::url::Action;

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
    Module(Module),
    #[allow(dead_code)]
    Course(Course),
}

pub async fn setup_vv(tucan: State<Tucan>, _input: Json<()>) -> Result<Response, MyError> {
    let stream = try_stream(move |mut stream| async move {
        stream
            .yield_item(Bytes::from("\nVV wird heruntergeladen..."))
            .await;

        let root = tucan.vv_root().await.unwrap();

        let mut inner_stream = prefetch_vv(
            tucan.0,
            Action {
                magic: root.0.tucan_id,
            },
        );

        while let Some(value) = inner_stream.next().await {
            stream.yield_item(value).await;
        }

        stream.yield_item(Bytes::from("\nFertig!")).await;

        let return_value: std::io::Result<()> = Ok(());

        return_value
    });

    let headers = [(header::CONTENT_TYPE, "text/plain")];

    Ok((headers, StreamBody::new(stream)).into_response())
}

fn prefetch_vv(
    tucan: Tucan<Unauthenticated>,
    action: Action,
) -> Pin<Box<dyn futures::Stream<Item = Bytes> + Send>> {
    Box::pin(async_stream::stream(move |mut stream| async move {
        let value = tucan.vv(action).await.unwrap();

        stream
            .yield_item(Bytes::from(format!("\nvv {}", value.0.name)))
            .await;

        let mut result = futures::stream::iter(value.1)
            .map(|submenu| {
                let t = tucan.clone();
                prefetch_vv(
                    t,
                    Action {
                        magic: submenu.tucan_id.clone(),
                    },
                )
            })
            .flatten_unordered(None);

        while let Some(value) = result.next().await {
            stream.yield_item(value).await;
        }
    }))
}

fn fetch_registration(
    tucan: Tucan<Authenticated>,
    parent: Registration,
    modules_or_courses: ModulesOrCourses,
) -> Pin<Box<dyn futures::Stream<Item = Bytes> + Send>> {
    Box::pin(async_stream::stream(move |mut stream| async move {
        let value = tucan.registration(parent.clone()).await.unwrap();

        stream
            .yield_item(Bytes::from(format!("\nmenu {}", value.0.name)))
            .await;

        let tucan_clone = tucan.clone();

        let mut result = futures::stream::iter(value.1.submenus)
            .map(|menu| {
                let t = tucan_clone.clone();
                fetch_registration(
                    t,
                    Registration {
                        path: menu.tucan_id.clone(),
                    },
                    modules_or_courses,
                )
            })
            .flatten_unordered(None);

        while let Some(value) = result.next().await {
            stream.yield_item(value).await;
        }

        for module in value.1.modules_and_courses {
            match modules_or_courses {
                ModulesOrCourses::Modules | ModulesOrCourses::Both => {
                    let module = tucan
                        .module(Moduledetails {
                            id: module.0.tucan_id.clone(),
                        })
                        .await
                        .unwrap();
                    stream
                        .yield_item(Bytes::from(format!("\nmodule {:?}", module.0.title)))
                        .await;
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
                                id: course.tucan_id.clone(),
                            })
                            .await
                            .unwrap()
                        {
                            tucant::tucan_user::CourseOrCourseGroup::Course(course) => {
                                stream
                                    .yield_item(Bytes::from(format!(
                                        "\ncourse {:?}",
                                        course.0.title
                                    )))
                                    .await;
                            }
                            tucant::tucan_user::CourseOrCourseGroup::CourseGroup(_) => panic!(),
                        }
                    }
                }
                ModulesOrCourses::Modules => {}
            }
        }
    }))
}

pub async fn setup(
    tucan: State<Tucan>,
    session: TucanSession,
    _input: Json<()>,
) -> Result<Response, MyError> {
    let stream = try_stream(move |mut stream| async move {
        stream
            .yield_item(Bytes::from(
                "\nAlle Module und Veranstaltungen werden heruntergeladen...",
            ))
            .await;

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
            stream.yield_item(value).await;
        }

        stream.yield_item(Bytes::from("\nFertig!")).await;

        let return_value: anyhow::Result<()> = Ok(());

        return_value
    });

    let headers = [(header::CONTENT_TYPE, "text/plain")];

    Ok((headers, StreamBody::new(stream)).into_response())
}
