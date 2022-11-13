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
    BulkParts, IndexParts, OpenSearch, SearchParts,
};
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

    // TODO FIXME mappings are not updated, fix that
    const INDEX_NAME: &str = "tucant_modules_v3";

    let response = client.indices()
    .create(IndicesCreateParts::Index(INDEX_NAME))
    .send()
    .await?;

   /* let exception = response.exception().await?;
    match exception {
        Some(exception) => Err(anyhow::anyhow!("{:?}", exception))?,
        None => {}
    };
*/
    let response = client
        .indices()
        .put_mapping(IndicesPutMappingParts::Index(&[INDEX_NAME]))
        .body(json!({
            "properties" : {
                "content": {
                    "type": "text",
                    "fielddata": true,
                    "fields": {
                      "de": {
                        "type":     "text",
                        "analyzer": "german"
                      },
                      "en": {
                          "type":     "text",
                          "analyzer": "english"
                      },
                      "raw": {
                        "type": "keyword"
                      }
                    }
                  },
                  "title": {
                    "type": "text",
                    "fielddata": true,
                    "fields": {
                      "de": {
                        "type":     "text",
                        "analyzer": "german"
                      },
                      "en": {
                          "type":     "text",
                          "analyzer": "english"
                      },
                      "raw": {
                        "type": "keyword"
                      }
                    }
                  }
            }
        }))
        .send()
        .await?;
    /*
        let response = client
            .indices()
            .create(IndicesCreateParts::Index(INDEX_NAME))
            .body(json!({
              "mappings": {
                "properties": {

                }
              }
            }
            ))
            .send()
            .await?;
    */
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
            // TODO FIXME we always increase the version as we always index all documents
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
        .bulk(BulkParts::Index(INDEX_NAME))
        .body(body)
        .send()
        .await?;

    let exception = response.exception().await?;
    match exception {
        Some(exception) => Err(anyhow::anyhow!("{:?}", exception))?,
        None => {}
    };

    let response = client
        .search(SearchParts::Index(&[INDEX_NAME]))
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
