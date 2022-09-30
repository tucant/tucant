use actix_web::{get, web::{self, Json}};
use tucant_derive::ts;

// cargo install cargo-expand
// cargo expand --test test

// https://github.com/dtolnay/proc-macro-workshop


#[ts]
pub struct Struct1 {
    val1: u32,
    val2: String,
}

#[ts]
#[get("/")]
async fn index(session: String) -> actix_web::Result<Json<String>> {
    Ok(web::Json("Welcome Anonymous!".to_owned()))
}

#[test]
pub fn test() {
    // cargo test -- --show-output
    println!("{}", <Struct1 as ::tucant::typescript::Typescriptable>::code()); 
    println!("{}", <index as ::tucant::typescript::Typescriptable>::code());       
}