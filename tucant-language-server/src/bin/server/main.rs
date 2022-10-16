mod idea;
mod parser;

use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc, vec, marker::PhantomData};

use clap::Parser;
use itertools::Itertools;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::{
    io::{
        self, AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt,
        BufReader, BufStream, ReadHalf, Stdin, Stdout, WriteHalf,
    },
    net::{TcpListener, UnixStream},
    sync::{mpsc, oneshot, RwLock},
};
use tucant_language_server_derive_output::*;

use crate::parser::{line_column_to_offset, parse_root, visitor, Error, Span};

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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<Req, Res> {
    id: String,
    pub params: Req,
    phantom_data: PhantomData<Res>,
}

impl<Req, Res> Request<Req, Res> {
    pub fn new(value: Req) -> Self {
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        Self {
            id: rand_string,
            params: value,
            phantom_data: PhantomData,
        }
    }

    pub async fn respond(&self, handler: Arc<Server>, value: Res) {}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification<T> {
    pub params: T,
}

impl<T> Notification<T> {
    pub fn new(value: T) -> Self {
        Self {
            params: value
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
pub enum ReceivedSomething {
    RequestType1(Request<i32, String>),
    RequestType2(Request<String, i32>),
    NotificationType1(Notification<i32>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
pub enum SendSomething {
    RequestType1(Request<i32, String>),
    RequestType2(Request<String, i32>),
    NotificationType1(Notification<i32>),
}

pub struct Server {
    documents: RwLock<HashMap<String, String>>,
    pending: RwLock<HashMap<String, oneshot::Sender<String>>>,
    tx: mpsc::Sender<String>,
}

impl Server {
    async fn handle_receiving<R: AsyncBufRead + std::marker::Unpin>(
        self: Arc<Self>,
        mut reader: R,
    ) -> io::Result<()> {
        let mut buf = Vec::new();
        loop {
            reader.read_until(b'\n', &mut buf).await?;

            if buf == [13, 10] {
                break;
            }

            let (key, value) = buf.split(|b| *b == b':').tuples().exactly_one().unwrap();

            assert!(key == b"Content-Length");

            let length_string = std::str::from_utf8(value).unwrap().trim();

            let length = length_string.parse::<usize>().unwrap() + 2;

            buf.resize(length, 0);

            reader.read_exact(&mut buf).await?;

            println!("read: {}", std::str::from_utf8(&buf).unwrap());

            let request: ReceivedSomething = serde_json::from_slice(&buf)?;

            buf.clear();

            let cloned_self = self.clone();

            // currently most of these are really not safe to run concurrently
            //tokio::spawn(async move {
            match request {
                ReceivedSomething::RequestType1(request) => {
                    cloned_self.handle_RequestType1(request).await.unwrap()
                }
                ReceivedSomething::RequestType2(_) => todo!(),
                ReceivedSomething::NotificationType1(_) => todo!(),
            }
            //});
        }

        Ok(())
    }

    async fn handle_RequestType1(self: Arc<Self>, request: Request<i32, String>) -> io::Result<()> {
        request
            .respond(self.clone(), format!("hello {}", request.params).to_string())
            .await;

        self.send_something(SendSomething::NotificationType1(Notification::new(5))).await?;

        Ok(())
    }

    // TODO FIXME create such a method for every type and don't use the enum then (at least not for the caller).
    async fn send_something(self: Arc<Self>, something: SendSomething) -> io::Result<String> {
        let (tx, rx) = oneshot::channel();
        match something {
            SendSomething::RequestType1(request) => {
                let mut pending = self.pending.write().await;
                pending.insert(request.id, tx);
            },
            SendSomething::RequestType2(_) => todo!(),
            SendSomething::NotificationType1(_) => todo!(),
        }
        
        let result = serde_json::to_string(&something)?;

        self.tx.send(result).await;

        Ok(rx.await?)
    }

    async fn handle_sending<W: AsyncWrite + std::marker::Unpin>(
        self: Arc<Self>,
        mut sender: W,
        mut rx: mpsc::Receiver<String>,
    ) -> io::Result<()> {
        while let Some(result) = rx.recv().await {
            sender
                .write_all(
                    format!("Content-Length: {}\r\n\r\n", result.as_bytes().len()).as_bytes(),
                )
                .await?;

            sender.write_all(result.as_bytes()).await?;

            sender.flush().await?;
        }

        Ok(())
    }

    async fn main_internal<
        R: AsyncBufRead + std::marker::Unpin + std::marker::Send + 'static,
        W: AsyncWrite + std::marker::Unpin + std::marker::Send + 'static,
    >(
        read: R,
        write: W,
    ) -> io::Result<()> {
        let (tx, rx) = mpsc::channel::<String>(3);

        let arc_self = Arc::new(Self {
            documents: RwLock::new(HashMap::new()),
            pending: HashMap::new(),
            tx,
        });

        let handle1 = tokio::spawn(arc_self.clone().handle_receiving(read));
        let handle2 = tokio::spawn(arc_self.handle_sending(write, rx));

        handle1.await??;
        handle2.await??;

        Ok(())
    }

    pub async fn recalculate_diagnostics(
        self: Arc<Self>,
        content: &str,
        uri: String,
        version: i64,
    ) -> io::Result<()> {
        let span = Span::new(content);
        let value = parse_root(span);

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
        Ok(())
    }

    async fn handle_TextDocumentSemanticTokensFullRequest(
        self: Arc<Self>,
        request: TextDocumentSemanticTokensFullRequest,
    ) -> io::Result<()> {
        let documents = self.documents.read().await;
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
                He98ccfdc940d4c1fa4b43794669192a12c560d6457d392bc00630cb4::Variant0(Box::new(
                    SemanticTokens {
                        result_id: None,
                        data: result,
                    },
                )),
            ),
        };

        Ok(())
    }

    async fn handle_ShutdownRequest(self: Arc<Self>, request: ShutdownRequest) -> io::Result<()> {
        let response = ShutdownResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(()),
        };

        Ok(())
    }

    async fn handle_TextDocumentDidOpenNotification(
        self: Arc<Self>,
        notification: TextDocumentDidOpenNotification,
    ) -> io::Result<()> {
        let mut documents = self.documents.write().await;
        documents.insert(
            notification.params.text_document.uri.clone(),
            notification.params.text_document.text.clone(),
        );

        drop(documents);

        self.recalculate_diagnostics(
            &notification.params.text_document.text,
            notification.params.text_document.uri,
            notification.params.text_document.version,
        )
        .await?;

        Ok(())
    }

    async fn handle_TextDocumentDidCloseNotification(
        self: Arc<Self>,
        notification: TextDocumentDidCloseNotification,
    ) -> io::Result<()> {
        let mut documents = self.documents.write().await;
        documents.remove(&notification.params.text_document.uri);

        Ok(())
    }

    // TODO FIXME these and quite some others need to respect some order
    async fn handle_TextDocumentDidChangeNotification(
        self: Arc<Self>,
        notification: TextDocumentDidChangeNotification,
    ) -> io::Result<()> {
        let mut documents = self.documents.write().await;
        let mut document = documents
            .get(&notification.params.text_document.variant0.uri)
            .unwrap()
            .clone();

        for change in notification.params.content_changes.iter() {
            match &**change {
        tucant_language_server_derive_output::H25fd6c7696dff041d913d0a9d3ce2232683e5362f0d4c6ca6179cf92::Variant0(incremental_changes) => {
            let start_offset = line_column_to_offset(&document, incremental_changes.range.start.line.try_into().unwrap(), incremental_changes.range.start.character.try_into().unwrap());
            let end_offset = line_column_to_offset(&document, incremental_changes.range.end.line.try_into().unwrap(), incremental_changes.range.end.character.try_into().unwrap());

            document = format!("{}{}{}", &document[..start_offset], incremental_changes.text, &document[end_offset..]);
        },
        tucant_language_server_derive_output::H25fd6c7696dff041d913d0a9d3ce2232683e5362f0d4c6ca6179cf92::Variant1(changes) => {
            documents.insert(notification.params.text_document.variant0.uri.clone(), changes.text.clone());
        },
    }
        }

        documents.insert(
            notification.params.text_document.variant0.uri.clone(),
            document.clone(),
        );

        println!("change notification");

        if notification.params.content_changes.len() == 1 {
            match &*(notification.params.content_changes[0]) {
        tucant_language_server_derive_output::H25fd6c7696dff041d913d0a9d3ce2232683e5362f0d4c6ca6179cf92::Variant0(ref incremental_changes) => {
            let _start_offset = line_column_to_offset(&document, incremental_changes.range.start.line.try_into().unwrap(), incremental_changes.range.start.character.try_into().unwrap());
            let _end_offset = line_column_to_offset(&document, incremental_changes.range.end.line.try_into().unwrap(), incremental_changes.range.end.character.try_into().unwrap());

            let response = Responses::WorkspaceApplyEditRequest(WorkspaceApplyEditRequest {
                jsonrpc: "2.0".to_string(),
                id: tucant_language_server_derive_output::StringOrNumber::String("1337".to_string()), // TODO FIXME
                params: Box::new(ApplyWorkspaceEditParams {
                    label: Some("insert matching paren".to_string()),
                    edit: Box::new(WorkspaceEdit {
                        changes: None,
                        document_changes: Some(vec![
                            H1332ceed95c3cca3c02eed7277ac86fcb37ac84398216e85560c37bf::Variant0(Box::new(TextDocumentEdit {
                                text_document: Box::new(OptionalVersionedTextDocumentIdentifier {
                                    variant0: Box::new(TextDocumentIdentifier { uri: notification.params.text_document.variant0.uri.clone() }),
                                    version: Hf7dce6b26d9e110d906dc3150d7d569f6983091049d0e763bb4a5cec::Variant0(notification.params.text_document.version)
                                }),
                                edits: vec![
                                    Hbc05edec65fcb6ecb06a32c6c6bd742b6b3682f1da78657cd86b8f05::Variant0(Box::new(TextEdit {
                                        range: Box::new(Range { start: Box::new(Position { line: incremental_changes.range.end.line, character: incremental_changes.range.end.character }), end: Box::new(Position { line: incremental_changes.range.end.line, character: incremental_changes.range.end.character }) }),
                                        new_text: r#"""#.to_string()
                                    }))
                                ]
                            }))
                        ]),
                        change_annotations: None,
                    }),
                }),
            });

        },
        _ => {}
    }
        }

        let contents = documents
            .get(&notification.params.text_document.variant0.uri)
            .unwrap()
            .clone();

        drop(documents);

        self.recalculate_diagnostics(
            &contents,
            notification.params.text_document.variant0.uri,
            notification.params.text_document.version,
        )
        .await?;

        Ok(())
    }

    async fn handle_initialized_notification(
        self: Arc<Self>,
        notification: InitializedNotification,
    ) -> io::Result<()> {
        let notification =
            Responses::WindowShowMessageNotification(WindowShowMessageNotification {
                jsonrpc: "2.0".to_string(),
                params: Box::new(ShowMessageParams {
                    r#type: Box::new(MessageType::Error),
                    message: "This is a test error".to_string(),
                }),
            });

        Ok(())
    }

    async fn handle_initialize(self: Arc<Self>, request: InitializeRequest) -> io::Result<()> {
        // TODO FIXME respond method on the request?
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
                        change: Some(Box::new(TextDocumentSyncKind::Incremental)),
                        save: None, // TODO FIXME
                    }))),
                    notebook_document_sync: None,
                    completion_provider: None,/*Some(Box::new(CompletionOptions {
                        variant0: Box::new(WorkDoneProgressOptions { work_done_progress: None }),
                        trigger_characters: Some(vec![r#"""#.to_string()]),
                        all_commit_characters: Some(vec![r#"""#.to_string()]),
                        resolve_provider: None,
                        completion_item: None,
                    })),*/
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
                    document_on_type_formatting_provider: None, /*Some(Box::new(DocumentOnTypeFormattingOptions {
                        first_trigger_character: r#"""#.to_string(),
                        more_trigger_character: None,
                    })),*/
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

        Ok(())
    }
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
        } => {
            let stream = UnixStream::connect(pipe).await?;
            let (read, write) = stream.into_split();
            Server::main_internal(BufReader::new(read), write).await
        }
        Args {
            port: Some(port),
            pipe: None,
            stdin: false,
        } => {
            let stream = TcpListener::bind(("127.0.0.1", port))
                .await?
                .accept()
                .await?
                .0;
            let (read, write) = stream.into_split();
            Server::main_internal(BufReader::new(read), write).await
        }
        Args { pipe: None, .. } => {
            Server::main_internal(BufReader::new(tokio::io::stdin()), tokio::io::stdout()).await
        }
        _ => {
            panic!("can't enable multiple modes at the same time")
        }
    }
}
