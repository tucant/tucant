// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::web;
use tucant::tucan::Tucan;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let tucan = web::Data::new(Tucan::new().await?);
    let tucan = tucan.login(&std::env::var("USERNAME").unwrap(), &std::env::var("PASSWORD").unwrap()).await?;



    Ok(())
}