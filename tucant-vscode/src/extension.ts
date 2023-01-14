import { ExtensionContext, commands, window } from "vscode";

export function activate(context: ExtensionContext) {
	console.log('activating')

	context.subscriptions.push(commands.registerCommand('tucant-vscode.helloWorld', () => {
		window.showInformationMessage('Hello World from tucant-vscode!');
	}));

	console.log('activated');
}

export function deactivate() {
	console.log('deactivated')
}
