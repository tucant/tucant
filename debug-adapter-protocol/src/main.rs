#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::vec_box,
    clippy::module_name_repetitions
)]
use async_trait::async_trait;
use futures_util::Sink;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use json_rpc_server::run_json_rpc_server;
use json_rpc_server::JsonRpcServer;
use serde::Deserialize;
use serde::Serialize;

use std::sync::Arc;

out_dir_include::out_dir_include!("debug-adapter-protocol.rs");

pub struct Server {}

impl Server {
    async fn handle<
        R: Stream<Item = Result<String, anyhow::Error>> + std::marker::Send + std::marker::Unpin,
        W: Sink<String, Error = anyhow::Error> + std::marker::Send + std::marker::Unpin,
    >(
        self: Arc<Self>,
        mut reader: R,
        mut sender: W,
    ) -> anyhow::Result<()> {
        let mut seq = 0;
        loop {
            let read_value = reader.next().await.unwrap()?;
            let request: Requests = serde_json::from_str(&read_value)?;

            match request {
                Requests::InitializeRequest(request) => {
                    let response = Response::<InitializeResponse> {
                        inner: Some(InitializeResponse { body: None }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::LaunchRequest(request) => {
                    // TODO FIXME abstract equal fields out
                    let response = Response::<LaunchResponse> {
                        inner: Some(LaunchResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::EvaluateRequest(request) => {
                    let response = Response::<EvaluateResponse> {
                        inner: Some(EvaluateResponse {
                            body: EvaluateResponseStructBody {
                                result: "42".to_string(),
                                r#type: Some("answer-to-question-about-sense-of-life".to_string()),
                                presentation_hint: Some(VariablePresentationHint {
                                    kind: Some("property".to_string()),
                                    attributes: Some(vec!["readOnly".to_string()]),
                                    visibility: Some("public".to_string()),
                                    lazy: Some(false),
                                }),
                                variables_reference: 1337,
                                named_variables: Some(10),
                                indexed_variables: Some(10),
                                memory_reference: Some("deadbeef".to_string()),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                request => unimplemented!("{:?}", request),
            }

            let _cloned_self = self.clone();
        }
    }
}

#[async_trait]
impl JsonRpcServer for Server {
    async fn run<
        R: Stream<Item = Result<String, anyhow::Error>>
            + std::marker::Unpin
            + std::marker::Send
            + 'static,
        W: Sink<String, Error = anyhow::Error> + std::marker::Unpin + std::marker::Send + 'static,
    >(
        read: R,
        write: W,
    ) -> anyhow::Result<()> {
        let arc_self = Arc::new(Self {});

        arc_self.handle(read, write).await?;

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    #[serde(flatten)]
    inner: T,
    seq: u64,
    r#type: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(flatten)]
    inner: Option<T>, // probably Result
    seq: u64,
    r#type: String,
    request_seq: u64,
    success: bool,
    message: Option<String>,
}

// cargo watch -x 'run -- --port 6009'
pub fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_json_rpc_server::<Server>().await })
}
