mod parser;

use std::{collections::HashMap, path::Path, pin::Pin, vec};

use clap::Parser;
use itertools::Itertools;
use tokio::{
    io::{
        self, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufStream,
        Stdin, Stdout,
    },
    net::{TcpListener, UnixStream},
};
use tucant_language_server_derive_output::{
    Diagnostic, DiagnosticSeverity, H07206713e0ac2e546d7755e84916a71622d6302f44063c913d615b41,
    H1e2267041560020dc953eb5d9d8f0c194de0f657a1193f66abeab062,
    H3424688d17603d45dbf7bc9bc9337e660ef00dd90b070777859fbf1e,
    H560683c9a528918bcd8e6562ca5d336a5b02f2a471cc7f47a6952222,
    Hb33d389f4db33e188f5f7289bda48f700ee05a6244701313be32e552,
    He98ccfdc940d4c1fa4b43794669192a12c560d6457d392bc00630cb4, InitializeResponse,
    InitializeResult, MessageType, Position, PublishDiagnosticsParams, Range, Requests, Responses,
    SemanticTokens, SemanticTokensLegend, SemanticTokensOptions, ServerCapabilities,
    ShowMessageParams, ShutdownResponse, TextDocumentPublishDiagnosticsNotification,
    TextDocumentSemanticTokensFullResponse, TextDocumentSyncKind, TextDocumentSyncOptions,
    WindowShowMessageNotification, WorkDoneProgressOptions,
};

use crate::parser::{parse_root, visitor, Error, Span};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pipe: Option<String>,

    #[arg(long)]
    port: Option<u16>,

    #[arg(long)]
    stdin: bool,
}

pub struct StdinoutStream {
    stdin: Pin<Box<Stdin>>,
    stdout: Pin<Box<Stdout>>,
}

impl AsyncRead for StdinoutStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.stdin.as_mut().poll_read(cx, buf)
    }
}

impl AsyncWrite for StdinoutStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        self.stdout.as_mut().poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.stdout.as_mut().poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.stdout.as_mut().poll_shutdown(cx)
    }
}

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

pub async fn recalculate_diagnostics<T: AsyncRead + AsyncWrite + std::marker::Unpin>(
    pipe: &mut BufStream<T>,
    content: &str,
    uri: String,
    version: i64,
) -> io::Result<()> {
    let span = Span::new(content);
    let value = parse_root(span);
    println!("{:?}", value);

    let diagnostics = if let Err(error) = value {
        let start_pos = error.location.start_line_column();
        let end_pos = error.location.end_line_column();

        vec![Box::new(Diagnostic {
            range: Box::new(Range {
                start: Box::new(Position {
                    line: start_pos.0.try_into().unwrap(),
                    character: start_pos.1.try_into().unwrap(),
                }),
                end: Box::new(Position {
                    line: end_pos.0.try_into().unwrap(),
                    character: end_pos.1.try_into().unwrap(),
                }),
            }),
            severity: Some(Box::new(DiagnosticSeverity::Error)),
            code: None,
            code_description: None,
            source: Some("tucant".to_string()),
            message: error.reason.to_string(),
            tags: None,
            related_information: None,
            data: None,
        })]
    } else {
        vec![]
    };

    let response = Responses::TextDocumentPublishDiagnosticsNotification(
        TextDocumentPublishDiagnosticsNotification {
            jsonrpc: "2.0".to_string(),
            params: Box::new(PublishDiagnosticsParams {
                uri,
                version: Some(version),
                diagnostics,
            }),
        },
    );

    let response = serde_json::to_string(&response)?;

    println!("{}", response);

    pipe.write_all(format!("Content-Length: {}\r\n\r\n", response.as_bytes().len()).as_bytes())
        .await?;

    pipe.write_all(response.as_bytes()).await?;

    pipe.flush().await?;

    println!("wrote diagnostics response!");

    Ok(())
}

