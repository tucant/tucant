use crate::MyError;
use actix_session::Session;

use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{
    get,
    web::{Data, Path},
};

use tucan_scraper::tucan::Tucan;
use tucan_scraper::tucan_user::RegistrationEnum;
use tucan_scraper::tucan_user::TucanSession;
use tucan_scraper::url::Registration;
// trailing slash is menu
#[get("/modules/{menu_id:.*}")]
pub async fn get_modules<'a>(
    session: Session,
    tucan: Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    match session.get::<TucanSession>("session").unwrap() {
        Some(session) => {
            let tucan = tucan.continue_session(session).await.unwrap();

            let value = if path.is_empty() {
                RegistrationEnum::Submenu(vec![tucan.root_registration().await?])
            } else {
                tucan
                    .registration(Registration {
                        path: base64::decode(path.as_bytes()).unwrap(),
                    })
                    .await?
                    .1
            };

            Ok(HttpResponse::Ok().content_type("text/plain").json(value))
        }
        None => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("not logged in")),
    }
}
