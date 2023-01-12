use quote::quote;

use crate::json_parser::JSONValue;
use crate::{json_schema::JSONSchema, json_schema_codegen::codegen};
use proc_macro2::TokenStream;

#[allow(clippy::too_many_lines)]
pub fn realistic() -> Result<TokenStream, syn::Error> {
    let value = quote! {
        {
            "$schema": "http://json-schema.org/draft-04/schema#",
            "title": "Debug Adapter Protocol",
            "description": "The Debug Adapter Protocol defines the protocol used between an editor or IDE and a debugger or runtime.",
            "type": "object",
            "definitions": {
                "ProtocolMessage": {
                    "type": "object",
                    "title": "Base Protocol",
                    "description": "Base class of requests, responses, and events.",
                    "properties": {
                        "seq": {
                            "type": "integer",
                            "description": "Sequence number of the message (also known as message ID). The `seq` for the first message sent by a client or debug adapter is 1, and for each subsequent message is 1 greater than the previous message sent by that actor. `seq` can be used to order requests, responses, and events, and to associate requests with their corresponding responses. For protocol messages of type `request` the sequence number can be used to cancel the request."
                        },
                        "type": {
                            "type": "string",
                            "description": "Message type.",
                            "_enum": [ "request", "response", "event" ]
                        }
                    },
                    "required": [ "seq", "type" ]
                },

                "Request": {
                    "allOf": [ { "$ref": "#/definitions/ProtocolMessage" }, {
                        "type": "object",
                        "description": "A client or debug adapter initiated request.",
                        "properties": {
                            "type": {
                                "type": "string",
                                "enum": [ "request" ]
                            },
                            "command": {
                                "type": "string",
                                "description": "The command to execute."
                            },
                            "arguments": {
                                "type": [ "array", "boolean", "integer", "null", "number" , "object", "string" ],
                                "description": "Object containing arguments for the command."
                            }
                        },
                        "required": [ "type", "command" ]
                    }]
                },

                "Event": {
                    "allOf": [ { "$ref": "#/definitions/ProtocolMessage" }, {
                        "type": "object",
                        "description": "A debug adapter initiated event.",
                        "properties": {
                            "type": {
                                "type": "string",
                                "enum": [ "event" ]
                            },
                            "event": {
                                "type": "string",
                                "description": "Type of event."
                            },
                            "body": {
                                "type": [ "array", "boolean", "integer", "null", "number" , "object", "string" ],
                                "description": "Event-specific information."
                            }
                        },
                        "required": [ "type", "event" ]
                    }]
                },

                "Response": {
                    "allOf": [ { "$ref": "#/definitions/ProtocolMessage" }, {
                        "type": "object",
                        "description": "Response for a request.",
                        "properties": {
                            "type": {
                                "type": "string",
                                "enum": [ "response" ]
                            },
                            "request_seq": {
                                "type": "integer",
                                "description": "Sequence number of the corresponding request."
                            },
                            "success": {
                                "type": "boolean",
                                "description": "Outcome of the request.\nIf true, the request was successful and the `body` attribute may contain the result of the request.\nIf the value is false, the attribute `message` contains the error in short form and the `body` may contain additional information (see `ErrorResponse.body.error`)."
                            },
                            "command": {
                                "type": "string",
                                "description": "The command requested."
                            },
                            "message": {
                                "type": "string",
                                "description": "Contains the raw error in short form if `success` is false.\nThis raw error might be interpreted by the client and is not shown in the UI.\nSome predefined values exist.",
                                "_enum": [ "cancelled", "notStopped" ],
                                "enumDescriptions": [
                                    "the request was cancelled.",
                                    "the request may be retried once the adapter is in a 'stopped' state."
                                ]
                            },
                            "body": {
                                "type": [ "array", "boolean", "integer", "null", "number" , "object", "string" ],
                                "description": "Contains request result if success is true and error details if success is false."
                            }
                        },
                        "required": [ "type", "request_seq", "success", "command" ]
                    }]
                },

                "ErrorResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "On error (whenever `success` is false), the body can provide more details.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "error": {
                                        "$ref": "#/definitions/Message",
                                        "description": "A structured error message."
                                    }
                                }
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "CancelRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The `cancel` request is used by the client in two situations:\n- to indicate that it is no longer interested in the result produced by a specific request issued earlier\n- to cancel a progress sequence. Clients should only call this request if the corresponding capability `supportsCancelRequest` is true.\nThis request has a hint characteristic: a debug adapter can only be expected to make a 'best effort' in honoring this request but there are no guarantees.\nThe `cancel` request may return an error if it could not cancel an operation but a client should refrain from presenting this error to end users.\nThe request that got cancelled still needs to send a response back. This can either be a normal result (`success` attribute true) or an error response (`success` attribute false and the `message` set to `cancelled`).\nReturning partial results from a cancelled request is possible but please note that a client has no generic way for detecting that a response is partial or not.\nThe progress that got cancelled still needs to send a `progressEnd` event back.\n A client should not assume that progress just got cancelled after sending the `cancel` request.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "cancel" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/CancelArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "CancelArguments": {
                    "type": "object",
                    "description": "Arguments for `cancel` request.",
                    "properties": {
                        "requestId": {
                            "type": "integer",
                            "description": "The ID (attribute `seq`) of the request to cancel. If missing no request is cancelled.\nBoth a `requestId` and a `progressId` can be specified in one request."
                        },
                        "progressId": {
                            "type": "string",
                            "description": "The ID (attribute `progressId`) of the progress to cancel. If missing no progress is cancelled.\nBoth a `requestId` and a `progressId` can be specified in one request."
                        }
                    }
                },
                "CancelResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `cancel` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "InitializedEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "title": "Events",
                        "description": "This event indicates that the debug adapter is ready to accept configuration requests (e.g. `setBreakpoints`, `setExceptionBreakpoints`).\nA debug adapter is expected to send this event when it is ready to accept configuration requests (but not before the `initialize` request has finished).\nThe sequence of events/requests is as follows:\n- adapters sends `initialized` event (after the `initialize` request has returned)\n- client sends zero or more `setBreakpoints` requests\n- client sends one `setFunctionBreakpoints` request (if corresponding capability `supportsFunctionBreakpoints` is true)\n- client sends a `setExceptionBreakpoints` request if one or more `exceptionBreakpointFilters` have been defined (or if `supportsConfigurationDoneRequest` is not true)\n- client sends other future configuration requests\n- client sends one `configurationDone` request to indicate the end of the configuration.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "initialized" ]
                            }
                        },
                        "required": [ "event" ]
                    }]
                },

                "StoppedEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that the execution of the debuggee has stopped due to some condition.\nThis can be caused by a breakpoint previously set, a stepping request has completed, by executing a debugger statement etc.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "stopped" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "reason": {
                                        "type": "string",
                                        "description": "The reason for the event.\nFor backward compatibility this string is shown in the UI if the `description` attribute is missing (but it must not be translated).",
                                        "_enum": [ "step", "breakpoint", "exception", "pause", "entry", "goto", "function breakpoint", "data breakpoint", "instruction breakpoint" ]
                                    },
                                    "description": {
                                        "type": "string",
                                        "description": "The full reason for the event, e.g. 'Paused on exception'. This string is shown in the UI as is and can be translated."
                                    },
                                    "threadId": {
                                        "type": "integer",
                                        "description": "The thread which was stopped."
                                    },
                                    "preserveFocusHint": {
                                        "type": "boolean",
                                        "description": "A value of true hints to the client that this event should not change the focus."
                                    },
                                    "text": {
                                        "type": "string",
                                        "description": "Additional information. E.g. if reason is `exception`, text contains the exception name. This string is shown in the UI."
                                    },
                                    "allThreadsStopped": {
                                        "type": "boolean",
                                        "description": "If `allThreadsStopped` is true, a debug adapter can announce that all threads have stopped.\n- The client should use this information to enable that all threads can be expanded to access their stacktraces.\n- If the attribute is missing or false, only the thread with the given `threadId` can be expanded."
                                    },
                                    "hitBreakpointIds": {
                                        "type": "array",
                                        "items": {
                                            "type": "integer"
                                        },
                                        "description": "Ids of the breakpoints that triggered the event. In most cases there is only a single breakpoint but here are some examples for multiple breakpoints:\n- Different types of breakpoints map to the same location.\n- Multiple source breakpoints get collapsed to the same instruction by the compiler/runtime.\n- Multiple function breakpoints with different function names map to the same location."
                                    }
                                },
                                "required": [ "reason" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ContinuedEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that the execution of the debuggee has continued.\nPlease note: a debug adapter is not expected to send this event in response to a request that implies that execution continues, e.g. `launch` or `continue`.\nIt is only necessary to send a `continued` event if there was no previous request that implied this.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "continued" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "threadId": {
                                        "type": "integer",
                                        "description": "The thread which was continued."
                                    },
                                    "allThreadsContinued": {
                                        "type": "boolean",
                                        "description": "If `allThreadsContinued` is true, a debug adapter can announce that all threads have continued."
                                    }
                                },
                                "required": [ "threadId" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ExitedEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that the debuggee has exited and returns its exit code.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "exited" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "exitCode": {
                                        "type": "integer",
                                        "description": "The exit code returned from the debuggee."
                                    }
                                },
                                "required": [ "exitCode" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "TerminatedEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that debugging of the debuggee has terminated. This does **not** mean that the debuggee itself has exited.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "terminated" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "restart": {
                                        "type": [ "array", "boolean", "integer", "null", "number", "object", "string" ],
                                        "description": "A debug adapter may set `restart` to true (or to an arbitrary object) to request that the client restarts the session.\nThe value is not interpreted by the client and passed unmodified as an attribute `__restart` to the `launch` and `attach` requests."
                                    }
                                }
                            }
                        },
                        "required": [ "event" ]
                    }]
                },

                "ThreadEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that a thread has started or exited.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "thread" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "reason": {
                                        "type": "string",
                                        "description": "The reason for the event.",
                                        "_enum": [ "started", "exited" ]
                                    },
                                    "threadId": {
                                        "type": "integer",
                                        "description": "The identifier of the thread."
                                    }
                                },
                                "required": ["reason", "threadId"]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "OutputEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that the target has produced some output.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "output" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "category": {
                                        "type": "string",
                                        "description": "The output category. If not specified or if the category is not understood by the client, `console` is assumed.",
                                        "_enum": [ "console", "important", "stdout", "stderr", "telemetry" ],
                                        "enumDescriptions": [
                                            "Show the output in the client's default message UI, e.g. a 'debug console'. This category should only be used for informational output from the debugger (as opposed to the debuggee).",
                                            "A hint for the client to show the output in the client's UI for important and highly visible information, e.g. as a popup notification. This category should only be used for important messages from the debugger (as opposed to the debuggee). Since this category value is a hint, clients might ignore the hint and assume the `console` category.",
                                            "Show the output as normal program output from the debuggee.",
                                            "Show the output as error program output from the debuggee.",
                                            "Send the output to telemetry instead of showing it to the user."
                                        ]
                                    },
                                    "output": {
                                        "type": "string",
                                        "description": "The output to report."
                                    },
                                    "group": {
                                        "type": "string",
                                        "description": "Support for keeping an output log organized by grouping related messages.",
                                        "enum": [ "start", "startCollapsed", "end" ],
                                        "enumDescriptions": [
                                            "Start a new group in expanded mode. Subsequent output events are members of the group and should be shown indented.\nThe `output` attribute becomes the name of the group and is not indented.",
                                            "Start a new group in collapsed mode. Subsequent output events are members of the group and should be shown indented (as soon as the group is expanded).\nThe `output` attribute becomes the name of the group and is not indented.",
                                            "End the current group and decrease the indentation of subsequent output events.\nA non-empty `output` attribute is shown as the unindented end of the group."
                                        ]
                                    },
                                    "variablesReference": {
                                        "type": "integer",
                                        "description": "If an attribute `variablesReference` exists and its value is > 0, the output contains objects which can be retrieved by passing `variablesReference` to the `variables` request as long as execution remains suspended. See 'Lifetime of Object References' in the Overview section for details."
                                    },
                                    "source": {
                                        "$ref": "#/definitions/Source",
                                        "description": "The source location where the output was produced."
                                    },
                                    "line": {
                                        "type": "integer",
                                        "description": "The source location's line where the output was produced."
                                    },
                                    "column": {
                                        "type": "integer",
                                        "description": "The position in `line` where the output was produced. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                                    },
                                    "data": {
                                        "type": [ "array", "boolean", "integer", "null", "number" , "object", "string" ],
                                        "description": "Additional data to report. For the `telemetry` category the data is sent to telemetry, for the other categories the data is shown in JSON format."
                                    }
                                },
                                "required": ["output"]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "BreakpointEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that some information about a breakpoint has changed.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "breakpoint" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "reason": {
                                        "type": "string",
                                        "description": "The reason for the event.",
                                        "_enum": [ "changed", "new", "removed" ]
                                    },
                                    "breakpoint": {
                                        "$ref": "#/definitions/Breakpoint",
                                        "description": "The `id` attribute is used to find the target breakpoint, the other attributes are used as the new values."
                                    }
                                },
                                "required": [ "reason", "breakpoint" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ModuleEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that some information about a module has changed.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "module" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "reason": {
                                        "type": "string",
                                        "description": "The reason for the event.",
                                        "enum": [ "new", "changed", "removed" ]
                                    },
                                    "module": {
                                        "$ref": "#/definitions/Module",
                                        "description": "The new, changed, or removed module. In case of `removed` only the module id is used."
                                    }
                                },
                                "required": [ "reason", "module" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "LoadedSourceEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that some source has been added, changed, or removed from the set of all loaded sources.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "loadedSource" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "reason": {
                                        "type": "string",
                                        "description": "The reason for the event.",
                                        "enum": [ "new", "changed", "removed" ]
                                    },
                                    "source": {
                                        "$ref": "#/definitions/Source",
                                        "description": "The new, changed, or removed source."
                                    }
                                },
                                "required": [ "reason", "source" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ProcessEvent": {
                    "allOf": [
                        { "$ref": "#/definitions/Event" },
                        {
                            "type": "object",
                            "description": "The event indicates that the debugger has begun debugging a new process. Either one that it has launched, or one that it has attached to.",
                            "properties": {
                                "event": {
                                    "type": "string",
                                    "enum": [ "process" ]
                                },
                                "body": {
                                    "type": "object",
                                    "properties": {
                                        "name": {
                                            "type": "string",
                                            "description": "The logical name of the process. This is usually the full path to process's executable file. Example: /home/example/myproj/program.js."
                                        },
                                        "systemProcessId": {
                                            "type": "integer",
                                            "description": "The system process id of the debugged process. This property is missing for non-system processes."
                                        },
                                        "isLocalProcess": {
                                            "type": "boolean",
                                            "description": "If true, the process is running on the same computer as the debug adapter."
                                        },
                                        "startMethod": {
                                            "type": "string",
                                            "enum": [ "launch", "attach", "attachForSuspendedLaunch" ],
                                            "description": "Describes how the debug engine started debugging this process.",
                                            "enumDescriptions": [
                                                "Process was launched under the debugger.",
                                                "Debugger attached to an existing process.",
                                                "A project launcher component has launched a new process in a suspended state and then asked the debugger to attach."
                                            ]
                                        },
                                        "pointerSize": {
                                            "type": "integer",
                                            "description": "The size of a pointer or address for this process, in bits. This value may be used by clients when formatting addresses for display."
                                        }
                                    },
                                    "required": [ "name" ]
                                }
                            },
                            "required": [ "event", "body" ]
                        }
                    ]
                },

                "CapabilitiesEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event indicates that one or more capabilities have changed.\nSince the capabilities are dependent on the client and its UI, it might not be possible to change that at random times (or too late).\nConsequently this event has a hint characteristic: a client can only be expected to make a 'best effort' in honoring individual capabilities but there are no guarantees.\nOnly changed capabilities need to be included, all other capabilities keep their values.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "capabilities" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "capabilities": {
                                        "$ref": "#/definitions/Capabilities",
                                        "description": "The set of updated capabilities."
                                    }
                                },
                                "required": [ "capabilities" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ProgressStartEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event signals that a long running operation is about to start and provides additional information for the client to set up a corresponding progress and cancellation UI.\nThe client is free to delay the showing of the UI in order to reduce flicker.\nThis event should only be sent if the corresponding capability `supportsProgressReporting` is true.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "progressStart" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "progressId": {
                                        "type": "string",
                                        "description": "An ID that can be used in subsequent `progressUpdate` and `progressEnd` events to make them refer to the same progress reporting.\nIDs must be unique within a debug session."
                                    },
                                    "title": {
                                        "type": "string",
                                        "description": "Short title of the progress reporting. Shown in the UI to describe the long running operation."
                                    },
                                    "requestId": {
                                        "type": "integer",
                                        "description": "The request ID that this progress report is related to. If specified a debug adapter is expected to emit progress events for the long running request until the request has been either completed or cancelled.\nIf the request ID is omitted, the progress report is assumed to be related to some general activity of the debug adapter."
                                    },
                                    "cancellable": {
                                        "type": "boolean",
                                        "description": "If true, the request that reports progress may be cancelled with a `cancel` request.\nSo this property basically controls whether the client should use UX that supports cancellation.\nClients that don't support cancellation are allowed to ignore the setting."
                                    },
                                    "message": {
                                        "type": "string",
                                        "description": "More detailed progress message."
                                    },
                                    "percentage": {
                                        "type": "number",
                                        "description": "Progress percentage to display (value range: 0 to 100). If omitted no percentage is shown."
                                    }
                                },
                                "required": [ "progressId", "title" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ProgressUpdateEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event signals that the progress reporting needs to be updated with a new message and/or percentage.\nThe client does not have to update the UI immediately, but the clients needs to keep track of the message and/or percentage values.\nThis event should only be sent if the corresponding capability `supportsProgressReporting` is true.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "progressUpdate" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "progressId": {
                                        "type": "string",
                                        "description": "The ID that was introduced in the initial `progressStart` event."
                                    },
                                    "message": {
                                        "type": "string",
                                        "description": "More detailed progress message. If omitted, the previous message (if any) is used."
                                    },
                                    "percentage": {
                                        "type": "number",
                                        "description": "Progress percentage to display (value range: 0 to 100). If omitted no percentage is shown."
                                    }
                                },
                                "required": [ "progressId" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "ProgressEndEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "The event signals the end of the progress reporting with a final message.\nThis event should only be sent if the corresponding capability `supportsProgressReporting` is true.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "progressEnd" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "progressId": {
                                        "type": "string",
                                        "description": "The ID that was introduced in the initial `ProgressStartEvent`."
                                    },
                                    "message": {
                                        "type": "string",
                                        "description": "More detailed progress message. If omitted, the previous message (if any) is used."
                                    }
                                },
                                "required": [ "progressId" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "InvalidatedEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "This event signals that some state in the debug adapter has changed and requires that the client needs to re-render the data snapshot previously requested.\nDebug adapters do not have to emit this event for runtime changes like stopped or thread events because in that case the client refetches the new state anyway. But the event can be used for example to refresh the UI after rendering formatting has changed in the debug adapter.\nThis event should only be sent if the corresponding capability `supportsInvalidatedEvent` is true.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "invalidated" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "areas": {
                                        "type": "array",
                                        "description": "Set of logical areas that got invalidated. This property has a hint characteristic: a client can only be expected to make a 'best effort' in honoring the areas but there are no guarantees. If this property is missing, empty, or if values are not understood, the client should assume a single value `all`.",
                                        "items": {
                                            "$ref": "#/definitions/InvalidatedAreas"
                                        }
                                    },
                                    "threadId": {
                                        "type": "integer",
                                        "description": "If specified, the client only needs to refetch data related to this thread."
                                    },
                                    "stackFrameId": {
                                        "type": "integer",
                                        "description": "If specified, the client only needs to refetch data related to this stack frame (and the `threadId` is ignored)."
                                    }
                                }
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "MemoryEvent": {
                    "allOf": [ { "$ref": "#/definitions/Event" }, {
                        "type": "object",
                        "description": "This event indicates that some memory range has been updated. It should only be sent if the corresponding capability `supportsMemoryEvent` is true.\nClients typically react to the event by re-issuing a `readMemory` request if they show the memory identified by the `memoryReference` and if the updated memory range overlaps the displayed range. Clients should not make assumptions how individual memory references relate to each other, so they should not assume that they are part of a single continuous address range and might overlap.\nDebug adapters can use this event to indicate that the contents of a memory range has changed due to some other request like `setVariable` or `setExpression`. Debug adapters are not expected to emit this event for each and every memory change of a running program, because that information is typically not available from debuggers and it would flood clients with too many events.",
                        "properties": {
                            "event": {
                                "type": "string",
                                "enum": [ "memory" ]
                            },
                            "body": {
                                "type": "object",
                                "properties": {
                                    "memoryReference": {
                                        "type": "string",
                                        "description": "Memory reference of a memory range that has been updated."
                                    },
                                    "offset": {
                                        "type": "integer",
                                        "description": "Starting offset in bytes where memory has been updated. Can be negative."
                                    },
                                    "count": {
                                        "type": "integer",
                                        "description": "Number of bytes updated."
                                    }
                                },
                                "required": [ "memoryReference", "offset", "count" ]
                            }
                        },
                        "required": [ "event", "body" ]
                    }]
                },

                "RunInTerminalRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "title": "Reverse Requests",
                        "description": "This request is sent from the debug adapter to the client to run a command in a terminal.\nThis is typically used to launch the debuggee in a terminal provided by the client.\nThis request should only be called if the corresponding client capability `supportsRunInTerminalRequest` is true.\nClient implementations of `runInTerminal` are free to run the command however they choose including issuing the command to a command line interpreter (aka 'shell'). Argument strings passed to the `runInTerminal` request must arrive verbatim in the command to be run. As a consequence, clients which use a shell are responsible for escaping any special shell characters in the argument strings to prevent them from being interpreted (and modified) by the shell.\nSome users may wish to take advantage of shell processing in the argument strings. For clients which implement `runInTerminal` using an intermediary shell, the `argsCanBeInterpretedByShell` property can be set to true. In this case the client is requested not to escape any special shell characters in the argument strings.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "runInTerminal" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/RunInTerminalRequestArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "RunInTerminalRequestArguments": {
                    "type": "object",
                    "description": "Arguments for `runInTerminal` request.",
                    "properties": {
                        "kind": {
                            "type": "string",
                            "enum": [ "integrated", "external" ],
                            "description": "What kind of terminal to launch. Defaults to `integrated` if not specified."
                        },
                        "title": {
                            "type": "string",
                            "description": "Title of the terminal."
                        },
                        "cwd": {
                            "type": "string",
                            "description": "Working directory for the command. For non-empty, valid paths this typically results in execution of a change directory command."
                        },
                        "args": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "List of arguments. The first argument is the command to run."
                        },
                        "env": {
                            "type": "object",
                            "description": "Environment key-value pairs that are added to or removed from the default environment.",
                            "additionalProperties": {
                                "type": [ "string", "null" ],
                                "description": "A string is a proper value for an environment variable. The value `null` removes the variable from the environment."
                            }
                        },
                        "argsCanBeInterpretedByShell": {
                            "type": "boolean",
                            "description": "This property should only be set if the corresponding capability `supportsArgsCanBeInterpretedByShell` is true. If the client uses an intermediary shell to launch the application, then the client must not attempt to escape characters with special meanings for the shell. The user is fully responsible for escaping as needed and that arguments using special characters may not be portable across shells."
                        }
                    },
                    "required": [ "args", "cwd" ]
                },
                "RunInTerminalResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `runInTerminal` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "processId": {
                                        "type": "integer",
                                        "description": "The process ID. The value should be less than or equal to 2147483647 (2^31-1)."
                                    },
                                    "shellProcessId": {
                                        "type": "integer",
                                        "description": "The process ID of the terminal shell. The value should be less than or equal to 2147483647 (2^31-1)."
                                    }
                                }
                            }
                        },
                        "required": [ "body" ]
                    }]
                },
                "StartDebuggingRequest": {
                    "allOf": [
                        {
                            "$ref": "#/definitions/Request"
                        },
                        {
                            "type": "object",
                            "description": "This request is sent from the debug adapter to the client to start a new debug session of the same type as the caller.\nThis request should only be sent if the corresponding client capability `supportsStartDebuggingRequest` is true.\nA client implementation of `startDebugging` should start a new debug session (of the same type as the caller) in the same way that the caller's session was started. If the client supports hierarchical debug sessions, the newly created session can be treated as a child of the caller session.",
                            "properties": {
                                "command": {
                                    "type": "string",
                                    "enum": [
                                        "startDebugging"
                                    ]
                                },
                                "arguments": {
                                    "$ref": "#/definitions/StartDebuggingRequestArguments"
                                }
                            },
                            "required": [
                                "command",
                                "arguments"
                            ]
                        }
                    ]
                },
                "StartDebuggingRequestArguments": {
                    "type": "object",
                    "description": "Arguments for `startDebugging` request.",
                    "properties": {
                        "configuration": {
                            "type": "object",
                            "additionalProperties": true,
                            "description": "Arguments passed to the new debug session. The arguments must only contain properties understood by the `launch` or `attach` requests of the debug adapter and they must not contain any client-specific properties (e.g. `type`) or client-specific features (e.g. substitutable 'variables')."
                        },
                        "request": {
                            "type": "string",
                            "enum": [
                                "launch",
                                "attach"
                            ],
                            "description": "Indicates whether the new debug session should be started with a `launch` or `attach` request."
                        }
                    },
                    "required": [
                        "configuration",
                        "request"
                    ]
                },
                "StartDebuggingResponse": {
                    "allOf": [
                        {
                            "$ref": "#/definitions/Response"
                        },
                        {
                            "type": "object",
                            "description": "Response to `startDebugging` request. This is just an acknowledgement, so no body field is required."
                        }
                    ]
                },

                "InitializeRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "title": "Requests",
                        "description": "The `initialize` request is sent as the first request from the client to the debug adapter in order to configure it with client capabilities and to retrieve capabilities from the debug adapter.\nUntil the debug adapter has responded with an `initialize` response, the client must not send any additional requests or events to the debug adapter.\nIn addition the debug adapter is not allowed to send any requests or events to the client until it has responded with an `initialize` response.\nThe `initialize` request may only be sent once.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "initialize" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/InitializeRequestArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "InitializeRequestArguments": {
                    "type": "object",
                    "description": "Arguments for `initialize` request.",
                    "properties": {
                        "clientID": {
                            "type": "string",
                            "description": "The ID of the client using this adapter."
                        },
                        "clientName": {
                            "type": "string",
                            "description": "The human-readable name of the client using this adapter."
                        },
                        "adapterID": {
                            "type": "string",
                            "description": "The ID of the debug adapter."
                        },
                        "locale": {
                            "type": "string",
                            "description": "The ISO-639 locale of the client using this adapter, e.g. en-US or de-CH."
                        },
                        "linesStartAt1": {
                            "type": "boolean",
                            "description": "If true all line numbers are 1-based (default)."
                        },
                        "columnsStartAt1": {
                            "type": "boolean",
                            "description": "If true all column numbers are 1-based (default)."
                        },
                        "pathFormat": {
                            "type": "string",
                            "_enum": [ "path", "uri" ],
                            "description": "Determines in what format paths are specified. The default is `path`, which is the native format."
                        },
                        "supportsVariableType": {
                            "type": "boolean",
                            "description": "Client supports the `type` attribute for variables."
                        },
                        "supportsVariablePaging": {
                            "type": "boolean",
                            "description": "Client supports the paging of variables."
                        },
                        "supportsRunInTerminalRequest": {
                            "type": "boolean",
                            "description": "Client supports the `runInTerminal` request."
                        },
                        "supportsMemoryReferences": {
                            "type": "boolean",
                            "description": "Client supports memory references."
                        },
                        "supportsProgressReporting": {
                            "type": "boolean",
                            "description": "Client supports progress reporting."
                        },
                        "supportsInvalidatedEvent": {
                            "type": "boolean",
                            "description": "Client supports the `invalidated` event."
                        },
                        "supportsMemoryEvent": {
                            "type": "boolean",
                            "description": "Client supports the `memory` event."
                        },
                        "supportsArgsCanBeInterpretedByShell": {
                            "type": "boolean",
                            "description": "Client supports the `argsCanBeInterpretedByShell` attribute on the `runInTerminal` request."
                        },
                        "supportsStartDebuggingRequest": {
                            "type": "boolean",
                            "description": "Client supports the `startDebugging` request."
                        }
                    },
                    "required": [ "adapterID" ]
                },
                "InitializeResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `initialize` request.",
                        "properties": {
                            "body": {
                                "$ref": "#/definitions/Capabilities",
                                "description": "The capabilities of this debug adapter."
                            }
                        }
                    }]
                },

                "ConfigurationDoneRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "This request indicates that the client has finished initialization of the debug adapter.\nSo it is the last request in the sequence of configuration requests (which was started by the `initialized` event).\nClients should only call this request if the corresponding capability `supportsConfigurationDoneRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "configurationDone" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ConfigurationDoneArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "ConfigurationDoneArguments": {
                    "type": "object",
                    "description": "Arguments for `configurationDone` request."
                },
                "ConfigurationDoneResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `configurationDone` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "LaunchRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "This launch request is sent from the client to the debug adapter to start the debuggee with or without debugging (if `noDebug` is true).\nSince launching is debugger/runtime specific, the arguments for this request are not part of this specification.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "launch" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/LaunchRequestArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "LaunchRequestArguments": {
                    "type": "object",
                    "description": "Arguments for `launch` request. Additional attributes are implementation specific.",
                    "properties": {
                        "noDebug": {
                            "type": "boolean",
                            "description": "If true, the launch request should launch the program without enabling debugging."
                        },
                        "__restart": {
                            "type": [ "array", "boolean", "integer", "null", "number", "object", "string" ],
                            "description": "Arbitrary data from the previous, restarted session.\nThe data is sent as the `restart` attribute of the `terminated` event.\nThe client should leave the data intact."
                        }
                    }
                },
                "LaunchResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `launch` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "AttachRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The `attach` request is sent from the client to the debug adapter to attach to a debuggee that is already running.\nSince attaching is debugger/runtime specific, the arguments for this request are not part of this specification.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "attach" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/AttachRequestArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "AttachRequestArguments": {
                    "type": "object",
                    "description": "Arguments for `attach` request. Additional attributes are implementation specific.",
                    "properties": {
                        "__restart": {
                            "type": [ "array", "boolean", "integer", "null", "number", "object", "string" ],
                            "description": "Arbitrary data from the previous, restarted session.\nThe data is sent as the `restart` attribute of the `terminated` event.\nThe client should leave the data intact."
                        }
                    }
                },
                "AttachResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `attach` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "RestartRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Restarts a debug session. Clients should only call this request if the corresponding capability `supportsRestartRequest` is true.\nIf the capability is missing or has the value false, a typical client emulates `restart` by terminating the debug adapter first and then launching it anew.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "restart" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/RestartArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "RestartArguments": {
                    "type": "object",
                    "description": "Arguments for `restart` request.",
                    "properties": {
                        "arguments": {
                            "oneOf": [
                                { "$ref": "#/definitions/LaunchRequestArguments" },
                                { "$ref": "#/definitions/AttachRequestArguments" }
                            ],
                            "description": "The latest version of the `launch` or `attach` configuration."
                        }
                    }
                },
                "RestartResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `restart` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "DisconnectRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The `disconnect` request asks the debug adapter to disconnect from the debuggee (thus ending the debug session) and then to shut down itself (the debug adapter).\nIn addition, the debug adapter must terminate the debuggee if it was started with the `launch` request. If an `attach` request was used to connect to the debuggee, then the debug adapter must not terminate the debuggee.\nThis implicit behavior of when to terminate the debuggee can be overridden with the `terminateDebuggee` argument (which is only supported by a debug adapter if the corresponding capability `supportTerminateDebuggee` is true).",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "disconnect" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/DisconnectArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "DisconnectArguments": {
                    "type": "object",
                    "description": "Arguments for `disconnect` request.",
                    "properties": {
                        "restart": {
                            "type": "boolean",
                            "description": "A value of true indicates that this `disconnect` request is part of a restart sequence."
                        },
                        "terminateDebuggee": {
                            "type": "boolean",
                            "description": "Indicates whether the debuggee should be terminated when the debugger is disconnected.\nIf unspecified, the debug adapter is free to do whatever it thinks is best.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportTerminateDebuggee` is true."
                        },
                        "suspendDebuggee": {
                            "type": "boolean",
                            "description": "Indicates whether the debuggee should stay suspended when the debugger is disconnected.\nIf unspecified, the debuggee should resume execution.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportSuspendDebuggee` is true."
                        }
                    }
                },
                "DisconnectResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `disconnect` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "TerminateRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The `terminate` request is sent from the client to the debug adapter in order to shut down the debuggee gracefully. Clients should only call this request if the capability `supportsTerminateRequest` is true.\nTypically a debug adapter implements `terminate` by sending a software signal which the debuggee intercepts in order to clean things up properly before terminating itself.\nPlease note that this request does not directly affect the state of the debug session: if the debuggee decides to veto the graceful shutdown for any reason by not terminating itself, then the debug session just continues.\nClients can surface the `terminate` request as an explicit command or they can integrate it into a two stage Stop command that first sends `terminate` to request a graceful shutdown, and if that fails uses `disconnect` for a forceful shutdown.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "terminate" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/TerminateArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "TerminateArguments": {
                    "type": "object",
                    "description": "Arguments for `terminate` request.",
                    "properties": {
                        "restart": {
                            "type": "boolean",
                            "description": "A value of true indicates that this `terminate` request is part of a restart sequence."
                        }
                    }
                },
                "TerminateResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `terminate` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "BreakpointLocationsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The `breakpointLocations` request returns all possible locations for source breakpoints in a given range.\nClients should only call this request if the corresponding capability `supportsBreakpointLocationsRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "breakpointLocations" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/BreakpointLocationsArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]

                },
                "BreakpointLocationsArguments": {
                    "type": "object",
                    "description": "Arguments for `breakpointLocations` request.",
                    "properties": {
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "The source location of the breakpoints; either `source.path` or `source.reference` must be specified."
                        },
                        "line": {
                            "type": "integer",
                            "description": "Start line of range to search possible breakpoint locations in. If only the line is specified, the request returns all possible locations in that line."
                        },
                        "column": {
                            "type": "integer",
                            "description": "Start position within `line` to search possible breakpoint locations in. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based. If no column is given, the first position in the start line is assumed."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "End line of range to search possible breakpoint locations in. If no end line is given, then the end line is assumed to be the start line."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "End position within `endLine` to search possible breakpoint locations in. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based. If no end column is given, the last position in the end line is assumed."
                        }
                    },
                    "required": [ "source", "line" ]
                },
                "BreakpointLocationsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `breakpointLocations` request.\nContains possible locations for source breakpoints.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "breakpoints": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/BreakpointLocation"
                                        },
                                        "description": "Sorted set of possible breakpoint locations."
                                    }
                                },
                                "required": [ "breakpoints" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetBreakpointsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Sets multiple breakpoints for a single source and clears all previous breakpoints in that source.\nTo clear all breakpoint for a source, specify an empty array.\nWhen a breakpoint is hit, a `stopped` event (with reason `breakpoint`) is generated.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "setBreakpoints" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetBreakpointsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SetBreakpointsArguments": {
                    "type": "object",
                    "description": "Arguments for `setBreakpoints` request.",
                    "properties": {
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "The source location of the breakpoints; either `source.path` or `source.sourceReference` must be specified."
                        },
                        "breakpoints": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/SourceBreakpoint"
                            },
                            "description": "The code locations of the breakpoints."
                        },
                        "lines": {
                            "type": "array",
                            "items": {
                                "type": "integer"
                            },
                            "description": "Deprecated: The code locations of the breakpoints."
                        },
                        "sourceModified": {
                            "type": "boolean",
                            "description": "A value of true indicates that the underlying source has been modified which results in new breakpoint locations."
                        }
                    },
                    "required": [ "source" ]
                },
                "SetBreakpointsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `setBreakpoints` request.\nReturned is information about each breakpoint created by this request.\nThis includes the actual code location and whether the breakpoint could be verified.\nThe breakpoints returned are in the same order as the elements of the `breakpoints`\n(or the deprecated `lines`) array in the arguments.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "breakpoints": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Breakpoint"
                                        },
                                        "description": "Information about the breakpoints.\nThe array elements are in the same order as the elements of the `breakpoints` (or the deprecated `lines`) array in the arguments."
                                    }
                                },
                                "required": [ "breakpoints" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetFunctionBreakpointsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Replaces all existing function breakpoints with new function breakpoints.\nTo clear all function breakpoints, specify an empty array.\nWhen a function breakpoint is hit, a `stopped` event (with reason `function breakpoint`) is generated.\nClients should only call this request if the corresponding capability `supportsFunctionBreakpoints` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "setFunctionBreakpoints" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetFunctionBreakpointsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SetFunctionBreakpointsArguments": {
                    "type": "object",
                    "description": "Arguments for `setFunctionBreakpoints` request.",
                    "properties": {
                        "breakpoints": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/FunctionBreakpoint"
                            },
                            "description": "The function names of the breakpoints."
                        }
                    },
                    "required": [ "breakpoints" ]
                },
                "SetFunctionBreakpointsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `setFunctionBreakpoints` request.\nReturned is information about each breakpoint created by this request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "breakpoints": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Breakpoint"
                                        },
                                        "description": "Information about the breakpoints. The array elements correspond to the elements of the `breakpoints` array."
                                    }
                                },
                                "required": [ "breakpoints" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetExceptionBreakpointsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request configures the debugger's response to thrown exceptions.\nIf an exception is configured to break, a `stopped` event is fired (with reason `exception`).\nClients should only call this request if the corresponding capability `exceptionBreakpointFilters` returns one or more filters.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "setExceptionBreakpoints" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetExceptionBreakpointsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SetExceptionBreakpointsArguments": {
                    "type": "object",
                    "description": "Arguments for `setExceptionBreakpoints` request.",
                    "properties": {
                        "filters": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Set of exception filters specified by their ID. The set of all possible exception filters is defined by the `exceptionBreakpointFilters` capability. The `filter` and `filterOptions` sets are additive."
                        },
                        "filterOptions": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ExceptionFilterOptions"
                            },
                            "description": "Set of exception filters and their options. The set of all possible exception filters is defined by the `exceptionBreakpointFilters` capability. This attribute is only honored by a debug adapter if the corresponding capability `supportsExceptionFilterOptions` is true. The `filter` and `filterOptions` sets are additive."
                        },
                        "exceptionOptions": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ExceptionOptions"
                            },
                            "description": "Configuration options for selected exceptions.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsExceptionOptions` is true."
                        }
                    },
                    "required": [ "filters" ]
                },
                "SetExceptionBreakpointsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `setExceptionBreakpoints` request.\nThe response contains an array of `Breakpoint` objects with information about each exception breakpoint or filter. The `Breakpoint` objects are in the same order as the elements of the `filters`, `filterOptions`, `exceptionOptions` arrays given as arguments. If both `filters` and `filterOptions` are given, the returned array must start with `filters` information first, followed by `filterOptions` information.\nThe `verified` property of a `Breakpoint` object signals whether the exception breakpoint or filter could be successfully created and whether the condition or hit count expressions are valid. In case of an error the `message` property explains the problem. The `id` property can be used to introduce a unique ID for the exception breakpoint or filter so that it can be updated subsequently by sending breakpoint events.\nFor backward compatibility both the `breakpoints` array and the enclosing `body` are optional. If these elements are missing a client is not able to show problems for individual exception breakpoints or filters.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "breakpoints": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Breakpoint"
                                        },
                                        "description": "Information about the exception breakpoints or filters.\nThe breakpoints returned are in the same order as the elements of the `filters`, `filterOptions`, `exceptionOptions` arrays in the arguments. If both `filters` and `filterOptions` are given, the returned array must start with `filters` information first, followed by `filterOptions` information."
                                    }
                                }
                            }
                        }
                    }]
                },

                "DataBreakpointInfoRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Obtains information on a possible data breakpoint that could be set on an expression or variable.\nClients should only call this request if the corresponding capability `supportsDataBreakpoints` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "dataBreakpointInfo" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/DataBreakpointInfoArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "DataBreakpointInfoArguments": {
                    "type": "object",
                    "description": "Arguments for `dataBreakpointInfo` request.",
                    "properties": {
                        "variablesReference": {
                            "type": "integer",
                            "description": "Reference to the variable container if the data breakpoint is requested for a child of the container. The `variablesReference` must have been obtained in the current suspended state. See 'Lifetime of Object References' in the Overview section for details."
                        },
                        "name": {
                            "type": "string",
                            "description": "The name of the variable's child to obtain data breakpoint information for.\nIf `variablesReference` isn't specified, this can be an expression."
                        },
                        "frameId": {
                            "type": "integer",
                            "description": "When `name` is an expression, evaluate it in the scope of this stack frame. If not specified, the expression is evaluated in the global scope. When `variablesReference` is specified, this property has no effect."
                        }
                    },
                    "required": [ "name" ]
                },
                "DataBreakpointInfoResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `dataBreakpointInfo` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "dataId": {
                                        "type": [ "string", "null" ],
                                        "description": "An identifier for the data on which a data breakpoint can be registered with the `setDataBreakpoints` request or null if no data breakpoint is available."
                                    },
                                    "description": {
                                        "type": "string",
                                        "description": "UI string that describes on what data the breakpoint is set on or why a data breakpoint is not available."
                                    },
                                    "accessTypes": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/DataBreakpointAccessType"
                                        },
                                        "description": "Attribute lists the available access types for a potential data breakpoint. A UI client could surface this information."
                                    },
                                    "canPersist": {
                                        "type": "boolean",
                                        "description": "Attribute indicates that a potential data breakpoint could be persisted across sessions."
                                    }
                                },
                                "required": [ "dataId", "description" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetDataBreakpointsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Replaces all existing data breakpoints with new data breakpoints.\nTo clear all data breakpoints, specify an empty array.\nWhen a data breakpoint is hit, a `stopped` event (with reason `data breakpoint`) is generated.\nClients should only call this request if the corresponding capability `supportsDataBreakpoints` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "setDataBreakpoints" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetDataBreakpointsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SetDataBreakpointsArguments": {
                    "type": "object",
                    "description": "Arguments for `setDataBreakpoints` request.",
                    "properties": {
                        "breakpoints": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/DataBreakpoint"
                            },
                            "description": "The contents of this array replaces all existing data breakpoints. An empty array clears all data breakpoints."
                        }
                    },
                    "required": [ "breakpoints" ]
                },
                "SetDataBreakpointsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `setDataBreakpoints` request.\nReturned is information about each breakpoint created by this request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "breakpoints": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Breakpoint"
                                        },
                                        "description": "Information about the data breakpoints. The array elements correspond to the elements of the input argument `breakpoints` array."
                                    }
                                },
                                "required": [ "breakpoints" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetInstructionBreakpointsRequest": {
                    "allOf": [
                        { "$ref": "#/definitions/Request" },
                        {
                            "type": "object",
                            "description": "Replaces all existing instruction breakpoints. Typically, instruction breakpoints would be set from a disassembly window. \nTo clear all instruction breakpoints, specify an empty array.\nWhen an instruction breakpoint is hit, a `stopped` event (with reason `instruction breakpoint`) is generated.\nClients should only call this request if the corresponding capability `supportsInstructionBreakpoints` is true.",
                            "properties": {
                                "command": {
                                "type": "string",
                                "enum": [ "setInstructionBreakpoints" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetInstructionBreakpointsArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "SetInstructionBreakpointsArguments": {
                    "type": "object",
                    "description": "Arguments for `setInstructionBreakpoints` request",
                    "properties": {
                        "breakpoints": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/InstructionBreakpoint"
                            },
                            "description": "The instruction references of the breakpoints"
                        }
                    },
                    "required": ["breakpoints"]
                },
                "SetInstructionBreakpointsResponse": {
                    "allOf": [
                        { "$ref": "#/definitions/Response" },
                        {
                            "type": "object",
                            "description": "Response to `setInstructionBreakpoints` request",
                            "properties": {
                                "body": {
                                    "type": "object",
                                    "properties": {
                                        "breakpoints": {
                                            "type": "array",
                                            "items": {
                                                "$ref": "#/definitions/Breakpoint"
                                            },
                                        "description": "Information about the breakpoints. The array elements correspond to the elements of the `breakpoints` array."
                                    }
                                },
                                "required": [ "breakpoints" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "ContinueRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request resumes execution of all threads. If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true resumes only the specified thread. If not all threads were resumed, the `allThreadsContinued` attribute of the response should be set to false.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "continue" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ContinueArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "ContinueArguments": {
                    "type": "object",
                    "description": "Arguments for `continue` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Specifies the active thread. If the debug adapter supports single thread execution (see `supportsSingleThreadExecutionRequests`) and the argument `singleThread` is true, only the thread with this ID is resumed."
                        },
                        "singleThread": {
                            "type": "boolean",
                            "description": "If this flag is true, execution is resumed only for the thread with given `threadId`."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "ContinueResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `continue` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "allThreadsContinued": {
                                        "type": "boolean",
                                        "description": "The value true (or a missing property) signals to the client that all threads have been resumed. The value false indicates that not all threads were resumed."
                                    }
                                }
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "NextRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request executes one step (in the given granularity) for the specified thread and allows all other threads to run freely by resuming them.\nIf the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.\nThe debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "next" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/NextArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "NextArguments": {
                    "type": "object",
                    "description": "Arguments for `next` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Specifies the thread for which to resume execution for one step (of the given granularity)."
                        },
                        "singleThread": {
                            "type": "boolean",
                            "description": "If this flag is true, all other suspended threads are not resumed."
                        },
                        "granularity": {
                            "$ref": "#/definitions/SteppingGranularity",
                            "description": "Stepping granularity. If no granularity is specified, a granularity of `statement` is assumed."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "NextResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `next` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "StepInRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request resumes the given thread to step into a function/method and allows all other threads to run freely by resuming them.\nIf the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.\nIf the request cannot step into a target, `stepIn` behaves like the `next` request.\nThe debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.\nIf there are multiple function/method calls (or other targets) on the source line,\nthe argument `targetId` can be used to control into which target the `stepIn` should occur.\nThe list of possible targets for a given source line can be retrieved via the `stepInTargets` request.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "stepIn" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/StepInArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "StepInArguments": {
                    "type": "object",
                    "description": "Arguments for `stepIn` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Specifies the thread for which to resume execution for one step-into (of the given granularity)."
                        },
                        "singleThread": {
                            "type": "boolean",
                            "description": "If this flag is true, all other suspended threads are not resumed."
                        },
                        "targetId": {
                            "type": "integer",
                            "description": "Id of the target to step into."
                        },
                        "granularity": {
                            "$ref": "#/definitions/SteppingGranularity",
                            "description": "Stepping granularity. If no granularity is specified, a granularity of `statement` is assumed."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "StepInResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `stepIn` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "StepOutRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request resumes the given thread to step out (return) from a function/method and allows all other threads to run freely by resuming them.\nIf the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.\nThe debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "stepOut" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/StepOutArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "StepOutArguments": {
                    "type": "object",
                    "description": "Arguments for `stepOut` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Specifies the thread for which to resume execution for one step-out (of the given granularity)."
                        },
                        "singleThread": {
                            "type": "boolean",
                            "description": "If this flag is true, all other suspended threads are not resumed."
                        },
                        "granularity": {
                            "$ref": "#/definitions/SteppingGranularity",
                            "description": "Stepping granularity. If no granularity is specified, a granularity of `statement` is assumed."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "StepOutResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `stepOut` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "StepBackRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request executes one backward step (in the given granularity) for the specified thread and allows all other threads to run backward freely by resuming them.\nIf the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true prevents other suspended threads from resuming.\nThe debug adapter first sends the response and then a `stopped` event (with reason `step`) after the step has completed.\nClients should only call this request if the corresponding capability `supportsStepBack` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "stepBack" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/StepBackArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "StepBackArguments": {
                    "type": "object",
                    "description": "Arguments for `stepBack` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Specifies the thread for which to resume execution for one step backwards (of the given granularity)."
                        },
                        "singleThread": {
                            "type": "boolean",
                            "description": "If this flag is true, all other suspended threads are not resumed."
                        },
                        "granularity": {
                            "$ref": "#/definitions/SteppingGranularity",
                            "description": "Stepping granularity to step. If no granularity is specified, a granularity of `statement` is assumed."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "StepBackResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `stepBack` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "ReverseContinueRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request resumes backward execution of all threads. If the debug adapter supports single thread execution (see capability `supportsSingleThreadExecutionRequests`), setting the `singleThread` argument to true resumes only the specified thread. If not all threads were resumed, the `allThreadsContinued` attribute of the response should be set to false.\nClients should only call this request if the corresponding capability `supportsStepBack` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "reverseContinue" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ReverseContinueArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "ReverseContinueArguments": {
                    "type": "object",
                    "description": "Arguments for `reverseContinue` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Specifies the active thread. If the debug adapter supports single thread execution (see `supportsSingleThreadExecutionRequests`) and the `singleThread` argument is true, only the thread with this ID is resumed."
                        },
                        "singleThread": {
                            "type": "boolean",
                            "description": "If this flag is true, backward execution is resumed only for the thread with given `threadId`."
                        }

                    },
                    "required": [ "threadId" ]
                },
                "ReverseContinueResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `reverseContinue` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "RestartFrameRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request restarts execution of the specified stack frame.\nThe debug adapter first sends the response and then a `stopped` event (with reason `restart`) after the restart has completed.\nClients should only call this request if the corresponding capability `supportsRestartFrame` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "restartFrame" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/RestartFrameArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "RestartFrameArguments": {
                    "type": "object",
                    "description": "Arguments for `restartFrame` request.",
                    "properties": {
                        "frameId": {
                            "type": "integer",
                            "description": "Restart the stack frame identified by `frameId`. The `frameId` must have been obtained in the current suspended state. See 'Lifetime of Object References' in the Overview section for details."
                        }
                    },
                    "required": [ "frameId" ]
                },
                "RestartFrameResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `restartFrame` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "GotoRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request sets the location where the debuggee will continue to run.\nThis makes it possible to skip the execution of code or to execute code again.\nThe code between the current location and the goto target is not executed but skipped.\nThe debug adapter first sends the response and then a `stopped` event with reason `goto`.\nClients should only call this request if the corresponding capability `supportsGotoTargetsRequest` is true (because only then goto targets exist that can be passed as arguments).",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "goto" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/GotoArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "GotoArguments": {
                    "type": "object",
                    "description": "Arguments for `goto` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Set the goto target for this thread."
                        },
                        "targetId": {
                            "type": "integer",
                            "description": "The location where the debuggee will continue to run."
                        }
                    },
                    "required": [ "threadId", "targetId" ]
                },
                "GotoResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `goto` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "PauseRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request suspends the debuggee.\nThe debug adapter first sends the response and then a `stopped` event (with reason `pause`) after the thread has been paused successfully.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "pause" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/PauseArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "PauseArguments": {
                    "type": "object",
                    "description": "Arguments for `pause` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Pause execution for this thread."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "PauseResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `pause` request. This is just an acknowledgement, so no body field is required."
                    }]
                },

                "StackTraceRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request returns a stacktrace from the current execution state of a given thread.\nA client can request all stack frames by omitting the startFrame and levels arguments. For performance-conscious clients and if the corresponding capability `supportsDelayedStackTraceLoading` is true, stack frames can be retrieved in a piecemeal way with the `startFrame` and `levels` arguments. The response of the `stackTrace` request may contain a `totalFrames` property that hints at the total number of frames in the stack. If a client needs this total number upfront, it can issue a request for a single (first) frame and depending on the value of `totalFrames` decide how to proceed. In any case a client should be prepared to receive fewer frames than requested, which is an indication that the end of the stack has been reached.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "stackTrace" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/StackTraceArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "StackTraceArguments": {
                    "type": "object",
                    "description": "Arguments for `stackTrace` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Retrieve the stacktrace for this thread."
                        },
                        "startFrame": {
                            "type": "integer",
                            "description": "The index of the first frame to return; if omitted frames start at 0."
                        },
                        "levels": {
                            "type": "integer",
                            "description": "The maximum number of frames to return. If levels is not specified or 0, all frames are returned."
                        },
                        "format": {
                            "$ref": "#/definitions/StackFrameFormat",
                            "description": "Specifies details on how to format the stack frames.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsValueFormattingOptions` is true."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "StackTraceResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `stackTrace` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "stackFrames": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/StackFrame"
                                        },
                                        "description": "The frames of the stack frame. If the array has length zero, there are no stack frames available.\nThis means that there is no location information available."
                                    },
                                    "totalFrames": {
                                        "type": "integer",
                                        "description": "The total number of frames available in the stack. If omitted or if `totalFrames` is larger than the available frames, a client is expected to request frames until a request returns less frames than requested (which indicates the end of the stack). Returning monotonically increasing `totalFrames` values for subsequent requests can be used to enforce paging in the client."
                                    }
                                },
                                "required": [ "stackFrames" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "ScopesRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request returns the variable scopes for a given stack frame ID.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "scopes" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ScopesArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "ScopesArguments": {
                    "type": "object",
                    "description": "Arguments for `scopes` request.",
                    "properties": {
                        "frameId": {
                            "type": "integer",
                            "description": "Retrieve the scopes for the stack frame identified by `frameId`. The `frameId` must have been obtained in the current suspended state. See 'Lifetime of Object References' in the Overview section for details."
                        }
                    },
                    "required": [ "frameId" ]
                },
                "ScopesResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `scopes` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "scopes": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Scope"
                                        },
                                        "description": "The scopes of the stack frame. If the array has length zero, there are no scopes available."
                                    }
                                },
                                "required": [ "scopes" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "VariablesRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Retrieves all child variables for the given variable reference.\nA filter can be used to limit the fetched children to either named or indexed children.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "variables" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/VariablesArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "VariablesArguments": {
                    "type": "object",
                    "description": "Arguments for `variables` request.",
                    "properties": {
                        "variablesReference": {
                            "type": "integer",
                            "description": "The variable for which to retrieve its children. The `variablesReference` must have been obtained in the current suspended state. See 'Lifetime of Object References' in the Overview section for details."
                        },
                        "filter": {
                            "type": "string",
                            "enum": [ "indexed", "named" ],
                            "description": "Filter to limit the child variables to either named or indexed. If omitted, both types are fetched."
                        },
                        "start": {
                            "type": "integer",
                            "description": "The index of the first variable to return; if omitted children start at 0."
                        },
                        "count": {
                            "type": "integer",
                            "description": "The number of variables to return. If count is missing or 0, all variables are returned."
                        },
                        "format": {
                            "$ref": "#/definitions/ValueFormat",
                            "description": "Specifies details on how to format the Variable values.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsValueFormattingOptions` is true."
                        }
                    },
                    "required": [ "variablesReference" ]
                },
                "VariablesResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `variables` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "variables": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Variable"
                                        },
                                        "description": "All (or a range) of variables for the given variable reference."
                                    }
                                },
                                "required": [ "variables" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetVariableRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Set the variable with the given name in the variable container to a new value. Clients should only call this request if the corresponding capability `supportsSetVariable` is true.\nIf a debug adapter implements both `setVariable` and `setExpression`, a client will only use `setExpression` if the variable has an `evaluateName` property.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "setVariable" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetVariableArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SetVariableArguments": {
                    "type": "object",
                    "description": "Arguments for `setVariable` request.",
                    "properties": {
                        "variablesReference": {
                            "type": "integer",
                            "description": "The reference of the variable container. The `variablesReference` must have been obtained in the current suspended state. See 'Lifetime of Object References' in the Overview section for details."
                        },
                        "name": {
                            "type": "string",
                            "description": "The name of the variable in the container."
                        },
                        "value": {
                            "type": "string",
                            "description": "The value of the variable."
                        },
                        "format": {
                            "$ref": "#/definitions/ValueFormat",
                            "description": "Specifies details on how to format the response value."
                        }
                    },
                    "required": [ "variablesReference", "name", "value" ]
                },
                "SetVariableResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `setVariable` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "value": {
                                        "type": "string",
                                        "description": "The new value of the variable."
                                    },
                                    "type": {
                                        "type": "string",
                                        "description": "The type of the new value. Typically shown in the UI when hovering over the value."
                                    },
                                    "variablesReference": {
                                        "type": "integer",
                                        "description": "If `variablesReference` is > 0, the new value is structured and its children can be retrieved by passing `variablesReference` to the `variables` request as long as execution remains suspended. See 'Lifetime of Object References' in the Overview section for details."
                                    },
                                    "namedVariables": {
                                        "type": "integer",
                                        "description": "The number of named child variables.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                                    },
                                    "indexedVariables": {
                                        "type": "integer",
                                        "description": "The number of indexed child variables.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                                    }
                                },
                                "required": [ "value" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SourceRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request retrieves the source code for a given source reference.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "source" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SourceArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SourceArguments": {
                    "type": "object",
                    "description": "Arguments for `source` request.",
                    "properties": {
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "Specifies the source content to load. Either `source.path` or `source.sourceReference` must be specified."
                        },
                        "sourceReference": {
                            "type": "integer",
                            "description": "The reference to the source. This is the same as `source.sourceReference`.\nThis is provided for backward compatibility since old clients do not understand the `source` attribute."
                        }
                    },
                    "required": [ "sourceReference" ]
                },
                "SourceResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `source` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "content": {
                                        "type": "string",
                                        "description": "Content of the source reference."
                                    },
                                    "mimeType": {
                                        "type": "string",
                                        "description": "Content type (MIME type) of the source."
                                    }
                                },
                                "required": [ "content" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "ThreadsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request retrieves a list of all threads.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "threads" ]
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "ThreadsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `threads` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "threads": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Thread"
                                        },
                                        "description": "All threads."
                                    }
                                },
                                "required": [ "threads" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "TerminateThreadsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "The request terminates the threads with the given ids.\nClients should only call this request if the corresponding capability `supportsTerminateThreadsRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "terminateThreads" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/TerminateThreadsArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "TerminateThreadsArguments": {
                    "type": "object",
                    "description": "Arguments for `terminateThreads` request.",
                    "properties": {
                        "threadIds": {
                            "type": "array",
                            "items": {
                                "type": "integer"
                            },
                            "description": "Ids of threads to be terminated."
                        }
                    }
                },
                "TerminateThreadsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `terminateThreads` request. This is just an acknowledgement, no body field is required."
                    }]
                },

                "ModulesRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Modules can be retrieved from the debug adapter with this request which can either return all modules or a range of modules to support paging.\nClients should only call this request if the corresponding capability `supportsModulesRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "modules" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ModulesArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "ModulesArguments": {
                    "type": "object",
                    "description": "Arguments for `modules` request.",
                    "properties": {
                        "startModule": {
                            "type": "integer",
                            "description": "The index of the first module to return; if omitted modules start at 0."
                        },
                        "moduleCount": {
                            "type": "integer",
                            "description": "The number of modules to return. If `moduleCount` is not specified or 0, all modules are returned."
                        }
                    }
                },
                "ModulesResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `modules` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "modules": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Module"
                                        },
                                        "description": "All modules or range of modules."
                                    },
                                    "totalModules": {
                                        "type": "integer",
                                        "description": "The total number of modules available."
                                    }
                                },
                                "required": [ "modules" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "LoadedSourcesRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Retrieves the set of all sources currently loaded by the debugged process.\nClients should only call this request if the corresponding capability `supportsLoadedSourcesRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "loadedSources" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/LoadedSourcesArguments"
                            }
                        },
                        "required": [ "command" ]
                    }]
                },
                "LoadedSourcesArguments": {
                    "type": "object",
                    "description": "Arguments for `loadedSources` request."
                },
                "LoadedSourcesResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `loadedSources` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "sources": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/Source"
                                        },
                                        "description": "Set of loaded sources."
                                    }
                                },
                                "required": [ "sources" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "EvaluateRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Evaluates the given expression in the context of the topmost stack frame.\nThe expression has access to any variables and arguments that are in scope.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "evaluate" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/EvaluateArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "EvaluateArguments": {
                    "type": "object",
                    "description": "Arguments for `evaluate` request.",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "The expression to evaluate."
                        },
                        "frameId": {
                            "type": "integer",
                            "description": "Evaluate the expression in the scope of this stack frame. If not specified, the expression is evaluated in the global scope."
                        },
                        "context": {
                            "type": "string",
                            "_enum": [ "watch", "repl", "hover", "clipboard", "variables" ],
                            "enumDescriptions": [
                                "evaluate is called from a watch view context.",
                                "evaluate is called from a REPL context.",
                                "evaluate is called to generate the debug hover contents.\nThis value should only be used if the corresponding capability `supportsEvaluateForHovers` is true.",
                                "evaluate is called to generate clipboard contents.\nThis value should only be used if the corresponding capability `supportsClipboardContext` is true.",
                                "evaluate is called from a variables view context."
                            ],
                            "description": "The context in which the evaluate request is used."
                        },
                        "format": {
                            "$ref": "#/definitions/ValueFormat",
                            "description": "Specifies details on how to format the result.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsValueFormattingOptions` is true."
                        }
                    },
                    "required": [ "expression" ]
                },
                "EvaluateResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `evaluate` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "result": {
                                        "type": "string",
                                        "description": "The result of the evaluate request."
                                    },
                                    "type": {
                                        "type": "string",
                                        "description": "The type of the evaluate result.\nThis attribute should only be returned by a debug adapter if the corresponding capability `supportsVariableType` is true."
                                    },
                                    "presentationHint": {
                                        "$ref": "#/definitions/VariablePresentationHint",
                                        "description": "Properties of an evaluate result that can be used to determine how to render the result in the UI."
                                    },
                                    "variablesReference": {
                                        "type": "integer",
                                        "description": "If `variablesReference` is > 0, the evaluate result is structured and its children can be retrieved by passing `variablesReference` to the `variables` request as long as execution remains suspended. See 'Lifetime of Object References' in the Overview section for details."
                                    },
                                    "namedVariables": {
                                        "type": "integer",
                                        "description": "The number of named child variables.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                                    },
                                    "indexedVariables": {
                                        "type": "integer",
                                        "description": "The number of indexed child variables.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                                    },
                                    "memoryReference": {
                                        "type": "string",
                                        "description": "A memory reference to a location appropriate for this result.\nFor pointer type eval results, this is generally a reference to the memory address contained in the pointer.\nThis attribute should be returned by a debug adapter if corresponding capability `supportsMemoryReferences` is true."
                                    }
                                },
                                "required": [ "result", "variablesReference" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "SetExpressionRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Evaluates the given `value` expression and assigns it to the `expression` which must be a modifiable l-value.\nThe expressions have access to any variables and arguments that are in scope of the specified frame.\nClients should only call this request if the corresponding capability `supportsSetExpression` is true.\nIf a debug adapter implements both `setExpression` and `setVariable`, a client uses `setExpression` if the variable has an `evaluateName` property.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "setExpression" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/SetExpressionArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "SetExpressionArguments": {
                    "type": "object",
                    "description": "Arguments for `setExpression` request.",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "The l-value expression to assign to."
                        },
                        "value": {
                            "type": "string",
                            "description": "The value expression to assign to the l-value expression."
                        },
                        "frameId": {
                            "type": "integer",
                            "description": "Evaluate the expressions in the scope of this stack frame. If not specified, the expressions are evaluated in the global scope."
                        },
                        "format": {
                            "$ref": "#/definitions/ValueFormat",
                            "description": "Specifies how the resulting value should be formatted."
                        }
                    },
                    "required": [ "expression", "value" ]
                },
                "SetExpressionResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `setExpression` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "value": {
                                        "type": "string",
                                        "description": "The new value of the expression."
                                    },
                                    "type": {
                                        "type": "string",
                                        "description": "The type of the value.\nThis attribute should only be returned by a debug adapter if the corresponding capability `supportsVariableType` is true."
                                    },
                                    "presentationHint": {
                                        "$ref": "#/definitions/VariablePresentationHint",
                                        "description": "Properties of a value that can be used to determine how to render the result in the UI."
                                    },
                                    "variablesReference": {
                                        "type": "integer",
                                        "description": "If `variablesReference` is > 0, the evaluate result is structured and its children can be retrieved by passing `variablesReference` to the `variables` request as long as execution remains suspended. See 'Lifetime of Object References' in the Overview section for details."
                                    },
                                    "namedVariables": {
                                        "type": "integer",
                                        "description": "The number of named child variables.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                                    },
                                    "indexedVariables": {
                                        "type": "integer",
                                        "description": "The number of indexed child variables.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                                    }
                                },
                                "required": [ "value" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "StepInTargetsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "This request retrieves the possible step-in targets for the specified stack frame.\nThese targets can be used in the `stepIn` request.\nClients should only call this request if the corresponding capability `supportsStepInTargetsRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "stepInTargets" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/StepInTargetsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "StepInTargetsArguments": {
                    "type": "object",
                    "description": "Arguments for `stepInTargets` request.",
                    "properties": {
                        "frameId": {
                            "type": "integer",
                            "description": "The stack frame for which to retrieve the possible step-in targets."
                        }
                    },
                    "required": [ "frameId" ]
                },
                "StepInTargetsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `stepInTargets` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "targets": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/StepInTarget"
                                        },
                                        "description": "The possible step-in targets of the specified source location."
                                    }
                                },
                                "required": [ "targets" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "GotoTargetsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "This request retrieves the possible goto targets for the specified source location.\nThese targets can be used in the `goto` request.\nClients should only call this request if the corresponding capability `supportsGotoTargetsRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "gotoTargets" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/GotoTargetsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "GotoTargetsArguments": {
                    "type": "object",
                    "description": "Arguments for `gotoTargets` request.",
                    "properties": {
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "The source location for which the goto targets are determined."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The line location for which the goto targets are determined."
                        },
                        "column": {
                            "type": "integer",
                            "description": "The position within `line` for which the goto targets are determined. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        }
                    },
                    "required": [ "source", "line" ]
                },
                "GotoTargetsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `gotoTargets` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "targets": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/GotoTarget"
                                        },
                                        "description": "The possible goto targets of the specified location."
                                    }
                                },
                                "required": [ "targets" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "CompletionsRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Returns a list of possible completions for a given caret position and text.\nClients should only call this request if the corresponding capability `supportsCompletionsRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "completions" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/CompletionsArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "CompletionsArguments": {
                    "type": "object",
                    "description": "Arguments for `completions` request.",
                    "properties": {
                        "frameId": {
                            "type": "integer",
                            "description": "Returns completions in the scope of this stack frame. If not specified, the completions are returned for the global scope."
                        },
                        "text": {
                            "type": "string",
                            "description": "One or more source lines. Typically this is the text users have typed into the debug console before they asked for completion."
                        },
                        "column": {
                            "type": "integer",
                            "description": "The position within `text` for which to determine the completion proposals. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "line": {
                            "type": "integer",
                            "description": "A line for which to determine the completion proposals. If missing the first line of the text is assumed."
                        }
                    },
                    "required": [ "text", "column" ]
                },
                "CompletionsResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `completions` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "targets": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/CompletionItem"
                                        },
                                        "description": "The possible completions for ."
                                    }
                                },
                                "required": [ "targets" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "ExceptionInfoRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Retrieves the details of the exception that caused this event to be raised.\nClients should only call this request if the corresponding capability `supportsExceptionInfoRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "exceptionInfo" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ExceptionInfoArguments"
                            }
                        },
                        "required": [ "command", "arguments"  ]
                    }]
                },
                "ExceptionInfoArguments": {
                    "type": "object",
                    "description": "Arguments for `exceptionInfo` request.",
                    "properties": {
                        "threadId": {
                            "type": "integer",
                            "description": "Thread for which exception information should be retrieved."
                        }
                    },
                    "required": [ "threadId" ]
                },
                "ExceptionInfoResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `exceptionInfo` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "exceptionId": {
                                        "type": "string",
                                        "description": "ID of the exception that was thrown."
                                    },
                                    "description": {
                                        "type": "string",
                                        "description": "Descriptive text for the exception."
                                    },
                                    "breakMode": {
                                        "$ref": "#/definitions/ExceptionBreakMode",
                                        "description": "Mode that caused the exception notification to be raised."
                                    },
                                    "details": {
                                        "$ref": "#/definitions/ExceptionDetails",
                                        "description": "Detailed information about the exception."
                                    }
                                },
                                "required": [ "exceptionId", "breakMode" ]
                            }
                        },
                        "required": [ "body" ]
                    }]
                },

                "ReadMemoryRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Reads bytes from memory at the provided location.\nClients should only call this request if the corresponding capability `supportsReadMemoryRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "readMemory" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/ReadMemoryArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "ReadMemoryArguments": {
                    "type": "object",
                    "description": "Arguments for `readMemory` request.",
                    "properties": {
                        "memoryReference": {
                            "type": "string",
                            "description": "Memory reference to the base location from which data should be read."
                        },
                        "offset": {
                            "type": "integer",
                            "description": "Offset (in bytes) to be applied to the reference location before reading data. Can be negative."
                        },
                        "count": {
                            "type": "integer",
                            "description": "Number of bytes to read at the specified location and offset."
                        }
                    },
                    "required": [ "memoryReference", "count" ]
                },
                "ReadMemoryResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `readMemory` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "address": {
                                        "type": "string",
                                        "description": "The address of the first byte of data returned.\nTreated as a hex value if prefixed with `0x`, or as a decimal value otherwise."
                                    },
                                    "unreadableBytes": {
                                        "type": "integer",
                                        "description": "The number of unreadable bytes encountered after the last successfully read byte.\nThis can be used to determine the number of bytes that should be skipped before a subsequent `readMemory` request succeeds."
                                    },
                                    "data": {
                                        "type": "string",
                                        "description": "The bytes read from memory, encoded using base64. If the decoded length of `data` is less than the requested `count` in the original `readMemory` request, and `unreadableBytes` is zero or omitted, then the client should assume it's reached the end of readable memory."
                                    }
                                },
                                "required": [ "address" ]
                            }
                        }
                    }]
                },

                "WriteMemoryRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Writes bytes to memory at the provided location.\nClients should only call this request if the corresponding capability `supportsWriteMemoryRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "writeMemory" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/WriteMemoryArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "WriteMemoryArguments": {
                    "type": "object",
                    "description": "Arguments for `writeMemory` request.",
                    "properties": {
                        "memoryReference": {
                            "type": "string",
                            "description": "Memory reference to the base location to which data should be written."
                        },
                        "offset": {
                            "type": "integer",
                            "description": "Offset (in bytes) to be applied to the reference location before writing data. Can be negative."
                        },
                        "allowPartial": {
                            "type": "boolean",
                            "description": "Property to control partial writes. If true, the debug adapter should attempt to write memory even if the entire memory region is not writable. In such a case the debug adapter should stop after hitting the first byte of memory that cannot be written and return the number of bytes written in the response via the `offset` and `bytesWritten` properties.\nIf false or missing, a debug adapter should attempt to verify the region is writable before writing, and fail the response if it is not."
                        },
                        "data": {
                            "type": "string",
                            "description": "Bytes to write, encoded using base64."
                        }
                    },
                    "required": [ "memoryReference", "data" ]
                },
                "WriteMemoryResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `writeMemory` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "offset": {
                                        "type": "integer",
                                        "description": "Property that should be returned when `allowPartial` is true to indicate the offset of the first byte of data successfully written. Can be negative."
                                    },
                                    "bytesWritten": {
                                        "type": "integer",
                                        "description": "Property that should be returned when `allowPartial` is true to indicate the number of bytes starting from address that were successfully written."
                                    }
                                }
                            }
                        }
                    }]
                },

                "DisassembleRequest": {
                    "allOf": [ { "$ref": "#/definitions/Request" }, {
                        "type": "object",
                        "description": "Disassembles code stored at the provided location.\nClients should only call this request if the corresponding capability `supportsDisassembleRequest` is true.",
                        "properties": {
                            "command": {
                                "type": "string",
                                "enum": [ "disassemble" ]
                            },
                            "arguments": {
                                "$ref": "#/definitions/DisassembleArguments"
                            }
                        },
                        "required": [ "command", "arguments" ]
                    }]
                },
                "DisassembleArguments": {
                    "type": "object",
                    "description": "Arguments for `disassemble` request.",
                    "properties": {
                        "memoryReference": {
                            "type": "string",
                            "description": "Memory reference to the base location containing the instructions to disassemble."
                        },
                        "offset": {
                            "type": "integer",
                            "description": "Offset (in bytes) to be applied to the reference location before disassembling. Can be negative."
                        },
                        "instructionOffset": {
                            "type": "integer",
                            "description": "Offset (in instructions) to be applied after the byte offset (if any) before disassembling. Can be negative."
                        },
                        "instructionCount": {
                            "type": "integer",
                            "description": "Number of instructions to disassemble starting at the specified location and offset.\nAn adapter must return exactly this number of instructions - any unavailable instructions should be replaced with an implementation-defined 'invalid instruction' value."
                        },
                        "resolveSymbols": {
                            "type": "boolean",
                            "description": "If true, the adapter should attempt to resolve memory addresses and other values to symbolic names."
                        }
                    },
                    "required": [ "memoryReference", "instructionCount" ]
                },
                "DisassembleResponse": {
                    "allOf": [ { "$ref": "#/definitions/Response" }, {
                        "type": "object",
                        "description": "Response to `disassemble` request.",
                        "properties": {
                            "body": {
                                "type": "object",
                                "properties": {
                                    "instructions": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/DisassembledInstruction"
                                        },
                                        "description": "The list of disassembled instructions."
                                    }
                                },
                                "required": [ "instructions" ]
                            }
                        }
                    }]
                },

                "Capabilities": {
                    "type": "object",
                    "title": "Types",
                    "description": "Information about the capabilities of a debug adapter.",
                    "properties": {
                        "supportsConfigurationDoneRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `configurationDone` request."
                        },
                        "supportsFunctionBreakpoints": {
                            "type": "boolean",
                            "description": "The debug adapter supports function breakpoints."
                        },
                        "supportsConditionalBreakpoints": {
                            "type": "boolean",
                            "description": "The debug adapter supports conditional breakpoints."
                        },
                        "supportsHitConditionalBreakpoints": {
                            "type": "boolean",
                            "description": "The debug adapter supports breakpoints that break execution after a specified number of hits."
                        },
                        "supportsEvaluateForHovers": {
                            "type": "boolean",
                            "description": "The debug adapter supports a (side effect free) `evaluate` request for data hovers."
                        },
                        "exceptionBreakpointFilters": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ExceptionBreakpointsFilter"
                            },
                            "description": "Available exception filter options for the `setExceptionBreakpoints` request."
                        },
                        "supportsStepBack": {
                            "type": "boolean",
                            "description": "The debug adapter supports stepping back via the `stepBack` and `reverseContinue` requests."
                        },
                        "supportsSetVariable": {
                            "type": "boolean",
                            "description": "The debug adapter supports setting a variable to a value."
                        },
                        "supportsRestartFrame": {
                            "type": "boolean",
                            "description": "The debug adapter supports restarting a frame."
                        },
                        "supportsGotoTargetsRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `gotoTargets` request."
                        },
                        "supportsStepInTargetsRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `stepInTargets` request."
                        },
                        "supportsCompletionsRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `completions` request."
                        },
                        "completionTriggerCharacters": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "The set of characters that should trigger completion in a REPL. If not specified, the UI should assume the `.` character."
                        },
                        "supportsModulesRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `modules` request."
                        },
                        "additionalModuleColumns": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ColumnDescriptor"
                            },
                            "description": "The set of additional module information exposed by the debug adapter."
                        },
                        "supportedChecksumAlgorithms": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ChecksumAlgorithm"
                            },
                            "description": "Checksum algorithms supported by the debug adapter."
                        },
                        "supportsRestartRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `restart` request. In this case a client should not implement `restart` by terminating and relaunching the adapter but by calling the `restart` request."
                        },
                        "supportsExceptionOptions": {
                            "type": "boolean",
                            "description": "The debug adapter supports `exceptionOptions` on the `setExceptionBreakpoints` request."
                        },
                        "supportsValueFormattingOptions": {
                            "type": "boolean",
                            "description": "The debug adapter supports a `format` attribute on the `stackTrace`, `variables`, and `evaluate` requests."
                        },
                        "supportsExceptionInfoRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `exceptionInfo` request."
                        },
                        "supportTerminateDebuggee": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `terminateDebuggee` attribute on the `disconnect` request."
                        },
                        "supportSuspendDebuggee": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `suspendDebuggee` attribute on the `disconnect` request."
                        },
                        "supportsDelayedStackTraceLoading": {
                            "type": "boolean",
                            "description": "The debug adapter supports the delayed loading of parts of the stack, which requires that both the `startFrame` and `levels` arguments and the `totalFrames` result of the `stackTrace` request are supported."
                        },
                        "supportsLoadedSourcesRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `loadedSources` request."
                        },
                        "supportsLogPoints": {
                            "type": "boolean",
                            "description": "The debug adapter supports log points by interpreting the `logMessage` attribute of the `SourceBreakpoint`."
                        },
                        "supportsTerminateThreadsRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `terminateThreads` request."
                        },
                        "supportsSetExpression": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `setExpression` request."
                        },
                        "supportsTerminateRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `terminate` request."
                        },
                        "supportsDataBreakpoints": {
                            "type": "boolean",
                            "description": "The debug adapter supports data breakpoints."
                        },
                        "supportsReadMemoryRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `readMemory` request."
                        },
                        "supportsWriteMemoryRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `writeMemory` request."
                        },
                        "supportsDisassembleRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `disassemble` request."
                        },
                        "supportsCancelRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `cancel` request."
                        },
                        "supportsBreakpointLocationsRequest": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `breakpointLocations` request."
                        },
                        "supportsClipboardContext": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `clipboard` context value in the `evaluate` request."
                        },
                        "supportsSteppingGranularity": {
                            "type": "boolean",
                            "description": "The debug adapter supports stepping granularities (argument `granularity`) for the stepping requests."
                        },
                        "supportsInstructionBreakpoints": {
                            "type": "boolean",
                            "description": "The debug adapter supports adding breakpoints based on instruction references."
                        },
                        "supportsExceptionFilterOptions": {
                            "type": "boolean",
                            "description": "The debug adapter supports `filterOptions` as an argument on the `setExceptionBreakpoints` request."
                        },
                        "supportsSingleThreadExecutionRequests": {
                            "type": "boolean",
                            "description": "The debug adapter supports the `singleThread` property on the execution requests (`continue`, `next`, `stepIn`, `stepOut`, `reverseContinue`, `stepBack`)."
                        }
                    }
                },

                "ExceptionBreakpointsFilter": {
                    "type": "object",
                    "description": "An `ExceptionBreakpointsFilter` is shown in the UI as an filter option for configuring how exceptions are dealt with.",
                    "properties": {
                        "filter": {
                            "type": "string",
                            "description": "The internal ID of the filter option. This value is passed to the `setExceptionBreakpoints` request."
                        },
                        "label": {
                            "type": "string",
                            "description": "The name of the filter option. This is shown in the UI."
                        },
                        "description": {
                            "type": "string",
                            "description": "A help text providing additional information about the exception filter. This string is typically shown as a hover and can be translated."
                        },
                        "default": {
                            "type": "boolean",
                            "description": "Initial value of the filter option. If not specified a value false is assumed."
                        },
                        "supportsCondition": {
                            "type": "boolean",
                            "description": "Controls whether a condition can be specified for this filter option. If false or missing, a condition can not be set."
                        },
                        "conditionDescription": {
                            "type": "string",
                            "description": "A help text providing information about the condition. This string is shown as the placeholder text for a text box and can be translated."
                        }
                    },
                    "required": [ "filter", "label" ]
                },

                "Message": {
                    "type": "object",
                    "description": "A structured message object. Used to return errors from requests.",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "Unique (within a debug adapter implementation) identifier for the message. The purpose of these error IDs is to help extension authors that have the requirement that every user visible error message needs a corresponding error number, so that users or customer support can find information about the specific error more easily."
                        },
                        "format": {
                            "type": "string",
                            "description": "A format string for the message. Embedded variables have the form `{name}`.\nIf variable name starts with an underscore character, the variable does not contain user data (PII) and can be safely used for telemetry purposes."
                        },
                        "variables": {
                            "type": "object",
                            "description": "An object used as a dictionary for looking up the variables in the format string.",
                            "additionalProperties": {
                                "type": "string",
                                "description": "All dictionary values must be strings."
                            }
                        },
                        "sendTelemetry": {
                            "type": "boolean",
                            "description": "If true send to telemetry."
                        },
                        "showUser": {
                            "type": "boolean",
                            "description": "If true show user."
                        },
                        "url": {
                            "type": "string",
                            "description": "A url where additional information about this message can be found."
                        },
                        "urlLabel": {
                            "type": "string",
                            "description": "A label that is presented to the user as the UI for opening the url."
                        }
                    },
                    "required": [ "id", "format" ]
                },

                "Module": {
                    "type": "object",
                    "description": "A Module object represents a row in the modules view.\nThe `id` attribute identifies a module in the modules view and is used in a `module` event for identifying a module for adding, updating or deleting.\nThe `name` attribute is used to minimally render the module in the UI.\n\nAdditional attributes can be added to the module. They show up in the module view if they have a corresponding `ColumnDescriptor`.\n\nTo avoid an unnecessary proliferation of additional attributes with similar semantics but different names, we recommend to re-use attributes from the 'recommended' list below first, and only introduce new attributes if nothing appropriate could be found.",
                    "properties": {
                        "id": {
                            "type": ["integer", "string"],
                            "description": "Unique identifier for the module."
                        },
                        "name": {
                            "type": "string",
                            "description": "A name of the module."
                        },
                        "path": {
                            "type": "string",
                            "description": "Logical full path to the module. The exact definition is implementation defined, but usually this would be a full path to the on-disk file for the module."
                        },
                        "isOptimized": {
                            "type": "boolean",
                            "description": "True if the module is optimized."
                        },
                        "isUserCode": {
                            "type": "boolean",
                            "description": "True if the module is considered 'user code' by a debugger that supports 'Just My Code'."
                        },
                        "version": {
                            "type": "string",
                            "description": "Version of Module."
                        },
                        "symbolStatus": {
                            "type": "string",
                            "description": "User-understandable description of if symbols were found for the module (ex: 'Symbols Loaded', 'Symbols not found', etc.)"
                        },
                        "symbolFilePath": {
                            "type": "string",
                            "description": "Logical full path to the symbol file. The exact definition is implementation defined."
                        },
                        "dateTimeStamp": {
                            "type": "string",
                            "description": "Module created or modified, encoded as a RFC 3339 timestamp."
                        },
                        "addressRange": {
                            "type": "string",
                            "description": "Address range covered by this module."
                        }
                    },
                    "required": [ "id", "name" ]
                },

                "ColumnDescriptor": {
                    "type": "object",
                    "description": "A `ColumnDescriptor` specifies what module attribute to show in a column of the modules view, how to format it,\nand what the column's label should be.\nIt is only used if the underlying UI actually supports this level of customization.",
                    "properties": {
                        "attributeName": {
                            "type": "string",
                            "description": "Name of the attribute rendered in this column."
                        },
                        "label": {
                            "type": "string",
                            "description": "Header UI label of column."
                        },
                        "format": {
                            "type": "string",
                            "description": "Format to use for the rendered values in this column. TBD how the format strings looks like."
                        },
                        "type": {
                            "type": "string",
                            "enum": [ "string", "number", "boolean", "unixTimestampUTC" ],
                            "description": "Datatype of values in this column. Defaults to `string` if not specified."
                        },
                        "width": {
                            "type": "integer",
                            "description": "Width of this column in characters (hint only)."
                        }
                    },
                    "required": [ "attributeName", "label"]
                },

                "ModulesViewDescriptor": {
                    "type": "object",
                    "description": "The ModulesViewDescriptor is the container for all declarative configuration options of a module view.\nFor now it only specifies the columns to be shown in the modules view.",
                    "properties": {
                        "columns": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ColumnDescriptor"
                            }
                        }
                    },
                    "required": [ "columns" ]
                },

                "Thread": {
                    "type": "object",
                    "description": "A Thread",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "Unique identifier for the thread."
                        },
                        "name": {
                            "type": "string",
                            "description": "The name of the thread."
                        }
                    },
                    "required": [ "id", "name" ]
                },

                "Source": {
                    "type": "object",
                    "description": "A `Source` is a descriptor for source code.\nIt is returned from the debug adapter as part of a `StackFrame` and it is used by clients when specifying breakpoints.",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The short name of the source. Every source returned from the debug adapter has a name.\nWhen sending a source to the debug adapter this name is optional."
                        },
                        "path": {
                            "type": "string",
                            "description": "The path of the source to be shown in the UI.\nIt is only used to locate and load the content of the source if no `sourceReference` is specified (or its value is 0)."
                        },
                        "sourceReference": {
                            "type": "integer",
                            "description": "If the value > 0 the contents of the source must be retrieved through the `source` request (even if a path is specified).\nSince a `sourceReference` is only valid for a session, it can not be used to persist a source.\nThe value should be less than or equal to 2147483647 (2^31-1)."
                        },
                        "presentationHint": {
                            "type": "string",
                            "description": "A hint for how to present the source in the UI.\nA value of `deemphasize` can be used to indicate that the source is not available or that it is skipped on stepping.",
                            "enum": [ "normal", "emphasize", "deemphasize" ]
                        },
                        "origin": {
                            "type": "string",
                            "description": "The origin of this source. For example, 'internal module', 'inlined content from source map', etc."
                        },
                        "sources": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/Source"
                            },
                            "description": "A list of sources that are related to this source. These may be the source that generated this source."
                        },
                        "adapterData": {
                            "type": [ "array", "boolean", "integer", "null", "number", "object", "string" ],
                            "description": "Additional data that a debug adapter might want to loop through the client.\nThe client should leave the data intact and persist it across sessions. The client should not interpret the data."
                        },
                        "checksums": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/Checksum"
                            },
                            "description": "The checksums associated with this file."
                        }
                    }
                },

                "StackFrame": {
                    "type": "object",
                    "description": "A Stackframe contains the source location.",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "An identifier for the stack frame. It must be unique across all threads.\nThis id can be used to retrieve the scopes of the frame with the `scopes` request or to restart the execution of a stack frame."
                        },
                        "name": {
                            "type": "string",
                            "description": "The name of the stack frame, typically a method name."
                        },
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "The source of the frame."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The line within the source of the frame. If the source attribute is missing or doesn't exist, `line` is 0 and should be ignored by the client."
                        },
                        "column": {
                            "type": "integer",
                            "description": "Start position of the range covered by the stack frame. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based. If attribute `source` is missing or doesn't exist, `column` is 0 and should be ignored by the client."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of the range covered by the stack frame."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "End position of the range covered by the stack frame. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "canRestart": {
                            "type": "boolean",
                            "description": "Indicates whether this frame can be restarted with the `restart` request. Clients should only use this if the debug adapter supports the `restart` request and the corresponding capability `supportsRestartRequest` is true."
                        },
                        "instructionPointerReference": {
                            "type": "string",
                            "description": "A memory reference for the current instruction pointer in this frame."
                        },
                        "moduleId": {
                            "type": ["integer", "string"],
                            "description": "The module associated with this frame, if any."
                        },
                        "presentationHint": {
                            "type": "string",
                            "enum": [ "normal", "label", "subtle" ],
                            "description": "A hint for how to present this frame in the UI.\nA value of `label` can be used to indicate that the frame is an artificial frame that is used as a visual label or separator. A value of `subtle` can be used to change the appearance of a frame in a 'subtle' way."
                        }
                    },
                    "required": [ "id", "name", "line", "column" ]
                },

                "Scope": {
                    "type": "object",
                    "description": "A `Scope` is a named container for variables. Optionally a scope can map to a source or a range within a source.",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This string is shown in the UI as is and can be translated."
                        },
                        "presentationHint": {
                            "type": "string",
                            "description": "A hint for how to present this scope in the UI. If this attribute is missing, the scope is shown with a generic UI.",
                            "_enum": [ "arguments", "locals", "registers" ],
                            "enumDescriptions": [
                                "Scope contains method arguments.",
                                "Scope contains local variables.",
                                "Scope contains registers. Only a single `registers` scope should be returned from a `scopes` request."
                            ]
                        },
                        "variablesReference": {
                            "type": "integer",
                            "description": "The variables of this scope can be retrieved by passing the value of `variablesReference` to the `variables` request as long as execution remains suspended. See 'Lifetime of Object References' in the Overview section for details."
                        },
                        "namedVariables": {
                            "type": "integer",
                            "description": "The number of named variables in this scope.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks."
                        },
                        "indexedVariables": {
                            "type": "integer",
                            "description": "The number of indexed variables in this scope.\nThe client can use this information to present the variables in a paged UI and fetch them in chunks."
                        },
                        "expensive": {
                            "type": "boolean",
                            "description": "If true, the number of variables in this scope is large or expensive to retrieve."
                        },
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "The source for this scope."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The start line of the range covered by this scope."
                        },
                        "column": {
                            "type": "integer",
                            "description": "Start position of the range covered by the scope. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of the range covered by this scope."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "End position of the range covered by the scope. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        }
                    },
                    "required": [ "name", "variablesReference", "expensive" ]
                },

                "Variable": {
                    "type": "object",
                    "description": "A Variable is a name/value pair.\nThe `type` attribute is shown if space permits or when hovering over the variable's name.\nThe `kind` attribute is used to render additional properties of the variable, e.g. different icons can be used to indicate that a variable is public or private.\nIf the value is structured (has children), a handle is provided to retrieve the children with the `variables` request.\nIf the number of named or indexed children is large, the numbers should be returned via the `namedVariables` and `indexedVariables` attributes.\nThe client can use this information to present the children in a paged UI and fetch them in chunks.",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The variable's name."
                        },
                        "value": {
                            "type": "string",
                            "description": "The variable's value.\nThis can be a multi-line text, e.g. for a function the body of a function.\nFor structured variables (which do not have a simple value), it is recommended to provide a one-line representation of the structured object. This helps to identify the structured object in the collapsed state when its children are not yet visible.\nAn empty string can be used if no value should be shown in the UI."
                        },
                        "type": {
                            "type": "string",
                            "description": "The type of the variable's value. Typically shown in the UI when hovering over the value.\nThis attribute should only be returned by a debug adapter if the corresponding capability `supportsVariableType` is true."
                        },
                        "presentationHint": {
                            "$ref": "#/definitions/VariablePresentationHint",
                            "description": "Properties of a variable that can be used to determine how to render the variable in the UI."
                        },
                        "evaluateName": {
                            "type": "string",
                            "description": "The evaluatable name of this variable which can be passed to the `evaluate` request to fetch the variable's value."
                        },
                        "variablesReference": {
                            "type": "integer",
                            "description": "If `variablesReference` is > 0, the variable is structured and its children can be retrieved by passing `variablesReference` to the `variables` request as long as execution remains suspended. See 'Lifetime of Object References' in the Overview section for details."
                        },
                        "namedVariables": {
                            "type": "integer",
                            "description": "The number of named child variables.\nThe client can use this information to present the children in a paged UI and fetch them in chunks."
                        },
                        "indexedVariables": {
                            "type": "integer",
                            "description": "The number of indexed child variables.\nThe client can use this information to present the children in a paged UI and fetch them in chunks."
                        },
                        "memoryReference": {
                            "type": "string",
                            "description": "The memory reference for the variable if the variable represents executable code, such as a function pointer.\nThis attribute is only required if the corresponding capability `supportsMemoryReferences` is true."
                        }
                    },
                    "required": [ "name", "value", "variablesReference" ]
                },

                "VariablePresentationHint": {
                    "type": "object",
                    "description": "Properties of a variable that can be used to determine how to render the variable in the UI.",
                    "properties": {
                        "kind": {
                            "description": "The kind of variable. Before introducing additional values, try to use the listed values.",
                            "type": "string",
                            "_enum": [ "property", "method", "class", "data", "event", "baseClass", "innerClass", "interface", "mostDerivedClass", "virtual", "dataBreakpoint" ],
                            "enumDescriptions": [
                                "Indicates that the object is a property.",
                                "Indicates that the object is a method.",
                                "Indicates that the object is a class.",
                                "Indicates that the object is data.",
                                "Indicates that the object is an event.",
                                "Indicates that the object is a base class.",
                                "Indicates that the object is an inner class.",
                                "Indicates that the object is an interface.",
                                "Indicates that the object is the most derived class.",
                                "Indicates that the object is virtual, that means it is a synthetic object introduced by the adapter for rendering purposes, e.g. an index range for large arrays.",
                                "Deprecated: Indicates that a data breakpoint is registered for the object. The `hasDataBreakpoint` attribute should generally be used instead."
                            ]
                        },
                        "attributes": {
                            "description": "Set of attributes represented as an array of strings. Before introducing additional values, try to use the listed values.",
                            "type": "array",
                            "items": {
                                "type": "string",
                                "_enum": [ "static", "constant", "readOnly", "rawString", "hasObjectId", "canHaveObjectId", "hasSideEffects", "hasDataBreakpoint" ],
                                "enumDescriptions": [
                                    "Indicates that the object is static.",
                                    "Indicates that the object is a constant.",
                                    "Indicates that the object is read only.",
                                    "Indicates that the object is a raw string.",
                                    "Indicates that the object can have an Object ID created for it.",
                                    "Indicates that the object has an Object ID associated with it.",
                                    "Indicates that the evaluation had side effects.",
                                    "Indicates that the object has its value tracked by a data breakpoint."
                                ]
                            }
                        },
                        "visibility": {
                            "description": "Visibility of variable. Before introducing additional values, try to use the listed values.",
                            "type": "string",
                            "_enum": [ "public", "private", "protected", "internal", "final" ]
                        },
                        "lazy": {
                            "description": "If true, clients can present the variable with a UI that supports a specific gesture to trigger its evaluation.\nThis mechanism can be used for properties that require executing code when retrieving their value and where the code execution can be expensive and/or produce side-effects. A typical example are properties based on a getter function.\nPlease note that in addition to the `lazy` flag, the variable's `variablesReference` is expected to refer to a variable that will provide the value through another `variable` request.",
                            "type": "boolean"
                        }
                    }
                },

                "BreakpointLocation": {
                    "type": "object",
                    "description": "Properties of a breakpoint location returned from the `breakpointLocations` request.",
                    "properties": {
                        "line": {
                            "type": "integer",
                            "description": "Start line of breakpoint location."
                        },
                        "column": {
                            "type": "integer",
                            "description": "The start position of a breakpoint location. Position is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of breakpoint location if the location covers a range."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "The end position of a breakpoint location (if the location covers a range). Position is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        }
                    },
                    "required": [ "line" ]
                },

                "SourceBreakpoint": {
                    "type": "object",
                    "description": "Properties of a breakpoint or logpoint passed to the `setBreakpoints` request.",
                    "properties": {
                        "line": {
                            "type": "integer",
                            "description": "The source line of the breakpoint or logpoint."
                        },
                        "column": {
                            "type": "integer",
                            "description": "Start position within source line of the breakpoint or logpoint. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "condition": {
                            "type": "string",
                            "description": "The expression for conditional breakpoints.\nIt is only honored by a debug adapter if the corresponding capability `supportsConditionalBreakpoints` is true."
                        },
                        "hitCondition": {
                            "type": "string",
                            "description": "The expression that controls how many hits of the breakpoint are ignored.\nThe debug adapter is expected to interpret the expression as needed.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsHitConditionalBreakpoints` is true."
                        },
                        "logMessage": {
                            "type": "string",
                            "description": "If this attribute exists and is non-empty, the debug adapter must not 'break' (stop)\nbut log the message instead. Expressions within `{}` are interpolated.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsLogPoints` is true."
                        }
                    },
                    "required": [ "line" ]
                },

                "FunctionBreakpoint": {
                    "type": "object",
                    "description": "Properties of a breakpoint passed to the `setFunctionBreakpoints` request.",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The name of the function."
                        },
                        "condition": {
                            "type": "string",
                            "description": "An expression for conditional breakpoints.\nIt is only honored by a debug adapter if the corresponding capability `supportsConditionalBreakpoints` is true."
                        },
                        "hitCondition": {
                            "type": "string",
                            "description": "An expression that controls how many hits of the breakpoint are ignored.\nThe debug adapter is expected to interpret the expression as needed.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsHitConditionalBreakpoints` is true."
                        }
                    },
                    "required": [ "name" ]
                },

                "DataBreakpointAccessType": {
                    "type": "string",
                    "description": "This enumeration defines all possible access types for data breakpoints.",
                    "enum": [ "read", "write", "readWrite" ]
                },

                "DataBreakpoint": {
                    "type": "object",
                    "description": "Properties of a data breakpoint passed to the `setDataBreakpoints` request.",
                    "properties": {
                        "dataId": {
                            "type": "string",
                            "description": "An id representing the data. This id is returned from the `dataBreakpointInfo` request."
                        },
                        "accessType": {
                            "$ref": "#/definitions/DataBreakpointAccessType",
                            "description": "The access type of the data."
                        },
                        "condition": {
                            "type": "string",
                            "description": "An expression for conditional breakpoints."
                        },
                        "hitCondition": {
                            "type": "string",
                            "description": "An expression that controls how many hits of the breakpoint are ignored.\nThe debug adapter is expected to interpret the expression as needed."
                        }
                    },
                    "required": [ "dataId" ]
                },

                "InstructionBreakpoint": {
                    "type": "object",
                    "description": "Properties of a breakpoint passed to the `setInstructionBreakpoints` request",
                    "properties": {
                        "instructionReference": {
                            "type": "string",
                            "description": "The instruction reference of the breakpoint.\nThis should be a memory or instruction pointer reference from an `EvaluateResponse`, `Variable`, `StackFrame`, `GotoTarget`, or `Breakpoint`."
                        },
                        "offset": {
                            "type": "integer",
                            "description": "The offset from the instruction reference.\nThis can be negative."
                        },
                        "condition": {
                            "type": "string",
                            "description": "An expression for conditional breakpoints.\nIt is only honored by a debug adapter if the corresponding capability `supportsConditionalBreakpoints` is true."
                        },
                        "hitCondition": {
                            "type": "string",
                            "description": "An expression that controls how many hits of the breakpoint are ignored.\nThe debug adapter is expected to interpret the expression as needed.\nThe attribute is only honored by a debug adapter if the corresponding capability `supportsHitConditionalBreakpoints` is true."
                        }
                    },
                    "required": [ "instructionReference" ]
                },

                "Breakpoint": {
                    "type": "object",
                    "description": "Information about a breakpoint created in `setBreakpoints`, `setFunctionBreakpoints`, `setInstructionBreakpoints`, or `setDataBreakpoints` requests.",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "The identifier for the breakpoint. It is needed if breakpoint events are used to update or remove breakpoints."
                        },
                        "verified": {
                            "type": "boolean",
                            "description": "If true, the breakpoint could be set (but not necessarily at the desired location)."
                        },
                        "message": {
                            "type": "string",
                            "description": "A message about the state of the breakpoint.\nThis is shown to the user and can be used to explain why a breakpoint could not be verified."
                        },
                        "source": {
                            "$ref": "#/definitions/Source",
                            "description": "The source where the breakpoint is located."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The start line of the actual range covered by the breakpoint."
                        },
                        "column": {
                            "type": "integer",
                            "description": "Start position of the source range covered by the breakpoint. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of the actual range covered by the breakpoint."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "End position of the source range covered by the breakpoint. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based.\nIf no end line is given, then the end column is assumed to be in the start line."
                        },
                        "instructionReference": {
                            "type": "string",
                            "description": "A memory reference to where the breakpoint is set."
                        },
                        "offset": {
                            "type": "integer",
                            "description": "The offset from the instruction reference.\nThis can be negative."
                        }
                    },
                    "required": [ "verified" ]
                },

                "SteppingGranularity": {
                    "type": "string",
                    "description": "The granularity of one 'step' in the stepping requests `next`, `stepIn`, `stepOut`, and `stepBack`.",
                    "enum": [ "statement", "line", "instruction" ],
                    "enumDescriptions": [
                        "The step should allow the program to run until the current statement has finished executing.\nThe meaning of a statement is determined by the adapter and it may be considered equivalent to a line.\nFor example 'for(int i = 0; i < 10; i++)' could be considered to have 3 statements 'int i = 0', 'i < 10', and 'i++'.",
                        "The step should allow the program to run until the current source line has executed.",
                        "The step should allow one instruction to execute (e.g. one x86 instruction)."
                    ]
                },

                "StepInTarget": {
                    "type": "object",
                    "description": "A `StepInTarget` can be used in the `stepIn` request and determines into which single target the `stepIn` request should step.",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "Unique identifier for a step-in target."
                        },
                        "label": {
                            "type": "string",
                            "description": "The name of the step-in target (shown in the UI)."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The line of the step-in target."
                        },
                        "column": {
                            "type": "integer",
                            "description": "Start position of the range covered by the step in target. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of the range covered by the step-in target."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "End position of the range covered by the step in target. It is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based."
                        }
                    },
                    "required": [ "id", "label" ]
                },

                "GotoTarget": {
                    "type": "object",
                    "description": "A `GotoTarget` describes a code location that can be used as a target in the `goto` request.\nThe possible goto targets can be determined via the `gotoTargets` request.",
                    "properties": {
                        "id": {
                            "type": "integer",
                            "description": "Unique identifier for a goto target. This is used in the `goto` request."
                        },
                        "label": {
                            "type": "string",
                            "description": "The name of the goto target (shown in the UI)."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The line of the goto target."
                        },
                        "column": {
                            "type": "integer",
                            "description": "The column of the goto target."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of the range covered by the goto target."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "The end column of the range covered by the goto target."
                        },
                        "instructionPointerReference": {
                            "type": "string",
                            "description": "A memory reference for the instruction pointer value represented by this target."
                        }
                    },
                    "required": [ "id", "label", "line" ]
                },

                "CompletionItem": {
                    "type": "object",
                    "description": "`CompletionItems` are the suggestions returned from the `completions` request.",
                    "properties": {
                        "label": {
                            "type": "string",
                            "description": "The label of this completion item. By default this is also the text that is inserted when selecting this completion."
                        },
                        "text": {
                            "type": "string",
                            "description": "If text is returned and not an empty string, then it is inserted instead of the label."
                        },
                        "sortText": {
                            "type": "string",
                            "description": "A string that should be used when comparing this item with other items. If not returned or an empty string, the `label` is used instead."
                        },
                        "detail": {
                            "type": "string",
                            "description": "A human-readable string with additional information about this item, like type or symbol information."
                        },
                        "type": {
                            "$ref": "#/definitions/CompletionItemType",
                            "description": "The item's type. Typically the client uses this information to render the item in the UI with an icon."
                        },
                        "start": {
                            "type": "integer",
                            "description": "Start position (within the `text` attribute of the `completions` request) where the completion text is added. The position is measured in UTF-16 code units and the client capability `columnsStartAt1` determines whether it is 0- or 1-based. If the start position is omitted the text is added at the location specified by the `column` attribute of the `completions` request."
                        },
                        "length": {
                            "type": "integer",
                            "description": "Length determines how many characters are overwritten by the completion text and it is measured in UTF-16 code units. If missing the value 0 is assumed which results in the completion text being inserted."
                        },
                        "selectionStart": {
                            "type": "integer",
                            "description": "Determines the start of the new selection after the text has been inserted (or replaced). `selectionStart` is measured in UTF-16 code units and must be in the range 0 and length of the completion text. If omitted the selection starts at the end of the completion text."
                        },
                        "selectionLength": {
                            "type": "integer",
                            "description": "Determines the length of the new selection after the text has been inserted (or replaced) and it is measured in UTF-16 code units. The selection can not extend beyond the bounds of the completion text. If omitted the length is assumed to be 0."
                        }
                    },
                    "required": [ "label" ]
                },

                "CompletionItemType": {
                    "type": "string",
                    "description": "Some predefined types for the CompletionItem. Please note that not all clients have specific icons for all of them.",
                    "enum": [ "method", "function", "constructor", "field", "variable", "class", "interface", "module", "property", "unit", "value", "enum", "keyword", "snippet", "text", "color", "file", "reference", "customcolor" ]
                },

                "ChecksumAlgorithm": {
                    "type": "string",
                    "description": "Names of checksum algorithms that may be supported by a debug adapter.",
                    "enum": [ "MD5", "SHA1", "SHA256", "timestamp" ]
                },

                "Checksum": {
                    "type": "object",
                    "description": "The checksum of an item calculated by the specified algorithm.",
                    "properties": {
                        "algorithm": {
                            "$ref": "#/definitions/ChecksumAlgorithm",
                            "description": "The algorithm used to calculate this checksum."
                        },
                        "checksum": {
                            "type": "string",
                            "description": "Value of the checksum, encoded as a hexadecimal value."
                        }
                    },
                    "required": [ "algorithm", "checksum" ]
                },

                "ValueFormat": {
                    "type": "object",
                    "description": "Provides formatting information for a value.",
                    "properties": {
                        "hex": {
                            "type": "boolean",
                            "description": "Display the value in hex."
                        }
                    }
                },

                "StackFrameFormat": {
                    "allOf": [ { "$ref": "#/definitions/ValueFormat" }, {
                        "type": "object",
                        "description": "Provides formatting information for a stack frame.",
                        "properties": {
                            "parameters": {
                                "type": "boolean",
                                "description": "Displays parameters for the stack frame."
                            },
                            "parameterTypes": {
                                "type": "boolean",
                                "description": "Displays the types of parameters for the stack frame."
                            },
                            "parameterNames": {
                                "type": "boolean",
                                "description": "Displays the names of parameters for the stack frame."
                            },
                            "parameterValues": {
                                "type": "boolean",
                                "description": "Displays the values of parameters for the stack frame."
                            },
                            "line": {
                                "type": "boolean",
                                "description": "Displays the line number of the stack frame."
                            },
                            "module": {
                                "type": "boolean",
                                "description": "Displays the module of the stack frame."
                            },
                            "includeAll": {
                                "type": "boolean",
                                "description": "Includes all stack frames, including those the debug adapter might otherwise hide."
                            }
                        }
                    }]
                },

                "ExceptionFilterOptions": {
                    "type": "object",
                    "description": "An `ExceptionFilterOptions` is used to specify an exception filter together with a condition for the `setExceptionBreakpoints` request.",
                    "properties": {
                        "filterId": {
                            "type": "string",
                            "description": "ID of an exception filter returned by the `exceptionBreakpointFilters` capability."
                        },
                        "condition": {
                            "type": "string",
                            "description": "An expression for conditional exceptions.\nThe exception breaks into the debugger if the result of the condition is true."
                        }
                    },
                    "required": [ "filterId" ]
                },

                "ExceptionOptions": {
                    "type": "object",
                    "description": "An `ExceptionOptions` assigns configuration options to a set of exceptions.",
                    "properties": {
                        "path": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ExceptionPathSegment"
                            },
                            "description": "A path that selects a single or multiple exceptions in a tree. If `path` is missing, the whole tree is selected.\nBy convention the first segment of the path is a category that is used to group exceptions in the UI."
                        },
                        "breakMode": {
                            "$ref": "#/definitions/ExceptionBreakMode",
                            "description": "Condition when a thrown exception should result in a break."
                        }
                    },
                    "required": [ "breakMode" ]
                },

                "ExceptionBreakMode": {
                    "type": "string",
                    "description": "This enumeration defines all possible conditions when a thrown exception should result in a break.\nnever: never breaks,\nalways: always breaks,\nunhandled: breaks when exception unhandled,\nuserUnhandled: breaks if the exception is not handled by user code.",
                    "enum": [ "never", "always", "unhandled", "userUnhandled" ]
                },

                "ExceptionPathSegment": {
                    "type": "object",
                    "description": "An `ExceptionPathSegment` represents a segment in a path that is used to match leafs or nodes in a tree of exceptions.\nIf a segment consists of more than one name, it matches the names provided if `negate` is false or missing, or it matches anything except the names provided if `negate` is true.",
                    "properties": {
                        "negate": {
                            "type": "boolean",
                            "description": "If false or missing this segment matches the names provided, otherwise it matches anything except the names provided."
                        },
                        "names": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Depending on the value of `negate` the names that should match or not match."
                        }
                    },
                    "required": [ "names" ]
                },

                "ExceptionDetails": {
                    "type": "object",
                    "description": "Detailed information about an exception that has occurred.",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Message contained in the exception."
                        },
                        "typeName": {
                            "type": "string",
                            "description": "Short type name of the exception object."
                        },
                        "fullTypeName": {
                            "type": "string",
                            "description": "Fully-qualified type name of the exception object."
                        },
                        "evaluateName": {
                            "type": "string",
                            "description": "An expression that can be evaluated in the current scope to obtain the exception object."
                        },
                        "stackTrace": {
                            "type": "string",
                            "description": "Stack trace at the time the exception was thrown."
                        },
                        "innerException": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/ExceptionDetails"
                            },
                            "description": "Details of the exception contained by this exception, if any."
                        }
                    }
                },

                "DisassembledInstruction": {
                    "type": "object",
                    "description": "Represents a single disassembled instruction.",
                    "properties": {
                        "address": {
                            "type": "string",
                            "description": "The address of the instruction. Treated as a hex value if prefixed with `0x`, or as a decimal value otherwise."
                        },
                        "instructionBytes": {
                            "type": "string",
                            "description": "Raw bytes representing the instruction and its operands, in an implementation-defined format."
                        },
                        "instruction": {
                            "type": "string",
                            "description": "Text representing the instruction and its operands, in an implementation-defined format."
                        },
                        "symbol": {
                            "type": "string",
                            "description": "Name of the symbol that corresponds with the location of this instruction, if any."
                        },
                        "location": {
                            "$ref": "#/definitions/Source",
                            "description": "Source location that corresponds to this instruction, if any.\nShould always be set (if available) on the first instruction returned,\nbut can be omitted afterwards if this instruction maps to the same source file as the previous instruction."
                        },
                        "line": {
                            "type": "integer",
                            "description": "The line within the source location that corresponds to this instruction, if any."
                        },
                        "column": {
                            "type": "integer",
                            "description": "The column within the line that corresponds to this instruction, if any."
                        },
                        "endLine": {
                            "type": "integer",
                            "description": "The end line of the range that corresponds to this instruction, if any."
                        },
                        "endColumn": {
                            "type": "integer",
                            "description": "The end column of the range that corresponds to this instruction, if any."
                        }
                    },
                    "required": [ "address", "instruction" ]
                },

                "InvalidatedAreas": {
                    "type": "string",
                    "description": "Logical areas that can be invalidated by the `invalidated` event.",
                    "_enum": [ "all", "stacks", "threads", "variables" ],
                    "enumDescriptions": [
                        "All previously fetched data has become invalid and needs to be refetched.",
                        "Previously fetched stack related data has become invalid and needs to be refetched.",
                        "Previously fetched thread related data has become invalid and needs to be refetched.",
                        "Previously fetched variable data has become invalid and needs to be refetched."
                    ]
                }

            }
        }
    };

    let value: JSONValue = syn::parse2(value)?;

    let value: JSONSchema = value.try_into()?;

    //println!("{_input:#?}");

    Ok(codegen(value))
}
