use async_trait::async_trait;
use bytes::Buf;
use bytes::BytesMut;
use clap::Parser;
use futures_util::Sink;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use itertools::Itertools;
use tokio::net::{TcpListener, UnixStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_util::codec::{Decoder, Encoder, FramedRead, FramedWrite};

#[async_trait]
pub trait JsonRpcServer {
    async fn run<
        R: Stream<Item = Result<String, anyhow::Error>>
            + std::marker::Unpin
            + std::marker::Send
            + 'static,
        W: Sink<String, Error = anyhow::Error> + std::marker::Unpin + std::marker::Send + 'static,
    >(
        read: R,
        write: W,
    ) -> anyhow::Result<()>;
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pipe: Option<String>,

    #[arg(long)]
    port: Option<u16>,

    #[arg(long)]
    stdin: bool,

    #[arg(long)]
    websocket: Option<u16>,
}

struct MyStringEncoder;

impl Encoder<String> for MyStringEncoder {
    type Error = anyhow::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(
            format!("Content-Length: {}\r\n\r\n", item.as_bytes().len()).as_bytes(),
        );
        dst.extend_from_slice(item.as_bytes());
        Ok(())
    }
}

struct MyStringDecoder;

impl Decoder for MyStringDecoder {
    type Item = String;
    type Error = anyhow::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // position, iter, split
        let mut it = buf
            .iter()
            .enumerate()
            .filter(|(_position, byte)| **byte == b'\n');
        let start = 0;
        if let Some((position, _)) = it.next() {
            //for (position, _) in it {
            let part = &buf[start..position];

            println!("Part {}", std::str::from_utf8(part).unwrap());

            let (key, value) = part.split(|b| *b == b':').tuples().exactly_one().unwrap();

            assert!(key == b"Content-Length");
            let length_string = std::str::from_utf8(value).unwrap().trim();
            let length = length_string.parse::<usize>().unwrap() + 2;

            println!(
                "len: {}, pos: {}, end: {}",
                buf.len(),
                position,
                position + length + 1
            );
            if position + length + 1 > buf.len() {
                return Ok(None);
            }
            let contents = &buf[position..=(position + length)];

            let return_value = std::str::from_utf8(contents).unwrap().to_string();
            buf.advance(position + length + 1);

            println!("{return_value}");
            return Ok(Some(return_value));

            //start = position;
            //}
        }
        Ok(None)
    }
}

// cargo doc --document-private-items --open
// cargo run -- --port 6008
// cargo watch -x 'run -- --port 6008'
pub async fn run_json_rpc_server<S: JsonRpcServer>() -> anyhow::Result<()> {
    let args = Args::parse();

    match args {
        Args {
            pipe: Some(pipe),
            stdin: false,
            port: None,
            websocket: None,
        } => {
            let stream = UnixStream::connect(pipe).await?;
            let (read, write) = stream.into_split();
            S::run(
                FramedRead::new(read, MyStringDecoder),
                FramedWrite::new(write, MyStringEncoder),
            )
            .await
        }
        Args {
            port: Some(port),
            pipe: None,
            stdin: false,
            websocket: None,
        } => {
            let listener = TcpListener::bind(("127.0.0.1", port)).await?;

            loop {
                let (stream, _) = listener.accept().await?;
                tokio::spawn(async {
                    let (read, write) = stream.into_split();
                    S::run(
                        FramedRead::new(read, MyStringDecoder),
                        FramedWrite::new(write, MyStringEncoder),
                    )
                    .await
                    .unwrap();
                });
            }
        }
        Args {
            websocket: Some(port),
            pipe: None,
            stdin: false,
            port: None,
        } => {
            let stream = TcpListener::bind(("127.0.0.1", port))
                .await?
                .accept()
                .await?
                .0;
            let ws_stream = tokio_tungstenite::accept_async(stream).await?;
            let (write, read) = ws_stream.split();
            S::run(
                read.filter_map(|item| {
                    Box::pin(async {
                        match item {
                            Ok(Message::Text(string)) => Some(Ok(string)),
                            _ => None,
                        }
                    })
                }),
                write.with(|v| Box::pin(async { Ok(Message::Text(v)) })),
            )
            .await
        }
        Args { pipe: None, .. } => {
            S::run(
                FramedRead::new(tokio::io::stdin(), MyStringDecoder),
                FramedWrite::new(tokio::io::stdout(), MyStringEncoder),
            )
            .await
        }
        _ => {
            panic!("can't enable multiple modes at the same time")
        }
    }
}
