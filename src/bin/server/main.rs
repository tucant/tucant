mod csrf_middleware;

use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, Responder};
use csrf_middleware::CsrfMiddleware;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResult {
    success: bool,
}

#[post("/login")]
async fn echo(login: web::Json<Login>) -> actix_web::Result<impl Responder> {
    Ok(web::Json(LoginResult {
        success: login.username == "hi"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_method().allow_any_header().allowed_origin("http://localhost:3000");

        App::new()
            .wrap(cors)
            .wrap(CsrfMiddleware {})
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
