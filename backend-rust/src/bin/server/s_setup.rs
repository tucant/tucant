// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Coursedetails;
use crate::Moduledetails;
use crate::Registration;
use crate::Tucan;
use crate::TucanSession;

use async_stream::Stream;
use axum::body::StreamBody;
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

#[derive(Clone, Copy)]
enum ModulesOrCourses {
    Modules,
    #[allow(dead_code)]
    Courses,
}

#[derive(Debug)]
enum ModuleOrCourse {
    #[allow(dead_code)]
    Module(Module),
    #[allow(dead_code)]
    Course(Course),
}

#[async_recursion::async_recursion]
async fn fetch_registration(
    stream: &mut Stream<Bytes>,
    tucan: Tucan<Authenticated>,
    parent: Registration,
    modules_or_courses: ModulesOrCourses,
) {
    let value = tucan.registration(parent.clone()).await.unwrap();

    stream
        .yield_item(Bytes::from(format!("\nmenu {}", value.0.name)))
        .await;

    let tucan_clone = tucan.clone();

    for menu in value.1.submenus {
        fetch_registration(
            stream,
            tucan_clone.clone(),
            Registration {
                path: menu.tucan_id,
            },
            modules_or_courses,
        )
        .await;
    }

    for module in value.1.modules_and_courses {
        match modules_or_courses {
            ModulesOrCourses::Modules => {
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
            ModulesOrCourses::Courses => {
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
                                .yield_item(Bytes::from(format!("\ncourse {:?}", course.0.title)))
                                .await;
                        }
                        tucant::tucan_user::CourseOrCourseGroup::CourseGroup(_) => panic!(),
                    }
                }
            }
        }
    }
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

        let tucan = tucan.continue_session(session);

        let root = tucan.root_registration().await.unwrap();

        fetch_registration(
            &mut stream,
            tucan,
            Registration {
                path: root.tucan_id,
            },
            ModulesOrCourses::Modules,
        )
        .await;

        stream.yield_item(Bytes::from("\nFertig!")).await;

        let return_value: std::io::Result<()> = Ok(());

        return_value
    });

    let headers = [(header::CONTENT_TYPE, "text/plain")];

    Ok((headers, StreamBody::new(stream)).into_response())
}
