use crate::MyError;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::Responder;
use actix_web::{
    post,
    web::{Data, Path},
};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use tucan_scraper::{
    models::Course,
    schema::courses_unfinished,
    tucan::Tucan,
};

#[post("/course/{id}")]
pub async fn course(
    _: Session,
    tucan: Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    let mut connection = tucan.pool.get().await?;

    let course_result = courses_unfinished::table
        .filter(courses_unfinished::tucan_id.eq(path.as_bytes()))
        .select((
            courses_unfinished::tucan_id,
            courses_unfinished::tucan_last_checked,
            courses_unfinished::title,
            courses_unfinished::course_id,
            courses_unfinished::sws,
            courses_unfinished::content,
            courses_unfinished::done,
        ))
        .load::<Course>(&mut connection)
        .await?;

    Ok(Json(course_result))
}
