
pub fn main() {
    let input1 = r#"{"jsonrpc": "2.0", "method": "RequestType2", "params": "test", "id": 1}"#;
    let value1: ReceivedSomething = serde_json::from_str(input1).unwrap();
    println!("{:?}", value1);
}