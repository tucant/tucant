use actix_web::get;
use tucant_derive::ts;

// cargo install cargo-expand
// cargo expand --test test

// https://github.com/dtolnay/proc-macro-workshop


#[ts]
pub struct Struct1 {
    val1: u32,
    val2: String,
}


#[test]
pub fn test() {
    // cargo test -- --show-output
    println!("{}", <Struct1 as ::tucant::typescript::Typescriptable>::code());    
}