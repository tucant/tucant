mod csrf_middleware;

use std::future::{ready, Ready};

use actix_cors::Cors;
use actix_web::dev::{Response, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorForbidden;
use actix_web::{get, http, post, web, App, Either, Error, HttpResponse, HttpServer, Responder};
use csrf_middleware::CsrfMiddleware;
use futures::future::{ok, LocalBoxFuture};
use futures::FutureExt;
use reqwest::header::{HeaderName, HeaderValue};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .allowed_header(HeaderName::from_static("x-csrf-protection"))
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(CsrfMiddleware {})
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
