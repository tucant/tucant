use crate::TucanError;
use bytes::Bytes;
use tokio::io::AsyncBufReadExt as _;

pub struct StreamingHttpParser<R: tokio::io::AsyncBufRead> {
    pub(crate) async_read: R,
}

impl<R: tokio::io::AsyncBufRead + std::marker::Unpin> StreamingHttpParser<R> {
    pub async fn doctype(&mut self) -> Result<(), TucanError> {
        let mut buffer = Vec::new();
        let size = self.async_read.read_until(b'>', &mut buffer).await?;
        println!("{:?}", buffer);
        Ok(())
    }
}
