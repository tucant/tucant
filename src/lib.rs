#![feature(async_closure)]
pub mod models;
pub mod schema;
pub mod tucan;
pub mod tucan_user;

use diesel::{PgConnection, Connection, r2d2::{Builder, ConnectionManager, Pool}};
use dotenvy::dotenv;
use scraper::{ElementRef, Html, Selector};

fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap()
}