use actix_web::{
    post,
    web::{self, Json},
};
use tucant_derive::{ts, Typescriptable};

// cargo install cargo-expand
// cargo expand --test test

// https://github.com/dtolnay/proc-macro-workshop

#[derive(Typescriptable)]
pub struct Struct1 {
    #[ts_type(String)]
    pub tucan_id: Vec<u8>,
}

#[ts]
#[post("/")]
async fn index(_session: String, _input: Json<()>) -> actix_web::Result<Json<String>> {
    Ok(web::Json("Welcome Anonymous!".to_owned()))
}

#[test]
pub fn test() {
    use tucant_derive_lib::Typescriptable;

    // cargo test -- --show-output
    println!("{:?}", <Struct1 as Typescriptable>::code());
    println!("{:?}", <index as Typescriptable>::code());
}
