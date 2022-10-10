use std::env;
use clap::Parser;
use itertools::Itertools;
use tokio::{fs::File, io::{BufWriter, self, AsyncWriteExt, BufStream, AsyncBufReadExt, AsyncReadExt}, net::UnixStream};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pipe: String
}

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

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

        let length: usize = length_string.parse().unwrap();

        buf.resize(length, 0);

        pipe.read_exact(&mut buf).await?;       
        
        println!("read: {}", std::str::from_utf8(&buf).unwrap());

        buf.clear();
    }

    Ok(())
}
