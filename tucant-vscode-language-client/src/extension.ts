"use strict";

import * as net from "net";
import * as vscode from 'vscode';
import {
  ExtensionContext,
  commands,
  window,
  ProgressLocation,
  languages,
  DebugConfigurationProvider,
  WorkspaceFolder,
  DebugConfiguration,
  CancellationToken,
  ProviderResult,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  StreamInfo,
} from "vscode-languageclient/node";
import { Trace } from "vscode-jsonrpc";

/** @type {LanguageClient} */
let client;

// https://github.com/microsoft/vscode-mock-debug

class MockConfigurationProvider implements DebugConfigurationProvider {

	/**
	 * Massage a debug configuration just before a debug session is being launched,
	 * e.g. add all missing attributes to the debug configuration.
	 */
	resolveDebugConfiguration(folder: WorkspaceFolder | undefined, config: DebugConfiguration, token?: CancellationToken): ProviderResult<DebugConfiguration> {

		// if launch.json is missing or empty
		if (!((config.type || config.request ) || config.name)) {
			const editor = vscode.window.activeTextEditor;
			if (editor && editor.document.languageId === 'markdown') {
				config.type = 'mock';
				config.name = 'Launch';
				config.request = 'launch';
				config.program = '${file}';
				config.stopOnEntry = true;
			}
		}

		if (!config.program) {
			return vscode.window.showInformationMessage("Cannot find a program to debug").then(_ => {
				return undefined;	// abort launch
			});
		}

		return config;
	}
}

export function activate(/** @type {ExtensionContext} */ context) {
  const provider = new MockConfigurationProvider();
	context.subscriptions.push(vscode.debug.registerDebugConfigurationProvider('mock', provider));




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
