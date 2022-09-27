use crate::{MyError, SearchQuery};
use actix_web::Responder;
use actix_web::{
    get,
    web::{Data, Json, Query},
};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::TextExpressionMethods;
use diesel_async::RunQueryDsl;
use diesel_full_text_search::TsVectorExtensions;
use diesel_full_text_search::{
    configuration::TsConfigurationByName, ts_headline_with_search_config, ts_rank_cd_normalized,
    websearch_to_tsquery_with_search_config,
};
use tucan_scraper::{schema::courses_unfinished, tucan::Tucan};

#[get("/search-course")]
pub async fn search_course(
    tucan: Data<Tucan>,
    search_query: Query<SearchQuery>,
) -> Result<impl Responder, MyError> {
    let mut connection = tucan.pool.get().await?;

    let config = TsConfigurationByName("tucan");
    let tsvector = courses_unfinished::tsv;
    let tsquery = websearch_to_tsquery_with_search_config(config, &search_query.q);
    let rank = ts_rank_cd_normalized(tsvector, tsquery, 1);
    let sql_query = courses_unfinished::table
        .filter(tsvector.matches(tsquery))
        .order_by(rank.desc())
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
            rank,
        ));

    let result = sql_query
        .load::<(Vec<u8>, String, String, f32)>(&mut connection)
        .await?;

    Ok(Json(result))
}
