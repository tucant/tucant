import * as vscode from 'vscode';

export function activateDebugger(context: vscode.ExtensionContext) {
    context.subscriptions.push(vscode.debug.registerDebugConfigurationProvider('tucant', {
		provideDebugConfigurations(folder, token) {
            return [
                {
                    name: "tucant Launch",
                    request: "launch",
                    type: "tucant",
                }
            ];
        },
	}, vscode.DebugConfigurationProviderTriggerKind.Dynamic));

    context.subscriptions.push(vscode.debug.registerDebugAdapterDescriptorFactory('tucant', {
        createDebugAdapterDescriptor(session, executable) {
            return new vscode.DebugAdapterServer(6009);
        },
    }));	
}