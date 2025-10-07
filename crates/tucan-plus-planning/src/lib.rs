use futures_util::stream::FuturesOrdered;
use futures_util::{FutureExt as _, StreamExt as _};
use tokio::io::AsyncWriteExt as _;
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

#[expect(clippy::manual_async_fn)]
pub fn recursive_anmeldung<'a, 'b>(
    tucan: &'a DynTucan<'static>,
    login_response: &'b LoginResponse,
    anmeldung_request: AnmeldungRequest,
) -> impl futures_util::Stream<Item = Vec<AnmeldungResponse>> + use<'a, 'b> {
     tucan.anmeldung(
        login_response,
        RevalidationStrategy::cache(),
        anmeldung_request.clone(),
    ).into_stream().flat_map(|element| {
        let element = element.unwrap();
        futures_util::stream::iter(element
            .submenus
            .into_iter())
            .flat_map(|entry| {
                recursive_anmeldung(tucan, login_response, entry.1.clone())
            })
    })
}

pub fn abc(anmeldung_responses: Vec<AnmeldungResponse>) {
    
}
