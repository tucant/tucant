use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::stream::{BoxStream, FuturesOrdered};
use futures_util::{FutureExt as _, StreamExt as _};
use tokio::io::AsyncWriteExt as _;
use tucan_types::TucanError;
use tucan_types::{
    DynTucan, LoginResponse, RevalidationStrategy, Tucan as _,
    registration::{AnmeldungRequest, AnmeldungResponse},
};

pub async fn compress(in_data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = async_compression::tokio::write::BrotliEncoder::with_quality(
        Vec::new(),
        async_compression::Level::Best,
    );
    encoder.write_all(in_data).await?;
    encoder.shutdown().await?;
    Ok(encoder.into_inner())
}

pub async fn decompress(in_data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = async_compression::tokio::write::BrotliDecoder::new(Vec::new());
    decoder.write_all(in_data).await?;
    decoder.shutdown().await?;
    Ok(decoder.into_inner())
}


pub fn abc(anmeldung_responses: Vec<AnmeldungResponse>) {
    
}
