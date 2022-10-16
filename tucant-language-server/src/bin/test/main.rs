use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<T> {
    id: i64,
    params: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification<T> {
    params: T
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
pub enum ReceivedSomething {
    RequestType1(Request<i32>),
    RequestType2(Request<String>),
    NotificationType1(Request<i32>)
}

pub fn main() {
    let input1 = r#"{"jsonrpc": "2.0", "method": "RequestType2", "params": "test", "id": 1}"#;
    let value1: ReceivedSomething = serde_json::from_str(input1).unwrap();
    println!("{:?}", value1);
}