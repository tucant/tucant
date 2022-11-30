// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use opensearch::{
    http::request::JsonBody, indices::IndicesCreateParts, params::Refresh, BulkParts,
};
use rand::Rng;

use serde_json::{json, Value};
use tucant::{models::Module, schema::modules_unfinished, tucan::Tucan};

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

    let exams = tucan.my_exams().await?;

    Ok(())
}
