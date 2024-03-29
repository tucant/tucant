// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::State, Json};
use tucant_core::{models::TucanSession, tucan::Tucan, MyError};
use tucant_derive::ts;

use crate::s_search_course::SearchResult;

#[ts]
pub async fn search_module(
    _: TucanSession,
    _tucan: State<Tucan>,
    _input: Json<String>,
) -> Result<Json<Vec<SearchResult>>, MyError> {
    #[cfg(feature = "full-text-search")]
    {
        // http://localhost:8080/search-module?q=digitale%20schaltung
        let mut connection = tucan.pool.get()?;

        let config = TsConfigurationByName("tucan");
        let tsvector = modules_unfinished::tsv;
        let tsquery = websearch_to_tsquery_with_search_config(config, &input.0);
        let rank = ts_rank_cd_normalized(tsvector, tsquery, 1);
        let sql_query = modules_unfinished::table
            .filter(tsvector.matches(tsquery))
            .order(rank.desc())
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
    #[cfg(not(feature = "full-text-search"))]
    unimplemented!()
}

#[ts]
pub async fn search_module_opensearch(
    _: TucanSession,
    _tucan: State<Tucan>,
    _input: Json<String>,
) -> Result<Json<Vec<SearchResult>>, MyError> {
    #[cfg(feature = "full-text-search")]
    {
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
    #[cfg(not(feature = "full-text-search"))]
    unimplemented!()
}
