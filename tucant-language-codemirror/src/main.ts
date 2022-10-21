import { EditorState } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
import { languageServer, LanguageServerClient } from 'codemirror-languageserver';

const serverUri = "ws://localhost:6009";
const filename = "test";

//const transport = new WebSocketTransport(serverUri)

var ls = languageServer({
	// WebSocket server uri and other client options.
	serverUri,
	rootUri: 'file:///',
  workspaceFolders: null,
	// Alternatively, to share the same client across multiple instances of this plugin.
  /*
	client: new LanguageServerClient({
		serverUri,
		rootUri: 'file:///'
	}),*/

	documentUri: `file:///${filename}`,
	languageId: 'cpp' // As defined at https://microsoft.github.io/language-server-protocol/specification#textDocumentItem.
});

// https://codemirror.net/docs/guide/

// cargo watch -x 'run -- --websocket 6009'
var view = new EditorView({
	state: EditorState.create({
    doc: `(hi "hello world")`,
		extensions: [
			// ...
			ls,
			// ...
		]
	}),
  parent: document.body
});