// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::web;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::{
        request::JsonBody,
        transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    },
    indices::{IndicesCreateParts, IndicesPutMappingParts},
    BulkParts, IndexParts, OpenSearch, SearchParts, params::Refresh,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::Url;
use serde_json::{json, Value};
use tucant::{models::Module, schema::modules_unfinished, tucan::Tucan, url::parse_tucan_url};

// $HOME/.cargo/bin/diesel database reset && cargo run --bin test_client
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Dashboard index pattern needs to not include timestamp

    env_logger::init();

    // https://codarium.substack.com/p/designing-an-optimal-multi-language
    // https://opensearch.org/docs/latest/opensearch/query-dsl/full-text/
    // https://opensearch.org/docs/latest/opensearch/query-dsl/text-analyzers

    let tucan = web::Data::new(Tucan::new().await?);

    let url = Url::parse("https://localhost:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .auth(Credentials::Basic("admin".to_string(), "admin".to_string()))
        .cert_validation(CertificateValidation::None)
        .build()?;
    let client = OpenSearch::new(transport);

    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let rand_string: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let index_name: String = format!("tucant_modules_{}", rand_string);

    // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-htmlstrip-charfilter.html
    // https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-lang-analyzer.html#english-analyzer
    let response = client
        .indices()
        .create(IndicesCreateParts::Index(&index_name))
        .body(json!({
            "settings": {
                "analysis": {
                    "analyzer": {
                        "english": {
                            "tokenizer": "standard",
                            "filter": [
                                "english_possessive_stemmer",
                                "lowercase",
                                "english_stop",
                                "english_keywords",
                                "english_stemmer"
                            ],
                            "char_filter": [
                                "html_strip"
                            ]
                        },
                        "german": {
                            "tokenizer": "standard",
                            "filter": [
                                "lowercase",
                                "german_stop",
                                "german_keywords",
                                "german_normalization",
                                "german_stemmer"
                            ],
                            "char_filter": [
                                "html_strip"
                            ]
                        }
                    },
                    "filter": {
                        "english_stop": {
                            "type": "stop",
                            "stopwords": "_english_"
                        },
                        "english_keywords": {
                            "type": "keyword_marker",
                            "keywords": [
                                "example"
                            ]
                        },
                        "english_stemmer": {
                            "type": "stemmer",
                            "language": "english"
                        },
                        "english_possessive_stemmer": {
                            "type": "stemmer",
                            "language": "possessive_english"
                        },
                        "german_stop": {
                            "type": "stop",
                            "stopwords": "_german_"
                        },
                        "german_keywords": {
                            "type": "keyword_marker",
                            "keywords": [
                                "Beispiel"
                            ]
                        },
                        "german_stemmer": {
                            "type": "stemmer",
                            "language": "light_german"
                        }
                    }
                }
            },
            "mappings": {
                "properties": {
                    "content": {
                        "type": "text",
                        "fielddata": true,
                        "fields": {
                            "de": {
                                "type": "text",
                                "analyzer": "german"
                            },
                            "en": {
                                "type": "text",
                                "analyzer": "english"
                            },
                        }
                    },
                    "title": {
                        "type": "text",
                        "fielddata": true,
                        "fields": {
                            "de": {
                                "type": "text",
                                "analyzer": "german"
                            },
                            "en": {
                                "type": "text",
                                "analyzer": "english"
                            },
                        }
                    }
                }
            }
        }
        ))
        .send()
        .await?;

    let exception = response.exception().await?;
    match exception {
        Some(exception) => Err(anyhow::anyhow!("{:?}", exception))?,
        None => {}
    };

    // let response_body = response.json::<Value>().await?;
    //println!("{:?}", response_body);

    let mut connection = tucan.pool.get().await?;
    let modules: Vec<Module> = modules_unfinished::table
        .select((
            modules_unfinished::tucan_id,
            modules_unfinished::tucan_last_checked,
            modules_unfinished::title,
            modules_unfinished::module_id,
            modules_unfinished::credits,
            modules_unfinished::content,
            modules_unfinished::done,
        ))
        .load::<Module>(&mut connection)
        .await?;

    let body: Vec<JsonBody<_>> = modules
        .into_iter()
        .flat_map(|m| {
            let base64_tucan_id = base64::encode_config(&m.tucan_id, base64::URL_SAFE_NO_PAD);
            [
                json!({"index": {"_id": base64_tucan_id}}).into(),
                json!({
                    "id": base64_tucan_id,
                    "last_checked": m.tucan_last_checked,
                    "title": m.title,
                    "module_id": m.module_id,
                    "credits": m.credits,
                    "content": m.content
                })
                .into(),
            ]
            .into_iter()
        })
        .collect_vec();

    let response = client
        .bulk(BulkParts::Index(&index_name))
        .refresh(Refresh::WaitFor)
        .body(body)
        .send()
        .await?;

    let exception = response.exception().await?;
    match exception {
        Some(exception) => Err(anyhow::anyhow!("{:?}", exception))?,
        None => {}
    };

    // https://www.elastic.co/guide/en/elasticsearch/reference/current/search-suggesters.html#completion-suggester

    let response = client.
    indices()
    .update_aliases()
    .body(json!({
        "actions": [
    {
      "remove": {
        "index": "tucant_modules_*",
        "alias": "tucant_modules"
      }
    },
    {
        "add": {
            "index": index_name,
            "alias": "tucant_modules"
        }
    }
  ]
    })).send()
    .await?;

let exception = response.exception().await?;
match exception {
    Some(exception) => Err(anyhow::anyhow!("{:?}", exception))?,
    None => {}
};


    let response = client
        .indices()
        .put_alias(opensearch::indices::IndicesPutAliasParts::IndexName(
            &[&index_name],
            "tucant_modules",
        ))
        .send()
        .await?;
    let exception = response.exception().await?;
    match exception {
        Some(exception) => Err(anyhow::anyhow!("{:?}", exception))?,
        None => {}
    };

    let response = client
        .search(SearchParts::Index(&["tucant_modules"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "multi_match": {
                    "query": "Funktional",
                    "fields": [
                      "title",
                      "title.de",
                      "title.en",
                      "content",
                      "content.de",
                      "content.en"
                    ],
                    "type": "most_fields"
                }
            }
        }))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    let took = response_body["took"].as_i64().unwrap();
    println!("took {}", took);
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        println!("{:?}", hit["_source"]["title"]);
    }

    let response = client
        .indices()
        .get(opensearch::indices::IndicesGetParts::Index(&[
            "tucant_modules_*",
        ]))
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    //println!("{}", response_body);
    // TODO FIXME delete indexes here

    /*
    let tucan = web::Data::new(Tucan::new().await?);
    let tucan = tucan
        .login(
            &std::env::var("TUCAN_USERNAME").unwrap(),
            &std::env::var("TUCAN_PASSWORD").unwrap(),
        )
        .await?;
    /*
        let tucant::url::TucanProgram::Coursedetails(coursedetails) = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N579216929454815,-N000274,-N376333755785484,-N382005035345541,-N382005035304542,-N0,-N0,-N0").program else { panic!() };
        let course = tucan.course_or_course_group(coursedetails).await?;
        //course.content = String::new();
        println!("{:?}", course);

        let tucant::url::TucanProgram::Coursedetails(coursedetails) = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N579216929454815,-N000274,-N376333755785484,-N382005035345541,-N382005035451545,-N0,-N0,-N0").program else { panic!() };
        let course = tucan.course_or_course_group(coursedetails).await?;
        //course.content = String::new();
        println!("{:?}", course);
    */
    let tucant::url::TucanProgram::Registration(program) = parse_tucan_url("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N115106201942566,-N000311,-N376333755785484,-N0,-N356175025101319,-N354165664027444").program else { panic!() };
    let _registration = tucan.registration(program).await?;
    */

    Ok(())
}
