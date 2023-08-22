import { ExtensionContext, commands, window } from "vscode";
import { activate as lspActivate, deactivate as lspDeactivate } from './language-server-protocol'
import { activate as dapActivate } from "./debug-adapter-protocol";

export function activate(context: ExtensionContext) {
	console.log('activating')

	context.subscriptions.push(commands.registerCommand('tucant-vscode.helloWorld', () => {
		window.showInformationMessage('Hello World from tucant-vscode!');
	}));

	lspActivate(context);
	dapActivate(context);

	console.log('activated');
}

export function deactivate() {
	lspDeactivate()

	console.log('deactivated')
}
