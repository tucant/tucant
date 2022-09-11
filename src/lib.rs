#![feature(async_closure)]
pub mod models;
pub mod schema;
pub mod tucan;
pub mod tucan_user;

use diesel::{
    Connection, PgConnection,
};
use diesel_async::{pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool}, AsyncPgConnection};
use dotenvy::dotenv;
use scraper::{ElementRef, Html, Selector};

fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}

fn get_config() -> AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    config
}

fn create_pool() -> deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    let config = get_config();
    Pool::builder(config).build().unwrap()
}