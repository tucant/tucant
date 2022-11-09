// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;

use actix_web::post;
use actix_web::web::Json;

use actix_web::web::Data;

use tucant::models::Module;
use tucant::models::TucanSession;
use tucant::tucan::Tucan;
use tucant_derive::ts;

#[tracing::instrument]
#[ts]
#[post("/my_modules")]
pub async fn my_modules(
    session: TucanSession,
    tucan: Data<Tucan>,
    _input: Json<()>,
) -> Result<Json<Vec<Module>>, MyError> {
    let tucan = tucan.continue_session(session).await.unwrap();

    let result = tucan.my_modules().await?;

    Ok(Json(result))
}