async fn main_internal<T: AsyncRead + AsyncWrite + std::marker::Unpin>(
    readwrite: T,
) -> io::Result<()> {
    let mut documents = HashMap::new();

    let mut pipe = BufStream::new(readwrite);

    let mut buf = Vec::new();
    loop {
        pipe.read_until(b'\n', &mut buf).await?;

        if buf == [13, 10] {
            break;
        }

        println!("read: {}", std::str::from_utf8(&buf).unwrap());

        let (key, value) = buf.split(|b| *b == b':').tuples().exactly_one().unwrap();

        assert!(key == b"Content-Length");

        let length_string = std::str::from_utf8(value).unwrap().trim();

        let length = length_string.parse::<usize>().unwrap() + 2;

        buf.resize(length, 0);

        pipe.read_exact(&mut buf).await?;

        println!("read: {}", std::str::from_utf8(&buf).unwrap());

        let request: Requests = serde_json::from_slice(&buf)?;

        buf.clear();

        match request {
            Requests::InitializeRequest(request) => {
                println!("got an initialize {:#?}", request);

                let result = InitializeResponse {
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
                                change: Some(Box::new(TextDocumentSyncKind::Full)),
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
                            semantic_tokens_provider: Some(Hb33d389f4db33e188f5f7289bda48f700ee05a6244701313be32e552::Variant0(Box::new(SemanticTokensOptions {
                                legend: Box::new(SemanticTokensLegend {
                                    token_types: vec!["string".to_string(), "number".to_string(), "type".to_string()],
                                    token_modifiers: vec![],
                                }),
                                variant0: Box::new(WorkDoneProgressOptions {
                                    work_done_progress: None,
                                }),
                                range: Some(H3424688d17603d45dbf7bc9bc9337e660ef00dd90b070777859fbf1e::Variant0(false)),
                                full: Some(H560683c9a528918bcd8e6562ca5d336a5b02f2a471cc7f47a6952222::Variant0(true)),
                            }))),
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
                };

                let result = serde_json::to_string(&result)?;

                println!("{}", result);

                pipe.write_all(
                    format!("Content-Length: {}\r\n\r\n", result.as_bytes().len()).as_bytes(),
                )
                .await?;

                pipe.write_all(result.as_bytes()).await?;

                pipe.flush().await?;

                println!("wrote initialize response!");
            }
            Requests::InitializedNotification(_notification) => {
                let notification =
                    Responses::WindowShowMessageNotification(WindowShowMessageNotification {
                        jsonrpc: "2.0".to_string(),
                        params: Box::new(ShowMessageParams {
                            r#type: Box::new(MessageType::Error),
                            message: "This is a test error".to_string(),
                        }),
                    });

                let result = serde_json::to_string(&notification)?;

                println!("{}", result);

                pipe.write_all(
                    format!("Content-Length: {}\r\n\r\n", result.as_bytes().len()).as_bytes(),
                )
                .await?;

                pipe.write_all(result.as_bytes()).await?;

                pipe.flush().await?;

                println!("wrote notification!");
            }
            Requests::TextDocumentDidChangeNotification(notification) => {
                println!("{:?}", notification.params.content_changes);

                for change in notification.params.content_changes {
                    match *change {
                        tucant_language_server_derive_output::H25fd6c7696dff041d913d0a9d3ce2232683e5362f0d4c6ca6179cf92::Variant0(_) => todo!(),
                        tucant_language_server_derive_output::H25fd6c7696dff041d913d0a9d3ce2232683e5362f0d4c6ca6179cf92::Variant1(changes) => {
                            documents.insert(notification.params.text_document.variant0.uri.clone(), changes.text.clone());
                        },
                    }
                }

                recalculate_diagnostics(
                    &mut pipe,
                    documents
                        .get(&notification.params.text_document.variant0.uri)
                        .unwrap(),
                    notification.params.text_document.variant0.uri,
                    notification.params.text_document.version,
                )
                .await?;
            }
            Requests::TextDocumentDidOpenNotification(notification) => {
                documents.insert(
                    notification.params.text_document.uri.clone(),
                    notification.params.text_document.text.clone(),
                );

                recalculate_diagnostics(
                    &mut pipe,
                    &notification.params.text_document.text,
                    notification.params.text_document.uri,
                    notification.params.text_document.version,
                )
                .await?;
            }
            Requests::ShutdownRequest(request) => {
                let response = ShutdownResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(()),
                };

                let response = serde_json::to_string(&response)?;

                println!("{}", response);

                pipe.write_all(
                    format!("Content-Length: {}\r\n\r\n", response.as_bytes().len()).as_bytes(),
                )
                .await?;

                pipe.write_all(response.as_bytes()).await?;

                pipe.flush().await?;

                println!("wrote shutdown response!");
            }
            Requests::ExitNotification(_notification) => {
                println!("exited!");
                break;
            }
            Requests::TextDocumentSemanticTokensFullRequest(request) => {
                let document = documents.get(&request.params.text_document.uri);
                let document = if let Some(document) = document {
                    document.clone()
                } else {
                    tokio::fs::read_to_string(request.params.text_document.uri).await?
                };

                let span = Span::new(&document);
                let value = match parse_root(span) {
                    Ok((value, _)) => value,
                    Err(Error { partial_parse, .. }) => partial_parse,
                };

                println!("{:?}", value);

                let paired_iterator = std::iter::once((0, 0, 0, 0, 0))
                    .chain(visitor(&value))
                    .zip(visitor(&value));
                let result = paired_iterator
                    .flat_map(|(last, this)| {
                        vec![
                            this.0 - last.0,
                            if this.0 == last.0 {
                                this.1 - last.1
                            } else {
                                this.1
                            },
                            this.2,
                            this.3,
                            this.4,
                        ]
                    })
                    .collect::<Vec<_>>();

                let response = TextDocumentSemanticTokensFullResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(
                        He98ccfdc940d4c1fa4b43794669192a12c560d6457d392bc00630cb4::Variant0(
                            Box::new(SemanticTokens {
                                result_id: None,
                                data: result,
                            }),
                        ),
                    ),
                };

                let response = serde_json::to_string(&response)?;

                println!("{}", response);

                pipe.write_all(
                    format!("Content-Length: {}\r\n\r\n", response.as_bytes().len()).as_bytes(),
                )
                .await?;

                pipe.write_all(response.as_bytes()).await?;

                pipe.flush().await?;

                println!("wrote semantic tokens response!");
            }
            Requests::SetTraceNotification(_) => {}
            Requests::TextDocumentDidCloseNotification(notification) => {
                documents.remove(&notification.params.text_document.uri);
            }
            other => panic!("{:?}", other),
        }
    }

    Ok(())
}

// cargo doc --document-private-items --open
// cargo run -- --port 6008
// cargo watch -x 'run -- --port 6008'
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    match args {
        Args {
            pipe: Some(pipe),
            stdin: false,
            port: None,
        } => main_internal(UnixStream::connect(pipe).await?).await,
        Args {
            port: Some(port),
            pipe: None,
            stdin: false,
        } => {
            main_internal(
                TcpListener::bind(("127.0.0.1", port))
                    .await?
                    .accept()
                    .await?
                    .0,
            )
            .await
        }
        Args { pipe: None, .. } => {
            main_internal(StdinoutStream {
                stdin: Box::pin(tokio::io::stdin()),
                stdout: Box::pin(tokio::io::stdout()),
            })
            .await
        }
        _ => {
            panic!("can't enable multiple modes at the same time")
        }
    }
}
