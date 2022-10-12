use std::{fs::File, io::{BufReader, Result}};

use tucant_language_server_derive::magic;

magic!();

#[test]
pub fn test_initialize() -> Result<()> {
    let file = File::open("tests/test_initialize.json")?;
    let reader = BufReader::new(file);
    let requests: Requests = serde_json::from_reader(reader)?;

    Ok(())
}