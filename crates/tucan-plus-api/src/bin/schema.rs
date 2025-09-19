use tucant_api::router;

pub fn main() {
    println!("{}", router().to_openapi().to_pretty_json().unwrap());
}
