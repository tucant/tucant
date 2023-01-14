#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::vec_box,
    clippy::module_name_repetitions
)]
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use futures_util::Stream;

out_dir_include::out_dir_include!("debug-adapter-protocol.rs");

pub struct Server {
    pm: ProtocolMessage
}

impl Server {
    
}


pub fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { todo!() })
}
