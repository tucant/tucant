use tucant_derive::typescript;

// cargo install cargo-expand
// cargo expand --test test

// https://github.com/dtolnay/proc-macro-workshop

#[typescript]
struct Struct1 {
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
