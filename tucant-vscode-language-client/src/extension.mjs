"use strict";

import * as net from "net";
import { ExtensionContext, commands, window, ProgressLocation } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  StreamInfo,
} from "vscode-languageclient/node";
import { Trace } from "vscode-jsonrpc";

/** @type {LanguageClient} */
let client;

export function activate(/** @type {ExtensionContext} */ context) {
  /** @type {ServerOptions} */
  const serverOptions = () => {
    // Connect to language server via socket
    let socket = net.createConnection(6008);
    /** @type {StreamInfo} */
    let result = {
      writer: socket,
      reader: socket,
    };
    return Promise.resolve(result);
  };

  /** @type {LanguageClientOptions} */
  const clientOptions = {
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
  });
}

export function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
