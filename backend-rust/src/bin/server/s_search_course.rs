// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;
use actix_session::Session;

use actix_web::{
    post,
    web::{Data, Json},
};
use diesel::pg::sql_types::Bytea;
use diesel::sql_types::Text;
use diesel::TextExpressionMethods;
use diesel::{sql_function, ExpressionMethods};
use diesel::{QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use diesel_full_text_search::TsVectorExtensions;
use diesel_full_text_search::{
    configuration::TsConfigurationByName, ts_headline_with_search_config, ts_rank_cd_normalized,
    websearch_to_tsquery_with_search_config,
};
use serde::Serialize;
use tucant::{schema::courses_unfinished, tucan::Tucan};
use tucant_derive::{ts, Typescriptable};

sql_function!(fn encode(bytes: Bytea, format: Text) -> Text);
sql_function!(fn rtrim(string: Text, characters: Text) -> Text);

#[derive(Queryable, Serialize, Typescriptable)]
pub struct SearchResult {
    tucan_id: String,
    title: String,
    excerpt: String,
    rank: f32,
}

#[ts]
#[post("/search-course")]
pub async fn search_course(
    _: Session,
    tucan: Data<Tucan>,
    input: Json<String>,
) -> Result<Json<Vec<SearchResult>>, MyError> {
    let mut connection = tucan.pool.get().await?;

    let config = TsConfigurationByName("tucan");
    let tsvector = courses_unfinished::tsv;
    let tsquery = websearch_to_tsquery_with_search_config(config, &input.0);
    let rank = ts_rank_cd_normalized(tsvector, tsquery, 1);
    let sql_query = courses_unfinished::table
        .filter(tsvector.matches(tsquery))
        .order_by(rank.desc())
        .select((
            rtrim(encode(courses_unfinished::tucan_id, "base64"), "="),
            courses_unfinished::title,
            ts_headline_with_search_config(
                config,
                courses_unfinished::course_id
                    .concat(" ")
                    .concat(courses_unfinished::title)
                    .concat(" ")
                    .concat(courses_unfinished::content),
                tsquery,
            ),
            rank,
        ));

    let result = sql_query.load::<SearchResult>(&mut connection).await?;

    Ok(Json(result))
}
