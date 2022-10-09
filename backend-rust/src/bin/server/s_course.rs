// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;

use actix_web::post;
use actix_web::web::Json;

use actix_web::web::Data;

use tucant::models::TucanSession;
use tucant::url::Coursedetails;
use tucant::{models::Course, tucan::Tucan};
use tucant_derive::ts;

#[ts]
#[post("/course")]
pub async fn course(
    session: TucanSession,
    tucan: Data<Tucan>,
    input: Json<String>,
) -> Result<Json<Course>, MyError> {
    let binary_path = base64::decode_config(input.as_bytes(), base64::URL_SAFE_NO_PAD).unwrap();

    let tucan = tucan.continue_session(session).await.unwrap();

    let result = tucan
        .course(Coursedetails {
            id: binary_path.clone(),
        })
        .await?;

    Ok(Json(result))
}
