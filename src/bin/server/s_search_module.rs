use actix_web::{get, web::{Data, Query, Json}};
use diesel_full_text_search::{configuration::TsConfigurationByName, websearch_to_tsquery_with_search_config, ts_rank_cd_normalized, ts_headline_with_search_config};
use tucan_scraper::{tucan::Tucan, schema::modules_unfinished};
use actix_web::Responder;
use crate::{SearchQuery, MyError};
use diesel::QueryDsl;
use diesel_full_text_search::TsVectorExtensions;
use diesel::ExpressionMethods;
use diesel::TextExpressionMethods;
use diesel_async::RunQueryDsl;

#[get("/search-module")]
pub async fn search_module(
    tucan: Data<Tucan>,
    search_query: Query<SearchQuery>,
) -> Result<impl Responder, MyError> {
    // http://localhost:8080/search-module?q=digitale%20schaltung
    let mut connection = tucan.pool.get().await?;

    let config = TsConfigurationByName("tucan");
    let tsvector = modules_unfinished::tsv;
    let tsquery = websearch_to_tsquery_with_search_config(config, &search_query.q);
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

    let result = sql_query
        .load::<(Vec<u8>, String, String, f32)>(&mut connection)
        .await?;

    Ok(Json(result))
}
