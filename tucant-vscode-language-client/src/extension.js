import * as path from "path";
import { ExtensionContext } from "vscode";

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

/** @type {LanguageClient} */
let client;

export function activate(/** @type {ExtensionContext} */ context) {
  const serverModule = context.asAbsolutePath(path.join("out", "test.sh"));
  const debugOptions = { execArgv: [] };

  /** @type {ServerOptions} */
  const serverOptions = {
    run: { module: serverModule, transport: TransportKind.stdio },
    debug: {
      module: serverModule,
      transport: TransportKind.ipc,
      options: debugOptions,
    },
  };

  /** @type {LanguageClientOptions} */
  const clientOptions = {
    documentSelector: [{ scheme: "file", language: "tucant" }],
    synchronize: {
      //fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
    },
  };

  client = new LanguageClient(
    "tucantLanguageServer",
    "TUCaN't Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
