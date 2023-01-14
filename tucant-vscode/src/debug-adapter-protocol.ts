import { debug, DebugAdapterServer, DebugConfigurationProviderTriggerKind, ExtensionContext } from 'vscode';

export function activate(context: ExtensionContext) {
    context.subscriptions.push(debug.registerDebugConfigurationProvider('tucant', {
		provideDebugConfigurations(folder, token) {
            return [
                {
                    name: "tucant Launch",
                    request: "launch",
                    type: "tucant",
                }
            ];
        },
	}, DebugConfigurationProviderTriggerKind.Dynamic));

    context.subscriptions.push(debug.registerDebugAdapterDescriptorFactory('tucant', {
        createDebugAdapterDescriptor(session, executable) {
            return new DebugAdapterServer(6009);
        },
    }));	
}