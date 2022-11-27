// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Coursedetails;
use crate::Moduledetails;
use crate::MyError;
use crate::Registration;
use crate::Tucan;
use crate::TucanSession;
use crate::TucanUser;

use anyhow::Error;
use async_stream::try_stream;
use axum::Json;
use axum::body::Bytes;
use axum::body::StreamBody;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use core::pin::Pin;
use futures::stream::FuturesUnordered;

use futures::FutureExt;
use futures::Stream;
use futures_util::StreamExt;
use tracing_futures::Instrument;
use tucant::models::Course;
use tucant::models::Module;

async fn yield_stream(
    stream: &mut async_stream::Stream<Bytes>,
    mut inner_stream: Pin<Box<dyn Stream<Item = Result<Bytes, MyError>>>>,
) -> Result<(), MyError> {
    loop {
        match inner_stream.next().await {
            Some(Ok(value)) => {
                stream.yield_item(value).await;
            }
            Some(err @ Err(_)) => {
                err?;
            }
            None => {
                break Ok(());
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ModulesOrCourses {
    Modules,
    #[allow(dead_code)]
    Courses,
}

#[derive(Debug)]
enum ModuleOrCourse {
    Module(Module),
    Course(Course),
}

// https://docs.rs/tracing-futures/0.2.5/tracing_futures/
fn fetch_registration(
    tucan: TucanUser,
    parent: Registration,
    modules_or_courses: ModulesOrCourses,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, MyError>>>> {
    Box::pin(
        try_stream(move |mut stream| async move {
            let value = tucan.registration(parent.clone()).await?;

            stream
                .yield_item(Bytes::from(format!("\nmenu {}", value.0.name)))
                .await;

            let tucan_clone = tucan.clone();

            yield_stream(
                &mut stream,
                Box::pin(
                    futures::stream::iter(value.1.submenus.into_iter())
                        .map(move |menu| {
                            fetch_registration(
                                tucan_clone.clone(),
                                Registration {
                                    path: menu.tucan_id,
                                },
                                modules_or_courses,
                            )
                        })
                        .flatten_unordered(None),
                ),
            )
            .await?;

            let mut futures: FuturesUnordered<_> = value.1.modules_and_courses
                .iter()
                .flat_map(|module| {
                    //                             .instrument(tracing::info_span!("magic"))
                        match modules_or_courses {
                            ModulesOrCourses::Modules => Box::new(std::iter::once((async {
                                let module = tucan
                                    .module(Moduledetails {
                                        id: module.0.tucan_id.clone(),
                                    })
                                    .await
                                    .unwrap();
                                ModuleOrCourse::Module(module.0)
                            }).boxed_local())) as Box<dyn Iterator<Item=_>>,
                            ModulesOrCourses::Courses => {
                                // some history modules have multiple courses per module
                                // so we have to fetch all here

                                Box::new(module.1.iter().map(|course| (async {
                                    ModuleOrCourse::Course(match
                                    tucan
                                    .course_or_course_group(Coursedetails {
                                        id: course.tucan_id.clone(),
                                    })
                                    .await
                                    .unwrap() {
                                        tucant::tucan_user::CourseOrCourseGroup::Course(c) => c,
                                        tucant::tucan_user::CourseOrCourseGroup::CourseGroup(_) => panic!(),
                                    })
                                }).boxed_local())) as Box<dyn Iterator<Item=_>>
                            }
                        }
                    })
                .collect();

            while let Some(module) = futures.next().await {
                stream
                    .yield_item(Bytes::from(match module {
                        ModuleOrCourse::Module(module) => format!("\nmodule {:?}", module.title),
                        ModuleOrCourse::Course(course) => format!("\ncourse {:?}", course.title),
                    }))
                    .await;
            }

            Ok(())
        })
        .instrument(tracing::info_span!("fetch_registration")),
    )
}

pub async fn setup(
    tucan: State<Tucan>,
    session: TucanSession,
    _input: Json<()>,
) -> Result<Response, MyError> {
    let stream = try_stream(move |mut stream| async move {
        stream
            .yield_item(Bytes::from("\nAlle Module werden heruntergeladen..."))
            .await;

        let tucan = tucan.continue_session(session).await.unwrap();

        let root = tucan.root_registration().await.unwrap();

        let input = fetch_registration(
            tucan,
            Registration {
                path: root.tucan_id,
            },
            ModulesOrCourses::Modules,
        );

        yield_stream(&mut stream, input).await.unwrap();

        stream.yield_item(Bytes::from("\nFertig!")).await;

        let return_value: Result<(), Error> = Ok(());

        return_value
    });

    // TODO FIXME search for <h1>Timeout!</h1>

    Ok("".into_response())

    //Ok(StreamBody::new(stream).into_response())
}
