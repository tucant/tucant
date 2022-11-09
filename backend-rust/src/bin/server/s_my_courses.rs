// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;

use actix_web::post;
use actix_web::web::Json;

use actix_web::web::Data;

use tucant::models::Course;
use tucant::models::TucanSession;
use tucant::tucan::Tucan;
use tucant_derive::ts;

#[tracing::instrument]
#[ts]
#[post("/my_courses")]
pub async fn my_courses(
    session: TucanSession,
    tucan: Data<Tucan>,
    _input: Json<()>,
) -> Result<Json<Vec<Course>>, MyError> {
    let tucan = tucan.continue_session(session).await.unwrap();

    let result = tucan.my_courses().await?;

    Ok(Json(result))
}
