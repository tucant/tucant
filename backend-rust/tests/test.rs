use axum::Json;
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
#[allow(dead_code)]
async fn index(_session: String, _input: Json<()>) -> axum::response::Result<Json<String>> {
    Ok(Json("Welcome Anonymous!".to_owned()))
}

#[test]
pub fn test() {
    use tucant_derive_lib::Typescriptable;

    // cargo test -- --show-output
    println!("{:?}", <Struct1 as Typescriptable>::code());
}
