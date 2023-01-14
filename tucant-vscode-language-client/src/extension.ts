"use strict";

import * as net from "net";
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  StreamInfo,
} from "vscode-languageclient/node";

let client: LanguageClient;

// https://github.com/microsoft/vscode-mock-debug


export function activate(context: ExtensionContext) {

  const serverOptions: ServerOptions = () => {
    // Connect to language server via socket
    let socket = net.createConnection(6008);
    let result: StreamInfo = {
      writer: socket,
      reader: socket,
    };
    return Promise.resolve(result);
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "tucant" }],
  };

  client = new LanguageClient(
    "tucantLanguageServer",
    "TUCaN't Language Server",
    serverOptions,
    clientOptions
  );

  //await client.setTrace(Trace.Verbose);
  client.start();

  context.subscriptions.push(
    commands.registerCommand("tucant.restart-language-server", () => {
      window.withProgress(
        {
          title: "TUCaN't: Restarting language server...",
          location: ProgressLocation.Notification,
          cancellable: false,
        },
        async (progress, token) => {
          try {
            await client.stop();
          } finally {
            client = new LanguageClient(
              "tucantLanguageServer",
              "TUCaN't Language Server",
              serverOptions,
              clientOptions
            );
            client.start();
          }
        }
      );
    })
  );
}

export function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
