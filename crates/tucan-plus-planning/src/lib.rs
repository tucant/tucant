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

// breath first for progress?
// maybe us a channel?
// atomic for progress?
#[expect(clippy::manual_async_fn)]
pub fn recursive_anmeldung<'a, 'b: 'a>(
    tucan: &'a DynTucan<'static>,
    login_response: &'b LoginResponse,
    atomic_current: &'b AtomicUsize,
    atomic_total: &'b AtomicUsize,
    anmeldung_request: AnmeldungRequest,
) -> BoxStream<'a, AnmeldungResponse> {
    tucan.anmeldung(
        login_response,
        RevalidationStrategy::cache(),
        anmeldung_request.clone(),
    ).into_stream().flat_map(move |element: Result<AnmeldungResponse, TucanError>| {
        let element = element.unwrap();
        atomic_total.fetch_add(element.submenus.len(), Ordering::Relaxed);
        atomic_current.fetch_add(1, Ordering::Relaxed);
        futures_util::stream::iter(element
            .submenus.clone()
            .into_iter())
            .flat_map(move |entry| {
                recursive_anmeldung(tucan, login_response, atomic_current, atomic_total, entry.1.clone())
            })
    }).boxed()
}

pub fn abc(anmeldung_responses: Vec<AnmeldungResponse>) {
    
}
