#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions,
    clippy::vec_box,
    clippy::module_name_repetitions
)]
use async_trait::async_trait;
use futures_util::Sink;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use json_rpc_server::run_json_rpc_server;
use json_rpc_server::JsonRpcServer;
use serde::Deserialize;
use serde::Serialize;

use std::sync::Arc;

out_dir_include::out_dir_include!("debug-adapter-protocol.rs");

pub struct Server {}

impl Server {
    #[allow(clippy::too_many_lines)]
    async fn handle<
        R: Stream<Item = Result<String, anyhow::Error>> + std::marker::Send + std::marker::Unpin,
        W: Sink<String, Error = anyhow::Error> + std::marker::Send + std::marker::Unpin,
    >(
        self: Arc<Self>,
        mut reader: R,
        mut sender: W,
    ) -> anyhow::Result<()> {
        let mut seq = 0;
        loop {
            let read_value = reader.next().await;
            let Some(read_value) = read_value else {
                break Ok(());
            };
            let read_value = read_value?;

            let request: Requests = serde_json::from_str(&read_value)?;

            let fake_source = Source {
                name: Some("test.tucant".to_string()),
                path: Some("/home/moritz/Documents/tucant/tucant-language/test.tucant".to_string()),
                source_reference: None,
                presentation_hint: Some(SourceStructPresentationHint::Emphasize),
                origin: Some("source code".to_string()),
                sources: Some(vec![]),
                adapter_data: None,
                checksums: Some(vec![]),
            };

            match request {
                Requests::InitializeRequest(request) => {
                    let response = Response::<InitializeResponse> {
                        inner: Some(InitializeResponse {
                            body: Some(Capabilities {
                                supports_configuration_done_request: Some(true),
                                supports_function_breakpoints: Some(true),
                                supports_conditional_breakpoints: Some(true),
                                supports_hit_conditional_breakpoints: Some(true),
                                supports_evaluate_for_hovers: Some(true),
                                exception_breakpoint_filters: Some(vec![]),
                                supports_step_back: Some(true),
                                supports_set_variable: Some(true),
                                supports_restart_frame: Some(true),
                                supports_goto_targets_request: Some(true),
                                supports_step_in_targets_request: Some(true),
                                supports_completions_request: Some(true),
                                completion_trigger_characters: Some(vec![
                                    ".".to_string(),
                                    " ".to_string(),
                                ]),
                                supports_modules_request: Some(true),
                                additional_module_columns: Some(vec![]),
                                supported_checksum_algorithms: Some(vec![
                                    ChecksumAlgorithm::Md5,
                                    ChecksumAlgorithm::Sha1,
                                    ChecksumAlgorithm::Sha256,
                                    ChecksumAlgorithm::Timestamp,
                                ]),
                                supports_restart_request: Some(true),
                                supports_exception_options: Some(true),
                                supports_value_formatting_options: Some(true),
                                supports_exception_info_request: Some(true),
                                support_terminate_debuggee: Some(true),
                                support_suspend_debuggee: Some(true),
                                supports_delayed_stack_trace_loading: Some(true),
                                supports_loaded_sources_request: Some(true),
                                supports_log_points: Some(true),
                                supports_terminate_threads_request: Some(true),
                                supports_set_expression: Some(true),
                                supports_terminate_request: Some(true),
                                supports_data_breakpoints: Some(true),
                                supports_read_memory_request: Some(true),
                                supports_write_memory_request: Some(true),
                                supports_disassemble_request: Some(true),
                                supports_cancel_request: Some(true),
                                supports_breakpoint_locations_request: Some(true),
                                supports_clipboard_context: Some(true),
                                supports_stepping_granularity: Some(true),
                                supports_instruction_breakpoints: Some(true),
                                supports_exception_filter_options: Some(true),
                                supports_single_thread_execution_requests: Some(true),
                            }),
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: InitializedEvent {
                            event: InitializedEventStructEvent::Initialized,
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::LaunchRequest(request) => {
                    // TODO FIXME make this pause at start

                    // TODO FIXME force matchup of request and response

                    // TODO FIXME abstract equal fields out
                    let response = Response::<LaunchResponse> {
                        inner: Some(LaunchResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetBreakpointsResponse {
                            body: SetBreakpointsResponseStructBody {
                                breakpoints: vec![Breakpoint {
                                    id: Some(133_333),
                                    verified: true,
                                    message: None,
                                    source: Some(fake_source),
                                    line: Some(1),
                                    column: Some(1),
                                    end_line: Some(1),
                                    end_column: Some(5),
                                    instruction_reference: None,
                                    offset: None,
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::BreakpointLocationsRequest(request) => {
                    // these are shown as hints when you set a breakpoint in that line
                    let response = Response {
                        inner: Some(BreakpointLocationsResponse {
                            body: BreakpointLocationsResponseStructBody {
                                breakpoints: vec![
                                    BreakpointLocation {
                                        line: 1,
                                        column: Some(1),
                                        end_line: None,
                                        end_column: None,
                                    },
                                    BreakpointLocation {
                                        line: 1,
                                        column: Some(2),
                                        end_line: None,
                                        end_column: None,
                                    },
                                    BreakpointLocation {
                                        line: 1,
                                        column: Some(3),
                                        end_line: None,
                                        end_column: None,
                                    },
                                ],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetFunctionBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetFunctionBreakpointsResponse {
                            body: SetFunctionBreakpointsResponseStructBody {
                                breakpoints: vec![],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetDataBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetDataBreakpointsResponse {
                            body: SetDataBreakpointsResponseStructBody {
                                breakpoints: vec![],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetInstructionBreakpointsRequest(request) => {
                    let response = Response {
                        inner: Some(SetInstructionBreakpointsResponse {
                            body: SetInstructionBreakpointsResponseStructBody {
                                breakpoints: vec![],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::LoadedSourcesRequest(request) => {
                    let response = Response {
                        inner: Some(LoadedSourcesResponse {
                            body: LoadedSourcesResponseStructBody {
                                sources: vec![fake_source],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::ConfigurationDoneRequest(request) => {
                    let response = Response {
                        inner: Some(ConfigurationDoneResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "entry".to_string(),
                                description: Some("The entry has been reached".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::ThreadsRequest(request) => {
                    let response = Response {
                        inner: Some(ThreadsResponse {
                            body: ThreadsResponseStructBody {
                                threads: vec![Thread {
                                    id: 1234,
                                    name: "the epic main thread".to_string(),
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::StackTraceRequest(request) => {
                    let response = Response {
                        inner: Some(StackTraceResponse {
                            body: StackTraceResponseStructBody {
                                stack_frames: vec![StackFrame {
                                    id: 15,
                                    name: "root stack frame".to_string(),
                                    source: Some(fake_source),
                                    line: 1,
                                    column: 1,
                                    end_line: None,
                                    end_column: None,
                                    can_restart: Some(true),
                                    instruction_pointer_reference: None,
                                    module_id: None, // TODO FIXME add module
                                    presentation_hint: Some(
                                        StackFrameStructPresentationHint::Normal,
                                    ),
                                }],
                                total_frames: Some(1),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::ScopesRequest(request) => {
                    let response = Response {
                        inner: Some(ScopesResponse {
                            body: ScopesResponseStructBody {
                                scopes: vec![Scope {
                                    name: "Registers".to_string(),
                                    presentation_hint: Some("registers".to_string()),
                                    variables_reference: 234,
                                    named_variables: Some(5),
                                    indexed_variables: Some(5),
                                    expensive: false,
                                    source: Some(fake_source),
                                    line: Some(2),
                                    column: Some(1),
                                    end_line: Some(3),
                                    end_column: Some(1),
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::EvaluateRequest(request) => {
                    let response = Response::<EvaluateResponse> {
                        inner: Some(EvaluateResponse {
                            body: EvaluateResponseStructBody {
                                result: "42".to_string(),
                                r#type: Some("answer-to-question-about-sense-of-life".to_string()),
                                presentation_hint: Some(VariablePresentationHint {
                                    kind: Some("property".to_string()),
                                    attributes: Some(vec!["readWrite".to_string()]),
                                    visibility: Some("public".to_string()),
                                    lazy: Some(false),
                                }),
                                variables_reference: 1337,
                                named_variables: Some(10),
                                indexed_variables: Some(10),
                                memory_reference: Some("deadbeef".to_string()),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::VariablesRequest(request) => {
                    let response = Response {
                        inner: Some(VariablesResponse {
                            body: VariablesResponseStructBody {
                                variables: vec![Variable {
                                    name: "cat".to_string(),
                                    value: "nicevalue".to_string(),
                                    r#type: Some("string".to_string()),
                                    presentation_hint: Some(VariablePresentationHint {
                                        kind: Some("property".to_string()),
                                        attributes: Some(vec!["readWrite".to_string()]),
                                        visibility: Some("public".to_string()),
                                        lazy: Some(false),
                                    }),
                                    evaluate_name: Some("evaluateName".to_string()),
                                    variables_reference: 0,
                                    named_variables: Some(0),
                                    indexed_variables: Some(0),
                                    memory_reference: None,
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::CompletionsRequest(request) => {
                    let response = Response {
                        inner: Some(CompletionsResponse {
                            body: CompletionsResponseStructBody {
                                targets: vec![CompletionItem {
                                    label: ".elephant".to_string(),
                                    text: None,
                                    sort_text: None,
                                    detail: Some("this is super nice".to_string()),
                                    r#type: Some(CompletionItemType::Function),
                                    start: Some(0),
                                    length: Some(0),
                                    selection_start: None,  //Some(1),
                                    selection_length: None, // Some(1),
                                }],
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::CancelRequest(request) => {
                    let response = Response {
                        inner: Some(CancelResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::RestartFrameRequest(request) => {
                    let response = Response {
                        inner: Some(RestartFrameResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "restart".to_string(),
                                description: Some("Frame has been restarted".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::SetExpressionRequest(request) => {
                    let response = Response {
                        inner: Some(SetExpressionResponse {
                            body: SetExpressionResponseStructBody {
                                value: "42".to_string(),
                                r#type: Some("elephant".to_string()),
                                presentation_hint: Some(VariablePresentationHint {
                                    kind: Some("property".to_string()),
                                    attributes: Some(vec!["readWrite".to_string()]),
                                    visibility: Some("public".to_string()),
                                    lazy: Some(false),
                                }),
                                variables_reference: Some(3208),
                                named_variables: Some(3),
                                indexed_variables: Some(3),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::SetVariableRequest(request) => {
                    let response = Response {
                        inner: Some(SetVariableResponse {
                            body: SetVariableResponseStructBody {
                                value: "42".to_string(),
                                r#type: Some("elephant".to_string()),
                                variables_reference: Some(3208),
                                named_variables: Some(3),
                                indexed_variables: Some(3),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::DataBreakpointInfoRequest(request) => {
                    let response = Response {
                        inner: Some(DataBreakpointInfoResponse {
                            body: DataBreakpointInfoResponseStructBody {
                                data_id: DataBreakpointInfoResponseStructBodyStructDataId::O0(
                                    "dfs".to_string(),
                                ),
                                description: "test description".to_string(),
                                access_types: Some(vec![DataBreakpointAccessType::ReadWrite]),
                                can_persist: Some(true),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::ContinueRequest(request) => {
                    let response = Response {
                        inner: Some(ContinueResponse {
                            body: ContinueResponseStructBody {
                                all_threads_continued: Some(true),
                            },
                        }),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::PauseRequest(request) => {
                    let response = Response {
                        inner: Some(PauseResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "pause".to_string(),
                                description: Some("Paused on request".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::TerminateRequest(request) => {
                    let response = Response {
                        inner: Some(TerminateResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: TerminatedEvent {
                            event: TerminatedEventStructEvent::Terminated,
                            body: Some(TerminatedEventStructBody { restart: None }),
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::DisconnectRequest(request) => {
                    let response = Response {
                        inner: Some(DisconnectResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::RestartRequest(request) => {
                    // TODO FIXME something is probably missing here, probably an terminated and started or so
                    let response = Response {
                        inner: Some(RestartResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::NextRequest(request) => {
                    let response = Response {
                        inner: Some(NextResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "step".to_string(),
                                description: Some("Stepped forward".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::StepInRequest(request) => {
                    let response = Response {
                        inner: Some(StepInResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "step".to_string(),
                                description: Some("Stepped in".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::StepOutRequest(request) => {
                    let response = Response {
                        inner: Some(StepOutResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "step".to_string(),
                                description: Some("Stepped out".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::StepBackRequest(request) => {
                    let response = Response {
                        inner: Some(StepBackResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;

                    let event = Event {
                        inner: StoppedEvent {
                            event: StoppedEventStructEvent::Stopped,
                            body: StoppedEventStructBody {
                                reason: "step".to_string(),
                                description: Some("Stepped back".to_string()),
                                thread_id: None, // TODO FIXME create threads
                                preserve_focus_hint: Some(false),
                                text: None,
                                all_threads_stopped: Some(true),
                                hit_breakpoint_ids: Some(vec![]),
                            },
                        },
                        r#type: "event".to_string(),
                    };

                    sender.send(serde_json::to_string(&event)?).await?;
                }
                Requests::ReverseContinueRequest(request) => {
                    let response = Response {
                        inner: Some(ReverseContinueResponse {}),
                        seq: {
                            seq += 1;
                            seq
                        },
                        r#type: "response".to_string(),
                        request_seq: request.seq,
                        success: true,
                        message: None,
                    };

                    sender.send(serde_json::to_string(&response)?).await?;
                }
                Requests::RunInTerminalRequest(_) => todo!(),
                Requests::StartDebuggingRequest(_) => todo!(),
                Requests::AttachRequest(_) => todo!(),
                Requests::SetExceptionBreakpointsRequest(_) => todo!(),
                Requests::GotoRequest(_) => todo!(),
                Requests::SourceRequest(_) => todo!(),
                Requests::TerminateThreadsRequest(_) => todo!(),
                Requests::ModulesRequest(_) => todo!(),
                Requests::StepInTargetsRequest(_) => todo!(),
                Requests::GotoTargetsRequest(_) => todo!(),
                Requests::ExceptionInfoRequest(_) => todo!(),
                Requests::ReadMemoryRequest(_) => todo!(),
                Requests::WriteMemoryRequest(_) => todo!(),
                Requests::DisassembleRequest(_) => todo!(),
            }

            let _cloned_self = self.clone();
        }
    }
}

#[async_trait]
impl JsonRpcServer for Server {
    async fn run<
        R: Stream<Item = Result<String, anyhow::Error>>
            + std::marker::Unpin
            + std::marker::Send
            + 'static,
        W: Sink<String, Error = anyhow::Error> + std::marker::Unpin + std::marker::Send + 'static,
    >(
        read: R,
        write: W,
    ) -> anyhow::Result<()> {
        let arc_self = Arc::new(Self {});

        arc_self.handle(read, write).await?;

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    #[serde(flatten)]
    inner: T,
    seq: u64,
    r#type: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(flatten)]
    inner: Option<T>, // probably Result
    seq: u64,
    r#type: String,
    request_seq: u64,
    success: bool,
    message: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Event<T> {
    #[serde(flatten)]
    inner: T,
    r#type: String,
}

// cargo watch -x 'run -- --port 6009'
pub fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_json_rpc_server::<Server>().await })
}
