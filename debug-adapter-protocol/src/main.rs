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
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};

out_dir_include::out_dir_include!("debug-adapter-protocol.rs");

pub struct Server {}

impl Server {
    async fn handle_sending<
        W: Sink<String, Error = anyhow::Error> + std::marker::Send + std::marker::Unpin,
    >(
        self: Arc<Self>,
        mut sender: W,
        mut rx: mpsc::Receiver<String>,
    ) -> anyhow::Result<()> {
        while let Some(result) = rx.recv().await {
            sender.send(result).await?;
        }

        Ok(())
    }

    async fn handle_receiving<
        R: Stream<Item = Result<String, anyhow::Error>> + std::marker::Send + std::marker::Unpin,
    >(
        self: Arc<Self>,
        mut reader: R,
    ) -> anyhow::Result<()> {
        loop {
            let read_value = reader.next().await.unwrap()?;
            println!("{read_value}");
            let request: Requests = serde_json::from_str(&read_value)?;
            // Requests

            println!("{:?}", request);

            let cloned_self = self.clone();
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
        let (tx, rx) = mpsc::channel::<String>(3);

        let arc_self = Arc::new(Self {});

        let handle1 = tokio::spawn(arc_self.clone().handle_receiving(read));
        let handle2 = tokio::spawn(arc_self.handle_sending(write, rx));

        handle1.await??;
        handle2.await??;

        Ok(())
    }
}

// cargo watch -x 'run -- --port 6009'
pub fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_json_rpc_server::<Server>().await })
}
