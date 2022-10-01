// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;
use actix_session::Session;
use actix_web::post;
use actix_web::web::Json;

use actix_web::web::Data;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tucant::{models::Course, schema::courses_unfinished, tucan::Tucan};
use tucant_derive::ts;

#[ts]
#[post("/course")]
pub async fn course(
    _: Session,
    tucan: Data<Tucan>,
    input: Json<String>,
) -> Result<Json<Course>, MyError> {
    let mut connection = tucan.pool.get().await?;

    let result = courses_unfinished::table
        .filter(courses_unfinished::tucan_id.eq(base64::decode(input.as_bytes()).unwrap()))
        .select((
            courses_unfinished::tucan_id,
            courses_unfinished::tucan_last_checked,
            courses_unfinished::title,
            courses_unfinished::course_id,
            courses_unfinished::sws,
            courses_unfinished::content,
            courses_unfinished::done,
        ))
        .get_result::<Course>(&mut connection)
        .await?;

    Ok(Json(result))
}
