import * as vscode from 'vscode';

import { LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "oxide-analyzer" is now active!');

	const serverOptions: ServerOptions = {
		command: "oxide",
		args: ["analyzer"],
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "oxide" }],
	};

	client = new LanguageClient("oxide-analyzer", serverOptions, clientOptions);
	client.start();
}

export function deactivate() {
	if (!client) { return undefined; }
	return client.stop();
}
