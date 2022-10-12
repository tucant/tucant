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
    documentSelector: _68fa36e739355dbaacdab669b64958dd521643d624bfecb524bb521d,
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
    documentChanges: Vec<_d316ca87d56770e1fb51709701280e04c968bd2b47eb09d223b9339e>,
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
    label: _751ee485e6058c4a3080ecf3b6a5dbd752ed2a3b7e33a2d44671cce9,
    kind: InlayHintKind,
    textEdits: Vec<TextEdit>,
    tooltip: _e6ce090e0ce82c31aa349477d96bcf027a004f1e906c7765330dbaa3,
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
        _a417e708932295d6eb768b35905743d2e0e383aac9387e48ddc81628,
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
    serverInfo: _6c38f7e9bdc0577eee61096e635dfe0c31d40892d1497ee6a4457de2,
}
struct InitializeError {
    retry: bool,
}
struct InitializedParams {}
struct DidChangeConfigurationParams {
    settings: LSPAny,
}
struct DidChangeConfigurationRegistrationOptions {
    section: _a277b7c03c53d0976fb96386b96e45b135493642e367f35cbc5680f2,
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
    documentation: _1b24d619712f37105138d63e3d332dd30e999ec34a4642d87a776ece,
    deprecated: bool,
    preselect: bool,
    sortText: String,
    filterText: String,
    insertText: String,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    textEdit: _1ef7d287a95b518f1021fd5e49a8564690bf2abc558b2d08e41df1b8,
    textEditText: String,
    additionalTextEdits: Vec<TextEdit>,
    commitCharacters: Vec<String>,
    command: Command,
    data: LSPAny,
}
struct CompletionList {
    isIncomplete: bool,
    itemDefaults: _b95593e6eeb1be1400618336cf68e8c3d2a9773d4484926eb5b22f8d,
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
    contents: _2c02e660313ada299ebf76e63b95d3a768c9ff69c4dc6dd559acb3df,
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
    disabled: _738afd3d6064ba2737db9e6541aec6fd02766da7e1860885d95f690a,
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
    location: _ae39669dfca05496888a44793a7df1c73226d812c0cf99ac4f6079dc,
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
    id: _fd1a5d69b8b283895197b531d243834eaf7f448805ef1be6b27ef9cc,
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
    range: _ddce9adec03ee09dcefd9d61ca98f404bad9213ab6c0492fb5f1237e,
    full: _7dbb9ca7339c6c34a1c9ca7220bb0d4af6e38a158abcd2262be7e928,
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
    edits: Vec<_8bdf1b951e3a8afce3396e3f5b634c708d142d3d459c0f49a0196221>,
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
    tooltip: _d45096c1324bae5cf55c81d8ad3e42f796f03f6d916e6283b0617eee,
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
        _7c1b1bd8ba3c43359806a0a90dd3229de05e78614e2ef430940b8b63,
    >,
}
struct RelatedUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    relatedDocuments: ::std::collections::HashMap<
        String,
        _6aeb20f5ac8163eb6c23eb08bb4df9e04959dbf8714bbf43a10be36f,
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
    cells: _54bc7a8d7a339e558af074c9c94906ecd237c5052ed52570baade9df,
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
    processId: _bd913a9cfc85e2e5f035f0f629a1991dc6a2850b0ad07a26e326fd55,
    clientInfo: _69e6df47420eb5a96e903e9e70150c252edaee2efb8b8b209d578e88,
    locale: String,
    rootPath: _93a3801d0a8292c67ecc842b50d0d7803a97c8817951647bc58ff33b,
    rootUri: _7b65ddc2e8568fc8b47da61b56cd404e6c93d60c066e95b782af74dc,
    capabilities: ClientCapabilities,
    initializationOptions: LSPAny,
    trace: _a0ec5881bf7050c80126c6c9fd17971d2c386fc72655ced79d71e7b9,
}
struct WorkspaceFoldersInitializeParams {
    workspaceFolders: _061733b995206fae495f8a60e64b625677c7e0e63c326b9aa0021bdd,
}
struct ServerCapabilities {
    positionEncoding: PositionEncodingKind,
    textDocumentSync: _a1da2ac2201da8aec8bc08f049af268588ae8da25166f4f436018348,
    notebookDocumentSync: _099c1faba4d82fb350f1e3ac359401eaf8da46ddef958f09edafc8a7,
    completionProvider: CompletionOptions,
    hoverProvider: _32bd01d1063cd618e90217ddbd3a867ea33157a8676c650f7affcbad,
    signatureHelpProvider: SignatureHelpOptions,
    declarationProvider: _403b1f7cc45a4cbd98b7982fe5a3137f9e2857118383b2b2d5b5b947,
    definitionProvider: _f98aa58f3d08ec32d2169a6aebb8fe503948cc64c5fdf5ba91289364,
    typeDefinitionProvider: _426edf92759ea45579ad28a3c9ec9cbdc1e7fad0a1c0f62679515a31,
    implementationProvider: _8786a8aa710ddf0eb946c9afab881ee83e63b36caf55b6711d2b3103,
    referencesProvider: _cc7094674fb13a8ba6177bb9fcf0cd1a1fc45e4a255b5e19dfa739b5,
    documentHighlightProvider: _5d32d6f8860bdee46b17c2e694e480a9c681fdccf2773dd0b7fab26d,
    documentSymbolProvider: _a11ee373a536ab53d1e2bc6798c0311459b1d2b4cf9898d15bc31047,
    codeActionProvider: _129c96747e4d995441b6833e01edba337095fee4b66ab8a5d8992e31,
    codeLensProvider: CodeLensOptions,
    documentLinkProvider: DocumentLinkOptions,
    colorProvider: _3d04a1d707f9667cb3b870456513a05d745618ca6b166a963551e4e0,
    workspaceSymbolProvider: _f1792ec3eafccc78af8184bb4dd68af8ab0f7cc04aea1487ab0a6d5a,
    documentFormattingProvider: _2d1a7c58062d48b7b5a065e8898d849cad85da7f16ceca11201ac4af,
    documentRangeFormattingProvider: _2b09ea7863df648baf74b21df97a624f431183136622fab32af3c64a,
    documentOnTypeFormattingProvider: DocumentOnTypeFormattingOptions,
    renameProvider: _d8b21620c6312d0b53404243370e472a32f548369fcbbfc8d58dc685,
    foldingRangeProvider: _031dd2726761a8bc088275c635f1618906da1c7dbcd48d5fe3df09c5,
    selectionRangeProvider: _4ba79ac2d718992e318323f5ab1860ef4c8a5d916dfc2b39bc644819,
    executeCommandProvider: ExecuteCommandOptions,
    callHierarchyProvider: _e7d9e7e0e21b19df7f722c7d98c88a762bec346ae8fe61bfc078fe3f,
    linkedEditingRangeProvider: _1c48fa58e01da04651803c604b74282edebe7b4f0b03741220f89ccc,
    semanticTokensProvider: _72fce9038efd721dd154b70c8cfb6e119d478e45f4a939ece93b8327,
    monikerProvider: _85b731e8423281c4c8592e6f02035a1c257a2fa75b6768fc7e7e9e78,
    typeHierarchyProvider: _9bbfb4decf91cc65a10244b1b2836700a89232e1d54bd4304403eeb7,
    inlineValueProvider: _d92f343851708f52dda572d9841a07ca2814d6033d2e906406fbf5f5,
    inlayHintProvider: _da8d06850fdfa19720b20442666513d24758232f26587bbdd2a3a343,
    diagnosticProvider: _b1a07f155097055ab647400c3de5f7ae28c7231df90aab0e525aaa6b,
    workspace: _bf7c1c869b776bad598568b38e429a1cc43dc08c5dfd76a9d8efff32,
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
    code: _006fb4475b4ff9a90877437b985df22b6d586a1731f1fd73e7559507,
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
    completionItem: _9af5298e8dced3352336503d822b6f1a0eb5b909e55afb10af2daa26,
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
    documentation: _aa51fd5e33e9172cc101fce84ba761d87c3b97dfce12cdf30b3b1019,
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
    version: _5a69ec5a684cfb83fdb25959715a7e6206122e3238d684442afb8cbb,
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
    version: _884abd6ac67c2478eafd1bb8fba27aa5602aae62963ae2e258acb73b,
}
struct WorkspaceUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    uri: String,
    version: _e098d24419db7c07f468925116bfbce9cba3a2bceb77c5cd71024c9d,
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
    save: _83b26d247e1a8ce4797a761158286940d3cf8c61471bd4e59c795d74,
}
struct NotebookDocumentSyncOptions {
    notebookSelector: Vec<_b7c1b78c415a3475e551a97f01add12a57224597b978edc52216984c>,
    save: bool,
}
struct NotebookDocumentSyncRegistrationOptions {
    _0: NotebookDocumentSyncOptions,
}
struct WorkspaceFoldersServerCapabilities {
    supported: bool,
    changeNotifications: _3d9835b70cf511ded6ec9b7fe4f8c936ba201c39f914af9ab3c939c6,
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
    label: _77558c401409214b4a1b585768dde1d28d09073a190aaffbd570009a,
    documentation: _204ca5852233da483217023e2a6d35a3abcfb2e54b51cd7938289c24,
}
struct NotebookCellTextDocumentFilter {
    notebook: _e18cf742e0b0fca21da58a1c7b1a35d6c35258edda57d4dcd2d56a8d,
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
    staleRequestSupport: _a60a812d65fdd530de0650b83427b9c84506142ff6dbf3f6ac26f375,
    regularExpressions: RegularExpressionsClientCapabilities,
    markdown: MarkdownClientCapabilities,
    positionEncodings: Vec<PositionEncodingKind>,
}
struct RelativePattern {
    baseUri: _44a298919024d4825ab5ef16d29896b4b291132f5423a9b063d716ab,
    pattern: Pattern,
}
struct WorkspaceEditClientCapabilities {
    documentChanges: bool,
    resourceOperations: Vec<ResourceOperationKind>,
    failureHandling: FailureHandlingKind,
    normalizesLineEndings: bool,
    changeAnnotationSupport: _80ff7a3a91805e2c196dedd672e0e74c45b4a0b81ba20f42a14df3c9,
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
    symbolKind: _25e26cd554ad7748b6feadbce7b8835c6f23fd2ef30bedb38ecf7c74,
    tagSupport: _00c7513e9c0e894c4c9713f4ebfed242b614eca929bb5e54782ebd4d,
    resolveSupport: _788efc372fd9ddaf4606940980cf09f0e827938991eeccaf0904faf1,
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
    completionItem: _ca833dea8086c0ff4807a64c049f35cab65f187fd96d5c38e04ddbcc,
    completionItemKind: _64f1a75d05d50427a8afd5ffb15d5fe6f63685b9b62ec107956061ad,
    insertTextMode: InsertTextMode,
    contextSupport: bool,
    completionList: _e38e4f889b2c19ef8a2ad97c845a3b80f52e28c807e52bbd69a7bb7c,
}
struct HoverClientCapabilities {
    dynamicRegistration: bool,
    contentFormat: Vec<MarkupKind>,
}
struct SignatureHelpClientCapabilities {
    dynamicRegistration: bool,
    signatureInformation: _63274311fcea199783cccaf1bf3bc698bc4f60596136ce75128310ac,
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
    symbolKind: _6f0eb61bfd957698265aa2d14a9fb79f28326e7d3530158c04ed6be9,
    hierarchicalDocumentSymbolSupport: bool,
    tagSupport: _05182c04a77bbe359b46ec93f36fb4f88d526d49345089e796ef5508,
    labelSupport: bool,
}
struct CodeActionClientCapabilities {
    dynamicRegistration: bool,
    codeActionLiteralSupport: _0bfc5ade5ffa396f4a7044b6e6b6389c240c351af6c3b7656fbfae86,
    isPreferredSupport: bool,
    disabledSupport: bool,
    dataSupport: bool,
    resolveSupport: _d95ec519131d7727f3094cdb0f211618b36fbed7141512257f6ee87f,
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
    foldingRangeKind: _6c3d69f9ac65e9ef92bf8b120524afe6d6627c4f4a3c263bfe03f21f,
    foldingRange: _3ed96d32fe84cf5f4a81bd30fc7985d19402ef8020ab7c709dd7cc97,
}
struct SelectionRangeClientCapabilities {
    dynamicRegistration: bool,
}
struct PublishDiagnosticsClientCapabilities {
    relatedInformation: bool,
    tagSupport: _125da3d4f10d2b5c11b42c01844c361366f0014385ce9cca28aaa24f,
    versionSupport: bool,
    codeDescriptionSupport: bool,
    dataSupport: bool,
}
struct CallHierarchyClientCapabilities {
    dynamicRegistration: bool,
}
struct SemanticTokensClientCapabilities {
    dynamicRegistration: bool,
    requests: _a6a7b6c565ec3b174123a44de6c950d5f578f346af0721dcfdc43bf4,
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
    resolveSupport: _0c217d234db4b668c81dbec89f9d4dbf3ba0b5973b74b7772f99860b,
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
    messageActionItem: _8ca7d318958d09d66994d30431468009ca1239a83e0153f70654f4de,
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
type Definition = _3ad603f6d622d97ac33e2c9aaa4a2bc4f15955889de71892988968d5;
type DefinitionLink = LocationLink;
type LSPArray = Vec<LSPAny>;
type LSPAny = _f49efc2d49ebb17e2fe51d773cdf89c4a088ebcb35d52f4c3838feaf;
type Declaration = _cbae34a64b57d02127dcd3ff65c0228c8b2dcdc19fe75b10a64730bb;
type DeclarationLink = LocationLink;
type InlineValue = _569e9eb50d34c916bf5037a8b8a48fe60c7f180c2891f403da138338;
type DocumentDiagnosticReport = _35040f864ae99ba2b5263239b6285dbd61d5fdf56984952fea4cff0d;
type PrepareRenameResult = _1959e16332064e210a6721995f53a8780509a6e5d8ec208a5a294181;
type ProgressToken = _ca144a177699a99188c2fe73bd4d39ed9b32b78bb97a0de1c1a1c772;
type DocumentSelector = Vec<DocumentFilter>;
type ChangeAnnotationIdentifier = String;
type WorkspaceDocumentDiagnosticReport = _ecfad5e5d9d1214299287e46d6a4b9dcd3de72911a22e62a9753726c;
type TextDocumentContentChangeEvent = _4388a7f257630b4628a8d9aaa5155d6dbc29d0918128ee154d1cca29;
type MarkedString = _78f4c5c59ed12c35e27233d57402563928d589a301e75bc6d3b9d202;
type DocumentFilter = _f49cdfd426f35bf5a4034897a623b6f50275b2bbd6a6d8c5f4482916;
type GlobPattern = _b9b11b4c3b66591928ed9304a1f283517eacdc117e786aca5dc662c5;
type TextDocumentFilter = _51c568666ea7631b14d02e28bf667335c50e5d09a06b85f01a5fe1dd;
type NotebookDocumentFilter = _3e9be7fd0f2351c0b752e1d43b99b636a6575537cdc242e2ce19171f;
type Pattern = String;
enum _68fa36e739355dbaacdab669b64958dd521643d624bfecb524bb521d {
    _0(DocumentSelector),
    _1(Null),
}
enum _d316ca87d56770e1fb51709701280e04c968bd2b47eb09d223b9339e {
    _0(TextDocumentEdit),
    _1(CreateFile),
    _2(RenameFile),
    _3(DeleteFile),
}
enum _751ee485e6058c4a3080ecf3b6a5dbd752ed2a3b7e33a2d44671cce9 {
    _0(String),
    _1(Vec<InlayHintLabelPart>),
}
enum _e6ce090e0ce82c31aa349477d96bcf027a004f1e906c7765330dbaa3 {
    _0(String),
    _1(MarkupContent),
}
enum _a417e708932295d6eb768b35905743d2e0e383aac9387e48ddc81628 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
struct _6c38f7e9bdc0577eee61096e635dfe0c31d40892d1497ee6a4457de2 {
    name: String,
    version: String,
}
enum _a277b7c03c53d0976fb96386b96e45b135493642e367f35cbc5680f2 {
    _0(String),
    _1(Vec<String>),
}
enum _1b24d619712f37105138d63e3d332dd30e999ec34a4642d87a776ece {
    _0(String),
    _1(MarkupContent),
}
enum _1ef7d287a95b518f1021fd5e49a8564690bf2abc558b2d08e41df1b8 {
    _0(TextEdit),
    _1(InsertReplaceEdit),
}
struct _b95593e6eeb1be1400618336cf68e8c3d2a9773d4484926eb5b22f8d {
    commitCharacters: Vec<String>,
    editRange: _a736497e07a20a783931d20e1cca6dd98945c1b013dac4953a3efd7f,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    data: LSPAny,
}
enum _a736497e07a20a783931d20e1cca6dd98945c1b013dac4953a3efd7f {
    _0(Range),
    _1(_b84a27fdfa2dfd7399ea69c5ef4ffa0ad49bedc59108cf5e38454585),
}
struct _b84a27fdfa2dfd7399ea69c5ef4ffa0ad49bedc59108cf5e38454585 {
    insert: Range,
    replace: Range,
}
enum _2c02e660313ada299ebf76e63b95d3a768c9ff69c4dc6dd559acb3df {
    _0(MarkupContent),
    _1(MarkedString),
    _2(Vec<MarkedString>),
}
struct _738afd3d6064ba2737db9e6541aec6fd02766da7e1860885d95f690a {
    reason: String,
}
enum _ae39669dfca05496888a44793a7df1c73226d812c0cf99ac4f6079dc {
    _0(Location),
    _1(_6611f3227d2a777dccf80e7c6736b30447d6a6b97b0f67bc9df1a1b4),
}
struct _6611f3227d2a777dccf80e7c6736b30447d6a6b97b0f67bc9df1a1b4 {
    uri: String,
}
enum _fd1a5d69b8b283895197b531d243834eaf7f448805ef1be6b27ef9cc {
    _0(i64),
    _1(String),
}
enum _ddce9adec03ee09dcefd9d61ca98f404bad9213ab6c0492fb5f1237e {
    _0(bool),
    _1(_8cb5f5032213e9fe9f14148061fd94bff35d4798c3c04f5b9ac8f676),
}
struct _8cb5f5032213e9fe9f14148061fd94bff35d4798c3c04f5b9ac8f676 {}
enum _7dbb9ca7339c6c34a1c9ca7220bb0d4af6e38a158abcd2262be7e928 {
    _0(bool),
    _1(_0e8a16b7e8f8ee7ed957addaeb64f50d364e13794e18d73ed74ea2f1),
}
struct _0e8a16b7e8f8ee7ed957addaeb64f50d364e13794e18d73ed74ea2f1 {
    delta: bool,
}
enum _8bdf1b951e3a8afce3396e3f5b634c708d142d3d459c0f49a0196221 {
    _0(TextEdit),
    _1(AnnotatedTextEdit),
}
enum _d45096c1324bae5cf55c81d8ad3e42f796f03f6d916e6283b0617eee {
    _0(String),
    _1(MarkupContent),
}
enum _7c1b1bd8ba3c43359806a0a90dd3229de05e78614e2ef430940b8b63 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
enum _6aeb20f5ac8163eb6c23eb08bb4df9e04959dbf8714bbf43a10be36f {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
struct _54bc7a8d7a339e558af074c9c94906ecd237c5052ed52570baade9df {
    structure: _86314b80fc521f10f9a4306027e62bb9e92afeac2c2d4f9f4bfeae9a,
    data: Vec<NotebookCell>,
    textContent: Vec<_f25315e1b44f2ead881683def459f724aa8a58fd2233b55f2ba09f83>,
}
struct _86314b80fc521f10f9a4306027e62bb9e92afeac2c2d4f9f4bfeae9a {
    array: NotebookCellArrayChange,
    didOpen: Vec<TextDocumentItem>,
    didClose: Vec<TextDocumentIdentifier>,
}
struct _f25315e1b44f2ead881683def459f724aa8a58fd2233b55f2ba09f83 {
    document: VersionedTextDocumentIdentifier,
    changes: Vec<TextDocumentContentChangeEvent>,
}
enum _bd913a9cfc85e2e5f035f0f629a1991dc6a2850b0ad07a26e326fd55 {
    _0(i64),
    _1(Null),
}
struct _69e6df47420eb5a96e903e9e70150c252edaee2efb8b8b209d578e88 {
    name: String,
    version: String,
}
enum _93a3801d0a8292c67ecc842b50d0d7803a97c8817951647bc58ff33b {
    _0(String),
    _1(Null),
}
enum _7b65ddc2e8568fc8b47da61b56cd404e6c93d60c066e95b782af74dc {
    _0(String),
    _1(Null),
}
enum _a0ec5881bf7050c80126c6c9fd17971d2c386fc72655ced79d71e7b9 {
    _0(()),
    _1(()),
    _2(()),
    _3(()),
}
enum _061733b995206fae495f8a60e64b625677c7e0e63c326b9aa0021bdd {
    _0(Vec<WorkspaceFolder>),
    _1(Null),
}
enum _a1da2ac2201da8aec8bc08f049af268588ae8da25166f4f436018348 {
    _0(TextDocumentSyncOptions),
    _1(TextDocumentSyncKind),
}
enum _099c1faba4d82fb350f1e3ac359401eaf8da46ddef958f09edafc8a7 {
    _0(NotebookDocumentSyncOptions),
    _1(NotebookDocumentSyncRegistrationOptions),
}
enum _32bd01d1063cd618e90217ddbd3a867ea33157a8676c650f7affcbad {
    _0(bool),
    _1(HoverOptions),
}
enum _403b1f7cc45a4cbd98b7982fe5a3137f9e2857118383b2b2d5b5b947 {
    _0(bool),
    _1(DeclarationOptions),
    _2(DeclarationRegistrationOptions),
}
enum _f98aa58f3d08ec32d2169a6aebb8fe503948cc64c5fdf5ba91289364 {
    _0(bool),
    _1(DefinitionOptions),
}
enum _426edf92759ea45579ad28a3c9ec9cbdc1e7fad0a1c0f62679515a31 {
    _0(bool),
    _1(TypeDefinitionOptions),
    _2(TypeDefinitionRegistrationOptions),
}
enum _8786a8aa710ddf0eb946c9afab881ee83e63b36caf55b6711d2b3103 {
    _0(bool),
    _1(ImplementationOptions),
    _2(ImplementationRegistrationOptions),
}
enum _cc7094674fb13a8ba6177bb9fcf0cd1a1fc45e4a255b5e19dfa739b5 {
    _0(bool),
    _1(ReferenceOptions),
}
enum _5d32d6f8860bdee46b17c2e694e480a9c681fdccf2773dd0b7fab26d {
    _0(bool),
    _1(DocumentHighlightOptions),
}
enum _a11ee373a536ab53d1e2bc6798c0311459b1d2b4cf9898d15bc31047 {
    _0(bool),
    _1(DocumentSymbolOptions),
}
enum _129c96747e4d995441b6833e01edba337095fee4b66ab8a5d8992e31 {
    _0(bool),
    _1(CodeActionOptions),
}
enum _3d04a1d707f9667cb3b870456513a05d745618ca6b166a963551e4e0 {
    _0(bool),
    _1(DocumentColorOptions),
    _2(DocumentColorRegistrationOptions),
}
enum _f1792ec3eafccc78af8184bb4dd68af8ab0f7cc04aea1487ab0a6d5a {
    _0(bool),
    _1(WorkspaceSymbolOptions),
}
enum _2d1a7c58062d48b7b5a065e8898d849cad85da7f16ceca11201ac4af {
    _0(bool),
    _1(DocumentFormattingOptions),
}
enum _2b09ea7863df648baf74b21df97a624f431183136622fab32af3c64a {
    _0(bool),
    _1(DocumentRangeFormattingOptions),
}
enum _d8b21620c6312d0b53404243370e472a32f548369fcbbfc8d58dc685 {
    _0(bool),
    _1(RenameOptions),
}
enum _031dd2726761a8bc088275c635f1618906da1c7dbcd48d5fe3df09c5 {
    _0(bool),
    _1(FoldingRangeOptions),
    _2(FoldingRangeRegistrationOptions),
}
enum _4ba79ac2d718992e318323f5ab1860ef4c8a5d916dfc2b39bc644819 {
    _0(bool),
    _1(SelectionRangeOptions),
    _2(SelectionRangeRegistrationOptions),
}
enum _e7d9e7e0e21b19df7f722c7d98c88a762bec346ae8fe61bfc078fe3f {
    _0(bool),
    _1(CallHierarchyOptions),
    _2(CallHierarchyRegistrationOptions),
}
enum _1c48fa58e01da04651803c604b74282edebe7b4f0b03741220f89ccc {
    _0(bool),
    _1(LinkedEditingRangeOptions),
    _2(LinkedEditingRangeRegistrationOptions),
}
enum _72fce9038efd721dd154b70c8cfb6e119d478e45f4a939ece93b8327 {
    _0(SemanticTokensOptions),
    _1(SemanticTokensRegistrationOptions),
}
enum _85b731e8423281c4c8592e6f02035a1c257a2fa75b6768fc7e7e9e78 {
    _0(bool),
    _1(MonikerOptions),
    _2(MonikerRegistrationOptions),
}
enum _9bbfb4decf91cc65a10244b1b2836700a89232e1d54bd4304403eeb7 {
    _0(bool),
    _1(TypeHierarchyOptions),
    _2(TypeHierarchyRegistrationOptions),
}
enum _d92f343851708f52dda572d9841a07ca2814d6033d2e906406fbf5f5 {
    _0(bool),
    _1(InlineValueOptions),
    _2(InlineValueRegistrationOptions),
}
enum _da8d06850fdfa19720b20442666513d24758232f26587bbdd2a3a343 {
    _0(bool),
    _1(InlayHintOptions),
    _2(InlayHintRegistrationOptions),
}
enum _b1a07f155097055ab647400c3de5f7ae28c7231df90aab0e525aaa6b {
    _0(DiagnosticOptions),
    _1(DiagnosticRegistrationOptions),
}
struct _bf7c1c869b776bad598568b38e429a1cc43dc08c5dfd76a9d8efff32 {
    workspaceFolders: WorkspaceFoldersServerCapabilities,
    fileOperations: FileOperationOptions,
}
enum _006fb4475b4ff9a90877437b985df22b6d586a1731f1fd73e7559507 {
    _0(i64),
    _1(String),
}
struct _9af5298e8dced3352336503d822b6f1a0eb5b909e55afb10af2daa26 {
    labelDetailsSupport: bool,
}
enum _aa51fd5e33e9172cc101fce84ba761d87c3b97dfce12cdf30b3b1019 {
    _0(String),
    _1(MarkupContent),
}
enum _5a69ec5a684cfb83fdb25959715a7e6206122e3238d684442afb8cbb {
    _0(i64),
    _1(Null),
}
enum _884abd6ac67c2478eafd1bb8fba27aa5602aae62963ae2e258acb73b {
    _0(i64),
    _1(Null),
}
enum _e098d24419db7c07f468925116bfbce9cba3a2bceb77c5cd71024c9d {
    _0(i64),
    _1(Null),
}
enum _83b26d247e1a8ce4797a761158286940d3cf8c61471bd4e59c795d74 {
    _0(bool),
    _1(SaveOptions),
}
enum _b7c1b78c415a3475e551a97f01add12a57224597b978edc52216984c {
    _0(_edd41bcb51cce28fadfebbb3723026572512ad061bebb470e2d0f8cc),
    _1(_e7d342f5c092a6af9123d178f1cbe4dc37ab1950fe4b76f06d74ed8a),
}
struct _edd41bcb51cce28fadfebbb3723026572512ad061bebb470e2d0f8cc {
    notebook: _5a63484bbdd3f61c8664387bb9e77f590ebea4b21a09fd8e5e361303,
    cells: Vec<_180d292fef329aa38936e1e34e0b2e6d9eb34fca7f620319730f5fa0>,
}
enum _5a63484bbdd3f61c8664387bb9e77f590ebea4b21a09fd8e5e361303 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _180d292fef329aa38936e1e34e0b2e6d9eb34fca7f620319730f5fa0 {
    language: String,
}
struct _e7d342f5c092a6af9123d178f1cbe4dc37ab1950fe4b76f06d74ed8a {
    notebook: _bd5949e4c71beaf87899d0cd2aa42e9fee7518f6e441ca7b9b795812,
    cells: Vec<_05ff29e27ac5ee529c097a12765fb88d6bb52554f67613e0f9ea85b8>,
}
enum _bd5949e4c71beaf87899d0cd2aa42e9fee7518f6e441ca7b9b795812 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _05ff29e27ac5ee529c097a12765fb88d6bb52554f67613e0f9ea85b8 {
    language: String,
}
enum _3d9835b70cf511ded6ec9b7fe4f8c936ba201c39f914af9ab3c939c6 {
    _0(String),
    _1(bool),
}
enum _77558c401409214b4a1b585768dde1d28d09073a190aaffbd570009a {
    _0(String),
    _1((u64, u64)),
}
enum _204ca5852233da483217023e2a6d35a3abcfb2e54b51cd7938289c24 {
    _0(String),
    _1(MarkupContent),
}
enum _e18cf742e0b0fca21da58a1c7b1a35d6c35258edda57d4dcd2d56a8d {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _a60a812d65fdd530de0650b83427b9c84506142ff6dbf3f6ac26f375 {
    cancel: bool,
    retryOnContentModified: Vec<String>,
}
enum _44a298919024d4825ab5ef16d29896b4b291132f5423a9b063d716ab {
    _0(WorkspaceFolder),
    _1(String),
}
struct _80ff7a3a91805e2c196dedd672e0e74c45b4a0b81ba20f42a14df3c9 {
    groupsOnLabel: bool,
}
struct _25e26cd554ad7748b6feadbce7b8835c6f23fd2ef30bedb38ecf7c74 {
    valueSet: Vec<SymbolKind>,
}
struct _00c7513e9c0e894c4c9713f4ebfed242b614eca929bb5e54782ebd4d {
    valueSet: Vec<SymbolTag>,
}
struct _788efc372fd9ddaf4606940980cf09f0e827938991eeccaf0904faf1 {
    properties: Vec<String>,
}
struct _ca833dea8086c0ff4807a64c049f35cab65f187fd96d5c38e04ddbcc {
    snippetSupport: bool,
    commitCharactersSupport: bool,
    documentationFormat: Vec<MarkupKind>,
    deprecatedSupport: bool,
    preselectSupport: bool,
    tagSupport: _81a4f94bed7f96bc77c9775b419c7b78cf8ffe2713a767e3f34f4c02,
    insertReplaceSupport: bool,
    resolveSupport: _3745097dd6f1489b9adcd4fbba72606658a7efe486bd53863e98fa2c,
    insertTextModeSupport: _41f6aec5fcd0624133a20f28e6332d3b9a4cc3fb08fc9312201988e2,
    labelDetailsSupport: bool,
}
struct _81a4f94bed7f96bc77c9775b419c7b78cf8ffe2713a767e3f34f4c02 {
    valueSet: Vec<CompletionItemTag>,
}
struct _3745097dd6f1489b9adcd4fbba72606658a7efe486bd53863e98fa2c {
    properties: Vec<String>,
}
struct _41f6aec5fcd0624133a20f28e6332d3b9a4cc3fb08fc9312201988e2 {
    valueSet: Vec<InsertTextMode>,
}
struct _64f1a75d05d50427a8afd5ffb15d5fe6f63685b9b62ec107956061ad {
    valueSet: Vec<CompletionItemKind>,
}
struct _e38e4f889b2c19ef8a2ad97c845a3b80f52e28c807e52bbd69a7bb7c {
    itemDefaults: Vec<String>,
}
struct _63274311fcea199783cccaf1bf3bc698bc4f60596136ce75128310ac {
    documentationFormat: Vec<MarkupKind>,
    parameterInformation: _2e12127f6fc414c0b1c6bfb2974e3776bd1e82ec8549f72a4c38658e,
    activeParameterSupport: bool,
}
struct _2e12127f6fc414c0b1c6bfb2974e3776bd1e82ec8549f72a4c38658e {
    labelOffsetSupport: bool,
}
struct _6f0eb61bfd957698265aa2d14a9fb79f28326e7d3530158c04ed6be9 {
    valueSet: Vec<SymbolKind>,
}
struct _05182c04a77bbe359b46ec93f36fb4f88d526d49345089e796ef5508 {
    valueSet: Vec<SymbolTag>,
}
struct _0bfc5ade5ffa396f4a7044b6e6b6389c240c351af6c3b7656fbfae86 {
    codeActionKind: _b05332dd63c61def74e56635cde9a0043cbca4fbec7c94500642a33c,
}
struct _b05332dd63c61def74e56635cde9a0043cbca4fbec7c94500642a33c {
    valueSet: Vec<CodeActionKind>,
}
struct _d95ec519131d7727f3094cdb0f211618b36fbed7141512257f6ee87f {
    properties: Vec<String>,
}
struct _6c3d69f9ac65e9ef92bf8b120524afe6d6627c4f4a3c263bfe03f21f {
    valueSet: Vec<FoldingRangeKind>,
}
struct _3ed96d32fe84cf5f4a81bd30fc7985d19402ef8020ab7c709dd7cc97 {
    collapsedText: bool,
}
struct _125da3d4f10d2b5c11b42c01844c361366f0014385ce9cca28aaa24f {
    valueSet: Vec<DiagnosticTag>,
}
struct _a6a7b6c565ec3b174123a44de6c950d5f578f346af0721dcfdc43bf4 {
    range: _416a3c8aa4733eebcfe8102f515261dc32cfc3d57548017bbf6f1df3,
    full: _5bc2f3f9023c1d1bacb9c7e243aeedf29ae0b54cd63d14a48b1dd4b7,
}
enum _416a3c8aa4733eebcfe8102f515261dc32cfc3d57548017bbf6f1df3 {
    _0(bool),
    _1(_a009f1323417ff60d4ef2e762e82e679ef6879f16f21dadcf84671b8),
}
struct _a009f1323417ff60d4ef2e762e82e679ef6879f16f21dadcf84671b8 {}
enum _5bc2f3f9023c1d1bacb9c7e243aeedf29ae0b54cd63d14a48b1dd4b7 {
    _0(bool),
    _1(_8f40738ae1516b1bb0f748f85d669d2814829e9608160a684bcb4e0b),
}
struct _8f40738ae1516b1bb0f748f85d669d2814829e9608160a684bcb4e0b {
    delta: bool,
}
struct _0c217d234db4b668c81dbec89f9d4dbf3ba0b5973b74b7772f99860b {
    properties: Vec<String>,
}
struct _8ca7d318958d09d66994d30431468009ca1239a83e0153f70654f4de {
    additionalPropertiesSupport: bool,
}
enum _3ad603f6d622d97ac33e2c9aaa4a2bc4f15955889de71892988968d5 {
    _0(Location),
    _1(Vec<Location>),
}
enum _f49efc2d49ebb17e2fe51d773cdf89c4a088ebcb35d52f4c3838feaf {
    _0(LSPObject),
    _1(LSPArray),
    _2(String),
    _3(i64),
    _4(u64),
    _5(f64),
    _6(bool),
    _7(Null),
}
enum _cbae34a64b57d02127dcd3ff65c0228c8b2dcdc19fe75b10a64730bb {
    _0(Location),
    _1(Vec<Location>),
}
enum _569e9eb50d34c916bf5037a8b8a48fe60c7f180c2891f403da138338 {
    _0(InlineValueText),
    _1(InlineValueVariableLookup),
    _2(InlineValueEvaluatableExpression),
}
enum _35040f864ae99ba2b5263239b6285dbd61d5fdf56984952fea4cff0d {
    _0(RelatedFullDocumentDiagnosticReport),
    _1(RelatedUnchangedDocumentDiagnosticReport),
}
enum _1959e16332064e210a6721995f53a8780509a6e5d8ec208a5a294181 {
    _0(Range),
    _1(_1d8cc09cc9c40f25fc62173c69215f60a6fd76fd69826b7684852d51),
    _2(_644bea3152a48f372d820ae0c4b8995ff5cd900e5412c13ae080a519),
}
struct _1d8cc09cc9c40f25fc62173c69215f60a6fd76fd69826b7684852d51 {
    range: Range,
    placeholder: String,
}
struct _644bea3152a48f372d820ae0c4b8995ff5cd900e5412c13ae080a519 {
    defaultBehavior: bool,
}
enum _ca144a177699a99188c2fe73bd4d39ed9b32b78bb97a0de1c1a1c772 {
    _0(i64),
    _1(String),
}
enum _ecfad5e5d9d1214299287e46d6a4b9dcd3de72911a22e62a9753726c {
    _0(WorkspaceFullDocumentDiagnosticReport),
    _1(WorkspaceUnchangedDocumentDiagnosticReport),
}
enum _4388a7f257630b4628a8d9aaa5155d6dbc29d0918128ee154d1cca29 {
    _0(_38af24d18613b020e42364829a3c5042b0aeea30272e1f5d52e5e640),
    _1(_12113d4a6737878e7e4f2eec2469ed1a9faf2a3c3de2245ff15ad9dd),
}
struct _38af24d18613b020e42364829a3c5042b0aeea30272e1f5d52e5e640 {
    range: Range,
    rangeLength: u64,
    text: String,
}
struct _12113d4a6737878e7e4f2eec2469ed1a9faf2a3c3de2245ff15ad9dd {
    text: String,
}
enum _78f4c5c59ed12c35e27233d57402563928d589a301e75bc6d3b9d202 {
    _0(String),
    _1(_70bdd9cb02f6d04e127ef228d80275f6337624fee627d012ce7bd458),
}
struct _70bdd9cb02f6d04e127ef228d80275f6337624fee627d012ce7bd458 {
    language: String,
    value: String,
}
enum _f49cdfd426f35bf5a4034897a623b6f50275b2bbd6a6d8c5f4482916 {
    _0(TextDocumentFilter),
    _1(NotebookCellTextDocumentFilter),
}
enum _b9b11b4c3b66591928ed9304a1f283517eacdc117e786aca5dc662c5 {
    _0(Pattern),
    _1(RelativePattern),
}
enum _51c568666ea7631b14d02e28bf667335c50e5d09a06b85f01a5fe1dd {
    _0(_2c9025fee22f9e8a7c9430033c1855c043f9b40bef871f77a2841ced),
    _1(_a760d257d76e10329dd5dffb452512e7c48e6c1255ea08b3352c2a46),
    _2(_eacb43192c3ec54518fdacd0c8d8e5b7d2fbde9de67dc1f4f57f827e),
}
struct _2c9025fee22f9e8a7c9430033c1855c043f9b40bef871f77a2841ced {
    language: String,
    scheme: String,
    pattern: String,
}
struct _a760d257d76e10329dd5dffb452512e7c48e6c1255ea08b3352c2a46 {
    language: String,
    scheme: String,
    pattern: String,
}
struct _eacb43192c3ec54518fdacd0c8d8e5b7d2fbde9de67dc1f4f57f827e {
    language: String,
    scheme: String,
    pattern: String,
}
enum _3e9be7fd0f2351c0b752e1d43b99b636a6575537cdc242e2ce19171f {
    _0(_e3c155c7fae824ba1f83937ca12c337e5067dc57ab4f046499d1f4f8),
    _1(_5f03c931a6acc2fc7d153b3d8710ca13545f41d43801ed7d1f784c04),
    _2(_440cfaff7e32386b379ea99ee2cd78c47e84d35e395e4a379a4bc9e3),
}
struct _e3c155c7fae824ba1f83937ca12c337e5067dc57ab4f046499d1f4f8 {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct _5f03c931a6acc2fc7d153b3d8710ca13545f41d43801ed7d1f784c04 {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct _440cfaff7e32386b379ea99ee2cd78c47e84d35e395e4a379a4bc9e3 {
    notebookType: String,
    scheme: String,
    pattern: String,
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
