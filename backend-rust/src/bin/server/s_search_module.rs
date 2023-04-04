// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::s_search_course::SearchResult;
use diesel::dsl::sql;
use diesel::sql_types::Double;
use tucant::MyError;

use axum::extract::State;
use axum::Json;
use base64::prelude::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::TextExpressionMethods;
use diesel_async::RunQueryDsl;
use diesel_full_text_search::TsVectorExtensions;
use diesel_full_text_search::{
    configuration::TsConfigurationByName, ts_headline_with_search_config, ts_rank_cd_normalized,
    websearch_to_tsquery_with_search_config,
};
use itertools::Itertools;
use opensearch::SearchParts;
use serde_json::{json, Value};
use tucant::models::TucanSession;
use tucant::{schema::modules_unfinished, tucan::Tucan};
use tucant_derive::ts;

#[ts]
pub async fn search_module(
    _: TucanSession,
    tucan: State<Tucan>,
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
            sql::<Double>("CAST(").bind(rank).sql(" as FLOAT8)"),
        ));

    let result = sql_query.load::<SearchResult>(&mut connection).await?;

    Ok(Json(result))
}

#[ts]
pub async fn search_module_opensearch(
    _: TucanSession,
    tucan: State<Tucan>,
    input: Json<String>,
) -> Result<Json<Vec<SearchResult>>, MyError> {
    let response = tucan
        .opensearch
        .search(SearchParts::Index(&["tucant_modules"]))
        .from(0)
        .size(20)
        .body(json!({
            "query": {
                "multi_match": {
                    "query": input.0,
                    "fields": [
                      "title.de^3",
                      "title.en^3",
                      "content.de",
                      "content.en"
                    ],
                }
            },
            "highlight": {
                "require_field_match": false,
                "fields": {
                    "title": {
                        // https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#specify-highlight-query
                        "matched_fields": [ "title.en", "title.de" ],
                        "type": "fvh",
                        "pre_tags": ["<b>", "<b>"],
                        "post_tags": ["</b>", "</b>"],
                    },
                    "content": {
                        // https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#specify-highlight-query
                        "matched_fields": [ "content.en", "content.de" ],
                        "type": "fvh",
                        "pre_tags": ["<b>", "<b>"],
                        "post_tags": ["</b>", "</b>"],
                    },
                }
            }
        }))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;

    let _took = response_body["took"].as_i64().unwrap();
    for _hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
    }

    let search_results: Vec<SearchResult> = response_body["hits"]["hits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|hit| SearchResult {
            tucan_id: BASE64_URL_SAFE_NO_PAD
                .decode(hit["_id"].as_str().unwrap())
                .unwrap(),
            title: hit["highlight"]["title"]
                .as_array()
                .unwrap_or(&vec![hit["_source"]["title"].clone()])
                .iter()
                .map(|e| e.as_str().unwrap())
                .join("[...]"),
            excerpt: hit["highlight"]["content"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|e| e.as_str().unwrap())
                .join("[...]"),
            rank: hit["_score"].as_f64().unwrap(),
        })
        .collect_vec();

    Ok(Json(search_results))
}
