// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use tucant::tucan::Tucan;

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

            //tucan.course_results().await?;

            let semesters = tucan.root_module_results().await?;
            for semester in semesters {
                tucan.module_results(semester).await?;
            }

            Ok(())
        })
}
