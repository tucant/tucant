#![feature(async_closure)]
pub mod tucan;
pub mod tucan_user;





use scraper::{ElementRef, Html, Selector};



fn s(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn element_by_selector<'a>(document: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    document.select(&s(selector)).next()
}
