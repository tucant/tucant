// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use tucant::tucan::Tucan;

// $HOME/.cargo/bin/diesel database reset && cargo run --bin test
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
                &std::env::var("TUCAN_USERNAME").unwrap(),
                &std::env::var("TUCAN_PASSWORD").unwrap(),
            )
            .await?;
    
        let exams = tucan.my_exams().await?;
    
        println!("{exams:#?}");
    
        Ok(())
    })
}
