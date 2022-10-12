import { ExtensionContext, commands, window, ProgressLocation } from "vscode";

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

/** @type {LanguageClient} */
let client;

export function activate(/** @type {ExtensionContext} */ context) {
  const serverModule = context.asAbsolutePath(
    "./tucant-language-server/target/debug/tucant-language-server"
  );

  /** @type {ServerOptions} */
  const serverOptions = {
    run: { command: serverModule, transport: TransportKind.pipe },
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

  client.start();

  commands.registerCommand("tucant.restart-language-server", () => {
    window.withProgress(
      {
        title: "TUCaN't: Restarting language server...",
        location: ProgressLocation.Notification,
        cancellable: false,
      },
      async (progress, token) => {
        await client.restart();
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
