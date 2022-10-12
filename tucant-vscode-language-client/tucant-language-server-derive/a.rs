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
    documentSelector: _a44ada36690aa9d6934048498ebb2d11d7e3ab87b446efd60b2de068,
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
    documentChanges: Vec<_20675497c287ff97f9244e435bca9e9a8b5c7936f347df9a84977dc6>,
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
    label: _ba0d6f1d93f0f52ae80f8653ed379208dfa122b9d6ab538eb5f7f5b8,
    kind: InlayHintKind,
    textEdits: Vec<TextEdit>,
    tooltip: _abfb8cb4ed84c41033c3e8c94c7bc4234afde188f0736e3282edf5dc,
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
        _4520b827a9946c409d4b42de2e4a18335321292200d8d65ce2a28876,
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
    serverInfo: _91c9dc45c080d1f1a95c6818893ae7fa3ec9e84798a2817872e75d03,
}
struct InitializeError {
    retry: bool,
}
struct InitializedParams {}
struct DidChangeConfigurationParams {
    settings: LSPAny,
}
struct DidChangeConfigurationRegistrationOptions {
    section: _caf5cf098d279755c6a146b930996a5222fb8f3cf7671675f9355ebf,
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
    documentation: _33791edbde5612b3e9a0a48ea5480ef154a9d1eed6a74c041df9b176,
    deprecated: bool,
    preselect: bool,
    sortText: String,
    filterText: String,
    insertText: String,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    textEdit: _d2dab3ccad894e72e0de4c2b4e889018dcd1b1e3b052fc28a4f4cd21,
    textEditText: String,
    additionalTextEdits: Vec<TextEdit>,
    commitCharacters: Vec<String>,
    command: Command,
    data: LSPAny,
}
struct CompletionList {
    isIncomplete: bool,
    itemDefaults: _8a8b6e6a8a871e6484eb8ced645e6a3e443e790082d095ad8cb8385c,
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
    contents: _bfec2e49c40d813ccb10029d4c7b8c2471ae38c4ff312706af10eeb0,
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
    disabled: _1ce6a9ccd9fcec313ffdcadb56b6160b36c92b5ac3ca6e40c9f66dc0,
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
    location: _18b5dc6db7895860b906cc51e292bb2ac211a660ae07470b25d2e8c1,
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
    kind: String,
    title: String,
    cancellable: bool,
    message: String,
    percentage: u64,
}
struct WorkDoneProgressReport {
    kind: String,
    cancellable: bool,
    message: String,
    percentage: u64,
}
struct WorkDoneProgressEnd {
    kind: String,
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
    id: _e484dcae964f0d94615a89d4de298aa4cc7ff17990d8efc175c243e8,
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
    range: _7cedacdb8f2608a3266928b05f54cdc82197d99a81ce841e29d10f84,
    full: _a1b9729827e5c2fd2a4e3da85261ac57dc84e8e0157a0cf4efa2aee7,
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
    edits: Vec<_d884e31411b1a38e866960107f8f2b2306585dc53400572442d68860>,
}
struct CreateFile {
    _0: ResourceOperation,
    kind: String,
    uri: String,
    options: CreateFileOptions,
}
struct RenameFile {
    _0: ResourceOperation,
    kind: String,
    oldUri: String,
    newUri: String,
    options: RenameFileOptions,
}
struct DeleteFile {
    _0: ResourceOperation,
    kind: String,
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
    tooltip: _eb8ecb8838ba4a00b282699457e32fccc703b1d1c3faebfb7e96ae30,
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
        _b3ca25fbfe8b20de6181622013377af7c1f6257cf6b9d86bfa2872b7,
    >,
}
struct RelatedUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    relatedDocuments: ::std::collections::HashMap<
        String,
        _1ca5b10c67e806deeb36127aeafa207dbdb8647a2a63b9af8df38207,
    >,
}
struct FullDocumentDiagnosticReport {
    kind: String,
    resultId: String,
    items: Vec<Diagnostic>,
}
struct UnchangedDocumentDiagnosticReport {
    kind: String,
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
    cells: _0827542da33a17d9f40414b9f03753572161a06e777dd45c6e42ec2a,
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
    processId: _bbf5da1a1972f59d774caf56f3ef97e5234edfbe24884bbbdc807518,
    clientInfo: _e81cedb4a7992d2c524c1332c27281220fa6b9b6b4785d3242d6c68b,
    locale: String,
    rootPath: _a6213563b2450a4eb22320e9bb7ebbb15da6df3cf993ac3c7180d4cd,
    rootUri: _e1cb7e71e106059ca1b81c378634d898d565366055db97b975b543f5,
    capabilities: ClientCapabilities,
    initializationOptions: LSPAny,
    trace: _2dc33e1f50a8cda2e91789f950352d11c9524eedff1aa4b66c8324fc,
}
struct WorkspaceFoldersInitializeParams {
    workspaceFolders: _df55105f2791f862d88387733d882c188fd7796b82554503ba371a7e,
}
struct ServerCapabilities {
    positionEncoding: PositionEncodingKind,
    textDocumentSync: _472e2438f119d76b775111dfb9dedc34cc2ce02b2c51bcb881ecac67,
    notebookDocumentSync: _631c1e348c9c0248262b4d663e3cec4418da06c0cfcfb0154b86a82c,
    completionProvider: CompletionOptions,
    hoverProvider: _cf775c8998c6a7b8a4a6fe5209e3deca20b72c9865c8ea5d6df7d643,
    signatureHelpProvider: SignatureHelpOptions,
    declarationProvider: _471c7d149428ca9c83ae8c0bec87ae584d6c99d2e6f2a331db1f5f7a,
    definitionProvider: _34120553501f416c49457cc8abfea84cd3b2803b0d59c52ea9a50dca,
    typeDefinitionProvider: _33316dc96fe5bf162c9a81a24d8e0067546c3d0b914ed2e3450b69d3,
    implementationProvider: _c31973563ba643ef0f655ff876eee0b182c45ce8d722a7e641bb0a14,
    referencesProvider: _20fe5f1b4b363fa62b85bf61a3ef8eaacde4133ec79bb25f0710b3c5,
    documentHighlightProvider: _b0213d5e10cb792421236a7d35262b8a34b4ea5fae7a041db4f6fb6e,
    documentSymbolProvider: _cb2f3a3a125b8eb211123187f73ca187f91dcc169309f52adc28a6d7,
    codeActionProvider: _0a9310fc2871c7abe46dc99211e8822b62a1ade507e0729e917d98f7,
    codeLensProvider: CodeLensOptions,
    documentLinkProvider: DocumentLinkOptions,
    colorProvider: _759c3263fd62a12b6f972b08030d81f1b7a891e4528a8a056d11577d,
    workspaceSymbolProvider: _b766abd82bdd5e9eb60e5afd17870f019c6bcbb0f68df14e9c3bbf20,
    documentFormattingProvider: _ee2c078b28dcd05719724fdf217e85f822848aadc1411bc2ea35c166,
    documentRangeFormattingProvider: _40730836c5df37e13ddbe5249a2a22941b241afdc0a1d76e5d03d293,
    documentOnTypeFormattingProvider: DocumentOnTypeFormattingOptions,
    renameProvider: _2da4bbcdf63df92264b0f186c1101404d3308406620555a2b2832602,
    foldingRangeProvider: _d1d4857902bb343b601d4229ae52fb51f1fb63f806e4ff617dad2b57,
    selectionRangeProvider: _72dd1afb7f78a6695603ed1d2d38cd2aa4e1c23b0bf3f0f0e7719b9c,
    executeCommandProvider: ExecuteCommandOptions,
    callHierarchyProvider: _759c4718cabf2f76222cd7b35b2bc2f40ecdd38e4bcef6cfd2da17c2,
    linkedEditingRangeProvider: _5b0665666407d3ae8dad3b5d12551db0325c81d06a1f53397309ac36,
    semanticTokensProvider: _df10bdc8b75a050b8725ea66d80d46547765c121fc6bf65d3294332b,
    monikerProvider: _f7e360ebc643990141b62cb57d0a2e60b1e25723421b6695fac74a9f,
    typeHierarchyProvider: _370e67f8a822b87f59e6d3a9e2c71d9e65b9731343b40d3aee6c04f7,
    inlineValueProvider: _34e5a53bd856e29fa59a168b49eafe9075cc1fc90fcbd66a37e9a8f7,
    inlayHintProvider: _3ca2f77a5178e2314989918bf506052bd741d59e066e757c16231187,
    diagnosticProvider: _7289765a9786ad6c1670a248df07ed18abe333d2794456fc30b208bc,
    workspace: _ab8afde3c03eda225c6957467f9d403a779ddc30b781d0da1817ca25,
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
    code: _3a98b10d8860f5560269068fb79c8f6d525edb98a6825b052917b812,
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
    completionItem: _9deab373943e594476f716e144173916f8e267057c142a036dfbe648,
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
    documentation: _48a512537f21818e7f05388de299cdf80541037c0dfb1da3fe55c4c5,
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
    version: _fa9e7cd9cb626f933f7c62eaee947ae76e2070b58679a3f05e23d705,
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
    version: _7882e51885e541f29d88c33b93b41dba3afd3fe6bd2bec34dccc17c3,
}
struct WorkspaceUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    uri: String,
    version: _50aac40081ea3f35f0a63acbd073ce48426f6c02d0b1552620c98402,
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
    save: _75f522d38560ad26a10fc53c9eb2011ae27430c5e3f0a50a248d3ce9,
}
struct NotebookDocumentSyncOptions {
    notebookSelector: Vec<_71710b5eeb134cdfd3faac4f3466cce13eba2d0cd09f29f1c7befd57>,
    save: bool,
}
struct NotebookDocumentSyncRegistrationOptions {
    _0: NotebookDocumentSyncOptions,
}
struct WorkspaceFoldersServerCapabilities {
    supported: bool,
    changeNotifications: _a5b1c7a5b38fa0cb4a633d1629b8ff234be7bd63fbb551b8efd12f55,
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
    label: _4ada4c59395876e63d0a7a4818d3c49aacce382a9edbfcc6a02edfc6,
    documentation: _d365b6956f33a59b45c02b054124b384b8e05392edcce4537230b64b,
}
struct NotebookCellTextDocumentFilter {
    notebook: _109998d954d8a8555a7930f92f093847cb1d2e8686cf7324de08154b,
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
    staleRequestSupport: _877bb6e73765d649dd9c8c7456ea5c545f30065d79a37e5e30a69d9b,
    regularExpressions: RegularExpressionsClientCapabilities,
    markdown: MarkdownClientCapabilities,
    positionEncodings: Vec<PositionEncodingKind>,
}
struct RelativePattern {
    baseUri: _8d73a06b22a77b0af506cf893505701a2af9ce8ae9c659dc9fbf630e,
    pattern: Pattern,
}
struct WorkspaceEditClientCapabilities {
    documentChanges: bool,
    resourceOperations: Vec<ResourceOperationKind>,
    failureHandling: FailureHandlingKind,
    normalizesLineEndings: bool,
    changeAnnotationSupport: _7cf4587e22dc7fe3c2c16b127cd5b5b2d4432963d9d6862ad806251b,
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
    symbolKind: _a5ee93db7bac8441b27dfaf3df0c808b950c56f2fba4c06a7898b076,
    tagSupport: _7f3ec240bcaae6e39d8ed59f2360e8bd2865b5011eb1ec929bc7d6ac,
    resolveSupport: _aa6b36b97757a707759f7a803a6c2deaa91129c2bfa04a13676d60a6,
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
    completionItem: _4377f9faebeb42fe2891e8feaa21e0e9246fec3262250294c1ab4328,
    completionItemKind: _fd010b92ea5ad219bc163a3b3b73bfd81e2d2d7ff03b76c076d753c9,
    insertTextMode: InsertTextMode,
    contextSupport: bool,
    completionList: _9ef0e4a9b616da9d5f5a35c13842e528ccb60b3ccc3eb1cd57e695b4,
}
struct HoverClientCapabilities {
    dynamicRegistration: bool,
    contentFormat: Vec<MarkupKind>,
}
struct SignatureHelpClientCapabilities {
    dynamicRegistration: bool,
    signatureInformation: _78cc05b724b41a4ac819ae4421efaa2e70b095997c30016530fbe9e4,
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
    symbolKind: _53feb9d647e5eac04c614f0e541477425c2b27c778c136a58db7a8cd,
    hierarchicalDocumentSymbolSupport: bool,
    tagSupport: _7bd184f8510b42d31dd38ce5079d82b3bea743a385fb9e4fd928f988,
    labelSupport: bool,
}
struct CodeActionClientCapabilities {
    dynamicRegistration: bool,
    codeActionLiteralSupport: _c22b892c96ed670d4fd0d701f9bd74444324760cb17fa0866acfc2fe,
    isPreferredSupport: bool,
    disabledSupport: bool,
    dataSupport: bool,
    resolveSupport: _d845d28de26a6562ab0f3071b0148ba78003f2388250522c14292103,
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
    foldingRangeKind: _b33a9d8f58762edd9c9ae397bf07c2276a289581dd512e56b8de54d1,
    foldingRange: _0505e161b62b03f89c9040894d9c584ddeb00fdfb5cc71400f79d800,
}
struct SelectionRangeClientCapabilities {
    dynamicRegistration: bool,
}
struct PublishDiagnosticsClientCapabilities {
    relatedInformation: bool,
    tagSupport: _e0ce07c89e64b9fc734b6a13028cea213fb910373a6d3671abbdeff9,
    versionSupport: bool,
    codeDescriptionSupport: bool,
    dataSupport: bool,
}
struct CallHierarchyClientCapabilities {
    dynamicRegistration: bool,
}
struct SemanticTokensClientCapabilities {
    dynamicRegistration: bool,
    requests: _6b9e5f4576aaadb5e4831cc0a3d43014b56e26ba5365f2c344ff4f91,
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
    resolveSupport: _ef7c979ec14639533fa3bb0704627fdb6b583546b37655fb83fe7070,
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
    messageActionItem: _b5f1d20437f2f7025880041fc630d405b7bb17f0e9ea9efee2540ad6,
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
type Definition = _57db34cbae3e865d37fbe18717f824151b68b0441fb5e562fb553571;
type DefinitionLink = LocationLink;
type LSPArray = Vec<LSPAny>;
type LSPAny = _f8075aa5d2087b5c0878a1972f58738346fbbe3c85b3edb9a00bd8ab;
type Declaration = _c6d47e4f103f8720ea2a11f8ec405e1bde5456d75462de0078ce5feb;
type DeclarationLink = LocationLink;
type InlineValue = _2e758aa6ee708a750718947a2d5b3a8392558191db9b509da97c5747;
type DocumentDiagnosticReport = _34ceea7c9a5184c7f66dd0f692565d4117a336d4717aa415372086fc;
type PrepareRenameResult = _24cd2f59a21d5cc2188ca1cc9d3c3c653d2b2f26784ea6cba7e33fc1;
type ProgressToken = _cc2b257df43de00fffbcedb2cb07aecc087f3c2f3d66207fd3732a0a;
type DocumentSelector = Vec<DocumentFilter>;
type ChangeAnnotationIdentifier = String;
type WorkspaceDocumentDiagnosticReport = _9f06980ce22b7ceb6af07a48b710d9034dbdabd9b50bb5234fb7b419;
type TextDocumentContentChangeEvent = _4cc02a265d9bed84724db0c1d65d23328b06e95e396366c244f4f58f;
type MarkedString = _7bc5227534f7b9dc132836606a76c4fe3bd5ea140c06bd2fcebe0a7d;
type DocumentFilter = _558cd18a8427d08f92d76714233029346a549a97ca8e2b6515b4e37d;
type GlobPattern = _6b7fd1908d90a0dabbfa8fbf9ba7df096ac5d7d0161feb71b055f24f;
type TextDocumentFilter = _2addba27f50770d7aad108720db2224f2753404b0dea9be4ae47c37a;
type NotebookDocumentFilter = _809a7858a37f3a1c67a06dbf4a4837c90797dc79c9cce00803115ea0;
type Pattern = String;
enum _a44ada36690aa9d6934048498ebb2d11d7e3ab87b446efd60b2de068 {
    _0(DocumentSelector),
    _1(()),
}
enum _20675497c287ff97f9244e435bca9e9a8b5c7936f347df9a84977dc6 {
    _0(TextDocumentEdit),
    _1(CreateFile),
    _2(RenameFile),
    _3(DeleteFile),
}
enum _ba0d6f1d93f0f52ae80f8653ed379208dfa122b9d6ab538eb5f7f5b8 {
    _0(String),
    _1(Vec<InlayHintLabelPart>),
}
enum _abfb8cb4ed84c41033c3e8c94c7bc4234afde188f0736e3282edf5dc {
    _0(String),
    _1(MarkupContent),
}
enum _4520b827a9946c409d4b42de2e4a18335321292200d8d65ce2a28876 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
struct _91c9dc45c080d1f1a95c6818893ae7fa3ec9e84798a2817872e75d03 {
    name: String,
    version: String,
}
enum _caf5cf098d279755c6a146b930996a5222fb8f3cf7671675f9355ebf {
    _0(String),
    _1(Vec<String>),
}
enum _33791edbde5612b3e9a0a48ea5480ef154a9d1eed6a74c041df9b176 {
    _0(String),
    _1(MarkupContent),
}
enum _d2dab3ccad894e72e0de4c2b4e889018dcd1b1e3b052fc28a4f4cd21 {
    _0(TextEdit),
    _1(InsertReplaceEdit),
}
struct _8a8b6e6a8a871e6484eb8ced645e6a3e443e790082d095ad8cb8385c {
    commitCharacters: Vec<String>,
    editRange: _4a661eb1629af6ffcd5f953adc06f1435bbd6ccd0de7f6d1a7e5c8d3,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    data: LSPAny,
}
enum _4a661eb1629af6ffcd5f953adc06f1435bbd6ccd0de7f6d1a7e5c8d3 {
    _0(Range),
    _1(_eec90c37cc838a249e4a871874c6c2e70086e9e239199fa6e2967d8b),
}
struct _eec90c37cc838a249e4a871874c6c2e70086e9e239199fa6e2967d8b {
    insert: Range,
    replace: Range,
}
enum _bfec2e49c40d813ccb10029d4c7b8c2471ae38c4ff312706af10eeb0 {
    _0(MarkupContent),
    _1(MarkedString),
    _2(Vec<MarkedString>),
}
struct _1ce6a9ccd9fcec313ffdcadb56b6160b36c92b5ac3ca6e40c9f66dc0 {
    reason: String,
}
enum _18b5dc6db7895860b906cc51e292bb2ac211a660ae07470b25d2e8c1 {
    _0(Location),
    _1(_bb0b00659ec15f1624ad7acf3928eacdc1439052056606a3f29ca9fa),
}
struct _bb0b00659ec15f1624ad7acf3928eacdc1439052056606a3f29ca9fa {
    uri: String,
}
enum _e484dcae964f0d94615a89d4de298aa4cc7ff17990d8efc175c243e8 {
    _0(i64),
    _1(String),
}
enum _7cedacdb8f2608a3266928b05f54cdc82197d99a81ce841e29d10f84 {
    _0(bool),
    _1(_9d562f1a0201cda8610075d0daea2be20a8a7968dd04d1bde874d38e),
}
struct _9d562f1a0201cda8610075d0daea2be20a8a7968dd04d1bde874d38e {}
enum _a1b9729827e5c2fd2a4e3da85261ac57dc84e8e0157a0cf4efa2aee7 {
    _0(bool),
    _1(_a1d587e2689fb6e728e56b3353fe26995e7b79f95c9e20f203259786),
}
struct _a1d587e2689fb6e728e56b3353fe26995e7b79f95c9e20f203259786 {
    delta: bool,
}
enum _d884e31411b1a38e866960107f8f2b2306585dc53400572442d68860 {
    _0(TextEdit),
    _1(AnnotatedTextEdit),
}
enum _eb8ecb8838ba4a00b282699457e32fccc703b1d1c3faebfb7e96ae30 {
    _0(String),
    _1(MarkupContent),
}
enum _b3ca25fbfe8b20de6181622013377af7c1f6257cf6b9d86bfa2872b7 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
enum _1ca5b10c67e806deeb36127aeafa207dbdb8647a2a63b9af8df38207 {
    _0(FullDocumentDiagnosticReport),
    _1(UnchangedDocumentDiagnosticReport),
}
struct _0827542da33a17d9f40414b9f03753572161a06e777dd45c6e42ec2a {
    structure: _07b5fe75c5f917885b0ab1f5f651ba8ad9361d4a8c72dab9c26d05b6,
    data: Vec<NotebookCell>,
    textContent: Vec<_d3a036b8284383fdb9d12ab4e89cfaaab1918575af527951c692d383>,
}
struct _07b5fe75c5f917885b0ab1f5f651ba8ad9361d4a8c72dab9c26d05b6 {
    array: NotebookCellArrayChange,
    didOpen: Vec<TextDocumentItem>,
    didClose: Vec<TextDocumentIdentifier>,
}
struct _d3a036b8284383fdb9d12ab4e89cfaaab1918575af527951c692d383 {
    document: VersionedTextDocumentIdentifier,
    changes: Vec<TextDocumentContentChangeEvent>,
}
enum _bbf5da1a1972f59d774caf56f3ef97e5234edfbe24884bbbdc807518 {
    _0(i64),
    _1(()),
}
struct _e81cedb4a7992d2c524c1332c27281220fa6b9b6b4785d3242d6c68b {
    name: String,
    version: String,
}
enum _a6213563b2450a4eb22320e9bb7ebbb15da6df3cf993ac3c7180d4cd {
    _0(String),
    _1(()),
}
enum _e1cb7e71e106059ca1b81c378634d898d565366055db97b975b543f5 {
    _0(String),
    _1(()),
}
enum _2dc33e1f50a8cda2e91789f950352d11c9524eedff1aa4b66c8324fc {
    _0(String),
    _1(String),
    _2(String),
    _3(String),
}
enum _df55105f2791f862d88387733d882c188fd7796b82554503ba371a7e {
    _0(Vec<WorkspaceFolder>),
    _1(()),
}
enum _472e2438f119d76b775111dfb9dedc34cc2ce02b2c51bcb881ecac67 {
    _0(TextDocumentSyncOptions),
    _1(TextDocumentSyncKind),
}
enum _631c1e348c9c0248262b4d663e3cec4418da06c0cfcfb0154b86a82c {
    _0(NotebookDocumentSyncOptions),
    _1(NotebookDocumentSyncRegistrationOptions),
}
enum _cf775c8998c6a7b8a4a6fe5209e3deca20b72c9865c8ea5d6df7d643 {
    _0(bool),
    _1(HoverOptions),
}
enum _471c7d149428ca9c83ae8c0bec87ae584d6c99d2e6f2a331db1f5f7a {
    _0(bool),
    _1(DeclarationOptions),
    _2(DeclarationRegistrationOptions),
}
enum _34120553501f416c49457cc8abfea84cd3b2803b0d59c52ea9a50dca {
    _0(bool),
    _1(DefinitionOptions),
}
enum _33316dc96fe5bf162c9a81a24d8e0067546c3d0b914ed2e3450b69d3 {
    _0(bool),
    _1(TypeDefinitionOptions),
    _2(TypeDefinitionRegistrationOptions),
}
enum _c31973563ba643ef0f655ff876eee0b182c45ce8d722a7e641bb0a14 {
    _0(bool),
    _1(ImplementationOptions),
    _2(ImplementationRegistrationOptions),
}
enum _20fe5f1b4b363fa62b85bf61a3ef8eaacde4133ec79bb25f0710b3c5 {
    _0(bool),
    _1(ReferenceOptions),
}
enum _b0213d5e10cb792421236a7d35262b8a34b4ea5fae7a041db4f6fb6e {
    _0(bool),
    _1(DocumentHighlightOptions),
}
enum _cb2f3a3a125b8eb211123187f73ca187f91dcc169309f52adc28a6d7 {
    _0(bool),
    _1(DocumentSymbolOptions),
}
enum _0a9310fc2871c7abe46dc99211e8822b62a1ade507e0729e917d98f7 {
    _0(bool),
    _1(CodeActionOptions),
}
enum _759c3263fd62a12b6f972b08030d81f1b7a891e4528a8a056d11577d {
    _0(bool),
    _1(DocumentColorOptions),
    _2(DocumentColorRegistrationOptions),
}
enum _b766abd82bdd5e9eb60e5afd17870f019c6bcbb0f68df14e9c3bbf20 {
    _0(bool),
    _1(WorkspaceSymbolOptions),
}
enum _ee2c078b28dcd05719724fdf217e85f822848aadc1411bc2ea35c166 {
    _0(bool),
    _1(DocumentFormattingOptions),
}
enum _40730836c5df37e13ddbe5249a2a22941b241afdc0a1d76e5d03d293 {
    _0(bool),
    _1(DocumentRangeFormattingOptions),
}
enum _2da4bbcdf63df92264b0f186c1101404d3308406620555a2b2832602 {
    _0(bool),
    _1(RenameOptions),
}
enum _d1d4857902bb343b601d4229ae52fb51f1fb63f806e4ff617dad2b57 {
    _0(bool),
    _1(FoldingRangeOptions),
    _2(FoldingRangeRegistrationOptions),
}
enum _72dd1afb7f78a6695603ed1d2d38cd2aa4e1c23b0bf3f0f0e7719b9c {
    _0(bool),
    _1(SelectionRangeOptions),
    _2(SelectionRangeRegistrationOptions),
}
enum _759c4718cabf2f76222cd7b35b2bc2f40ecdd38e4bcef6cfd2da17c2 {
    _0(bool),
    _1(CallHierarchyOptions),
    _2(CallHierarchyRegistrationOptions),
}
enum _5b0665666407d3ae8dad3b5d12551db0325c81d06a1f53397309ac36 {
    _0(bool),
    _1(LinkedEditingRangeOptions),
    _2(LinkedEditingRangeRegistrationOptions),
}
enum _df10bdc8b75a050b8725ea66d80d46547765c121fc6bf65d3294332b {
    _0(SemanticTokensOptions),
    _1(SemanticTokensRegistrationOptions),
}
enum _f7e360ebc643990141b62cb57d0a2e60b1e25723421b6695fac74a9f {
    _0(bool),
    _1(MonikerOptions),
    _2(MonikerRegistrationOptions),
}
enum _370e67f8a822b87f59e6d3a9e2c71d9e65b9731343b40d3aee6c04f7 {
    _0(bool),
    _1(TypeHierarchyOptions),
    _2(TypeHierarchyRegistrationOptions),
}
enum _34e5a53bd856e29fa59a168b49eafe9075cc1fc90fcbd66a37e9a8f7 {
    _0(bool),
    _1(InlineValueOptions),
    _2(InlineValueRegistrationOptions),
}
enum _3ca2f77a5178e2314989918bf506052bd741d59e066e757c16231187 {
    _0(bool),
    _1(InlayHintOptions),
    _2(InlayHintRegistrationOptions),
}
enum _7289765a9786ad6c1670a248df07ed18abe333d2794456fc30b208bc {
    _0(DiagnosticOptions),
    _1(DiagnosticRegistrationOptions),
}
struct _ab8afde3c03eda225c6957467f9d403a779ddc30b781d0da1817ca25 {
    workspaceFolders: WorkspaceFoldersServerCapabilities,
    fileOperations: FileOperationOptions,
}
enum _3a98b10d8860f5560269068fb79c8f6d525edb98a6825b052917b812 {
    _0(i64),
    _1(String),
}
struct _9deab373943e594476f716e144173916f8e267057c142a036dfbe648 {
    labelDetailsSupport: bool,
}
enum _48a512537f21818e7f05388de299cdf80541037c0dfb1da3fe55c4c5 {
    _0(String),
    _1(MarkupContent),
}
enum _fa9e7cd9cb626f933f7c62eaee947ae76e2070b58679a3f05e23d705 {
    _0(i64),
    _1(()),
}
enum _7882e51885e541f29d88c33b93b41dba3afd3fe6bd2bec34dccc17c3 {
    _0(i64),
    _1(()),
}
enum _50aac40081ea3f35f0a63acbd073ce48426f6c02d0b1552620c98402 {
    _0(i64),
    _1(()),
}
enum _75f522d38560ad26a10fc53c9eb2011ae27430c5e3f0a50a248d3ce9 {
    _0(bool),
    _1(SaveOptions),
}
enum _71710b5eeb134cdfd3faac4f3466cce13eba2d0cd09f29f1c7befd57 {
    _0(_fc748e7efbd170598cfc955974595e2de85d6779a22427fcfe4c290d),
    _1(_a038bbd73aacecb07ada8dd45331eae81a172dfcbfc79b5581091453),
}
struct _fc748e7efbd170598cfc955974595e2de85d6779a22427fcfe4c290d {
    notebook: _d82e7596bccc9c65bac5a82e74833ae8d9880f9226ee1f907041e675,
    cells: Vec<_66db0f4aa9683e57075794b314042bd66b897f206016088084b8304d>,
}
enum _d82e7596bccc9c65bac5a82e74833ae8d9880f9226ee1f907041e675 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _66db0f4aa9683e57075794b314042bd66b897f206016088084b8304d {
    language: String,
}
struct _a038bbd73aacecb07ada8dd45331eae81a172dfcbfc79b5581091453 {
    notebook: _3992fbfcd6b64d84a40505ebb61496b93a47511e59c4c870098d7162,
    cells: Vec<_438a7e4ffcd45d11161da5ab68c7c8e42aff9d7b313519fea89544db>,
}
enum _3992fbfcd6b64d84a40505ebb61496b93a47511e59c4c870098d7162 {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _438a7e4ffcd45d11161da5ab68c7c8e42aff9d7b313519fea89544db {
    language: String,
}
enum _a5b1c7a5b38fa0cb4a633d1629b8ff234be7bd63fbb551b8efd12f55 {
    _0(String),
    _1(bool),
}
enum _4ada4c59395876e63d0a7a4818d3c49aacce382a9edbfcc6a02edfc6 {
    _0(String),
    _1((u64, u64)),
}
enum _d365b6956f33a59b45c02b054124b384b8e05392edcce4537230b64b {
    _0(String),
    _1(MarkupContent),
}
enum _109998d954d8a8555a7930f92f093847cb1d2e8686cf7324de08154b {
    _0(String),
    _1(NotebookDocumentFilter),
}
struct _877bb6e73765d649dd9c8c7456ea5c545f30065d79a37e5e30a69d9b {
    cancel: bool,
    retryOnContentModified: Vec<String>,
}
enum _8d73a06b22a77b0af506cf893505701a2af9ce8ae9c659dc9fbf630e {
    _0(WorkspaceFolder),
    _1(String),
}
struct _7cf4587e22dc7fe3c2c16b127cd5b5b2d4432963d9d6862ad806251b {
    groupsOnLabel: bool,
}
struct _a5ee93db7bac8441b27dfaf3df0c808b950c56f2fba4c06a7898b076 {
    valueSet: Vec<SymbolKind>,
}
struct _7f3ec240bcaae6e39d8ed59f2360e8bd2865b5011eb1ec929bc7d6ac {
    valueSet: Vec<SymbolTag>,
}
struct _aa6b36b97757a707759f7a803a6c2deaa91129c2bfa04a13676d60a6 {
    properties: Vec<String>,
}
struct _4377f9faebeb42fe2891e8feaa21e0e9246fec3262250294c1ab4328 {
    snippetSupport: bool,
    commitCharactersSupport: bool,
    documentationFormat: Vec<MarkupKind>,
    deprecatedSupport: bool,
    preselectSupport: bool,
    tagSupport: _577a751b4c3159fb1257d64e099fce2ede35b0c0cc1e37ad0fafebc5,
    insertReplaceSupport: bool,
    resolveSupport: _3dab7f1ec8e89fd81be06547c799c4add8996541b75a784b4791203c,
    insertTextModeSupport: _7484980ef95111dcf633ecc9c946341dccdd9290d15f13a554573b5c,
    labelDetailsSupport: bool,
}
struct _577a751b4c3159fb1257d64e099fce2ede35b0c0cc1e37ad0fafebc5 {
    valueSet: Vec<CompletionItemTag>,
}
struct _3dab7f1ec8e89fd81be06547c799c4add8996541b75a784b4791203c {
    properties: Vec<String>,
}
struct _7484980ef95111dcf633ecc9c946341dccdd9290d15f13a554573b5c {
    valueSet: Vec<InsertTextMode>,
}
struct _fd010b92ea5ad219bc163a3b3b73bfd81e2d2d7ff03b76c076d753c9 {
    valueSet: Vec<CompletionItemKind>,
}
struct _9ef0e4a9b616da9d5f5a35c13842e528ccb60b3ccc3eb1cd57e695b4 {
    itemDefaults: Vec<String>,
}
struct _78cc05b724b41a4ac819ae4421efaa2e70b095997c30016530fbe9e4 {
    documentationFormat: Vec<MarkupKind>,
    parameterInformation: _66a9e7c83615243bc05d2e70619744d7a45604bb9f6202f2bff4fa37,
    activeParameterSupport: bool,
}
struct _66a9e7c83615243bc05d2e70619744d7a45604bb9f6202f2bff4fa37 {
    labelOffsetSupport: bool,
}
struct _53feb9d647e5eac04c614f0e541477425c2b27c778c136a58db7a8cd {
    valueSet: Vec<SymbolKind>,
}
struct _7bd184f8510b42d31dd38ce5079d82b3bea743a385fb9e4fd928f988 {
    valueSet: Vec<SymbolTag>,
}
struct _c22b892c96ed670d4fd0d701f9bd74444324760cb17fa0866acfc2fe {
    codeActionKind: _c9659be8f205d8ccdad7d1ac3fa3964785f875d38c47cd3c7565e339,
}
struct _c9659be8f205d8ccdad7d1ac3fa3964785f875d38c47cd3c7565e339 {
    valueSet: Vec<CodeActionKind>,
}
struct _d845d28de26a6562ab0f3071b0148ba78003f2388250522c14292103 {
    properties: Vec<String>,
}
struct _b33a9d8f58762edd9c9ae397bf07c2276a289581dd512e56b8de54d1 {
    valueSet: Vec<FoldingRangeKind>,
}
struct _0505e161b62b03f89c9040894d9c584ddeb00fdfb5cc71400f79d800 {
    collapsedText: bool,
}
struct _e0ce07c89e64b9fc734b6a13028cea213fb910373a6d3671abbdeff9 {
    valueSet: Vec<DiagnosticTag>,
}
struct _6b9e5f4576aaadb5e4831cc0a3d43014b56e26ba5365f2c344ff4f91 {
    range: _90fb067fdfb5cbdaf70ebab612f19dd398bcd95c3bc7b04e0164e050,
    full: _f085831ce61299a7f9f15f5d292ab1bbf62e68bfed37f3c537b0a8c2,
}
enum _90fb067fdfb5cbdaf70ebab612f19dd398bcd95c3bc7b04e0164e050 {
    _0(bool),
    _1(_1c061a5aecdf1d3d206f5f4b3f55a568b39b43ab0963db6955fabbb5),
}
struct _1c061a5aecdf1d3d206f5f4b3f55a568b39b43ab0963db6955fabbb5 {}
enum _f085831ce61299a7f9f15f5d292ab1bbf62e68bfed37f3c537b0a8c2 {
    _0(bool),
    _1(_7fcc9c5571cbf64f8eadb9b946482a4d783f9d6ef48a514078672377),
}
struct _7fcc9c5571cbf64f8eadb9b946482a4d783f9d6ef48a514078672377 {
    delta: bool,
}
struct _ef7c979ec14639533fa3bb0704627fdb6b583546b37655fb83fe7070 {
    properties: Vec<String>,
}
struct _b5f1d20437f2f7025880041fc630d405b7bb17f0e9ea9efee2540ad6 {
    additionalPropertiesSupport: bool,
}
enum _57db34cbae3e865d37fbe18717f824151b68b0441fb5e562fb553571 {
    _0(Location),
    _1(Vec<Location>),
}
enum _f8075aa5d2087b5c0878a1972f58738346fbbe3c85b3edb9a00bd8ab {
    _0(LSPObject),
    _1(LSPArray),
    _2(String),
    _3(i64),
    _4(u64),
    _5(f64),
    _6(bool),
    _7(()),
}
enum _c6d47e4f103f8720ea2a11f8ec405e1bde5456d75462de0078ce5feb {
    _0(Location),
    _1(Vec<Location>),
}
enum _2e758aa6ee708a750718947a2d5b3a8392558191db9b509da97c5747 {
    _0(InlineValueText),
    _1(InlineValueVariableLookup),
    _2(InlineValueEvaluatableExpression),
}
enum _34ceea7c9a5184c7f66dd0f692565d4117a336d4717aa415372086fc {
    _0(RelatedFullDocumentDiagnosticReport),
    _1(RelatedUnchangedDocumentDiagnosticReport),
}
enum _24cd2f59a21d5cc2188ca1cc9d3c3c653d2b2f26784ea6cba7e33fc1 {
    _0(Range),
    _1(_c17afae22d1e9d86a4f70b71625da81224cb77af35ca9d06ede767ab),
    _2(_2f0a3483e6a6b72347939589d4bec7399f0e83c61a651ab08f87c9d0),
}
struct _c17afae22d1e9d86a4f70b71625da81224cb77af35ca9d06ede767ab {
    range: Range,
    placeholder: String,
}
struct _2f0a3483e6a6b72347939589d4bec7399f0e83c61a651ab08f87c9d0 {
    defaultBehavior: bool,
}
enum _cc2b257df43de00fffbcedb2cb07aecc087f3c2f3d66207fd3732a0a {
    _0(i64),
    _1(String),
}
enum _9f06980ce22b7ceb6af07a48b710d9034dbdabd9b50bb5234fb7b419 {
    _0(WorkspaceFullDocumentDiagnosticReport),
    _1(WorkspaceUnchangedDocumentDiagnosticReport),
}
enum _4cc02a265d9bed84724db0c1d65d23328b06e95e396366c244f4f58f {
    _0(_4a4710efc1ea1e620ddc9227721dac1fe62c29fbf433636fea33d333),
    _1(_20ddc8727b9a06cb4f0ffb4122486834686cacabb628a10d4743809c),
}
struct _4a4710efc1ea1e620ddc9227721dac1fe62c29fbf433636fea33d333 {
    range: Range,
    rangeLength: u64,
    text: String,
}
struct _20ddc8727b9a06cb4f0ffb4122486834686cacabb628a10d4743809c {
    text: String,
}
enum _7bc5227534f7b9dc132836606a76c4fe3bd5ea140c06bd2fcebe0a7d {
    _0(String),
    _1(_8317cc78a62f99cf10c5aa90b3dbdb3452f45d3138109914e8d10bfa),
}
struct _8317cc78a62f99cf10c5aa90b3dbdb3452f45d3138109914e8d10bfa {
    language: String,
    value: String,
}
enum _558cd18a8427d08f92d76714233029346a549a97ca8e2b6515b4e37d {
    _0(TextDocumentFilter),
    _1(NotebookCellTextDocumentFilter),
}
enum _6b7fd1908d90a0dabbfa8fbf9ba7df096ac5d7d0161feb71b055f24f {
    _0(Pattern),
    _1(RelativePattern),
}
enum _2addba27f50770d7aad108720db2224f2753404b0dea9be4ae47c37a {
    _0(_eae44dc401016bf477ff6a94cadc7723bd9d22df525aba82e50a314c),
    _1(_b1735898896674cd2eed4b35eaec9e53b943b36ed3d480d1ad3bef1b),
    _2(_e0bd0041ea27f042422739f742388a7d67f55c4fd4a08355baa82877),
}
struct _eae44dc401016bf477ff6a94cadc7723bd9d22df525aba82e50a314c {
    language: String,
    scheme: String,
    pattern: String,
}
struct _b1735898896674cd2eed4b35eaec9e53b943b36ed3d480d1ad3bef1b {
    language: String,
    scheme: String,
    pattern: String,
}
struct _e0bd0041ea27f042422739f742388a7d67f55c4fd4a08355baa82877 {
    language: String,
    scheme: String,
    pattern: String,
}
enum _809a7858a37f3a1c67a06dbf4a4837c90797dc79c9cce00803115ea0 {
    _0(_e52544f1f1c92f0221467304f012719387a0f0df65a76666d358acae),
    _1(_c07d24f5c4bb7c0fdf1a97f9830984870d2c9d1e790ae48c19fb2840),
    _2(_7ea381a4a158976a877fbb0f7e5e6b42381dcc2dfc0b69745c691473),
}
struct _e52544f1f1c92f0221467304f012719387a0f0df65a76666d358acae {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct _c07d24f5c4bb7c0fdf1a97f9830984870d2c9d1e790ae48c19fb2840 {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct _7ea381a4a158976a877fbb0f7e5e6b42381dcc2dfc0b69745c691473 {
    notebookType: String,
    scheme: String,
    pattern: String,
}
struct textDocument_implementationRequest {
    id: StringOrNumber,
    method: String,
    params: ImplementationParams,
}
struct textDocument_typeDefinitionRequest {
    id: StringOrNumber,
    method: String,
    params: TypeDefinitionParams,
}
struct workspace_workspaceFoldersResponse {}
struct workspace_configurationResponse {}
struct textDocument_documentColorRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentColorParams,
}
struct textDocument_colorPresentationRequest {
    id: StringOrNumber,
    method: String,
    params: ColorPresentationParams,
}
struct textDocument_foldingRangeRequest {
    id: StringOrNumber,
    method: String,
    params: FoldingRangeParams,
}
struct textDocument_declarationRequest {
    id: StringOrNumber,
    method: String,
    params: DeclarationParams,
}
struct textDocument_selectionRangeRequest {
    id: StringOrNumber,
    method: String,
    params: SelectionRangeParams,
}
struct window_workDoneProgress_createResponse {}
struct textDocument_prepareCallHierarchyRequest {
    id: StringOrNumber,
    method: String,
    params: CallHierarchyPrepareParams,
}
struct callHierarchy_incomingCallsRequest {
    id: StringOrNumber,
    method: String,
    params: CallHierarchyIncomingCallsParams,
}
struct callHierarchy_outgoingCallsRequest {
    id: StringOrNumber,
    method: String,
    params: CallHierarchyOutgoingCallsParams,
}
struct textDocument_semanticTokens_fullRequest {
    id: StringOrNumber,
    method: String,
    params: SemanticTokensParams,
}
struct textDocument_semanticTokens_full_deltaRequest {
    id: StringOrNumber,
    method: String,
    params: SemanticTokensDeltaParams,
}
struct textDocument_semanticTokens_rangeRequest {
    id: StringOrNumber,
    method: String,
    params: SemanticTokensRangeParams,
}
struct workspace_semanticTokens_refreshRequest {
    id: StringOrNumber,
    method: String,
    params: (),
}
struct window_showDocumentResponse {}
struct textDocument_linkedEditingRangeRequest {
    id: StringOrNumber,
    method: String,
    params: LinkedEditingRangeParams,
}
struct workspace_willCreateFilesRequest {
    id: StringOrNumber,
    method: String,
    params: CreateFilesParams,
}
struct workspace_willRenameFilesRequest {
    id: StringOrNumber,
    method: String,
    params: RenameFilesParams,
}
struct workspace_willDeleteFilesRequest {
    id: StringOrNumber,
    method: String,
    params: DeleteFilesParams,
}
struct textDocument_monikerRequest {
    id: StringOrNumber,
    method: String,
    params: MonikerParams,
}
struct textDocument_prepareTypeHierarchyRequest {
    id: StringOrNumber,
    method: String,
    params: TypeHierarchyPrepareParams,
}
struct typeHierarchy_supertypesRequest {
    id: StringOrNumber,
    method: String,
    params: TypeHierarchySupertypesParams,
}
struct typeHierarchy_subtypesRequest {
    id: StringOrNumber,
    method: String,
    params: TypeHierarchySubtypesParams,
}
struct textDocument_inlineValueRequest {
    id: StringOrNumber,
    method: String,
    params: InlineValueParams,
}
struct workspace_inlineValue_refreshRequest {
    id: StringOrNumber,
    method: String,
    params: (),
}
struct textDocument_inlayHintRequest {
    id: StringOrNumber,
    method: String,
    params: InlayHintParams,
}
struct inlayHint_resolveRequest {
    id: StringOrNumber,
    method: String,
    params: InlayHint,
}
struct workspace_inlayHint_refreshRequest {
    id: StringOrNumber,
    method: String,
    params: (),
}
struct textDocument_diagnosticRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentDiagnosticParams,
}
struct workspace_diagnosticRequest {
    id: StringOrNumber,
    method: String,
    params: WorkspaceDiagnosticParams,
}
struct workspace_diagnostic_refreshRequest {
    id: StringOrNumber,
    method: String,
    params: (),
}
struct client_registerCapabilityResponse {}
struct client_unregisterCapabilityResponse {}
struct initializeRequest {
    id: StringOrNumber,
    method: String,
    params: InitializeParams,
}
struct shutdownRequest {
    id: StringOrNumber,
    method: String,
    params: (),
}
struct window_showMessageRequestResponse {}
struct textDocument_willSaveWaitUntilRequest {
    id: StringOrNumber,
    method: String,
    params: WillSaveTextDocumentParams,
}
struct textDocument_completionRequest {
    id: StringOrNumber,
    method: String,
    params: CompletionParams,
}
struct completionItem_resolveRequest {
    id: StringOrNumber,
    method: String,
    params: CompletionItem,
}
struct textDocument_hoverRequest {
    id: StringOrNumber,
    method: String,
    params: HoverParams,
}
struct textDocument_signatureHelpRequest {
    id: StringOrNumber,
    method: String,
    params: SignatureHelpParams,
}
struct textDocument_definitionRequest {
    id: StringOrNumber,
    method: String,
    params: DefinitionParams,
}
struct textDocument_referencesRequest {
    id: StringOrNumber,
    method: String,
    params: ReferenceParams,
}
struct textDocument_documentHighlightRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentHighlightParams,
}
struct textDocument_documentSymbolRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentSymbolParams,
}
struct textDocument_codeActionRequest {
    id: StringOrNumber,
    method: String,
    params: CodeActionParams,
}
struct codeAction_resolveRequest {
    id: StringOrNumber,
    method: String,
    params: CodeAction,
}
struct workspace_symbolRequest {
    id: StringOrNumber,
    method: String,
    params: WorkspaceSymbolParams,
}
struct workspaceSymbol_resolveRequest {
    id: StringOrNumber,
    method: String,
    params: WorkspaceSymbol,
}
struct textDocument_codeLensRequest {
    id: StringOrNumber,
    method: String,
    params: CodeLensParams,
}
struct codeLens_resolveRequest {
    id: StringOrNumber,
    method: String,
    params: CodeLens,
}
struct workspace_codeLens_refreshResponse {}
struct textDocument_documentLinkRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentLinkParams,
}
struct documentLink_resolveRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentLink,
}
struct textDocument_formattingRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentFormattingParams,
}
struct textDocument_rangeFormattingRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentRangeFormattingParams,
}
struct textDocument_onTypeFormattingRequest {
    id: StringOrNumber,
    method: String,
    params: DocumentOnTypeFormattingParams,
}
struct textDocument_renameRequest {
    id: StringOrNumber,
    method: String,
    params: RenameParams,
}
struct textDocument_prepareRenameRequest {
    id: StringOrNumber,
    method: String,
    params: PrepareRenameParams,
}
struct workspace_executeCommandRequest {
    id: StringOrNumber,
    method: String,
    params: ExecuteCommandParams,
}
struct workspace_applyEditResponse {}
#[serde(untagged)]
enum StringOrNumber {
    String(String),
    Number(i64),
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for StringOrNumber {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                StringOrNumber::String(ref __field0) => {
                    _serde::Serialize::serialize(__field0, __serializer)
                }
                StringOrNumber::Number(ref __field0) => {
                    _serde::Serialize::serialize(__field0, __serializer)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StringOrNumber {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                __deserializer,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            if let _serde::__private::Ok(__ok)
                = _serde::__private::Result::map(
                    <String as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<
                            __D::Error,
                        >::new(&__content),
                    ),
                    StringOrNumber::String,
                ) {
                return _serde::__private::Ok(__ok);
            }
            if let _serde::__private::Ok(__ok)
                = _serde::__private::Result::map(
                    <i64 as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<
                            __D::Error,
                        >::new(&__content),
                    ),
                    StringOrNumber::Number,
                ) {
                return _serde::__private::Ok(__ok);
            }
            _serde::__private::Err(
                _serde::de::Error::custom(
                    "data did not match any variant of untagged enum StringOrNumber",
                ),
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for StringOrNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            StringOrNumber::String(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "String", &__self_0)
            }
            StringOrNumber::Number(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Number", &__self_0)
            }
        }
    }
}
enum Requests {
    textDocument_implementationRequest(textDocument_implementationRequest),
    textDocument_typeDefinitionRequest(textDocument_typeDefinitionRequest),
    textDocument_documentColorRequest(textDocument_documentColorRequest),
    textDocument_colorPresentationRequest(textDocument_colorPresentationRequest),
    textDocument_foldingRangeRequest(textDocument_foldingRangeRequest),
    textDocument_declarationRequest(textDocument_declarationRequest),
    textDocument_selectionRangeRequest(textDocument_selectionRangeRequest),
    textDocument_prepareCallHierarchyRequest(textDocument_prepareCallHierarchyRequest),
    callHierarchy_incomingCallsRequest(callHierarchy_incomingCallsRequest),
    callHierarchy_outgoingCallsRequest(callHierarchy_outgoingCallsRequest),
    textDocument_semanticTokens_fullRequest(textDocument_semanticTokens_fullRequest),
    textDocument_semanticTokens_full_deltaRequest(
        textDocument_semanticTokens_full_deltaRequest,
    ),
    textDocument_semanticTokens_rangeRequest(textDocument_semanticTokens_rangeRequest),
    workspace_semanticTokens_refreshRequest(workspace_semanticTokens_refreshRequest),
    textDocument_linkedEditingRangeRequest(textDocument_linkedEditingRangeRequest),
    workspace_willCreateFilesRequest(workspace_willCreateFilesRequest),
    workspace_willRenameFilesRequest(workspace_willRenameFilesRequest),
    workspace_willDeleteFilesRequest(workspace_willDeleteFilesRequest),
    textDocument_monikerRequest(textDocument_monikerRequest),
    textDocument_prepareTypeHierarchyRequest(textDocument_prepareTypeHierarchyRequest),
    typeHierarchy_supertypesRequest(typeHierarchy_supertypesRequest),
    typeHierarchy_subtypesRequest(typeHierarchy_subtypesRequest),
    textDocument_inlineValueRequest(textDocument_inlineValueRequest),
    workspace_inlineValue_refreshRequest(workspace_inlineValue_refreshRequest),
    textDocument_inlayHintRequest(textDocument_inlayHintRequest),
    inlayHint_resolveRequest(inlayHint_resolveRequest),
    workspace_inlayHint_refreshRequest(workspace_inlayHint_refreshRequest),
    textDocument_diagnosticRequest(textDocument_diagnosticRequest),
    workspace_diagnosticRequest(workspace_diagnosticRequest),
    workspace_diagnostic_refreshRequest(workspace_diagnostic_refreshRequest),
    initializeRequest(initializeRequest),
    shutdownRequest(shutdownRequest),
    textDocument_willSaveWaitUntilRequest(textDocument_willSaveWaitUntilRequest),
    textDocument_completionRequest(textDocument_completionRequest),
    completionItem_resolveRequest(completionItem_resolveRequest),
    textDocument_hoverRequest(textDocument_hoverRequest),
    textDocument_signatureHelpRequest(textDocument_signatureHelpRequest),
    textDocument_definitionRequest(textDocument_definitionRequest),
    textDocument_referencesRequest(textDocument_referencesRequest),
    textDocument_documentHighlightRequest(textDocument_documentHighlightRequest),
    textDocument_documentSymbolRequest(textDocument_documentSymbolRequest),
    textDocument_codeActionRequest(textDocument_codeActionRequest),
    codeAction_resolveRequest(codeAction_resolveRequest),
    workspace_symbolRequest(workspace_symbolRequest),
    workspaceSymbol_resolveRequest(workspaceSymbol_resolveRequest),
    textDocument_codeLensRequest(textDocument_codeLensRequest),
    codeLens_resolveRequest(codeLens_resolveRequest),
    textDocument_documentLinkRequest(textDocument_documentLinkRequest),
    documentLink_resolveRequest(documentLink_resolveRequest),
    textDocument_formattingRequest(textDocument_formattingRequest),
    textDocument_rangeFormattingRequest(textDocument_rangeFormattingRequest),
    textDocument_onTypeFormattingRequest(textDocument_onTypeFormattingRequest),
    textDocument_renameRequest(textDocument_renameRequest),
    textDocument_prepareRenameRequest(textDocument_prepareRenameRequest),
    workspace_executeCommandRequest(workspace_executeCommandRequest),
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
