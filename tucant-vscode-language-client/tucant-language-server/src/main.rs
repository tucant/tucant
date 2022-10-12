
use clap::Parser;
use itertools::Itertools;
use tokio::{io::{self, BufStream, AsyncBufReadExt, AsyncReadExt}, net::UnixStream};
use tucant_language_server_derive::magic;

magic!();

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pipe: String
}

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

// cargo doc --document-private-items --open
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Pipe: {}", args.pipe);

    let pipe = UnixStream::connect(args.pipe).await?;
    let mut pipe = BufStream::new(pipe);

    let mut buf = Vec::new();
    loop {
        pipe.read_until(b'\n', &mut buf).await?;

        if buf == [13, 10] {
            break;
        }

        let (key, value) = buf.split(|b| *b == b':').tuples().exactly_one().unwrap();

        assert!(key == b"Content-Length");

        let length_string = std::str::from_utf8(value).unwrap().trim();

        let length = length_string.parse::<usize>().unwrap() + 2;

        buf.resize(length, 0);

        pipe.read_exact(&mut buf).await?;       
        
        // println!("read: {}", std::str::from_utf8(&buf).unwrap());

        let request: Requests = serde_json::from_slice(&buf)?;

        match request {
            Requests::InitializeRequest(request) => {
                println!("got an initialize {:#?}", request);

                let result = InitializeResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    error: None,
                    result: InitializeResult {

                    }
                };
            }
            _ => panic!("unknown request")
        }

        buf.clear();
    }

    Ok(())
}
