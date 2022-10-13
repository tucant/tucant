
use clap::Parser;
use itertools::Itertools;
use tokio::{io::{self, BufStream, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::UnixStream};
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

                let result = Responses::InitializeResponse(InitializeResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    error: None,
                    result: Some(Box::new(InitializeResult {
                        capabilities: Box::new(ServerCapabilities {
                            position_encoding: None,
                            text_document_sync: Some(H1e2267041560020dc953eb5d9d8f0c194de0f657a1193f66abeab062::Variant0(Box::new(TextDocumentSyncOptions {
                                open_close: Some(true),
                                will_save: None,
                                will_save_wait_until: None,
                                change: None, // TODO FIXME
                                save: None, // TODO FIXME
                            }))),
                            notebook_document_sync: None,
                            completion_provider: None,
                            hover_provider: None,
                            signature_help_provider: None,
                            declaration_provider: None,
                            definition_provider: None,
                            type_definition_provider: None,
                            implementation_provider: None,
                            references_provider: None,
                            document_highlight_provider: None,
                            document_symbol_provider: None,
                            code_action_provider: None,
                            code_lens_provider: None,
                            document_link_provider: None,
                            color_provider: None,
                            workspace_symbol_provider: None,
                            document_formatting_provider: None,
                            document_range_formatting_provider: None,
                            document_on_type_formatting_provider: None,
                            rename_provider: None,
                            folding_range_provider: None,
                            selection_range_provider: None,
                            execute_command_provider: None,
                            call_hierarchy_provider: None,
                            linked_editing_range_provider: None,
                            semantic_tokens_provider: None,
                            moniker_provider: None,
                            type_hierarchy_provider: None,
                            inline_value_provider: None,
                            inlay_hint_provider: None,
                            diagnostic_provider: None,
                            workspace: None,
                            experimental: None
                        }),
                        server_info: Some(H07206713e0ac2e546d7755e84916a71622d6302f44063c913d615b41 { name: "TUCaN't".to_string(), version: Some("0.0.1".to_string()) })
                    }))
                });

                let result = serde_json::to_string(&result)?;

                println!("{}", result);

                pipe.write_all(format!("Content-Length: {}\r\n\r\n", result.as_bytes().len()).as_bytes()).await?;

                pipe.write_all(result.as_bytes()).await?;

                println!("wrote response 6!");
            }
            _ => panic!("unknown request")
        }

        buf.clear();
    }

    Ok(())
}
