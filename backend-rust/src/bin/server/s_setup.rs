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
use actix_session::Session;
use actix_web::post;
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::Responder;
use anyhow::Error;
use async_stream::try_stream;
use core::pin::Pin;
use futures::stream::FuturesUnordered;
use futures::Stream;
use futures_util::StreamExt;
use tucant::tucan_user::RegistrationEnum;

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

fn fetch_registration(
    tucan: TucanUser,
    parent: Registration,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, MyError>>>> {
    Box::pin(try_stream(move |mut stream| async move {
        let value = tucan.registration(parent.clone()).await?;

        stream
            .yield_item(Bytes::from(format!("menu {}", value.0.name)))
            .await;

        match value.1 {
            RegistrationEnum::Submenu(submenu) => {
                yield_stream(
                    &mut stream,
                    Box::pin(
                        futures::stream::iter(submenu.into_iter())
                            .map(move |menu| {
                                fetch_registration(
                                    tucan.clone(),
                                    Registration {
                                        path: menu.tucan_id,
                                    },
                                )
                            })
                            .flatten_unordered(None),
                    ),
                )
                .await?;
            }
            RegistrationEnum::Modules(modules) => {
                let mut futures: FuturesUnordered<_> = modules
                    .iter()
                    .map(|module| async {
                        // TODO FIXME make this a nested stream like above so we can yield_item in here also for courses
                        let module = tucan
                            .module(Moduledetails {
                                id: module.tucan_id.clone(),
                            })
                            .await
                            .unwrap();

                        // TODO FIXME make this in parallel for absolute overkill?
                        for course in module.1 {
                            tucan
                                .course(Coursedetails {
                                    id: course.tucan_id.clone(),
                                })
                                .await
                                .unwrap();
                        }

                        module.0
                    })
                    .collect();

                while let Some(module) = futures.next().await {
                    stream
                        .yield_item(Bytes::from(format!("module {}", module.title)))
                        .await;
                }
            }
        }

        Ok(())
    }))
}

#[post("/setup")]
pub async fn setup(tucan: Data<Tucan>, session: Session) -> Result<impl Responder, MyError> {
    match session.get::<TucanSession>("session").unwrap() {
        Some(session) => {
            let stream = try_stream(move |mut stream| async move {
                stream
                    .yield_item(Bytes::from("Alle Module werden heruntergeladen..."))
                    .await;

                let tucan = tucan.continue_session(session).await.unwrap();

                let root = tucan.root_registration().await.unwrap();

                let input = fetch_registration(
                    tucan,
                    Registration {
                        path: root.tucan_id,
                    },
                );

                yield_stream(&mut stream, input).await.unwrap();

                stream.yield_item(Bytes::from("Fertig!")).await;

                let return_value: Result<(), Error> = Ok(());

                return_value
            });

            // TODO FIXME search for <h1>Timeout!</h1>

            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .streaming(stream))
        }
        None => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("not logged in")),
    }
}
