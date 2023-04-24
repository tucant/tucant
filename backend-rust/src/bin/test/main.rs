// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use tucant::{
    tucan::{Authenticated, Tucan},
    url::Registration,
};

#[async_recursion::async_recursion]
async fn recursive_reg(
    tucan: &Tucan<Authenticated>,
    registration: Registration,
) -> anyhow::Result<()> {
    let (_, registration) = tucan.registration(registration).await?;

    registration
        .modules_and_courses
        .iter()
        .map(|(module)| {
            println!("{:?}", module);
            //tucan.module(tucant::url::Moduledetails { id: () })
        })
        .collect_vec();

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
    let c: anyhow::Result<Vec<()>> = b.into_iter().collect();

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
