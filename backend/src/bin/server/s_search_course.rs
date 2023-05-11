// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::State, Json};
use diesel::Queryable;

use serde::Serialize;

use tucant_core::{
    models::{as_base64, TucanSession},
    tucan::Tucan,
    MyError,
};
use tucant_derive::{ts, Typescriptable};

#[derive(Queryable, Serialize, Typescriptable)]
pub struct SearchResult {
    #[cfg_attr(feature = "server", ts_type(String))]
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tucan_id: Vec<u8>,
    pub title: String,
    pub excerpt: String,
    pub rank: f64,
}

#[ts]
pub async fn search_course(
    _: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<Vec<SearchResult>>, MyError> {
    #[cfg(feature = "full-text-search")]
    {
        let mut connection = tucan.pool.get()?;

        let config = TsConfigurationByName("tucan");
        let tsvector = courses_unfinished::tsv;
        let tsquery = websearch_to_tsquery_with_search_config(config, &input.0);
        let rank = ts_rank_cd_normalized(tsvector, tsquery, 1);
        let sql_query = courses_unfinished::table
            .filter(tsvector.matches(tsquery))
            .order(rank.desc())
            .select((
                courses_unfinished::tucan_id,
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
                sql::<Double>("CAST(").bind(rank).sql(" as FLOAT8)"),
            ));

        let result = sql_query.load::<SearchResult>(&mut connection).await?;

        Ok(Json(result))
    }
    #[cfg(not(feature = "full-text-search"))]
    unimplemented!()
}
