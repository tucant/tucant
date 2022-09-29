// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::Responder;
use actix_web::{
    get,
    web::{Data, Path},
};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tucan_scraper::{models::Module, schema::modules_unfinished, tucan::Tucan};

#[get("/module/{id:.*}")]
pub async fn module(
    _: Session,
    tucan: Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    let mut connection = tucan.pool.get().await?;

    let result = modules_unfinished::table
        .filter(modules_unfinished::tucan_id.eq(base64::decode(path.as_bytes()).unwrap()))
        .select((
            modules_unfinished::tucan_id,
            modules_unfinished::tucan_last_checked,
            modules_unfinished::title,
            modules_unfinished::module_id,
            modules_unfinished::credits,
            modules_unfinished::content,
            modules_unfinished::done,
        ))
        .get_result::<Module>(&mut connection)
        .await?;

    Ok(Json(result))
}
