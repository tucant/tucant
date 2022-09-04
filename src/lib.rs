#![feature(async_closure)]
pub mod tucan;
pub mod tucan_user;

use std::{
    io::{Error, ErrorKind},
    str::FromStr,
    sync::Arc,
};

use futures::stream::FuturesUnordered;
use reqwest::{cookie::Jar, Client, Url};
use scraper::{ElementRef, Html, Selector};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::sync::Semaphore;

fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn link_by_text<'a>(document: &'a Html, text: &str) -> &'a str {
    document
        .select(&s(r#"a"#))
        .find(|e| e.inner_html() == text)
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}
