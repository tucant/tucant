// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use tucant::tucan::Tucan;

// $HOME/.cargo/bin/diesel database reset && cargo run --bin test
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let tucan = Tucan::new().await?;
    let tucan = tucan
        .login(
            &std::env::var("TUCAN_USERNAME").unwrap(),
            &std::env::var("TUCAN_PASSWORD").unwrap(),
        )
        .await?;

    tucan.my_exams().await?;

    Ok(())
}
