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
use itertools::Itertools;
use opensearch::SearchParts;
use serde_json::{json, Value};
use tucant::models::TucanSession;
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


#[ts]
#[post("/search-module-opensearch")]
pub async fn search_module_opensearch(
    _: TucanSession,
    tucan: Data<Tucan>,
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
                      //"title.de^3",
                      //"title.en^3",
                      "content.de",
                      //"content.en"
                    ],
                }
            },
            "highlight": {
                "fields": {
                    "content.de": {
                        // https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#specify-highlight-query
                        "matched_fields": [ "content", "content.de" ],
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
    println!("{}", response_body);

    let took = response_body["took"].as_i64().unwrap();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        //println!("{}", hit);
    }

    let search_results: Vec<SearchResult> = response_body["hits"]["hits"].as_array().unwrap().into_iter().map(|hit| {
        SearchResult {
            tucan_id: base64::decode_config(hit["_id"].as_str().unwrap(), base64::URL_SAFE_NO_PAD).unwrap(),
            title: hit["_source"]["title"].as_str().unwrap().to_string(),
            excerpt: hit["highlight"]["content.de"].as_array().unwrap_or(&Vec::new()).into_iter().map(|e| e.as_str().unwrap()).join(" ... ").to_string(),
            rank: hit["_score"].as_f64().unwrap() as f32,
        }
    }).collect_vec();

    Ok(Json(search_results))
}
