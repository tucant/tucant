// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::s_search_course::SearchResult;
use crate::MyError;

use actix_web::post;
use actix_web::web::{Data, Json};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::TextExpressionMethods;
use diesel_async::RunQueryDsl;
use diesel_full_text_search::TsVectorExtensions;
use diesel_full_text_search::{
    configuration::TsConfigurationByName, ts_headline_with_search_config, ts_rank_cd_normalized,
    websearch_to_tsquery_with_search_config,
};
use tucant::tucan_user::TucanSession;
use tucant::{schema::modules_unfinished, tucan::Tucan};
use tucant_derive::ts;

#[ts]
#[post("/search-module")]
pub async fn search_module(
    _: TucanSession,
    tucan: Data<Tucan>,
    input: Json<String>,
) -> Result<Json<Vec<SearchResult>>, MyError> {
    // http://localhost:8080/search-module?q=digitale%20schaltung
    let mut connection = tucan.pool.get().await?;

    let config = TsConfigurationByName("tucan");
    let tsvector = modules_unfinished::tsv;
    let tsquery = websearch_to_tsquery_with_search_config(config, &input.0);
    let rank = ts_rank_cd_normalized(tsvector, tsquery, 1);
    let sql_query = modules_unfinished::table
        .filter(tsvector.matches(tsquery))
        .order_by(rank.desc())
        .select((
            modules_unfinished::tucan_id,
            modules_unfinished::title,
            ts_headline_with_search_config(
                config,
                modules_unfinished::module_id
                    .concat(" ")
                    .concat(modules_unfinished::title)
                    .concat(" ")
                    .concat(modules_unfinished::content),
                tsquery,
            ),
            rank,
        ));

    let result = sql_query.load::<SearchResult>(&mut connection).await?;

    Ok(Json(result))
}
