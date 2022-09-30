use tucant_derive::typescript;

// cargo install cargo-expand
// cargo expand --test test

// https://github.com/dtolnay/proc-macro-workshop

pub trait Typescriptable {
    fn name() -> String;
    fn code() -> String { "".to_string() }
}

impl Typescriptable for u32 {
    fn name() -> String {
        "number".to_string()
    }
}


impl Typescriptable for String {
    fn name() -> String {
        "string".to_string()
    }
}

#[typescript]
pub struct Struct1 {
    val1: u32,
    val2: String,
}

#[typescript]
pub fn fake_request(input: Struct1) -> Struct1 {
    Struct1 {
        val1: 1,
        val2: "".to_string(),
    }
}

#[test]
pub fn test() {
    println!("{}", Struct1::name());
}