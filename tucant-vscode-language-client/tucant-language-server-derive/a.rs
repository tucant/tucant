#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tucant_language_server_derive::magic;
struct ImplementationParams {
    _0: TextDocumentPositionParams,
}
struct Location {
    uri: String,
    range: Range,
}
struct ImplementationRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: ImplementationOptions,
}
struct TypeDefinitionParams {
    _0: TextDocumentPositionParams,
}
struct TypeDefinitionRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: TypeDefinitionOptions,
}
struct WorkspaceFolder {
    uri: String,
    name: String,
}
struct DidChangeWorkspaceFoldersParams {
    event: WorkspaceFoldersChangeEvent,
}
struct ConfigurationParams {
    items: Vec<ConfigurationItem>,
}
struct PartialResultParams {
    partialResultToken: ProgressToken,
}
struct DocumentColorParams {
    textDocument: TextDocumentIdentifier,
}
struct ColorInformation {
    range: Range,
    color: Color,
}
struct DocumentColorRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentColorOptions,
}
struct ColorPresentationParams {
    textDocument: TextDocumentIdentifier,
    color: Color,
    range: Range,
}
struct ColorPresentation {
    label: String,
    textEdit: TextEdit,
    additionalTextEdits: Vec<TextEdit>,
}
struct WorkDoneProgressOptions {
    workDoneProgress: bool,
}
struct TextDocumentRegistrationOptions {
    documentSelector: _a056750a60f1952caf9266e34e49b31c360e6a1f1807ab19a4cb9e3c,
}
struct FoldingRangeParams {
    textDocument: TextDocumentIdentifier,
}
struct FoldingRange {
    startLine: u64,
    startCharacter: u64,
    endLine: u64,
    endCharacter: u64,
    kind: FoldingRangeKind,
    collapsedText: String,
}
struct FoldingRangeRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: FoldingRangeOptions,
}
struct DeclarationParams {
    _0: TextDocumentPositionParams,
}
struct DeclarationRegistrationOptions {
    _0: DeclarationOptions,
    _1: TextDocumentRegistrationOptions,
}
struct SelectionRangeParams {
    textDocument: TextDocumentIdentifier,
    positions: Vec<Position>,
}
struct SelectionRange {
    range: Range,
    parent: SelectionRange,
}
struct SelectionRangeRegistrationOptions {
    _0: SelectionRangeOptions,
    _1: TextDocumentRegistrationOptions,
}
struct WorkDoneProgressCreateParams {
    token: ProgressToken,
}
struct WorkDoneProgressCancelParams {
    token: ProgressToken,
}
struct CallHierarchyPrepareParams {
    _0: TextDocumentPositionParams,
}
struct CallHierarchyItem {
    name: String,
    kind: SymbolKind,
    tags: Vec<SymbolTag>,
    detail: String,
    uri: String,
    range: Range,
    selectionRange: Range,
    data: LSPAny,
}
struct CallHierarchyRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: CallHierarchyOptions,
}
struct CallHierarchyIncomingCallsParams {
    item: CallHierarchyItem,
}
struct CallHierarchyIncomingCall {
    from: CallHierarchyItem,
    fromRanges: Vec<Range>,
}
struct CallHierarchyOutgoingCallsParams {
    item: CallHierarchyItem,
}
struct CallHierarchyOutgoingCall {
    to: CallHierarchyItem,
    fromRanges: Vec<Range>,
}
struct SemanticTokensParams {
    textDocument: TextDocumentIdentifier,
}
struct SemanticTokens {
    resultId: String,
    data: Vec<u64>,
}
struct SemanticTokensPartialResult {
    data: Vec<u64>,
}
struct SemanticTokensRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: SemanticTokensOptions,
}
struct SemanticTokensDeltaParams {
    textDocument: TextDocumentIdentifier,
    previousResultId: String,
}
struct SemanticTokensDelta {
    resultId: String,
    edits: Vec<SemanticTokensEdit>,
}
struct SemanticTokensDeltaPartialResult {
    edits: Vec<SemanticTokensEdit>,
}
struct SemanticTokensRangeParams {
    textDocument: TextDocumentIdentifier,
    range: Range,
}
struct ShowDocumentParams {
    uri: String,
    external: bool,
    takeFocus: bool,
    selection: Range,
}
struct ShowDocumentResult {
    success: bool,
}
struct LinkedEditingRangeParams {
    _0: TextDocumentPositionParams,
}
struct LinkedEditingRanges {
    ranges: Vec<Range>,
    wordPattern: String,
}
struct LinkedEditingRangeRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: LinkedEditingRangeOptions,
}
struct CreateFilesParams {
    files: Vec<FileCreate>,
}
struct WorkspaceEdit {
    changes: ::std::collections::HashMap<String, Vec<TextEdit>>,
    documentChanges: Vec<_7191064c973478b80fc8c60220997a16961f2f6a6fd3733edfb8722a>,
    changeAnnotations: ::std::collections::HashMap<
        ChangeAnnotationIdentifier,
        ChangeAnnotation,
    >,
}
struct FileOperationRegistrationOptions {
    filters: Vec<FileOperationFilter>,
}
struct RenameFilesParams {
    files: Vec<FileRename>,
}
struct DeleteFilesParams {
    files: Vec<FileDelete>,
}
struct MonikerParams {
    _0: TextDocumentPositionParams,
}
struct Moniker {
    scheme: String,
    identifier: String,
    unique: UniquenessLevel,
    kind: MonikerKind,
}
struct MonikerRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: MonikerOptions,
}
struct TypeHierarchyPrepareParams {
    _0: TextDocumentPositionParams,
}
struct TypeHierarchyItem {
    name: String,
    kind: SymbolKind,
    tags: Vec<SymbolTag>,
    detail: String,
    uri: String,
    range: Range,
    selectionRange: Range,
    data: LSPAny,
}
struct TypeHierarchyRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: TypeHierarchyOptions,
}
struct TypeHierarchySupertypesParams {
    item: TypeHierarchyItem,
}
struct TypeHierarchySubtypesParams {
    item: TypeHierarchyItem,
}
struct InlineValueParams {
    textDocument: TextDocumentIdentifier,
    range: Range,
    context: InlineValueContext,
}
struct InlineValueRegistrationOptions {
    _0: InlineValueOptions,
    _1: TextDocumentRegistrationOptions,
}
struct InlayHintParams {
    textDocument: TextDocumentIdentifier,
    range: Range,
}
struct InlayHint {
    position: Position,
    label: _ff638392c2084be979bca73c67a9ff5ce48b4d831488b7a96d522fab,
    kind: InlayHintKind,
    textEdits: Vec<TextEdit>,
    tooltip: _884cc41f89ac76d14ad276f05f61d17b0a84b1738faa7d62b8af4d08,
    paddingLeft: bool,
    paddingRight: bool,
    data: LSPAny,
}
struct InlayHintRegistrationOptions {
    _0: InlayHintOptions,
    _1: TextDocumentRegistrationOptions,
}
struct DocumentDiagnosticParams {
    textDocument: TextDocumentIdentifier,
    identifier: String,
    previousResultId: String,
}
struct DocumentDiagnosticReportPartialResult {
    relatedDocuments: ::std::collections::HashMap<
        String,
        _3fc208fa8531e16f51fe2d6556f3d4ccf38ceff71ec52d486dcde26d,
    >,
}
struct DiagnosticServerCancellationData {
    retriggerRequest: bool,
}
struct DiagnosticRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DiagnosticOptions,
}
struct WorkspaceDiagnosticParams {
    identifier: String,
    previousResultIds: Vec<PreviousResultId>,
}
struct WorkspaceDiagnosticReport {
    items: Vec<WorkspaceDocumentDiagnosticReport>,
}
struct WorkspaceDiagnosticReportPartialResult {
    items: Vec<WorkspaceDocumentDiagnosticReport>,
}
struct DidOpenNotebookDocumentParams {
    notebookDocument: NotebookDocument,
    cellTextDocuments: Vec<TextDocumentItem>,
}
struct DidChangeNotebookDocumentParams {
    notebookDocument: VersionedNotebookDocumentIdentifier,
    change: NotebookDocumentChangeEvent,
}
struct DidSaveNotebookDocumentParams {
    notebookDocument: NotebookDocumentIdentifier,
}
struct DidCloseNotebookDocumentParams {
    notebookDocument: NotebookDocumentIdentifier,
    cellTextDocuments: Vec<TextDocumentIdentifier>,
}
struct RegistrationParams {
    registrations: Vec<Registration>,
}
struct UnregistrationParams {
    unregisterations: Vec<Unregistration>,
}
struct InitializeParams {
    _0: _InitializeParams,
    _1: WorkspaceFoldersInitializeParams,
}
struct InitializeResult {
    capabilities: ServerCapabilities,
    serverInfo: _a61bc5d7496fb1efa1e9b725864fd45cb9a1408070cc0dffd85a70c5,
}
struct InitializeError {
    retry: bool,
}
struct InitializedParams {}
struct DidChangeConfigurationParams {
    settings: LSPAny,
}
struct DidChangeConfigurationRegistrationOptions {
    section: _a4685177a66e3b501e6cc62916bace54742fa85908984cd2881ed04d,
}
struct ShowMessageParams {
    r#type: MessageType,
    message: String,
}
struct ShowMessageRequestParams {
    r#type: MessageType,
    message: String,
    actions: Vec<MessageActionItem>,
}
struct MessageActionItem {
    title: String,
}
struct LogMessageParams {
    r#type: MessageType,
    message: String,
}
struct DidOpenTextDocumentParams {
    textDocument: TextDocumentItem,
}
struct DidChangeTextDocumentParams {
    textDocument: VersionedTextDocumentIdentifier,
    contentChanges: Vec<TextDocumentContentChangeEvent>,
}
struct TextDocumentChangeRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    syncKind: TextDocumentSyncKind,
}
struct DidCloseTextDocumentParams {
    textDocument: TextDocumentIdentifier,
}
struct DidSaveTextDocumentParams {
    textDocument: TextDocumentIdentifier,
    text: String,
}
struct TextDocumentSaveRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: SaveOptions,
}
struct WillSaveTextDocumentParams {
    textDocument: TextDocumentIdentifier,
    reason: TextDocumentSaveReason,
}
struct TextEdit {
    range: Range,
    newText: String,
}
struct DidChangeWatchedFilesParams {
    changes: Vec<FileEvent>,
}
struct DidChangeWatchedFilesRegistrationOptions {
    watchers: Vec<FileSystemWatcher>,
}
struct PublishDiagnosticsParams {
    uri: String,
    version: i64,
    diagnostics: Vec<Diagnostic>,
}
struct CompletionParams {
    _0: TextDocumentPositionParams,
    context: CompletionContext,
}
struct CompletionItem {
    label: String,
    labelDetails: CompletionItemLabelDetails,
    kind: CompletionItemKind,
    tags: Vec<CompletionItemTag>,
    detail: String,
    documentation: _7d6bb204775151a30d94c8f3f290f43785b27796aebdfd57661b1c53,
    deprecated: bool,
    preselect: bool,
    sortText: String,
    filterText: String,
    insertText: String,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    textEdit: _b9458109bbf5e70d956d873a8321de8414cc9ef003d7d317b8295e4c,
    textEditText: String,
    additionalTextEdits: Vec<TextEdit>,
    commitCharacters: Vec<String>,
    command: Command,
    data: LSPAny,
}
struct CompletionList {
    isIncomplete: bool,
    itemDefaults: _3e2b373e868b42c09d4b77ec92f0c9be680e8745ad1ebc61234242fe,
    items: Vec<CompletionItem>,
}
struct CompletionRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: CompletionOptions,
}
struct HoverParams {
    _0: TextDocumentPositionParams,
}
struct Hover {
    contents: _4b1a093db38b713ca100a78f933a283d4d84d2ff15a951ec34bc1dd0,
    range: Range,
}
struct HoverRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: HoverOptions,
}
struct SignatureHelpParams {
    _0: TextDocumentPositionParams,
    context: SignatureHelpContext,
}
struct SignatureHelp {
    signatures: Vec<SignatureInformation>,
    activeSignature: u64,
    activeParameter: u64,
}
struct SignatureHelpRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: SignatureHelpOptions,
}
struct DefinitionParams {
    _0: TextDocumentPositionParams,
}
struct DefinitionRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DefinitionOptions,
}
struct ReferenceParams {
    _0: TextDocumentPositionParams,
    context: ReferenceContext,
}
struct ReferenceRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: ReferenceOptions,
}
struct DocumentHighlightParams {
    _0: TextDocumentPositionParams,
}
struct DocumentHighlight {
    range: Range,
    kind: DocumentHighlightKind,
}
struct DocumentHighlightRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentHighlightOptions,
}
struct DocumentSymbolParams {
    textDocument: TextDocumentIdentifier,
}
struct SymbolInformation {
    _0: BaseSymbolInformation,
    deprecated: bool,
    location: Location,
}
struct DocumentSymbol {
    name: String,
    detail: String,
    kind: SymbolKind,
    tags: Vec<SymbolTag>,
    deprecated: bool,
    range: Range,
    selectionRange: Range,
    children: Vec<DocumentSymbol>,
}
struct DocumentSymbolRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentSymbolOptions,
}
struct CodeActionParams {
    textDocument: TextDocumentIdentifier,
    range: Range,
    context: CodeActionContext,
}
struct Command {
    title: String,
    command: String,
    arguments: Vec<LSPAny>,
}
struct CodeAction {
    title: String,
    kind: CodeActionKind,
    diagnostics: Vec<Diagnostic>,
    isPreferred: bool,
    disabled: _c608c61340f31402c88067bab181725abdfedc887eade9a02cef744f,
    edit: WorkspaceEdit,
    command: Command,
    data: LSPAny,
}
struct CodeActionRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: CodeActionOptions,
}
struct WorkspaceSymbolParams {
    query: String,
}
struct WorkspaceSymbol {
    _0: BaseSymbolInformation,
    location: _8f2dab9edc049982d2fe47d233fbd3d0c8a543108c48ca253448ab24,
    data: LSPAny,
}
struct WorkspaceSymbolRegistrationOptions {
    _0: WorkspaceSymbolOptions,
}
struct CodeLensParams {
    textDocument: TextDocumentIdentifier,
}
struct CodeLens {
    range: Range,
    command: Command,
    data: LSPAny,
}
struct CodeLensRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: CodeLensOptions,
}
struct DocumentLinkParams {
    textDocument: TextDocumentIdentifier,
}
struct DocumentLink {
    range: Range,
    target: String,
    tooltip: String,
    data: LSPAny,
}
struct DocumentLinkRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentLinkOptions,
}
struct DocumentFormattingParams {
    textDocument: TextDocumentIdentifier,
    options: FormattingOptions,
}
struct DocumentFormattingRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentFormattingOptions,
}
struct DocumentRangeFormattingParams {
    textDocument: TextDocumentIdentifier,
    range: Range,
    options: FormattingOptions,
}
struct DocumentRangeFormattingRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentRangeFormattingOptions,
}
struct DocumentOnTypeFormattingParams {
    textDocument: TextDocumentIdentifier,
    position: Position,
    ch: String,
    options: FormattingOptions,
}
struct DocumentOnTypeFormattingRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: DocumentOnTypeFormattingOptions,
}
struct RenameParams {
    textDocument: TextDocumentIdentifier,
    position: Position,
    newName: String,
}
struct RenameRegistrationOptions {
    _0: TextDocumentRegistrationOptions,
    _1: RenameOptions,
}
struct PrepareRenameParams {
    _0: TextDocumentPositionParams,
}
struct ExecuteCommandParams {
    command: String,
    arguments: Vec<LSPAny>,
}
struct ExecuteCommandRegistrationOptions {
    _0: ExecuteCommandOptions,
}
struct ApplyWorkspaceEditParams {
    label: String,
    edit: WorkspaceEdit,
}
struct ApplyWorkspaceEditResult {
    applied: bool,
    failureReason: String,
    failedChange: u64,
}
struct WorkDoneProgressBegin {
    kind: (),
    title: String,
    cancellable: bool,
    message: String,
    percentage: u64,
}
struct WorkDoneProgressReport {
    kind: (),
    cancellable: bool,
    message: String,
    percentage: u64,
}
struct WorkDoneProgressEnd {
    kind: (),
    message: String,
}
struct SetTraceParams {
    value: TraceValues,
}
struct LogTraceParams {
    message: String,
    verbose: String,
}
struct CancelParams {
    id: _46876517743bdc4dc23598ff397bfe4060b99be6b3cdacdf95e7b692,
}
struct ProgressParams {
    token: ProgressToken,
    value: LSPAny,
}
struct TextDocumentPositionParams {
    textDocument: TextDocumentIdentifier,
    position: Position,
}
struct WorkDoneProgressParams {
    workDoneToken: ProgressToken,
}
struct LocationLink {
    originSelectionRange: Range,
    targetUri: String,
    targetRange: Range,
    targetSelectionRange: Range,
}
struct Range {
    start: Position,
    end: Position,
}
struct ImplementationOptions {}
struct StaticRegistrationOptions {
    id: String,
}
struct TypeDefinitionOptions {}
struct WorkspaceFoldersChangeEvent {
    added: Vec<WorkspaceFolder>,
    removed: Vec<WorkspaceFolder>,
}
struct ConfigurationItem {
    scopeUri: String,
    section: String,
}
struct TextDocumentIdentifier {
    uri: String,
}
struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}
struct DocumentColorOptions {}
struct FoldingRangeOptions {}
struct DeclarationOptions {}
struct Position {
    line: u64,
    character: u64,
}
struct SelectionRangeOptions {}
struct CallHierarchyOptions {}
struct SemanticTokensOptions {
    legend: SemanticTokensLegend,
    range: _b570f9f1b1bd0545adafef02995449a937f718c9e07ea93a7067e91e,
    full: _3d62efa3984910a9decfd267ff8cc3a28d8ef7ccdb27012e13a4b196,
}
struct SemanticTokensEdit {
    start: u64,
    deleteCount: u64,
    data: Vec<u64>,
}
struct LinkedEditingRangeOptions {}
struct FileCreate {
    uri: String,
}
struct TextDocumentEdit {
    textDocument: OptionalVersionedTextDocumentIdentifier,
    edits: Vec<_86c355212a6ac4cde2daeb9e4abacb35470d8ddc2f09f5603c45eab5>,
}
struct CreateFile {
    _0: ResourceOperation,
    kind: (),
    uri: String,
    options: CreateFileOptions,
}
struct RenameFile {
    _0: ResourceOperation,
    kind: (),
    oldUri: String,
    newUri: String,
    options: RenameFileOptions,
}
struct DeleteFile {
    _0: ResourceOperation,
    kind: (),
    uri: String,
    options: DeleteFileOptions,
}
struct ChangeAnnotation {
    label: String,
    needsConfirmation: bool,
    description: String,
}
struct FileOperationFilter {
    scheme: String,
    pattern: FileOperationPattern,
}
struct FileRename {
    oldUri: String,
    newUri: String,
}
struct FileDelete {
    uri: String,
}
struct MonikerOptions {}
struct TypeHierarchyOptions {}
struct InlineValueContext {
    frameId: i64,
    stoppedLocation: Range,
}
struct InlineValueText {
    range: Range,
    text: String,
}
struct InlineValueVariableLookup {
    range: Range,
    variableName: String,
    caseSensitiveLookup: bool,
}
struct InlineValueEvaluatableExpression {
    range: Range,
    expression: String,
}
struct InlineValueOptions {}
struct InlayHintLabelPart {
    value: String,
    tooltip: _55c44133dddc6460f9bd95e7bd4ae1954b6fdc3ac0a277ceaf7aff03,
    location: Location,
    command: Command,
}
struct MarkupContent {
    kind: MarkupKind,
    value: String,
}
struct InlayHintOptions {
    resolveProvider: bool,
}
struct RelatedFullDocumentDiagnosticReport {
    _0: FullDocumentDiagnosticReport,
    relatedDocuments: ::std::collections::HashMap<
        String,
        _6d4d7397e719a0c242e2443276252a12751959ca3ff66421fd91f902,
    >,
}
struct RelatedUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    relatedDocuments: ::std::collections::HashMap<
        String,
        _da4007d8065e0cfbc8daf1fc291e41264f9c9966c8bf1ee89ad11858,
    >,
}
struct FullDocumentDiagnosticReport {
    kind: (),
    resultId: String,
    items: Vec<Diagnostic>,
}
struct UnchangedDocumentDiagnosticReport {
    kind: (),
    resultId: String,
}
struct DiagnosticOptions {
    identifier: String,
    interFileDependencies: bool,
    workspaceDiagnostics: bool,
}
struct PreviousResultId {
    uri: String,
    value: String,
}
struct NotebookDocument {
    uri: String,
    notebookType: String,
    version: i64,
    metadata: LSPObject,
    cells: Vec<NotebookCell>,
}
struct TextDocumentItem {
    uri: String,
    languageId: String,
    version: i64,
    text: String,
}
struct VersionedNotebookDocumentIdentifier {
    version: i64,
    uri: String,
}
struct NotebookDocumentChangeEvent {
    metadata: LSPObject,
    cells: _8686c18ca013eba49728040785b6375a8f875efe117e7f99d479300b,
}
struct NotebookDocumentIdentifier {
    uri: String,
}
struct Registration {
    id: String,
    method: String,
    registerOptions: LSPAny,
}
struct Unregistration {
    id: String,
    method: String,
}
struct _InitializeParams {
    processId: _4741baf77e5c28311ca100a74e2b7a42263c81f17fbf1de0cb17ed7f,
    clientInfo: _cbb7ad32da0385d751c075c64aa8156298bac14e31f888175df60a65,
    locale: String,
    rootPath: _4e0c08e9e816138aef047f8aa74a40d6c847d63128617e9a29c49f6f,
    rootUri: _c125ac7b3b9c1b26f77c2d8738f29748ce6a58ed7880cdcdbf8c5765,
    capabilities: ClientCapabilities,
    initializationOptions: LSPAny,
    trace: _5e1fd87b1246e9ef8ff39c1b0e0a45421dd55d5766381d4dc899a0cc,
}
struct WorkspaceFoldersInitializeParams {
    workspaceFolders: _5868a169622a556bfcf7731695edfdb33b898f01f9d0350a52a77f7e,
}
struct ServerCapabilities {
    positionEncoding: PositionEncodingKind,
    textDocumentSync: _b1df196d752f1c7feb0315def8e74b71e062b97ea211598052596d7c,
    notebookDocumentSync: _595182541430ce6a312c274105fa59a127c14d1d17a1e25c6c129d39,
    completionProvider: CompletionOptions,
    hoverProvider: _a0c90c4ed2f85bdf6d6ade31475a91c2b6e528f3cdf3a6fce7c28c48,
    signatureHelpProvider: SignatureHelpOptions,
    declarationProvider: _04317f8f91f555b30cf0fe639c4a6cf89998bfa11af1e020f565b7e1,
    definitionProvider: _726064a7dd875570b101a00559dbbbea4e4fb7e7f397095f7a2f9a80,
    typeDefinitionProvider: _3d304a6b55694e7e08b55b6ad74e76b9269f6685c1c790d771c20967,
    implementationProvider: _9a04310084e02a69ec04c1e6a6c574e184bfff6a77584f33cd67d8b0,
    referencesProvider: _f0cf0d87d16d412504c9a4cb798aca07129307ac35d239b74863cba5,
    documentHighlightProvider: _e222f1036626f9d16353835dc3d53219c5926f19b8de60f19287d8b1,
    documentSymbolProvider: _e96adf8509731b7d08ac976b369b7f37a66fb17996416ea183ebcf66,
    codeActionProvider: _02a8ba13640b991b671fbd9c60c0c32b0a42a45f2f515782fce974f2,
    codeLensProvider: CodeLensOptions,
    documentLinkProvider: DocumentLinkOptions,
    colorProvider: _26c0166cf265412c6232bcd7db1a3bf9a814bf0ca902c7e9c0ca852d,
    workspaceSymbolProvider: _6d1693fd1d58ffd1345642cebc8631754b1956701e4538bb175e8ee0,
    documentFormattingProvider: _e6fe21d118d621ee9bd07de8bd6f25de4738ea2eebfff67bd3b93c86,
    documentRangeFormattingProvider: _7b91c96dbdd4cf7328bb6b714c2e9183af61d6cc08396f4dc4bae568,
    documentOnTypeFormattingProvider: DocumentOnTypeFormattingOptions,
    renameProvider: _aa9bf9fdb1f89b77538d1909dfa7cfd192a75e91547501ba395ba7d3,
    foldingRangeProvider: _e759e7a403d62cb3109d326e3c424b596eccb1a9e99bc7c3693c9513,
    selectionRangeProvider: _505a646dc962718cb3169c822e33a538d79d540e58cf080545dbf9ae,
    executeCommandProvider: ExecuteCommandOptions,
    callHierarchyProvider: _48fb1fa3da14d65578b5d664c9158ae81311eefb97acc98fa0e31b98,
    linkedEditingRangeProvider: _734c41cfb5333dab38738128fcb520d279027501d1639841dd1083fa,
    semanticTokensProvider: _a23047d956bbb441b6f0c388d6f70c57ad2a259fc66e89fd607cd974,
    monikerProvider: _8702b8c310a621de29bb724ae324d822ff769d4055a37a01da756c38,
    typeHierarchyProvider: _9b377653dfd0cac2b3c93d4227eed3e9735cb45cdeb84ef0b81c4a5f,
    inlineValueProvider: _0cab083e9333223d5b76b08ac7d5aefac1820a3af21f1f5a414d2a9b,
    inlayHintProvider: _531b32e5541cdc8ec6bc8aa8e0ed39d106f3bc108195c096b6106e9f,
    diagnosticProvider: _4e24549c800892ecca8f3ea185df09e2fed5062e9cd529e33667d9d0,
    workspace: _5418b79a89f5cc28e7a102d33ce199381b6f1e9bf0bdb3f46968c229,
    experimental: LSPAny,
}
struct VersionedTextDocumentIdentifier {
    _0: TextDocumentIdentifier,
    version: i64,
}
struct SaveOptions {
    includeText: bool,
}
struct FileEvent {
    uri: String,
    r#type: FileChangeType,
}
struct FileSystemWatcher {
    globPattern: GlobPattern,
    kind: WatchKind,
}
struct Diagnostic {
    range: Range,
    severity: DiagnosticSeverity,
    code: _18d7e36fdbc505296f71e2f93b82efebeeb9426e15069f2ab43ba542,
    codeDescription: CodeDescription,
    source: String,
    message: String,
    tags: Vec<DiagnosticTag>,
    relatedInformation: Vec<DiagnosticRelatedInformation>,
    data: LSPAny,
}
struct CompletionContext {
    triggerKind: CompletionTriggerKind,
    triggerCharacter: String,
}
struct CompletionItemLabelDetails {
    detail: String,
    description: String,
}
struct InsertReplaceEdit {
    newText: String,
    insert: Range,
    replace: Range,
}
struct CompletionOptions {
    triggerCharacters: Vec<String>,
    allCommitCharacters: Vec<String>,
    resolveProvider: bool,
    completionItem: _688088efd1b14986d98f20cf44230f728f6e1a914b331ee9a6126f4f,
}
struct HoverOptions {}
struct SignatureHelpContext {
    triggerKind: SignatureHelpTriggerKind,
    triggerCharacter: String,
    isRetrigger: bool,
    activeSignatureHelp: SignatureHelp,
}
struct SignatureInformation {
    label: String,
    documentation: _664ca25c59505bd4079b05c93b742c52fe16fc98f1c8e5a3a59ab064,
    parameters: Vec<ParameterInformation>,
    activeParameter: u64,
}
struct SignatureHelpOptions {
    triggerCharacters: Vec<String>,
    retriggerCharacters: Vec<String>,
}
struct DefinitionOptions {}
struct ReferenceContext {
    includeDeclaration: bool,
}
struct ReferenceOptions {}
struct DocumentHighlightOptions {}
struct BaseSymbolInformation {
    name: String,
    kind: SymbolKind,
    tags: Vec<SymbolTag>,
    containerName: String,
}
struct DocumentSymbolOptions {
    label: String,
}
struct CodeActionContext {
    diagnostics: Vec<Diagnostic>,
    only: Vec<CodeActionKind>,
    triggerKind: CodeActionTriggerKind,
}
struct CodeActionOptions {
    codeActionKinds: Vec<CodeActionKind>,
    resolveProvider: bool,
}
struct WorkspaceSymbolOptions {
    resolveProvider: bool,
}
struct CodeLensOptions {
    resolveProvider: bool,
}
struct DocumentLinkOptions {
    resolveProvider: bool,
}
struct FormattingOptions {
    tabSize: u64,
    insertSpaces: bool,
    trimTrailingWhitespace: bool,
    insertFinalNewline: bool,
    trimFinalNewlines: bool,
}
struct DocumentFormattingOptions {}
struct DocumentRangeFormattingOptions {}
struct DocumentOnTypeFormattingOptions {
    firstTriggerCharacter: String,
    moreTriggerCharacter: Vec<String>,
}
struct RenameOptions {
    prepareProvider: bool,
}
struct ExecuteCommandOptions {
    commands: Vec<String>,
}
struct SemanticTokensLegend {
    tokenTypes: Vec<String>,
    tokenModifiers: Vec<String>,
}
struct OptionalVersionedTextDocumentIdentifier {
    _0: TextDocumentIdentifier,
    version: _e911abe98649132bc4f7460d1ede2881cb9018c613e2d7d167c8ca90,
}
struct AnnotatedTextEdit {
    _0: TextEdit,
    annotationId: ChangeAnnotationIdentifier,
}
struct ResourceOperation {
    kind: String,
    annotationId: ChangeAnnotationIdentifier,
}
struct CreateFileOptions {
    overwrite: bool,
    ignoreIfExists: bool,
}
struct RenameFileOptions {
    overwrite: bool,
    ignoreIfExists: bool,
}
struct DeleteFileOptions {
    recursive: bool,
    ignoreIfNotExists: bool,
}
struct FileOperationPattern {
    glob: String,
    matches: FileOperationPatternKind,
    options: FileOperationPatternOptions,
}
struct WorkspaceFullDocumentDiagnosticReport {
    _0: FullDocumentDiagnosticReport,
    uri: String,
    version: _e5075c3e3fc87f4559c38c96edfac0a0f57e50eb51032e5fcacd5dfc,
}
struct WorkspaceUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    uri: String,
    version: _eb3c8858872159820455156ca8344d61c40c1fe79e60d9c500ec5736,
}
struct LSPObject {}
struct NotebookCell {
    kind: NotebookCellKind,
    document: String,
    metadata: LSPObject,
    executionSummary: ExecutionSummary,
}
struct NotebookCellArrayChange {
    start: u64,
    deleteCount: u64,
    cells: Vec<NotebookCell>,
}
struct ClientCapabilities {
    workspace: WorkspaceClientCapabilities,
    textDocument: TextDocumentClientCapabilities,
    notebookDocument: NotebookDocumentClientCapabilities,
    window: WindowClientCapabilities,
    general: GeneralClientCapabilities,
    experimental: LSPAny,
}
struct TextDocumentSyncOptions {
    openClose: bool,
    change: TextDocumentSyncKind,
    willSave: bool,
    willSaveWaitUntil: bool,
    save: _63cb47a5952dcc1e06bf042e05f7d29db500a5d7acae00fc07d4dd8f,
}
struct NotebookDocumentSyncOptions {
    notebookSelector: Vec<_f0494010a863ad5b5c35127a6170fa4f5109dbe797c8c290bfa69502>,
    save: bool,
}
struct NotebookDocumentSyncRegistrationOptions {
    _0: NotebookDocumentSyncOptions,
}
struct WorkspaceFoldersServerCapabilities {
    supported: bool,
    changeNotifications: _fe81a511bd7bdf218978e741051f1f7d417ffb3ad154f24e718e1390,
}
struct FileOperationOptions {
    didCreate: FileOperationRegistrationOptions,
    willCreate: FileOperationRegistrationOptions,
    didRename: FileOperationRegistrationOptions,
    willRename: FileOperationRegistrationOptions,
    didDelete: FileOperationRegistrationOptions,
    willDelete: FileOperationRegistrationOptions,
}
struct CodeDescription {
    href: String,
}
struct DiagnosticRelatedInformation {
    location: Location,
    message: String,
}
struct ParameterInformation {
    label: _a6d429ebad4d0c5cb627a44498b1513a14e0f6597fb2b59ec3f05120,
    documentation: _f6c860f8fae52697f4faa203922b9c62d994532ee6667fedbd642380,
}
struct NotebookCellTextDocumentFilter {
    notebook: _be533a4f33e057440c4f64a408d63f6c22aa65744e3102f8d12875c6,
    language: String,
}
struct FileOperationPatternOptions {
    ignoreCase: bool,
}
struct ExecutionSummary {
    executionOrder: u64,
    success: bool,
}
struct WorkspaceClientCapabilities {
    applyEdit: bool,
    workspaceEdit: WorkspaceEditClientCapabilities,
    didChangeConfiguration: DidChangeConfigurationClientCapabilities,
    didChangeWatchedFiles: DidChangeWatchedFilesClientCapabilities,
    symbol: WorkspaceSymbolClientCapabilities,
    executeCommand: ExecuteCommandClientCapabilities,
    workspaceFolders: bool,
    configuration: bool,
    semanticTokens: SemanticTokensWorkspaceClientCapabilities,
    codeLens: CodeLensWorkspaceClientCapabilities,
    fileOperations: FileOperationClientCapabilities,
    inlineValue: InlineValueWorkspaceClientCapabilities,
    inlayHint: InlayHintWorkspaceClientCapabilities,
    diagnostics: DiagnosticWorkspaceClientCapabilities,
}
struct TextDocumentClientCapabilities {
    synchronization: TextDocumentSyncClientCapabilities,
    completion: CompletionClientCapabilities,
    hover: HoverClientCapabilities,
    signatureHelp: SignatureHelpClientCapabilities,
    declaration: DeclarationClientCapabilities,
    definition: DefinitionClientCapabilities,
    typeDefinition: TypeDefinitionClientCapabilities,
    implementation: ImplementationClientCapabilities,
    references: ReferenceClientCapabilities,
    documentHighlight: DocumentHighlightClientCapabilities,
    documentSymbol: DocumentSymbolClientCapabilities,
    codeAction: CodeActionClientCapabilities,
    codeLens: CodeLensClientCapabilities,
    documentLink: DocumentLinkClientCapabilities,
    colorProvider: DocumentColorClientCapabilities,
    formatting: DocumentFormattingClientCapabilities,
    rangeFormatting: DocumentRangeFormattingClientCapabilities,
    onTypeFormatting: DocumentOnTypeFormattingClientCapabilities,
    rename: RenameClientCapabilities,
    foldingRange: FoldingRangeClientCapabilities,
    selectionRange: SelectionRangeClientCapabilities,
    publishDiagnostics: PublishDiagnosticsClientCapabilities,
    callHierarchy: CallHierarchyClientCapabilities,
    semanticTokens: SemanticTokensClientCapabilities,
    linkedEditingRange: LinkedEditingRangeClientCapabilities,
    moniker: MonikerClientCapabilities,
    typeHierarchy: TypeHierarchyClientCapabilities,
    inlineValue: InlineValueClientCapabilities,
    inlayHint: InlayHintClientCapabilities,
    diagnostic: DiagnosticClientCapabilities,
}
struct NotebookDocumentClientCapabilities {
    synchronization: NotebookDocumentSyncClientCapabilities,
}
struct WindowClientCapabilities {
    workDoneProgress: bool,
    showMessage: ShowMessageRequestClientCapabilities,
    showDocument: ShowDocumentClientCapabilities,
}
struct GeneralClientCapabilities {
    staleRequestSupport: _003415e244d2612d789a19dab8421b511187b3755b9473930b674d12,
    regularExpressions: RegularExpressionsClientCapabilities,
    markdown: MarkdownClientCapabilities,
    positionEncodings: Vec<PositionEncodingKind>,
}
struct RelativePattern {
    baseUri: _55e35d0ecaaf41c17687df82ca01b08921e446491ab4731af65fe581,
    pattern: Pattern,
}
struct WorkspaceEditClientCapabilities {
    documentChanges: bool,
    resourceOperations: Vec<ResourceOperationKind>,
    failureHandling: FailureHandlingKind,
    normalizesLineEndings: bool,
    changeAnnotationSupport: _b6707bcef1be5e545e98099e53ea44f18347faf75ed7d7edf6d44ef0,
}
struct DidChangeConfigurationClientCapabilities {
    dynamicRegistration: bool,
}
struct DidChangeWatchedFilesClientCapabilities {
    dynamicRegistration: bool,
    relativePatternSupport: bool,
}
struct WorkspaceSymbolClientCapabilities {
    dynamicRegistration: bool,
    symbolKind: _0d8e0f734067929cc8e5b9d0da8d1d3f54e69aa40241215a69208b1f,
    tagSupport: _591e4745558d2dc75c1523b59943c321d24eabd0847dddfbd9af505e,
    resolveSupport: _b9b6711c52b95e4c40da4d25dfaa2e5c9f540bbb7739fbfcc6b56568,
}
struct ExecuteCommandClientCapabilities {
    dynamicRegistration: bool,
}
struct SemanticTokensWorkspaceClientCapabilities {
    refreshSupport: bool,
}
struct CodeLensWorkspaceClientCapabilities {
    refreshSupport: bool,
}
struct FileOperationClientCapabilities {
    dynamicRegistration: bool,
    didCreate: bool,
    willCreate: bool,
    didRename: bool,
    willRename: bool,
    didDelete: bool,
    willDelete: bool,
}
struct InlineValueWorkspaceClientCapabilities {
    refreshSupport: bool,
}
struct InlayHintWorkspaceClientCapabilities {
    refreshSupport: bool,
}
struct DiagnosticWorkspaceClientCapabilities {
    refreshSupport: bool,
}
struct TextDocumentSyncClientCapabilities {
    dynamicRegistration: bool,
    willSave: bool,
    willSaveWaitUntil: bool,
    didSave: bool,
}
struct CompletionClientCapabilities {
    dynamicRegistration: bool,
    completionItem: _abed05221375ee6f658341ceb8a2fd3422dd58eefa7cf4603ab2244d,
    completionItemKind: _f9a77c24bb228bd169055f2a5897495d25038ea2f0416d17a90e2452,
    insertTextMode: InsertTextMode,
    contextSupport: bool,
    completionList: _c97efab1d6dddb53f99b49656a2402f88a4737325119a6483e2b5db5,
}
struct HoverClientCapabilities {
    dynamicRegistration: bool,
    contentFormat: Vec<MarkupKind>,
}
struct SignatureHelpClientCapabilities {
    dynamicRegistration: bool,
    signatureInformation: _ca746e2f1bb1fdee9b60dfe2e4a60af75e7fe3ec193521018781c496,
    contextSupport: bool,
}
struct DeclarationClientCapabilities {
    dynamicRegistration: bool,
    linkSupport: bool,
}
struct DefinitionClientCapabilities {
    dynamicRegistration: bool,
    linkSupport: bool,
}
struct TypeDefinitionClientCapabilities {
    dynamicRegistration: bool,
    linkSupport: bool,
}
struct ImplementationClientCapabilities {
    dynamicRegistration: bool,
    linkSupport: bool,
}
struct ReferenceClientCapabilities {
    dynamicRegistration: bool,
}
struct DocumentHighlightClientCapabilities {
    dynamicRegistration: bool,
}
struct DocumentSymbolClientCapabilities {
    dynamicRegistration: bool,
    symbolKind: _2e39f4b7aeaa9e12fc785358cdaf981e9fa1f74c27deeef45fa32aa0,
    hierarchicalDocumentSymbolSupport: bool,
    tagSupport: _a103d42a15a2985aa612bb759666278a5ce67d2a782dd5e1e3271823,
    labelSupport: bool,
}
struct CodeActionClientCapabilities {
    dynamicRegistration: bool,
    codeActionLiteralSupport: _f58e6e95aac5dafb26357ad6a6d171269bfce7b2ddc435a2f6f057bc,
    isPreferredSupport: bool,
    disabledSupport: bool,
    dataSupport: bool,
    resolveSupport: _b5e7a4ebd7246f63611beec6ae6a7ecc6bee967c3b921da16371bfc0,
    honorsChangeAnnotations: bool,
}
struct CodeLensClientCapabilities {
    dynamicRegistration: bool,
}
struct DocumentLinkClientCapabilities {
    dynamicRegistration: bool,
    tooltipSupport: bool,
}
struct DocumentColorClientCapabilities {
    dynamicRegistration: bool,
}
struct DocumentFormattingClientCapabilities {
    dynamicRegistration: bool,
}
struct DocumentRangeFormattingClientCapabilities {
    dynamicRegistration: bool,
}
struct DocumentOnTypeFormattingClientCapabilities {
    dynamicRegistration: bool,
}
struct RenameClientCapabilities {
    dynamicRegistration: bool,
    prepareSupport: bool,
    prepareSupportDefaultBehavior: PrepareSupportDefaultBehavior,
    honorsChangeAnnotations: bool,
}
struct FoldingRangeClientCapabilities {
    dynamicRegistration: bool,
    rangeLimit: u64,
    lineFoldingOnly: bool,
    foldingRangeKind: _6de36e91868f2b78b3e86b8f4028da34d8457ce95ca8f9ca5842c0db,
    foldingRange: _e508b892eb91775d344b084cb01a1f3f32162e3cd19294e8a7962903,
}
struct SelectionRangeClientCapabilities {
    dynamicRegistration: bool,
}
struct PublishDiagnosticsClientCapabilities {
    relatedInformation: bool,
    tagSupport: _1b0c8a8bf5ec254687aa4e85350349aa8e1d874454ce6e58d4ad3c81,
    versionSupport: bool,
    codeDescriptionSupport: bool,
    dataSupport: bool,
}
struct CallHierarchyClientCapabilities {
    dynamicRegistration: bool,
}
struct SemanticTokensClientCapabilities {
    dynamicRegistration: bool,
    requests: _76956417a800c7206b54add57d83af24eae22079e702e05e38173c6b,
    tokenTypes: Vec<String>,
    tokenModifiers: Vec<String>,
    formats: Vec<TokenFormat>,
    overlappingTokenSupport: bool,
    multilineTokenSupport: bool,
    serverCancelSupport: bool,
    augmentsSyntaxTokens: bool,
}
struct LinkedEditingRangeClientCapabilities {
    dynamicRegistration: bool,
}
struct MonikerClientCapabilities {
    dynamicRegistration: bool,
}
struct TypeHierarchyClientCapabilities {
    dynamicRegistration: bool,
}
struct InlineValueClientCapabilities {
    dynamicRegistration: bool,
}
struct InlayHintClientCapabilities {
    dynamicRegistration: bool,
    resolveSupport: _598578991bb319e2e390858fa8230eb695b82100ae440f642ccb9968,
}
struct DiagnosticClientCapabilities {
    dynamicRegistration: bool,
    relatedDocumentSupport: bool,
}
struct NotebookDocumentSyncClientCapabilities {
    dynamicRegistration: bool,
    executionSummarySupport: bool,
}
struct ShowMessageRequestClientCapabilities {
    messageActionItem: _310d9d6f7229eea387894a6e9eeefb6de5b6a34f5bb997c33ffd34e5,
}
struct ShowDocumentClientCapabilities {
    support: bool,
}
struct RegularExpressionsClientCapabilities {
    engine: String,
    version: String,
}
struct MarkdownClientCapabilities {
    parser: String,
    version: String,
    allowedTags: Vec<String>,
}
enum SemanticTokenTypes {
    namespace,
    r#type,
    class,
    r#enum,
    interface,
    r#struct,
    typeParameter,
    parameter,
    variable,
    property,
    enumMember,
    event,
    function,
    method,
    r#macro,
    keyword,
    modifier,
    comment,
    string,
    number,
    regexp,
    operator,
    decorator,
}
enum SemanticTokenModifiers {
    declaration,
    definition,
    readonly,
    r#static,
    deprecated,
    r#abstract,
    r#async,
    modification,
    documentation,
    defaultLibrary,
}
enum DocumentDiagnosticReportKind {
    Full,
    Unchanged,
}
#[repr(i64)]
enum ErrorCodes {
    ParseError = -32700i64,
    InvalidRequest = -32600i64,
    MethodNotFound = -32601i64,
    InvalidParams = -32602i64,
    InternalError = -32603i64,
    ServerNotInitialized = -32002i64,
    UnknownErrorCode = -32001i64,
}
#[repr(i64)]
enum LSPErrorCodes {
    RequestFailed = -32803i64,
    ServerCancelled = -32802i64,
    ContentModified = -32801i64,
    RequestCancelled = -32800i64,
}
enum FoldingRangeKind {
    Comment,
    Imports,
    Region,
}
#[repr(i64)]
enum SymbolKind {
    File = 1i64,
    Module = 2i64,
    Namespace = 3i64,
    Package = 4i64,
    Class = 5i64,
    Method = 6i64,
    Property = 7i64,
    Field = 8i64,
    Constructor = 9i64,
    Enum = 10i64,
    Interface = 11i64,
    Function = 12i64,
    Variable = 13i64,
    Constant = 14i64,
    String = 15i64,
    Number = 16i64,
    Boolean = 17i64,
    Array = 18i64,
    Object = 19i64,
    Key = 20i64,
    Null = 21i64,
    EnumMember = 22i64,
    Struct = 23i64,
    Event = 24i64,
    Operator = 25i64,
    TypeParameter = 26i64,
}
#[repr(i64)]
enum SymbolTag {
    Deprecated = 1i64,
}
enum UniquenessLevel {
    document,
    project,
    group,
    scheme,
    global,
}
enum MonikerKind {
    import,
    export,
    local,
}
#[repr(i64)]
enum InlayHintKind {
    Type = 1i64,
    Parameter = 2i64,
}
#[repr(i64)]
enum MessageType {
    Error = 1i64,
    Warning = 2i64,
    Info = 3i64,
    Log = 4i64,
}
#[repr(i64)]
enum TextDocumentSyncKind {
    None = 0i64,
    Full = 1i64,
    Incremental = 2i64,
}
#[repr(i64)]
enum TextDocumentSaveReason {
    Manual = 1i64,
    AfterDelay = 2i64,
    FocusOut = 3i64,
}
#[repr(i64)]
enum CompletionItemKind {
    Text = 1i64,
    Method = 2i64,
    Function = 3i64,
    Constructor = 4i64,
    Field = 5i64,
    Variable = 6i64,
    Class = 7i64,
    Interface = 8i64,
    Module = 9i64,
    Property = 10i64,
    Unit = 11i64,
    Value = 12i64,
    Enum = 13i64,
    Keyword = 14i64,
    Snippet = 15i64,
    Color = 16i64,
    File = 17i64,
    Reference = 18i64,
    Folder = 19i64,
    EnumMember = 20i64,
    Constant = 21i64,
    Struct = 22i64,
    Event = 23i64,
    Operator = 24i64,
    TypeParameter = 25i64,
}
#[repr(i64)]
enum CompletionItemTag {
    Deprecated = 1i64,
}
#[repr(i64)]
enum InsertTextFormat {
    PlainText = 1i64,
    Snippet = 2i64,
}
#[repr(i64)]
enum InsertTextMode {
    asIs = 1i64,
    adjustIndentation = 2i64,
}
#[repr(i64)]
enum DocumentHighlightKind {
    Text = 1i64,
    Read = 2i64,
    Write = 3i64,
}
enum CodeActionKind {
    Empty,
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
    Source,
    SourceOrganizeImports,
    SourceFixAll,
}
enum TraceValues {
    Off,
    Messages,
    Verbose,
}
enum MarkupKind {
    PlainText,
    Markdown,
}
enum PositionEncodingKind {
    UTF8,
    UTF16,
    UTF32,
}
#[repr(i64)]
enum FileChangeType {
    Created = 1i64,
    Changed = 2i64,
    Deleted = 3i64,
}
#[repr(i64)]
enum WatchKind {
    Create = 1i64,
    Change = 2i64,
    Delete = 4i64,
}
#[repr(i64)]
enum DiagnosticSeverity {
    Error = 1i64,
    Warning = 2i64,
    Information = 3i64,
    Hint = 4i64,
}
#[repr(i64)]
enum DiagnosticTag {
    Unnecessary = 1i64,
    Deprecated = 2i64,
}
#[repr(i64)]
enum CompletionTriggerKind {
    Invoked = 1i64,
    TriggerCharacter = 2i64,
    TriggerForIncompleteCompletions = 3i64,
}
#[repr(i64)]
enum SignatureHelpTriggerKind {
    Invoked = 1i64,
    TriggerCharacter = 2i64,
    ContentChange = 3i64,
}
#[repr(i64)]
enum CodeActionTriggerKind {
    Invoked = 1i64,
    Automatic = 2i64,
}
enum FileOperationPatternKind {
    file,
    folder,
}
#[repr(i64)]
enum NotebookCellKind {
    Markup = 1i64,
    Code = 2i64,
}
enum ResourceOperationKind {
    Create,
    Rename,
    Delete,
}
enum FailureHandlingKind {
    Abort,
    Transactional,
    TextOnlyTransactional,
    Undo,
}
#[repr(i64)]
enum PrepareSupportDefaultBehavior {
    Identifier = 1i64,
}
enum TokenFormat {
    Relative,
}
type Definition = _7d00f70d7c70a0a2fc97c00cf98aaffa2116c600cedc4680d0e2826f;
type DefinitionLink = LocationLink;
type LSPArray = Vec<LSPAny>;
type LSPAny = _ba101a3798c9f0f62231075cfbee9ed781238babbb2a538f97e41e10;
type Declaration = _593dc727e9f6426a65edf3cc967071489c5e33d7ba8ac69c517780ae;
type DeclarationLink = LocationLink;
type InlineValue = _b2a503fde3d06d3a4ace49fbb762ff9adab60a96f568ecdd04bd8631;
type DocumentDiagnosticReport = _2a42f4e83d6483635081daa6bf3fdbe7dca1486c19fe7895ec86c7c6;
type PrepareRenameResult = _fba93686f5a56329c6d0f516609839553e7b6c1a3fda79e23d4bff52;
type ProgressToken = _ca720ec3ac8719ab1592e127cdfde53da10e7f1455b874501a37d732;
type DocumentSelector = Vec<DocumentFilter>;
type ChangeAnnotationIdentifier = String;
type WorkspaceDocumentDiagnosticReport = _41e6cfaaca3360e543cf714742a7421f5b5df079b8984285e0a4b31f;
type TextDocumentContentChangeEvent = _2011ab7bc3bde22e3caa415e54c859e382f3337bbe445f969c496b36;
type MarkedString = _8a11f9b93ae449d085ef0e8b9314e8301351317a08020a3f3ddc6cbf;
type DocumentFilter = _c822a55395a19f4050cc4fcc03f6ad568cab0d855de31f9ac74c87f6;
type GlobPattern = _cc37b2d840f6c78b854d2bb23520f3a9eea1342ed9fe532d5acf7c91;
type TextDocumentFilter = _68055c228d7044947d80776ce1dfaef481bd941cdc5e6afaa1984d84;
type NotebookDocumentFilter = _a161ce038f036e4c63f9edd54e16304491db88c580b564cb758c09e5;
type Pattern = String;
enum _a056750a60f1952caf9266e34e49b31c360e6a1f1807ab19a4cb9e3c {
    _0(DocumentSelector),
    _1(()),
}
enum _7191064c973478b80fc8c60220997a16961f2f6a6fd3733edfb8722a {
    _0(TextDocumentEdit),
    _1(CreateFile),
    _2(RenameFile),
    _3(DeleteFile),
}
enum _ff638392c2084be979bca73c67a9ff5ce48b4d831488b7a96d522fab {
    _0(String),
    _1(Vec<InlayHintLabelPart>),
}
enum _884cc41f89ac76d14ad276f05f61d17b0a84b1738faa7d62b8af4d08 {
    _0(String),
    _1(MarkupContent),
}
enum _3fc208fa8531e16f51fe2d6556f3d4ccf38ceff71ec52d486dcde26d {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
struct _a61bc5d7496fb1efa1e9b725864fd45cb9a1408070cc0dffd85a70c5 {
    name: String,
    version: String,
}
enum _a4685177a66e3b501e6cc62916bace54742fa85908984cd2881ed04d {
    _0(String),
    _1(Vec<String>),
}
enum _7d6bb204775151a30d94c8f3f290f43785b27796aebdfd57661b1c53 {
    _0(String),
    _1(MarkupContent),
}
enum _b9458109bbf5e70d956d873a8321de8414cc9ef003d7d317b8295e4c {
    _0(TextEdit),
    _1(InsertReplaceEdit),
}
struct _3e2b373e868b42c09d4b77ec92f0c9be680e8745ad1ebc61234242fe {
    commitCharacters: Vec<String>,
    editRange: _6270490502445dc6e513fb3d611e21ce9cc440d0b44e133ccc74a0f6,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    data: LSPAny,
}
enum _6270490502445dc6e513fb3d611e21ce9cc440d0b44e133ccc74a0f6 {
    _0(Range),
    _1(_d11db4ffa6d25baa8a8eb19ad5309bee7391cceaf724a014882d4a59),
}
struct _d11db4ffa6d25baa8a8eb19ad5309bee7391cceaf724a014882d4a59 {
    insert: Range,
    replace: Range,
}
enum _4b1a093db38b713ca100a78f933a283d4d84d2ff15a951ec34bc1dd0 {
    _0(MarkupContent),
    _1(MarkedString),
    _2(Vec<MarkedString>),
}
struct _c608c61340f31402c88067bab181725abdfedc887eade9a02cef744f {
    reason: String,
}
enum _8f2dab9edc049982d2fe47d233fbd3d0c8a543108c48ca253448ab24 {
    _0(Location),
    _1(_463a33b810a2a9c977e3a4fa000d946d30534cbd203ecfeb2982e4a6),
}
struct _463a33b810a2a9c977e3a4fa000d946d30534cbd203ecfeb2982e4a6 {
    uri: String,
}
enum _46876517743bdc4dc23598ff397bfe4060b99be6b3cdacdf95e7b692 {
    _0(i64),
    _1(String),
}
enum _b570f9f1b1bd0545adafef02995449a937f718c9e07ea93a7067e91e {
    _0(bool),
    _1(_6dfe5f6209afd6de59c91109468aae36b44db38482379e9711cb5e10),
}
struct _6dfe5f6209afd6de59c91109468aae36b44db38482379e9711cb5e10 {}
enum _3d62efa3984910a9decfd267ff8cc3a28d8ef7ccdb27012e13a4b196 {
    _0(bool),
    _1(_cfdc059e86f529182ac373eb120495defc57dd6417461f294c02cea8),
}
struct _cfdc059e86f529182ac373eb120495defc57dd6417461f294c02cea8 {
    delta: bool,
}
enum _86c355212a6ac4cde2daeb9e4abacb35470d8ddc2f09f5603c45eab5 {
    _0(TextEdit),
    _1(AnnotatedTextEdit),
}
enum _55c44133dddc6460f9bd95e7bd4ae1954b6fdc3ac0a277ceaf7aff03 {
    _0(String),
    _1(MarkupContent),
}
enum _6d4d7397e719a0c242e2443276252a12751959ca3ff66421fd91f902 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
enum _da4007d8065e0cfbc8daf1fc291e41264f9c9966c8bf1ee89ad11858 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
struct _8686c18ca013eba49728040785b6375a8f875efe117e7f99d479300b {
    structure: _18f28dadf6726dfc06292ac2befe882cc5fa11059a6d6224f6b6cb75,
    data: Vec<NotebookCell>,
    textContent: Vec<_c6d8c106086edd9dde195ea15eace30e44cb9d25b0772bbc021e7d2d>,
}
struct _18f28dadf6726dfc06292ac2befe882cc5fa11059a6d6224f6b6cb75 {
    array: NotebookCellArrayChange,
    didOpen: Vec<TextDocumentItem>,
    didClose: Vec<TextDocumentIdentifier>,
}
struct _c6d8c106086edd9dde195ea15eace30e44cb9d25b0772bbc021e7d2d {
    document: VersionedTextDocumentIdentifier,
    changes: Vec<TextDocumentContentChangeEvent>,
}
enum _4741baf77e5c28311ca100a74e2b7a42263c81f17fbf1de0cb17ed7f {
    _0(i64),
    _1(()),
}
struct _cbb7ad32da0385d751c075c64aa8156298bac14e31f888175df60a65 {
    name: String,
    version: String,
}
enum _4e0c08e9e816138aef047f8aa74a40d6c847d63128617e9a29c49f6f {
    _0(String),
    _1(()),
}
enum _c125ac7b3b9c1b26f77c2d8738f29748ce6a58ed7880cdcdbf8c5765 {
    _0(String),
    _1(()),
}
enum _5e1fd87b1246e9ef8ff39c1b0e0a45421dd55d5766381d4dc899a0cc {
    _0(()),
    _1(()),
    _2(()),
    _3(()),
}
enum _5868a169622a556bfcf7731695edfdb33b898f01f9d0350a52a77f7e {
    _0(Vec<WorkspaceFolder>),
    _1(()),
}
enum _b1df196d752f1c7feb0315def8e74b71e062b97ea211598052596d7c {
    _0(TextDocumentSyncOptions),
    _1(TextDocumentSyncKind),
}
enum _595182541430ce6a312c274105fa59a127c14d1d17a1e25c6c129d39 {
    _0(NotebookDocumentSyncOptions),
    _1(NotebookDocumentSyncRegistrationOptions),
}
enum _a0c90c4ed2f85bdf6d6ade31475a91c2b6e528f3cdf3a6fce7c28c48 {
    _0(bool),
    _1(HoverOptions),
}
enum _04317f8f91f555b30cf0fe639c4a6cf89998bfa11af1e020f565b7e1 {
    _0(bool),
    _1(DeclarationOptions),
    _2(DeclarationRegistrationOptions),
}
enum _726064a7dd875570b101a00559dbbbea4e4fb7e7f397095f7a2f9a80 {
    _0(bool),
    _1(DefinitionOptions),
}
enum _3d304a6b55694e7e08b55b6ad74e76b9269f6685c1c790d771c20967 {
    _0(bool),
    _1(TypeDefinitionOptions),
    _2(TypeDefinitionRegistrationOptions),
}
enum _9a04310084e02a69ec04c1e6a6c574e184bfff6a77584f33cd67d8b0 {
    _0(bool),
    _1(ImplementationOptions),
    _2(ImplementationRegistrationOptions),
}
enum _f0cf0d87d16d412504c9a4cb798aca07129307ac35d239b74863cba5 {
    _0(bool),
    _1(ReferenceOptions),
}
enum _e222f1036626f9d16353835dc3d53219c5926f19b8de60f19287d8b1 {
    _0(bool),
    _1(DocumentHighlightOptions),
}
enum _e96adf8509731b7d08ac976b369b7f37a66fb17996416ea183ebcf66 {
    _0(bool),
    _1(DocumentSymbolOptions),
}
enum _02a8ba13640b991b671fbd9c60c0c32b0a42a45f2f515782fce974f2 {
    _0(bool),
    _1(CodeActionOptions),
}
enum _26c0166cf265412c6232bcd7db1a3bf9a814bf0ca902c7e9c0ca852d {
    _0(bool),
    _1(DocumentColorOptions),
    _2(DocumentColorRegistrationOptions),
}
enum _6d1693fd1d58ffd1345642cebc8631754b1956701e4538bb175e8ee0 {
    _0(bool),
    _1(WorkspaceSymbolOptions),
}
enum _e6fe21d118d621ee9bd07de8bd6f25de4738ea2eebfff67bd3b93c86 {
    _0(bool),
    _1(DocumentFormattingOptions),
}
enum _7b91c96dbdd4cf7328bb6b714c2e9183af61d6cc08396f4dc4bae568 {
    _0(bool),
    _1(DocumentRangeFormattingOptions),
}
enum _aa9bf9fdb1f89b77538d1909dfa7cfd192a75e91547501ba395ba7d3 {
    _0(bool),
    _1(RenameOptions),
}
enum _e759e7a403d62cb3109d326e3c424b596eccb1a9e99bc7c3693c9513 {
    _0(bool),
    _1(FoldingRangeOptions),
    _2(FoldingRangeRegistrationOptions),
}
enum _505a646dc962718cb3169c822e33a538d79d540e58cf080545dbf9ae {
    _0(bool),
    _1(SelectionRangeOptions),
    _2(SelectionRangeRegistrationOptions),
}
enum _48fb1fa3da14d65578b5d664c9158ae81311eefb97acc98fa0e31b98 {
    _0(bool),
    _1(CallHierarchyOptions),
    _2(CallHierarchyRegistrationOptions),
}
enum _734c41cfb5333dab38738128fcb520d279027501d1639841dd1083fa {
    _0(bool),
    _1(LinkedEditingRangeOptions),
    _2(LinkedEditingRangeRegistrationOptions),
}
enum _a23047d956bbb441b6f0c388d6f70c57ad2a259fc66e89fd607cd974 {
    _0(SemanticTokensOptions),
    _1(SemanticTokensRegistrationOptions),
}
enum _8702b8c310a621de29bb724ae324d822ff769d4055a37a01da756c38 {
    _0(bool),
    _1(MonikerOptions),
    _2(MonikerRegistrationOptions),
}
enum _9b377653dfd0cac2b3c93d4227eed3e9735cb45cdeb84ef0b81c4a5f {
    _0(bool),
    _1(TypeHierarchyOptions),
    _2(TypeHierarchyRegistrationOptions),
}
enum _0cab083e9333223d5b76b08ac7d5aefac1820a3af21f1f5a414d2a9b {
    _0(bool),
    _1(InlineValueOptions),
    _2(InlineValueRegistrationOptions),
}
enum _531b32e5541cdc8ec6bc8aa8e0ed39d106f3bc108195c096b6106e9f {
    _0(bool),
    _1(InlayHintOptions),
    _2(InlayHintRegistrationOptions),
}
enum _4e24549c800892ecca8f3ea185df09e2fed5062e9cd529e33667d9d0 {
    _0(DiagnosticOptions),
    _1(DiagnosticRegistrationOptions),
}
struct _5418b79a89f5cc28e7a102d33ce199381b6f1e9bf0bdb3f46968c229 {
    workspaceFolders: WorkspaceFoldersServerCapabilities,
    fileOperations: FileOperationOptions,
}
enum _18d7e36fdbc505296f71e2f93b82efebeeb9426e15069f2ab43ba542 {
    _0(i64),
    _1(String),
}
struct _688088efd1b14986d98f20cf44230f728f6e1a914b331ee9a6126f4f {
    labelDetailsSupport: bool,
}
enum _664ca25c59505bd4079b05c93b742c52fe16fc98f1c8e5a3a59ab064 {
    _0(String),
    _1(MarkupContent),
}
enum _e911abe98649132bc4f7460d1ede2881cb9018c613e2d7d167c8ca90 {
    _0(i64),
    _1(()),
}
enum _e5075c3e3fc87f4559c38c96edfac0a0f57e50eb51032e5fcacd5dfc {
    _0(i64),
    _1(()),
}
enum _eb3c8858872159820455156ca8344d61c40c1fe79e60d9c500ec5736 {
    _0(i64),
    _1(()),
}
enum _63cb47a5952dcc1e06bf042e05f7d29db500a5d7acae00fc07d4dd8f {
    _0(bool),
    _1(SaveOptions),
}
enum _f0494010a863ad5b5c35127a6170fa4f5109dbe797c8c290bfa69502 {
    _0(_a7967a8e9bd9e3ca10a628c28972dc3f293580d0955670cdf7859c06),
    _1(_599188607bc2e8702d74b42d3f32265b2f410a378a132a4210adb901),
}
struct _a7967a8e9bd9e3ca10a628c28972dc3f293580d0955670cdf7859c06 {
    notebook: _d40ce8f249ca116bbfdc84dfc555ca14fd4b17b950a869e9dfa22038,
    cells: Vec<_f54578ee99eb534d0d668bb39e1da7c07c7b0be407d8030aafa66f37>,
}
enum _d40ce8f249ca116bbfdc84dfc555ca14fd4b17b950a869e9dfa22038 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _f54578ee99eb534d0d668bb39e1da7c07c7b0be407d8030aafa66f37 {
    language: String,
}
struct _599188607bc2e8702d74b42d3f32265b2f410a378a132a4210adb901 {
    notebook: _794c5496bd7b3212921ae2c322fd2c13d401881abbf556ccdd466b48,
    cells: Vec<_18d5f395024f5f91722be644af56d599154b980e8627901dd4c2725a>,
}
enum _794c5496bd7b3212921ae2c322fd2c13d401881abbf556ccdd466b48 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _18d5f395024f5f91722be644af56d599154b980e8627901dd4c2725a {
    language: String,
}
enum _fe81a511bd7bdf218978e741051f1f7d417ffb3ad154f24e718e1390 {
    _0(String),
    _1(bool),
}
enum _a6d429ebad4d0c5cb627a44498b1513a14e0f6597fb2b59ec3f05120 {
    _0(String),
    _1((u64, u64)),
}
enum _f6c860f8fae52697f4faa203922b9c62d994532ee6667fedbd642380 {
    _0(String),
    _1(MarkupContent),
}
enum _be533a4f33e057440c4f64a408d63f6c22aa65744e3102f8d12875c6 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _003415e244d2612d789a19dab8421b511187b3755b9473930b674d12 {
    cancel: bool,
    retryOnContentModified: Vec<String>,
}
enum _55e35d0ecaaf41c17687df82ca01b08921e446491ab4731af65fe581 {
    _0(WorkspaceFolder),
    _1(String),
}
struct _b6707bcef1be5e545e98099e53ea44f18347faf75ed7d7edf6d44ef0 {
    groupsOnLabel: bool,
}
struct _0d8e0f734067929cc8e5b9d0da8d1d3f54e69aa40241215a69208b1f {
    valueSet: Vec<SymbolKind>,
}
struct _591e4745558d2dc75c1523b59943c321d24eabd0847dddfbd9af505e {
    valueSet: Vec<SymbolTag>,
}
struct _b9b6711c52b95e4c40da4d25dfaa2e5c9f540bbb7739fbfcc6b56568 {
    properties: Vec<String>,
}
struct _abed05221375ee6f658341ceb8a2fd3422dd58eefa7cf4603ab2244d {
    snippetSupport: bool,
    commitCharactersSupport: bool,
    documentationFormat: Vec<MarkupKind>,
    deprecatedSupport: bool,
    preselectSupport: bool,
    tagSupport: _c247e21d7db40ebcf5d8c61ce12e079ca2e5f2dd1c3d5dac8af88712,
    insertReplaceSupport: bool,
    resolveSupport: _3a2cfe89bd7eaabd54691d111f9ccce617a24a8b4353a860b3140d08,
    insertTextModeSupport: _7c5fbc0302d62079da34aab0ae5d73d3fdee6f4ec96f684477ba00aa,
    labelDetailsSupport: bool,
}
struct _c247e21d7db40ebcf5d8c61ce12e079ca2e5f2dd1c3d5dac8af88712 {
    valueSet: Vec<CompletionItemTag>,
}
struct _3a2cfe89bd7eaabd54691d111f9ccce617a24a8b4353a860b3140d08 {
    properties: Vec<String>,
}
struct _7c5fbc0302d62079da34aab0ae5d73d3fdee6f4ec96f684477ba00aa {
    valueSet: Vec<InsertTextMode>,
}
struct _f9a77c24bb228bd169055f2a5897495d25038ea2f0416d17a90e2452 {
    valueSet: Vec<CompletionItemKind>,
}
struct _c97efab1d6dddb53f99b49656a2402f88a4737325119a6483e2b5db5 {
    itemDefaults: Vec<String>,
}
struct _ca746e2f1bb1fdee9b60dfe2e4a60af75e7fe3ec193521018781c496 {
    documentationFormat: Vec<MarkupKind>,
    parameterInformation: _cf048e4a7d221ae07a48ad4675710dd0e3ecba292360dd6a2ca90b0b,
    activeParameterSupport: bool,
}
struct _cf048e4a7d221ae07a48ad4675710dd0e3ecba292360dd6a2ca90b0b {
    labelOffsetSupport: bool,
}
struct _2e39f4b7aeaa9e12fc785358cdaf981e9fa1f74c27deeef45fa32aa0 {
    valueSet: Vec<SymbolKind>,
}
struct _a103d42a15a2985aa612bb759666278a5ce67d2a782dd5e1e3271823 {
    valueSet: Vec<SymbolTag>,
}
struct _f58e6e95aac5dafb26357ad6a6d171269bfce7b2ddc435a2f6f057bc {
    codeActionKind: _46f12128952cde5cb4c35d71a1a02c1f5015570732655f9208379222,
}
struct _46f12128952cde5cb4c35d71a1a02c1f5015570732655f9208379222 {
    valueSet: Vec<CodeActionKind>,
}
struct _b5e7a4ebd7246f63611beec6ae6a7ecc6bee967c3b921da16371bfc0 {
    properties: Vec<String>,
}
struct _6de36e91868f2b78b3e86b8f4028da34d8457ce95ca8f9ca5842c0db {
    valueSet: Vec<FoldingRangeKind>,
}
struct _e508b892eb91775d344b084cb01a1f3f32162e3cd19294e8a7962903 {
    collapsedText: bool,
}
struct _1b0c8a8bf5ec254687aa4e85350349aa8e1d874454ce6e58d4ad3c81 {
    valueSet: Vec<DiagnosticTag>,
}
struct _76956417a800c7206b54add57d83af24eae22079e702e05e38173c6b {
    range: _0b9cc544c4d3fdadd71602916cc5d868aabafbc43c14378c06469fe1,
    full: _cad75675432c4b06a64252cd5129cdc771fba324cfd66ec492d306fa,
}
enum _0b9cc544c4d3fdadd71602916cc5d868aabafbc43c14378c06469fe1 {
    _0(bool),
    _1(_e7f2f8efe37078238932640f47f01656c881c58a7be92775183feb5a),
}
struct _e7f2f8efe37078238932640f47f01656c881c58a7be92775183feb5a {}
enum _cad75675432c4b06a64252cd5129cdc771fba324cfd66ec492d306fa {
    _0(bool),
    _1(_e74326b5b465bcd7e98a73490f0dbbd29fe9353c6b330887a9d47a2b),
}
struct _e74326b5b465bcd7e98a73490f0dbbd29fe9353c6b330887a9d47a2b {
    delta: bool,
}
struct _598578991bb319e2e390858fa8230eb695b82100ae440f642ccb9968 {
    properties: Vec<String>,
}
struct _310d9d6f7229eea387894a6e9eeefb6de5b6a34f5bb997c33ffd34e5 {
    additionalPropertiesSupport: bool,
}
enum _7d00f70d7c70a0a2fc97c00cf98aaffa2116c600cedc4680d0e2826f {
    _0(Location),
    _1(Vec<Location>),
}
enum _ba101a3798c9f0f62231075cfbee9ed781238babbb2a538f97e41e10 {
    _0(LSPObject),
    _1(LSPArray),
    _2(String),
    _3(i64),
    _4(u64),
    _5(f64),
    _6(bool),
    _7(()),
}
enum _593dc727e9f6426a65edf3cc967071489c5e33d7ba8ac69c517780ae {
    _0(Location),
    _1(Vec<Location>),
}
enum _b2a503fde3d06d3a4ace49fbb762ff9adab60a96f568ecdd04bd8631 {
    _0(InlineValueText),
    _1(InlineValueVariableLookup),
    _2(InlineValueEvaluatableExpression),
}
enum _2a42f4e83d6483635081daa6bf3fdbe7dca1486c19fe7895ec86c7c6 {
    _0(RelatedFullDocumentDiagnosticReport),
    _1(RelatedUnchangedDocumentDiagnosticReport),
}
enum _fba93686f5a56329c6d0f516609839553e7b6c1a3fda79e23d4bff52 {
    _0(Range),
    _1(_f2737a9b8eade6f6009d0e825331b043d651c57c921241b5ae264b55),
    _2(_8b71ef301f710841d62568b8f3c859367838614d32febbc2ffeed67e),
}
struct _f2737a9b8eade6f6009d0e825331b043d651c57c921241b5ae264b55 {
    range: Range,
    placeholder: String,
}
struct _8b71ef301f710841d62568b8f3c859367838614d32febbc2ffeed67e {
    defaultBehavior: bool,
}
enum _ca720ec3ac8719ab1592e127cdfde53da10e7f1455b874501a37d732 {
    _0(i64),
    _1(String),
}
enum _41e6cfaaca3360e543cf714742a7421f5b5df079b8984285e0a4b31f {
    _0(WorkspaceFullDocumentDiagnosticReport),
    _1(WorkspaceUnchangedDocumentDiagnosticReport),
}
enum _2011ab7bc3bde22e3caa415e54c859e382f3337bbe445f969c496b36 {
    _0(_0194b2158e43870f9ed1729c0be737bb9989612ac67eb4d3c0f6c1b1),
    _1(_6cbcf4df659a6abc9d7ad0e6b43bab8f1a05681482a58b133f297138),
}
struct _0194b2158e43870f9ed1729c0be737bb9989612ac67eb4d3c0f6c1b1 {
    range: Range,
    rangeLength: u64,
    text: String,
}
struct _6cbcf4df659a6abc9d7ad0e6b43bab8f1a05681482a58b133f297138 {
    text: String,
}
enum _8a11f9b93ae449d085ef0e8b9314e8301351317a08020a3f3ddc6cbf {
    _0(String),
    _1(_988a26ef30ae52e58a75837d07f858145fe8d2d0789ef87b69903d3f),
}
struct _988a26ef30ae52e58a75837d07f858145fe8d2d0789ef87b69903d3f {
    language: String,
    value: String,
}
enum _c822a55395a19f4050cc4fcc03f6ad568cab0d855de31f9ac74c87f6 {
    _0(TextDocumentFilter),
    _1(NotebookCellTextDocumentFilter),
}
enum _cc37b2d840f6c78b854d2bb23520f3a9eea1342ed9fe532d5acf7c91 {
    _0(Pattern),
    _1(RelativePattern),
}
enum _68055c228d7044947d80776ce1dfaef481bd941cdc5e6afaa1984d84 {
    _0(_a5c0dc11809667513876076647999c7681d937005a0ee416a2199309),
    _1(_dd376675a079e7b3105bb428be2f1370a24aa597daafe00400fe98f3),
    _2(_af5eee3314105b5316d5569ed5cbc0a0da2540d70ef23cf2839bcffc),
}
struct _a5c0dc11809667513876076647999c7681d937005a0ee416a2199309 {
    language: String,
    scheme: String,
    pattern: String,
}
struct _dd376675a079e7b3105bb428be2f1370a24aa597daafe00400fe98f3 {
    language: String,
    scheme: String,
    pattern: String,
}
struct _af5eee3314105b5316d5569ed5cbc0a0da2540d70ef23cf2839bcffc {
    language: String,
    scheme: String,
    pattern: String,
}
enum _a161ce038f036e4c63f9edd54e16304491db88c580b564cb758c09e5 {
    _0(_884a48069b11155b64329846e13fe00f66ffe3d9203bf5eab782f4cd),
    _1(_74a75563be555ab1c2f75bd8f0bf30cc397ae9d4dafa8713819c9823),
    _2(_c06b72935257b2230afd45ee6ecaf4e391c9e12886b04497a88b981c),
}
struct _884a48069b11155b64329846e13fe00f66ffe3d9203bf5eab782f4cd {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct _74a75563be555ab1c2f75bd8f0bf30cc397ae9d4dafa8713819c9823 {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct _c06b72935257b2230afd45ee6ecaf4e391c9e12886b04497a88b981c {
    notebookType: String,
    scheme: String,
    pattern: String,
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
