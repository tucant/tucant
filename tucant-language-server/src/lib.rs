use std::sync::Arc;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<Req, Res> {
    id: i64,
    pub params: Req,
    phantom_data: Res,
}

impl<Req, Res> Request<Req, Res> {
    pub async fn respond(&self, handler: Arc<()>, value: Res) {

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification<T> {
    pub params: T
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
pub enum ReceivedSomething {
    RequestType1(Request<i32, String>),
    RequestType2(Request<String, i32>),
    NotificationType1(Notification<i32>)
}
