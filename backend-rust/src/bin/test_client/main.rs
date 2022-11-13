// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_web::web;
use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::{
        request::JsonBody,
        transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    },
    indices::IndicesCreateParts,
    BulkParts, IndexParts, OpenSearch, SearchParts,
};
use reqwest::Url;
use serde_json::{json, Value};
use tucant::{tucan::Tucan, url::parse_tucan_url};

// $HOME/.cargo/bin/diesel database reset && cargo run --bin test_client
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let url = Url::parse("https://localhost:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .auth(Credentials::Basic("admin".to_string(), "admin".to_string()))
        .cert_validation(CertificateValidation::None)
        .build()?;
    let client = OpenSearch::new(transport);

    client
        .indices()
        .create(IndicesCreateParts::Index("test_index"))
        .body(json!({
            "mappings" : {
                "properties" : {
                    "message" : { "type" : "text" }
                }
            }
        }))
        .send()
        .await?;

    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);

    // add the first operation and document
    body.push(json!({"index": {"_id": "1"}}).into());
    body.push(
        json!({
            "id": 1,
            "user": "kimchy",
            "post_date": "2009-11-15T00:00:00Z",
            "message": "Trying out OpenSearch, so far so good?"
        })
        .into(),
    );

    // add the second operation and document
    body.push(json!({"index": {"_id": "2"}}).into());
    body.push(
        json!({
            "id": 2,
            "user": "forloop",
            "post_date": "2020-01-08T00:00:00Z",
            "message": "Bulk indexing with the rust client, yeah!"
        })
        .into(),
    );

    let response = client
        .bulk(BulkParts::Index("test_index"))
        .body(body)
        .send()
        .await?;

    let response = client
        .search(SearchParts::Index(&["test_index"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "match": {
                    "message": "Open"
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
        println!("{:?}", hit["_source"]);
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
