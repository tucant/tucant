// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::{stdout, Write};

use base64::{
    engine::{
        general_purpose::{NO_PAD, PAD},
        GeneralPurpose,
    },
    prelude::*,
};
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
            //let vv = tucan.vv().await?;

            Ok(())
        })
}
