use std::{fs::File, io::{BufReader, Result}};

include!(concat!(env!("OUT_DIR"), "/lsp.rs"));

#[test]
pub fn test_initialize() -> Result<()> {
    let file = File::open("tests/test_initialize.json")?;
    let reader = BufReader::new(file);
    let requests: Requests = serde_json::from_reader(reader)?;
    println!("{:#?}", requests);
    Ok(())
}

#[test]
pub fn test_initialize2() -> Result<()> {
    let file = File::open("tests/test_initialize2.json")?;
    let reader = BufReader::new(file);
    let requests: Requests = serde_json::from_reader(reader)?;
    println!("{:#?}", requests);
    Ok(())
}