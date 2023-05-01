// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use futures::{stream::FuturesUnordered, StreamExt};

use tucant::{
    tucan::{Authenticated, Tucan},
    url::{Moduledetails, Registration},
};

#[async_recursion::async_recursion]
async fn recursive_reg(
    tucan: &Tucan<Authenticated>,
    registration: Registration,
) -> anyhow::Result<()> {
    let (_, registration) = tucan.registration(registration).await?;

    let g = registration
        .modules_and_courses
        .iter()
        .map(|module| {
            tucan.module(Moduledetails {
                id: module.0.tucan_id().clone(),
            })
        })
        .collect::<FuturesUnordered<_>>();

    let g = g.collect::<Vec<_>>().await;
    let _g: anyhow::Result<Vec<_>> = g.into_iter().collect();

    let a = registration
        .submenus
        .iter()
        .map(|menu| {
            recursive_reg(
                tucan,
                Registration {
                    path: menu.tucan_id.clone(),
                },
            )
        })
        .collect::<FuturesUnordered<_>>();

    let b = a.collect::<Vec<_>>().await;
    let _c: anyhow::Result<Vec<_>> = b.into_iter().collect();

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            env_logger::init();

            let tucan = Tucan::new()?;
            let tucan = tucan
                .login(
                    &std::env::var("TUCAN_USERNAME")?,
                    &std::env::var("TUCAN_PASSWORD")?,
                )
                .await?;

            let _module = tucan
                .module(Moduledetails {
                    id: base64::prelude::BASE64_URL_SAFE_NO_PAD
                        .decode("AAFWTy6e-KU")
                        .unwrap(),
                })
                .await?;

            let root_reg = tucan.root_registration().await?;

            recursive_reg(
                &tucan,
                Registration {
                    path: root_reg.tucan_id,
                },
            )
            .await?;

            //tucan.course_results().await?;
            /*
                        let semesters = tucan.root_module_results().await?;
                        for semester in semesters {
                            tucan.module_results(semester).await?;
                        }
            */
            Ok(())
        })
}
