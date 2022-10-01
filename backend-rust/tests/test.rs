use actix_web::{
    get,
    web::{self, Json},
};
use tucant_derive::{ts, Typescriptable};

// cargo install cargo-expand
// cargo expand --test test

// https://github.com/dtolnay/proc-macro-workshop

#[derive(Typescriptable)]
pub struct Struct1 {
    _val1: u32,
    _val2: String,
}

#[ts]
#[get("/")]
async fn index(_session: String, _input: Json<()>) -> actix_web::Result<Json<String>> {
    Ok(web::Json("Welcome Anonymous!".to_owned()))
}

#[test]
pub fn test() {
    // cargo test -- --show-output
    println!(
        "{:?}",
        <Struct1 as ::tucant::typescript::Typescriptable>::code()
    );
    println!(
        "{:?}",
        <index as ::tucant::typescript::Typescriptable>::code()
    );
}
