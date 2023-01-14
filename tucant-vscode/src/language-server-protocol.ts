import { createConnection } from 'net';
import { commands, ExtensionContext, ProgressLocation, window } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, StreamInfo } from 'vscode-languageclient/node';

// https://github.com/microsoft/vscode-extension-samples/blob/main/lsp-sample/client/src/extension.ts

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext) {
    const serverOptions: ServerOptions = () => {
        let socket = createConnection(6008);
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
    
      client.start();
    
      context.subscriptions.push(
        commands.registerCommand("tucant-vscode.restart-language-server", () => {
          window.withProgress(
            {
              title: "TUCaN't: Restarting language server...",
              location: ProgressLocation.Notification,
              cancellable: false,
            },
            async (progress, token) => {
              try {
                await client?.stop();
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

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}