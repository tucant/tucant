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
    documentSelector: (),
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
    changes: (),
    documentChanges: Vec<()>,
    changeAnnotations: (),
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
    label: (),
    kind: InlayHintKind,
    textEdits: Vec<TextEdit>,
    tooltip: (),
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
    relatedDocuments: (),
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
    serverInfo: _a5bc90b4e274fd1dfe6c37f2977a1724fad6dab81ddb997d93176e830c732244314f26ceb5df30335a18650ced74c2785b8a725ac0eaa63822e80a46bfb14714,
}
struct InitializeError {
    retry: bool,
}
struct InitializedParams {}
struct DidChangeConfigurationParams {
    settings: LSPAny,
}
struct DidChangeConfigurationRegistrationOptions {
    section: (),
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
    documentation: (),
    deprecated: bool,
    preselect: bool,
    sortText: String,
    filterText: String,
    insertText: String,
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    textEdit: (),
    textEditText: String,
    additionalTextEdits: Vec<TextEdit>,
    commitCharacters: Vec<String>,
    command: Command,
    data: LSPAny,
}
struct CompletionList {
    isIncomplete: bool,
    itemDefaults: _7ae7e55a8a79b0971e84ecd7062ed3de19819bafca1fac6bd4d9685d50550ed78eb68d6b22deca9ca6b224119d6b7136f3f896284da13f1b95e1921b9580514f,
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
    contents: (),
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
    disabled: _ce1090e415d966ccb27f2a997f901efccd17ac40366663a71ce3bb09a5a4e9a63df27ffac45793458022ddeb4b1795ba091724b324e25383fa228f38463258c1,
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
    location: (),
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
    id: (),
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
    range: (),
    full: (),
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
    edits: Vec<()>,
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
    tooltip: (),
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
    relatedDocuments: (),
}
struct RelatedUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    relatedDocuments: (),
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
    cells: _1ebbbe894fc68ac5cd2e39a59705aba06f8b8d811e54864460b0a724e3e5d7e2a6a777d1d5ddd22ae3f22f366cc7423997521a2b9da799e8ea90b2f0d65472c6,
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
    processId: (),
    clientInfo: _8e9eec0a047ad6b4edd461f2ca17d65d05c6a610f60dc85839911e9b7b66f8139928ed36fcc97dde7ba973cf5c09af6cf11b1e6f9e24851eefd7bcf7ec577cff,
    locale: String,
    rootPath: (),
    rootUri: (),
    capabilities: ClientCapabilities,
    initializationOptions: LSPAny,
    trace: (),
}
struct WorkspaceFoldersInitializeParams {
    workspaceFolders: (),
}
struct ServerCapabilities {
    positionEncoding: PositionEncodingKind,
    textDocumentSync: (),
    notebookDocumentSync: (),
    completionProvider: CompletionOptions,
    hoverProvider: (),
    signatureHelpProvider: SignatureHelpOptions,
    declarationProvider: (),
    definitionProvider: (),
    typeDefinitionProvider: (),
    implementationProvider: (),
    referencesProvider: (),
    documentHighlightProvider: (),
    documentSymbolProvider: (),
    codeActionProvider: (),
    codeLensProvider: CodeLensOptions,
    documentLinkProvider: DocumentLinkOptions,
    colorProvider: (),
    workspaceSymbolProvider: (),
    documentFormattingProvider: (),
    documentRangeFormattingProvider: (),
    documentOnTypeFormattingProvider: DocumentOnTypeFormattingOptions,
    renameProvider: (),
    foldingRangeProvider: (),
    selectionRangeProvider: (),
    executeCommandProvider: ExecuteCommandOptions,
    callHierarchyProvider: (),
    linkedEditingRangeProvider: (),
    semanticTokensProvider: (),
    monikerProvider: (),
    typeHierarchyProvider: (),
    inlineValueProvider: (),
    inlayHintProvider: (),
    diagnosticProvider: (),
    workspace: _36de340c982b6a081a5bdff8d29d6d8938a1dae81bd312be301a0591a65917dbd16cfc8ef79dfc87d0922ab425f43d23abc8b57ee6d96c3d51e460c825c5c3df,
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
    code: (),
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
    completionItem: _b5e527483acb8945bd5eca8784653bb555de57482d0df5a3748e6b72861e2a5aa154f350c5da05e8b027b51b7f02c0b1f0a11b8f3a138852a8dc032b45499dd6,
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
    documentation: (),
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
    version: (),
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
    version: (),
}
struct WorkspaceUnchangedDocumentDiagnosticReport {
    _0: UnchangedDocumentDiagnosticReport,
    uri: String,
    version: (),
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
    save: (),
}
struct NotebookDocumentSyncOptions {
    notebookSelector: Vec<()>,
    save: bool,
}
struct NotebookDocumentSyncRegistrationOptions {
    _0: NotebookDocumentSyncOptions,
}
struct WorkspaceFoldersServerCapabilities {
    supported: bool,
    changeNotifications: (),
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
    label: (),
    documentation: (),
}
struct NotebookCellTextDocumentFilter {
    notebook: (),
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
    staleRequestSupport: _7fcf6aa68293a2b4614dc94d58ca257915ce4e0538d72651edf5c458023826a5db214355b4f0dbed7127f6da8a8a5a367f16c009fdd98fde9d2275d229a2c62b,
    regularExpressions: RegularExpressionsClientCapabilities,
    markdown: MarkdownClientCapabilities,
    positionEncodings: Vec<PositionEncodingKind>,
}
struct RelativePattern {
    baseUri: (),
    pattern: Pattern,
}
struct WorkspaceEditClientCapabilities {
    documentChanges: bool,
    resourceOperations: Vec<ResourceOperationKind>,
    failureHandling: FailureHandlingKind,
    normalizesLineEndings: bool,
    changeAnnotationSupport: _fdffbb0f38f59a1d10cfb8c2569663f1805bfa4fe53b6ef608ad0426abd91bd895266c8b7e777b70e72d17475fd75204e58d3641a56bb349e71c72a36cfcbd56,
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
    symbolKind: _21d303f52f0333abd47205a7887a5324b848d6157926fd601929f7f72b13919d40c9248962648c13fd146a2e564b6bf4b2972b456bfb77ee25860604f46c39a6,
    tagSupport: _806faf2a8ba70a755f5b5e2e06ecb7114b9fdb5cabaf8ad4011da6c3c8b8713c6ce50ae916a542fa24b89270d37b20afe5d495364e786893b612156b5ee71f42,
    resolveSupport: _e67ced71b750eb19e9db7df593ee2cfb82565f084c734396e196585999a93a93629d24439c0e22b5a315d93888f7026d67ef3e25e00f8ffca289a2502e928d10,
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
    completionItem: _7f1acadde6489bc941a252e04a12b385814bdb081ccd4efb8f8c45529b871eaf7345858db7cee396e5c5d04ba260bcd60dece33e2d38cbf64d1fd31c90611d12,
    completionItemKind: _112ee96493dbd1af80c741be48287de36e748dfe2787be1d12031a724aa62510ed5c84f8a425273051c0ba775ad9dfae16fad8ab56a9c893c37087ffd0687da1,
    insertTextMode: InsertTextMode,
    contextSupport: bool,
    completionList: _f09316013831cd16817cdc4e93ff7742daeb134af0593e2e7528b180c256b0d533dcb2ce0313517b4d0f676069c8ac75cedffde11905e714ff9078f22f314896,
}
struct HoverClientCapabilities {
    dynamicRegistration: bool,
    contentFormat: Vec<MarkupKind>,
}
struct SignatureHelpClientCapabilities {
    dynamicRegistration: bool,
    signatureInformation: _27c4c3e44bc0d3be9515ede577d8651e493629884f1b2f33bda7070d9f2d262059e94c50b4b86b4e8a2db815e5a42b7c18510dbb588c68a34e8b8d6297bf5322,
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
    symbolKind: _b105da1b775754f6969b9af37f32d5dfcbac5f1d88aa9c9676e1d43dc11e35f896acb31001dd77192b857d38bf141d8e161bea90348ff51ef2b60dd8a68fc494,
    hierarchicalDocumentSymbolSupport: bool,
    tagSupport: _162dc8b47d41b280e6a2c8cf5e6cd8d1c17fc12a36fda2fa40caaf9550ec03086fefebd9997dc602f43950f18d1cb4bbe9825758bf69e9d6d1866f4061667ec0,
    labelSupport: bool,
}
struct CodeActionClientCapabilities {
    dynamicRegistration: bool,
    codeActionLiteralSupport: _f42244e16477cb41bd4d88a591a848939ec6da820b03065797359e167f83d2bf94c32671ebb9f7c5e0afba632c811de68f9fd446e2dcc8798386568af5679bf1,
    isPreferredSupport: bool,
    disabledSupport: bool,
    dataSupport: bool,
    resolveSupport: _72613928c93e66504aa7e63a79f7510b3b923b17d0f0536f814cc744ddbceb90335b271a67d869b11d2c01dee698f512d84575c963cbb9bb1ca258dcf50d480f,
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
    foldingRangeKind: _e7443b70f8dd8f53ef9e6c7718eaedc077237826481a5bba027f2e363e6342685da8048c6e80d804d4468bcc42a0da1626b2810e0f0eee7a7825d009c9292330,
    foldingRange: _14a067214339c724729317fe39539073e349f3e1bdc1eb89930d34087aee869e0224609bb296082db201a675d87c185ad9ea577c805f575424de48d9b468995c,
}
struct SelectionRangeClientCapabilities {
    dynamicRegistration: bool,
}
struct PublishDiagnosticsClientCapabilities {
    relatedInformation: bool,
    tagSupport: _59b898e0c0c5afe9493233c28c4cf6ab41107db22f1363849c443415161b5c9656b52bc4bd0d5ddd1f647709eb3f16afb99679cf98de4af217ec4c640c8b0ee0,
    versionSupport: bool,
    codeDescriptionSupport: bool,
    dataSupport: bool,
}
struct CallHierarchyClientCapabilities {
    dynamicRegistration: bool,
}
struct SemanticTokensClientCapabilities {
    dynamicRegistration: bool,
    requests: _b1f0972623f5caad1fb4a9450e63197ec12a1f68de46c61d83b7bee46cd04a5ee516229a814a38227093abfb2a162abf4a2cf9fa21de4894cda2d90c50c538f1,
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
    resolveSupport: _a9cb216a72ad09a7b75a04b96f0336300b7b138f8cf648f1e556d786e1cd9dc607822ebe7132d714e44a651f08f3a9dc28838f68803c1b8eddfc703fdbac1111,
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
    messageActionItem: _d6d88b24c8a62fc4f6708f29adaefd6e16fa2715626a0a276635eff7cfcefae41123c9efc024620be428440319d37882a201533f77c717722f3a7cb55222fa08,
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
    #[serde(rename = "namespace")]
    namespace,
    #[serde(rename = "type")]
    r#type,
    #[serde(rename = "class")]
    class,
    #[serde(rename = "enum")]
    r#enum,
    #[serde(rename = "interface")]
    interface,
    #[serde(rename = "struct")]
    r#struct,
    #[serde(rename = "typeParameter")]
    typeParameter,
    #[serde(rename = "parameter")]
    parameter,
    #[serde(rename = "variable")]
    variable,
    #[serde(rename = "property")]
    property,
    #[serde(rename = "enumMember")]
    enumMember,
    #[serde(rename = "event")]
    event,
    #[serde(rename = "function")]
    function,
    #[serde(rename = "method")]
    method,
    #[serde(rename = "macro")]
    r#macro,
    #[serde(rename = "keyword")]
    keyword,
    #[serde(rename = "modifier")]
    modifier,
    #[serde(rename = "comment")]
    comment,
    #[serde(rename = "string")]
    string,
    #[serde(rename = "number")]
    number,
    #[serde(rename = "regexp")]
    regexp,
    #[serde(rename = "operator")]
    operator,
    #[serde(rename = "decorator")]
    decorator,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SemanticTokenTypes {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                SemanticTokenTypes::namespace => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        0u32,
                        "namespace",
                    )
                }
                SemanticTokenTypes::r#type => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        1u32,
                        "type",
                    )
                }
                SemanticTokenTypes::class => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        2u32,
                        "class",
                    )
                }
                SemanticTokenTypes::r#enum => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        3u32,
                        "enum",
                    )
                }
                SemanticTokenTypes::interface => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        4u32,
                        "interface",
                    )
                }
                SemanticTokenTypes::r#struct => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        5u32,
                        "struct",
                    )
                }
                SemanticTokenTypes::typeParameter => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        6u32,
                        "typeParameter",
                    )
                }
                SemanticTokenTypes::parameter => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        7u32,
                        "parameter",
                    )
                }
                SemanticTokenTypes::variable => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        8u32,
                        "variable",
                    )
                }
                SemanticTokenTypes::property => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        9u32,
                        "property",
                    )
                }
                SemanticTokenTypes::enumMember => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        10u32,
                        "enumMember",
                    )
                }
                SemanticTokenTypes::event => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        11u32,
                        "event",
                    )
                }
                SemanticTokenTypes::function => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        12u32,
                        "function",
                    )
                }
                SemanticTokenTypes::method => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        13u32,
                        "method",
                    )
                }
                SemanticTokenTypes::r#macro => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        14u32,
                        "macro",
                    )
                }
                SemanticTokenTypes::keyword => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        15u32,
                        "keyword",
                    )
                }
                SemanticTokenTypes::modifier => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        16u32,
                        "modifier",
                    )
                }
                SemanticTokenTypes::comment => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        17u32,
                        "comment",
                    )
                }
                SemanticTokenTypes::string => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        18u32,
                        "string",
                    )
                }
                SemanticTokenTypes::number => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        19u32,
                        "number",
                    )
                }
                SemanticTokenTypes::regexp => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        20u32,
                        "regexp",
                    )
                }
                SemanticTokenTypes::operator => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        21u32,
                        "operator",
                    )
                }
                SemanticTokenTypes::decorator => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenTypes",
                        22u32,
                        "decorator",
                    )
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
    impl<'de> _serde::Deserialize<'de> for SemanticTokenTypes {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
                __field15,
                __field16,
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        10u64 => _serde::__private::Ok(__Field::__field10),
                        11u64 => _serde::__private::Ok(__Field::__field11),
                        12u64 => _serde::__private::Ok(__Field::__field12),
                        13u64 => _serde::__private::Ok(__Field::__field13),
                        14u64 => _serde::__private::Ok(__Field::__field14),
                        15u64 => _serde::__private::Ok(__Field::__field15),
                        16u64 => _serde::__private::Ok(__Field::__field16),
                        17u64 => _serde::__private::Ok(__Field::__field17),
                        18u64 => _serde::__private::Ok(__Field::__field18),
                        19u64 => _serde::__private::Ok(__Field::__field19),
                        20u64 => _serde::__private::Ok(__Field::__field20),
                        21u64 => _serde::__private::Ok(__Field::__field21),
                        22u64 => _serde::__private::Ok(__Field::__field22),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 23",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "namespace" => _serde::__private::Ok(__Field::__field0),
                        "type" => _serde::__private::Ok(__Field::__field1),
                        "class" => _serde::__private::Ok(__Field::__field2),
                        "enum" => _serde::__private::Ok(__Field::__field3),
                        "interface" => _serde::__private::Ok(__Field::__field4),
                        "struct" => _serde::__private::Ok(__Field::__field5),
                        "typeParameter" => _serde::__private::Ok(__Field::__field6),
                        "parameter" => _serde::__private::Ok(__Field::__field7),
                        "variable" => _serde::__private::Ok(__Field::__field8),
                        "property" => _serde::__private::Ok(__Field::__field9),
                        "enumMember" => _serde::__private::Ok(__Field::__field10),
                        "event" => _serde::__private::Ok(__Field::__field11),
                        "function" => _serde::__private::Ok(__Field::__field12),
                        "method" => _serde::__private::Ok(__Field::__field13),
                        "macro" => _serde::__private::Ok(__Field::__field14),
                        "keyword" => _serde::__private::Ok(__Field::__field15),
                        "modifier" => _serde::__private::Ok(__Field::__field16),
                        "comment" => _serde::__private::Ok(__Field::__field17),
                        "string" => _serde::__private::Ok(__Field::__field18),
                        "number" => _serde::__private::Ok(__Field::__field19),
                        "regexp" => _serde::__private::Ok(__Field::__field20),
                        "operator" => _serde::__private::Ok(__Field::__field21),
                        "decorator" => _serde::__private::Ok(__Field::__field22),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"namespace" => _serde::__private::Ok(__Field::__field0),
                        b"type" => _serde::__private::Ok(__Field::__field1),
                        b"class" => _serde::__private::Ok(__Field::__field2),
                        b"enum" => _serde::__private::Ok(__Field::__field3),
                        b"interface" => _serde::__private::Ok(__Field::__field4),
                        b"struct" => _serde::__private::Ok(__Field::__field5),
                        b"typeParameter" => _serde::__private::Ok(__Field::__field6),
                        b"parameter" => _serde::__private::Ok(__Field::__field7),
                        b"variable" => _serde::__private::Ok(__Field::__field8),
                        b"property" => _serde::__private::Ok(__Field::__field9),
                        b"enumMember" => _serde::__private::Ok(__Field::__field10),
                        b"event" => _serde::__private::Ok(__Field::__field11),
                        b"function" => _serde::__private::Ok(__Field::__field12),
                        b"method" => _serde::__private::Ok(__Field::__field13),
                        b"macro" => _serde::__private::Ok(__Field::__field14),
                        b"keyword" => _serde::__private::Ok(__Field::__field15),
                        b"modifier" => _serde::__private::Ok(__Field::__field16),
                        b"comment" => _serde::__private::Ok(__Field::__field17),
                        b"string" => _serde::__private::Ok(__Field::__field18),
                        b"number" => _serde::__private::Ok(__Field::__field19),
                        b"regexp" => _serde::__private::Ok(__Field::__field20),
                        b"operator" => _serde::__private::Ok(__Field::__field21),
                        b"decorator" => _serde::__private::Ok(__Field::__field22),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SemanticTokenTypes>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SemanticTokenTypes;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#SemanticTokenTypes",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::namespace)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::r#type)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::class)
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::r#enum)
                        }
                        (__Field::__field4, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::interface)
                        }
                        (__Field::__field5, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::r#struct)
                        }
                        (__Field::__field6, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::typeParameter)
                        }
                        (__Field::__field7, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::parameter)
                        }
                        (__Field::__field8, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::variable)
                        }
                        (__Field::__field9, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::property)
                        }
                        (__Field::__field10, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::enumMember)
                        }
                        (__Field::__field11, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::event)
                        }
                        (__Field::__field12, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::function)
                        }
                        (__Field::__field13, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::method)
                        }
                        (__Field::__field14, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::r#macro)
                        }
                        (__Field::__field15, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::keyword)
                        }
                        (__Field::__field16, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::modifier)
                        }
                        (__Field::__field17, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::comment)
                        }
                        (__Field::__field18, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::string)
                        }
                        (__Field::__field19, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::number)
                        }
                        (__Field::__field20, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::regexp)
                        }
                        (__Field::__field21, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::operator)
                        }
                        (__Field::__field22, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenTypes::decorator)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &[
                "namespace",
                "type",
                "class",
                "enum",
                "interface",
                "struct",
                "typeParameter",
                "parameter",
                "variable",
                "property",
                "enumMember",
                "event",
                "function",
                "method",
                "macro",
                "keyword",
                "modifier",
                "comment",
                "string",
                "number",
                "regexp",
                "operator",
                "decorator",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SemanticTokenTypes",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SemanticTokenTypes>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SemanticTokenTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SemanticTokenTypes::namespace => {
                ::core::fmt::Formatter::write_str(f, "namespace")
            }
            SemanticTokenTypes::r#type => ::core::fmt::Formatter::write_str(f, "type"),
            SemanticTokenTypes::class => ::core::fmt::Formatter::write_str(f, "class"),
            SemanticTokenTypes::r#enum => ::core::fmt::Formatter::write_str(f, "enum"),
            SemanticTokenTypes::interface => {
                ::core::fmt::Formatter::write_str(f, "interface")
            }
            SemanticTokenTypes::r#struct => {
                ::core::fmt::Formatter::write_str(f, "struct")
            }
            SemanticTokenTypes::typeParameter => {
                ::core::fmt::Formatter::write_str(f, "typeParameter")
            }
            SemanticTokenTypes::parameter => {
                ::core::fmt::Formatter::write_str(f, "parameter")
            }
            SemanticTokenTypes::variable => {
                ::core::fmt::Formatter::write_str(f, "variable")
            }
            SemanticTokenTypes::property => {
                ::core::fmt::Formatter::write_str(f, "property")
            }
            SemanticTokenTypes::enumMember => {
                ::core::fmt::Formatter::write_str(f, "enumMember")
            }
            SemanticTokenTypes::event => ::core::fmt::Formatter::write_str(f, "event"),
            SemanticTokenTypes::function => {
                ::core::fmt::Formatter::write_str(f, "function")
            }
            SemanticTokenTypes::method => ::core::fmt::Formatter::write_str(f, "method"),
            SemanticTokenTypes::r#macro => ::core::fmt::Formatter::write_str(f, "macro"),
            SemanticTokenTypes::keyword => {
                ::core::fmt::Formatter::write_str(f, "keyword")
            }
            SemanticTokenTypes::modifier => {
                ::core::fmt::Formatter::write_str(f, "modifier")
            }
            SemanticTokenTypes::comment => {
                ::core::fmt::Formatter::write_str(f, "comment")
            }
            SemanticTokenTypes::string => ::core::fmt::Formatter::write_str(f, "string"),
            SemanticTokenTypes::number => ::core::fmt::Formatter::write_str(f, "number"),
            SemanticTokenTypes::regexp => ::core::fmt::Formatter::write_str(f, "regexp"),
            SemanticTokenTypes::operator => {
                ::core::fmt::Formatter::write_str(f, "operator")
            }
            SemanticTokenTypes::decorator => {
                ::core::fmt::Formatter::write_str(f, "decorator")
            }
        }
    }
}
enum SemanticTokenModifiers {
    #[serde(rename = "declaration")]
    declaration,
    #[serde(rename = "definition")]
    definition,
    #[serde(rename = "readonly")]
    readonly,
    #[serde(rename = "static")]
    r#static,
    #[serde(rename = "deprecated")]
    deprecated,
    #[serde(rename = "abstract")]
    r#abstract,
    #[serde(rename = "async")]
    r#async,
    #[serde(rename = "modification")]
    modification,
    #[serde(rename = "documentation")]
    documentation,
    #[serde(rename = "defaultLibrary")]
    defaultLibrary,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SemanticTokenModifiers {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                SemanticTokenModifiers::declaration => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        0u32,
                        "declaration",
                    )
                }
                SemanticTokenModifiers::definition => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        1u32,
                        "definition",
                    )
                }
                SemanticTokenModifiers::readonly => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        2u32,
                        "readonly",
                    )
                }
                SemanticTokenModifiers::r#static => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        3u32,
                        "static",
                    )
                }
                SemanticTokenModifiers::deprecated => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        4u32,
                        "deprecated",
                    )
                }
                SemanticTokenModifiers::r#abstract => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        5u32,
                        "abstract",
                    )
                }
                SemanticTokenModifiers::r#async => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        6u32,
                        "async",
                    )
                }
                SemanticTokenModifiers::modification => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        7u32,
                        "modification",
                    )
                }
                SemanticTokenModifiers::documentation => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        8u32,
                        "documentation",
                    )
                }
                SemanticTokenModifiers::defaultLibrary => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SemanticTokenModifiers",
                        9u32,
                        "defaultLibrary",
                    )
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
    impl<'de> _serde::Deserialize<'de> for SemanticTokenModifiers {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 10",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "declaration" => _serde::__private::Ok(__Field::__field0),
                        "definition" => _serde::__private::Ok(__Field::__field1),
                        "readonly" => _serde::__private::Ok(__Field::__field2),
                        "static" => _serde::__private::Ok(__Field::__field3),
                        "deprecated" => _serde::__private::Ok(__Field::__field4),
                        "abstract" => _serde::__private::Ok(__Field::__field5),
                        "async" => _serde::__private::Ok(__Field::__field6),
                        "modification" => _serde::__private::Ok(__Field::__field7),
                        "documentation" => _serde::__private::Ok(__Field::__field8),
                        "defaultLibrary" => _serde::__private::Ok(__Field::__field9),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"declaration" => _serde::__private::Ok(__Field::__field0),
                        b"definition" => _serde::__private::Ok(__Field::__field1),
                        b"readonly" => _serde::__private::Ok(__Field::__field2),
                        b"static" => _serde::__private::Ok(__Field::__field3),
                        b"deprecated" => _serde::__private::Ok(__Field::__field4),
                        b"abstract" => _serde::__private::Ok(__Field::__field5),
                        b"async" => _serde::__private::Ok(__Field::__field6),
                        b"modification" => _serde::__private::Ok(__Field::__field7),
                        b"documentation" => _serde::__private::Ok(__Field::__field8),
                        b"defaultLibrary" => _serde::__private::Ok(__Field::__field9),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SemanticTokenModifiers>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SemanticTokenModifiers;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#SemanticTokenModifiers",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::declaration)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::definition)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::readonly)
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::r#static)
                        }
                        (__Field::__field4, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::deprecated)
                        }
                        (__Field::__field5, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::r#abstract)
                        }
                        (__Field::__field6, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::r#async)
                        }
                        (__Field::__field7, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::modification)
                        }
                        (__Field::__field8, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::documentation)
                        }
                        (__Field::__field9, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(SemanticTokenModifiers::defaultLibrary)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &[
                "declaration",
                "definition",
                "readonly",
                "static",
                "deprecated",
                "abstract",
                "async",
                "modification",
                "documentation",
                "defaultLibrary",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SemanticTokenModifiers",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SemanticTokenModifiers>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for SemanticTokenModifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SemanticTokenModifiers::declaration => {
                ::core::fmt::Formatter::write_str(f, "declaration")
            }
            SemanticTokenModifiers::definition => {
                ::core::fmt::Formatter::write_str(f, "definition")
            }
            SemanticTokenModifiers::readonly => {
                ::core::fmt::Formatter::write_str(f, "readonly")
            }
            SemanticTokenModifiers::r#static => {
                ::core::fmt::Formatter::write_str(f, "static")
            }
            SemanticTokenModifiers::deprecated => {
                ::core::fmt::Formatter::write_str(f, "deprecated")
            }
            SemanticTokenModifiers::r#abstract => {
                ::core::fmt::Formatter::write_str(f, "abstract")
            }
            SemanticTokenModifiers::r#async => {
                ::core::fmt::Formatter::write_str(f, "async")
            }
            SemanticTokenModifiers::modification => {
                ::core::fmt::Formatter::write_str(f, "modification")
            }
            SemanticTokenModifiers::documentation => {
                ::core::fmt::Formatter::write_str(f, "documentation")
            }
            SemanticTokenModifiers::defaultLibrary => {
                ::core::fmt::Formatter::write_str(f, "defaultLibrary")
            }
        }
    }
}
enum DocumentDiagnosticReportKind {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "unchanged")]
    Unchanged,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for DocumentDiagnosticReportKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                DocumentDiagnosticReportKind::Full => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "DocumentDiagnosticReportKind",
                        0u32,
                        "full",
                    )
                }
                DocumentDiagnosticReportKind::Unchanged => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "DocumentDiagnosticReportKind",
                        1u32,
                        "unchanged",
                    )
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
    impl<'de> _serde::Deserialize<'de> for DocumentDiagnosticReportKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "full" => _serde::__private::Ok(__Field::__field0),
                        "unchanged" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"full" => _serde::__private::Ok(__Field::__field0),
                        b"unchanged" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<DocumentDiagnosticReportKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DocumentDiagnosticReportKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#DocumentDiagnosticReportKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(DocumentDiagnosticReportKind::Full)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(
                                DocumentDiagnosticReportKind::Unchanged,
                            )
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["full", "unchanged"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "DocumentDiagnosticReportKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        DocumentDiagnosticReportKind,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for DocumentDiagnosticReportKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DocumentDiagnosticReportKind::Full => {
                ::core::fmt::Formatter::write_str(f, "Full")
            }
            DocumentDiagnosticReportKind::Unchanged => {
                ::core::fmt::Formatter::write_str(f, "Unchanged")
            }
        }
    }
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
impl serde::Serialize for ErrorCodes {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            ErrorCodes::ParseError => ErrorCodes::ParseError as i64,
            ErrorCodes::InvalidRequest => ErrorCodes::InvalidRequest as i64,
            ErrorCodes::MethodNotFound => ErrorCodes::MethodNotFound as i64,
            ErrorCodes::InvalidParams => ErrorCodes::InvalidParams as i64,
            ErrorCodes::InternalError => ErrorCodes::InternalError as i64,
            ErrorCodes::ServerNotInitialized => ErrorCodes::ServerNotInitialized as i64,
            ErrorCodes::UnknownErrorCode => ErrorCodes::UnknownErrorCode as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCodes {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const ParseError: i64 = ErrorCodes::ParseError as i64;
            const InvalidRequest: i64 = ErrorCodes::InvalidRequest as i64;
            const MethodNotFound: i64 = ErrorCodes::MethodNotFound as i64;
            const InvalidParams: i64 = ErrorCodes::InvalidParams as i64;
            const InternalError: i64 = ErrorCodes::InternalError as i64;
            const ServerNotInitialized: i64 = ErrorCodes::ServerNotInitialized as i64;
            const UnknownErrorCode: i64 = ErrorCodes::UnknownErrorCode as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::ParseError => core::result::Result::Ok(ErrorCodes::ParseError),
            discriminant::InvalidRequest => {
                core::result::Result::Ok(ErrorCodes::InvalidRequest)
            }
            discriminant::MethodNotFound => {
                core::result::Result::Ok(ErrorCodes::MethodNotFound)
            }
            discriminant::InvalidParams => {
                core::result::Result::Ok(ErrorCodes::InvalidParams)
            }
            discriminant::InternalError => {
                core::result::Result::Ok(ErrorCodes::InternalError)
            }
            discriminant::ServerNotInitialized => {
                core::result::Result::Ok(ErrorCodes::ServerNotInitialized)
            }
            discriminant::UnknownErrorCode => {
                core::result::Result::Ok(ErrorCodes::UnknownErrorCode)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "invalid value: ",
                                ", expected one of: ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::ParseError,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::InvalidRequest,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::MethodNotFound,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::InvalidParams,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::InternalError,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::ServerNotInitialized,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::UnknownErrorCode,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ErrorCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ErrorCodes::ParseError => ::core::fmt::Formatter::write_str(f, "ParseError"),
            ErrorCodes::InvalidRequest => {
                ::core::fmt::Formatter::write_str(f, "InvalidRequest")
            }
            ErrorCodes::MethodNotFound => {
                ::core::fmt::Formatter::write_str(f, "MethodNotFound")
            }
            ErrorCodes::InvalidParams => {
                ::core::fmt::Formatter::write_str(f, "InvalidParams")
            }
            ErrorCodes::InternalError => {
                ::core::fmt::Formatter::write_str(f, "InternalError")
            }
            ErrorCodes::ServerNotInitialized => {
                ::core::fmt::Formatter::write_str(f, "ServerNotInitialized")
            }
            ErrorCodes::UnknownErrorCode => {
                ::core::fmt::Formatter::write_str(f, "UnknownErrorCode")
            }
        }
    }
}
#[repr(i64)]
enum LSPErrorCodes {
    RequestFailed = -32803i64,
    ServerCancelled = -32802i64,
    ContentModified = -32801i64,
    RequestCancelled = -32800i64,
}
impl serde::Serialize for LSPErrorCodes {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            LSPErrorCodes::RequestFailed => LSPErrorCodes::RequestFailed as i64,
            LSPErrorCodes::ServerCancelled => LSPErrorCodes::ServerCancelled as i64,
            LSPErrorCodes::ContentModified => LSPErrorCodes::ContentModified as i64,
            LSPErrorCodes::RequestCancelled => LSPErrorCodes::RequestCancelled as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for LSPErrorCodes {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const RequestFailed: i64 = LSPErrorCodes::RequestFailed as i64;
            const ServerCancelled: i64 = LSPErrorCodes::ServerCancelled as i64;
            const ContentModified: i64 = LSPErrorCodes::ContentModified as i64;
            const RequestCancelled: i64 = LSPErrorCodes::RequestCancelled as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::RequestFailed => {
                core::result::Result::Ok(LSPErrorCodes::RequestFailed)
            }
            discriminant::ServerCancelled => {
                core::result::Result::Ok(LSPErrorCodes::ServerCancelled)
            }
            discriminant::ContentModified => {
                core::result::Result::Ok(LSPErrorCodes::ContentModified)
            }
            discriminant::RequestCancelled => {
                core::result::Result::Ok(LSPErrorCodes::RequestCancelled)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "invalid value: ",
                                ", expected one of: ",
                                ", ",
                                ", ",
                                ", ",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::RequestFailed,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::ServerCancelled,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::ContentModified,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::RequestCancelled,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for LSPErrorCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            LSPErrorCodes::RequestFailed => {
                ::core::fmt::Formatter::write_str(f, "RequestFailed")
            }
            LSPErrorCodes::ServerCancelled => {
                ::core::fmt::Formatter::write_str(f, "ServerCancelled")
            }
            LSPErrorCodes::ContentModified => {
                ::core::fmt::Formatter::write_str(f, "ContentModified")
            }
            LSPErrorCodes::RequestCancelled => {
                ::core::fmt::Formatter::write_str(f, "RequestCancelled")
            }
        }
    }
}
enum FoldingRangeKind {
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "imports")]
    Imports,
    #[serde(rename = "region")]
    Region,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for FoldingRangeKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                FoldingRangeKind::Comment => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FoldingRangeKind",
                        0u32,
                        "comment",
                    )
                }
                FoldingRangeKind::Imports => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FoldingRangeKind",
                        1u32,
                        "imports",
                    )
                }
                FoldingRangeKind::Region => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FoldingRangeKind",
                        2u32,
                        "region",
                    )
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
    impl<'de> _serde::Deserialize<'de> for FoldingRangeKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "comment" => _serde::__private::Ok(__Field::__field0),
                        "imports" => _serde::__private::Ok(__Field::__field1),
                        "region" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"comment" => _serde::__private::Ok(__Field::__field0),
                        b"imports" => _serde::__private::Ok(__Field::__field1),
                        b"region" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FoldingRangeKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FoldingRangeKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#FoldingRangeKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FoldingRangeKind::Comment)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FoldingRangeKind::Imports)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FoldingRangeKind::Region)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["comment", "imports", "region"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "FoldingRangeKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FoldingRangeKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for FoldingRangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            FoldingRangeKind::Comment => ::core::fmt::Formatter::write_str(f, "Comment"),
            FoldingRangeKind::Imports => ::core::fmt::Formatter::write_str(f, "Imports"),
            FoldingRangeKind::Region => ::core::fmt::Formatter::write_str(f, "Region"),
        }
    }
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
impl serde::Serialize for SymbolKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            SymbolKind::File => SymbolKind::File as i64,
            SymbolKind::Module => SymbolKind::Module as i64,
            SymbolKind::Namespace => SymbolKind::Namespace as i64,
            SymbolKind::Package => SymbolKind::Package as i64,
            SymbolKind::Class => SymbolKind::Class as i64,
            SymbolKind::Method => SymbolKind::Method as i64,
            SymbolKind::Property => SymbolKind::Property as i64,
            SymbolKind::Field => SymbolKind::Field as i64,
            SymbolKind::Constructor => SymbolKind::Constructor as i64,
            SymbolKind::Enum => SymbolKind::Enum as i64,
            SymbolKind::Interface => SymbolKind::Interface as i64,
            SymbolKind::Function => SymbolKind::Function as i64,
            SymbolKind::Variable => SymbolKind::Variable as i64,
            SymbolKind::Constant => SymbolKind::Constant as i64,
            SymbolKind::String => SymbolKind::String as i64,
            SymbolKind::Number => SymbolKind::Number as i64,
            SymbolKind::Boolean => SymbolKind::Boolean as i64,
            SymbolKind::Array => SymbolKind::Array as i64,
            SymbolKind::Object => SymbolKind::Object as i64,
            SymbolKind::Key => SymbolKind::Key as i64,
            SymbolKind::Null => SymbolKind::Null as i64,
            SymbolKind::EnumMember => SymbolKind::EnumMember as i64,
            SymbolKind::Struct => SymbolKind::Struct as i64,
            SymbolKind::Event => SymbolKind::Event as i64,
            SymbolKind::Operator => SymbolKind::Operator as i64,
            SymbolKind::TypeParameter => SymbolKind::TypeParameter as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for SymbolKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const File: i64 = SymbolKind::File as i64;
            const Module: i64 = SymbolKind::Module as i64;
            const Namespace: i64 = SymbolKind::Namespace as i64;
            const Package: i64 = SymbolKind::Package as i64;
            const Class: i64 = SymbolKind::Class as i64;
            const Method: i64 = SymbolKind::Method as i64;
            const Property: i64 = SymbolKind::Property as i64;
            const Field: i64 = SymbolKind::Field as i64;
            const Constructor: i64 = SymbolKind::Constructor as i64;
            const Enum: i64 = SymbolKind::Enum as i64;
            const Interface: i64 = SymbolKind::Interface as i64;
            const Function: i64 = SymbolKind::Function as i64;
            const Variable: i64 = SymbolKind::Variable as i64;
            const Constant: i64 = SymbolKind::Constant as i64;
            const String: i64 = SymbolKind::String as i64;
            const Number: i64 = SymbolKind::Number as i64;
            const Boolean: i64 = SymbolKind::Boolean as i64;
            const Array: i64 = SymbolKind::Array as i64;
            const Object: i64 = SymbolKind::Object as i64;
            const Key: i64 = SymbolKind::Key as i64;
            const Null: i64 = SymbolKind::Null as i64;
            const EnumMember: i64 = SymbolKind::EnumMember as i64;
            const Struct: i64 = SymbolKind::Struct as i64;
            const Event: i64 = SymbolKind::Event as i64;
            const Operator: i64 = SymbolKind::Operator as i64;
            const TypeParameter: i64 = SymbolKind::TypeParameter as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::File => core::result::Result::Ok(SymbolKind::File),
            discriminant::Module => core::result::Result::Ok(SymbolKind::Module),
            discriminant::Namespace => core::result::Result::Ok(SymbolKind::Namespace),
            discriminant::Package => core::result::Result::Ok(SymbolKind::Package),
            discriminant::Class => core::result::Result::Ok(SymbolKind::Class),
            discriminant::Method => core::result::Result::Ok(SymbolKind::Method),
            discriminant::Property => core::result::Result::Ok(SymbolKind::Property),
            discriminant::Field => core::result::Result::Ok(SymbolKind::Field),
            discriminant::Constructor => {
                core::result::Result::Ok(SymbolKind::Constructor)
            }
            discriminant::Enum => core::result::Result::Ok(SymbolKind::Enum),
            discriminant::Interface => core::result::Result::Ok(SymbolKind::Interface),
            discriminant::Function => core::result::Result::Ok(SymbolKind::Function),
            discriminant::Variable => core::result::Result::Ok(SymbolKind::Variable),
            discriminant::Constant => core::result::Result::Ok(SymbolKind::Constant),
            discriminant::String => core::result::Result::Ok(SymbolKind::String),
            discriminant::Number => core::result::Result::Ok(SymbolKind::Number),
            discriminant::Boolean => core::result::Result::Ok(SymbolKind::Boolean),
            discriminant::Array => core::result::Result::Ok(SymbolKind::Array),
            discriminant::Object => core::result::Result::Ok(SymbolKind::Object),
            discriminant::Key => core::result::Result::Ok(SymbolKind::Key),
            discriminant::Null => core::result::Result::Ok(SymbolKind::Null),
            discriminant::EnumMember => core::result::Result::Ok(SymbolKind::EnumMember),
            discriminant::Struct => core::result::Result::Ok(SymbolKind::Struct),
            discriminant::Event => core::result::Result::Ok(SymbolKind::Event),
            discriminant::Operator => core::result::Result::Ok(SymbolKind::Operator),
            discriminant::TypeParameter => {
                core::result::Result::Ok(SymbolKind::TypeParameter)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "invalid value: ",
                                ", expected one of: ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::File),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Module),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Namespace,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Package,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Class),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Method),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Property,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Field),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Constructor,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Enum),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Interface,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Function,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Variable,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Constant,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::String),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Number),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Boolean,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Array),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Object),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Key),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Null),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::EnumMember,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Struct),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Event),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Operator,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::TypeParameter,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for SymbolKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SymbolKind::File => ::core::fmt::Formatter::write_str(f, "File"),
            SymbolKind::Module => ::core::fmt::Formatter::write_str(f, "Module"),
            SymbolKind::Namespace => ::core::fmt::Formatter::write_str(f, "Namespace"),
            SymbolKind::Package => ::core::fmt::Formatter::write_str(f, "Package"),
            SymbolKind::Class => ::core::fmt::Formatter::write_str(f, "Class"),
            SymbolKind::Method => ::core::fmt::Formatter::write_str(f, "Method"),
            SymbolKind::Property => ::core::fmt::Formatter::write_str(f, "Property"),
            SymbolKind::Field => ::core::fmt::Formatter::write_str(f, "Field"),
            SymbolKind::Constructor => {
                ::core::fmt::Formatter::write_str(f, "Constructor")
            }
            SymbolKind::Enum => ::core::fmt::Formatter::write_str(f, "Enum"),
            SymbolKind::Interface => ::core::fmt::Formatter::write_str(f, "Interface"),
            SymbolKind::Function => ::core::fmt::Formatter::write_str(f, "Function"),
            SymbolKind::Variable => ::core::fmt::Formatter::write_str(f, "Variable"),
            SymbolKind::Constant => ::core::fmt::Formatter::write_str(f, "Constant"),
            SymbolKind::String => ::core::fmt::Formatter::write_str(f, "String"),
            SymbolKind::Number => ::core::fmt::Formatter::write_str(f, "Number"),
            SymbolKind::Boolean => ::core::fmt::Formatter::write_str(f, "Boolean"),
            SymbolKind::Array => ::core::fmt::Formatter::write_str(f, "Array"),
            SymbolKind::Object => ::core::fmt::Formatter::write_str(f, "Object"),
            SymbolKind::Key => ::core::fmt::Formatter::write_str(f, "Key"),
            SymbolKind::Null => ::core::fmt::Formatter::write_str(f, "Null"),
            SymbolKind::EnumMember => ::core::fmt::Formatter::write_str(f, "EnumMember"),
            SymbolKind::Struct => ::core::fmt::Formatter::write_str(f, "Struct"),
            SymbolKind::Event => ::core::fmt::Formatter::write_str(f, "Event"),
            SymbolKind::Operator => ::core::fmt::Formatter::write_str(f, "Operator"),
            SymbolKind::TypeParameter => {
                ::core::fmt::Formatter::write_str(f, "TypeParameter")
            }
        }
    }
}
#[repr(i64)]
enum SymbolTag {
    Deprecated = 1i64,
}
impl serde::Serialize for SymbolTag {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            SymbolTag::Deprecated => SymbolTag::Deprecated as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for SymbolTag {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Deprecated: i64 = SymbolTag::Deprecated as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Deprecated => core::result::Result::Ok(SymbolTag::Deprecated),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Deprecated,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for SymbolTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Deprecated")
    }
}
enum UniquenessLevel {
    #[serde(rename = "document")]
    document,
    #[serde(rename = "project")]
    project,
    #[serde(rename = "group")]
    group,
    #[serde(rename = "scheme")]
    scheme,
    #[serde(rename = "global")]
    global,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for UniquenessLevel {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                UniquenessLevel::document => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "UniquenessLevel",
                        0u32,
                        "document",
                    )
                }
                UniquenessLevel::project => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "UniquenessLevel",
                        1u32,
                        "project",
                    )
                }
                UniquenessLevel::group => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "UniquenessLevel",
                        2u32,
                        "group",
                    )
                }
                UniquenessLevel::scheme => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "UniquenessLevel",
                        3u32,
                        "scheme",
                    )
                }
                UniquenessLevel::global => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "UniquenessLevel",
                        4u32,
                        "global",
                    )
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
    impl<'de> _serde::Deserialize<'de> for UniquenessLevel {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 5",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "document" => _serde::__private::Ok(__Field::__field0),
                        "project" => _serde::__private::Ok(__Field::__field1),
                        "group" => _serde::__private::Ok(__Field::__field2),
                        "scheme" => _serde::__private::Ok(__Field::__field3),
                        "global" => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"document" => _serde::__private::Ok(__Field::__field0),
                        b"project" => _serde::__private::Ok(__Field::__field1),
                        b"group" => _serde::__private::Ok(__Field::__field2),
                        b"scheme" => _serde::__private::Ok(__Field::__field3),
                        b"global" => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<UniquenessLevel>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = UniquenessLevel;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#UniquenessLevel",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(UniquenessLevel::document)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(UniquenessLevel::project)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(UniquenessLevel::group)
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(UniquenessLevel::scheme)
                        }
                        (__Field::__field4, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(UniquenessLevel::global)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &[
                "document",
                "project",
                "group",
                "scheme",
                "global",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "UniquenessLevel",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<UniquenessLevel>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for UniquenessLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            UniquenessLevel::document => ::core::fmt::Formatter::write_str(f, "document"),
            UniquenessLevel::project => ::core::fmt::Formatter::write_str(f, "project"),
            UniquenessLevel::group => ::core::fmt::Formatter::write_str(f, "group"),
            UniquenessLevel::scheme => ::core::fmt::Formatter::write_str(f, "scheme"),
            UniquenessLevel::global => ::core::fmt::Formatter::write_str(f, "global"),
        }
    }
}
enum MonikerKind {
    #[serde(rename = "import")]
    import,
    #[serde(rename = "export")]
    export,
    #[serde(rename = "local")]
    local,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for MonikerKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                MonikerKind::import => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "MonikerKind",
                        0u32,
                        "import",
                    )
                }
                MonikerKind::export => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "MonikerKind",
                        1u32,
                        "export",
                    )
                }
                MonikerKind::local => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "MonikerKind",
                        2u32,
                        "local",
                    )
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
    impl<'de> _serde::Deserialize<'de> for MonikerKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "import" => _serde::__private::Ok(__Field::__field0),
                        "export" => _serde::__private::Ok(__Field::__field1),
                        "local" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"import" => _serde::__private::Ok(__Field::__field0),
                        b"export" => _serde::__private::Ok(__Field::__field1),
                        b"local" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MonikerKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MonikerKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#MonikerKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(MonikerKind::import)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(MonikerKind::export)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(MonikerKind::local)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["import", "export", "local"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MonikerKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MonikerKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for MonikerKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MonikerKind::import => ::core::fmt::Formatter::write_str(f, "import"),
            MonikerKind::export => ::core::fmt::Formatter::write_str(f, "export"),
            MonikerKind::local => ::core::fmt::Formatter::write_str(f, "local"),
        }
    }
}
#[repr(i64)]
enum InlayHintKind {
    Type = 1i64,
    Parameter = 2i64,
}
impl serde::Serialize for InlayHintKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            InlayHintKind::Type => InlayHintKind::Type as i64,
            InlayHintKind::Parameter => InlayHintKind::Parameter as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for InlayHintKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Type: i64 = InlayHintKind::Type as i64;
            const Parameter: i64 = InlayHintKind::Parameter as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Type => core::result::Result::Ok(InlayHintKind::Type),
            discriminant::Parameter => core::result::Result::Ok(InlayHintKind::Parameter),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected ", " or "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Type),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Parameter,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for InlayHintKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            InlayHintKind::Type => ::core::fmt::Formatter::write_str(f, "Type"),
            InlayHintKind::Parameter => ::core::fmt::Formatter::write_str(f, "Parameter"),
        }
    }
}
#[repr(i64)]
enum MessageType {
    Error = 1i64,
    Warning = 2i64,
    Info = 3i64,
    Log = 4i64,
}
impl serde::Serialize for MessageType {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            MessageType::Error => MessageType::Error as i64,
            MessageType::Warning => MessageType::Warning as i64,
            MessageType::Info => MessageType::Info as i64,
            MessageType::Log => MessageType::Log as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for MessageType {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Error: i64 = MessageType::Error as i64;
            const Warning: i64 = MessageType::Warning as i64;
            const Info: i64 = MessageType::Info as i64;
            const Log: i64 = MessageType::Log as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Error => core::result::Result::Ok(MessageType::Error),
            discriminant::Warning => core::result::Result::Ok(MessageType::Warning),
            discriminant::Info => core::result::Result::Ok(MessageType::Info),
            discriminant::Log => core::result::Result::Ok(MessageType::Log),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "invalid value: ",
                                ", expected one of: ",
                                ", ",
                                ", ",
                                ", ",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Error),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Warning,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Info),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Log),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MessageType::Error => ::core::fmt::Formatter::write_str(f, "Error"),
            MessageType::Warning => ::core::fmt::Formatter::write_str(f, "Warning"),
            MessageType::Info => ::core::fmt::Formatter::write_str(f, "Info"),
            MessageType::Log => ::core::fmt::Formatter::write_str(f, "Log"),
        }
    }
}
#[repr(i64)]
enum TextDocumentSyncKind {
    None = 0i64,
    Full = 1i64,
    Incremental = 2i64,
}
impl serde::Serialize for TextDocumentSyncKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            TextDocumentSyncKind::None => TextDocumentSyncKind::None as i64,
            TextDocumentSyncKind::Full => TextDocumentSyncKind::Full as i64,
            TextDocumentSyncKind::Incremental => TextDocumentSyncKind::Incremental as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for TextDocumentSyncKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const None: i64 = TextDocumentSyncKind::None as i64;
            const Full: i64 = TextDocumentSyncKind::Full as i64;
            const Incremental: i64 = TextDocumentSyncKind::Incremental as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::None => core::result::Result::Ok(TextDocumentSyncKind::None),
            discriminant::Full => core::result::Result::Ok(TextDocumentSyncKind::Full),
            discriminant::Incremental => {
                core::result::Result::Ok(TextDocumentSyncKind::Incremental)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::None),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Full),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Incremental,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TextDocumentSyncKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TextDocumentSyncKind::None => ::core::fmt::Formatter::write_str(f, "None"),
            TextDocumentSyncKind::Full => ::core::fmt::Formatter::write_str(f, "Full"),
            TextDocumentSyncKind::Incremental => {
                ::core::fmt::Formatter::write_str(f, "Incremental")
            }
        }
    }
}
#[repr(i64)]
enum TextDocumentSaveReason {
    Manual = 1i64,
    AfterDelay = 2i64,
    FocusOut = 3i64,
}
impl serde::Serialize for TextDocumentSaveReason {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            TextDocumentSaveReason::Manual => TextDocumentSaveReason::Manual as i64,
            TextDocumentSaveReason::AfterDelay => {
                TextDocumentSaveReason::AfterDelay as i64
            }
            TextDocumentSaveReason::FocusOut => TextDocumentSaveReason::FocusOut as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for TextDocumentSaveReason {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Manual: i64 = TextDocumentSaveReason::Manual as i64;
            const AfterDelay: i64 = TextDocumentSaveReason::AfterDelay as i64;
            const FocusOut: i64 = TextDocumentSaveReason::FocusOut as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Manual => {
                core::result::Result::Ok(TextDocumentSaveReason::Manual)
            }
            discriminant::AfterDelay => {
                core::result::Result::Ok(TextDocumentSaveReason::AfterDelay)
            }
            discriminant::FocusOut => {
                core::result::Result::Ok(TextDocumentSaveReason::FocusOut)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Manual),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::AfterDelay,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::FocusOut,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TextDocumentSaveReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TextDocumentSaveReason::Manual => {
                ::core::fmt::Formatter::write_str(f, "Manual")
            }
            TextDocumentSaveReason::AfterDelay => {
                ::core::fmt::Formatter::write_str(f, "AfterDelay")
            }
            TextDocumentSaveReason::FocusOut => {
                ::core::fmt::Formatter::write_str(f, "FocusOut")
            }
        }
    }
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
impl serde::Serialize for CompletionItemKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            CompletionItemKind::Text => CompletionItemKind::Text as i64,
            CompletionItemKind::Method => CompletionItemKind::Method as i64,
            CompletionItemKind::Function => CompletionItemKind::Function as i64,
            CompletionItemKind::Constructor => CompletionItemKind::Constructor as i64,
            CompletionItemKind::Field => CompletionItemKind::Field as i64,
            CompletionItemKind::Variable => CompletionItemKind::Variable as i64,
            CompletionItemKind::Class => CompletionItemKind::Class as i64,
            CompletionItemKind::Interface => CompletionItemKind::Interface as i64,
            CompletionItemKind::Module => CompletionItemKind::Module as i64,
            CompletionItemKind::Property => CompletionItemKind::Property as i64,
            CompletionItemKind::Unit => CompletionItemKind::Unit as i64,
            CompletionItemKind::Value => CompletionItemKind::Value as i64,
            CompletionItemKind::Enum => CompletionItemKind::Enum as i64,
            CompletionItemKind::Keyword => CompletionItemKind::Keyword as i64,
            CompletionItemKind::Snippet => CompletionItemKind::Snippet as i64,
            CompletionItemKind::Color => CompletionItemKind::Color as i64,
            CompletionItemKind::File => CompletionItemKind::File as i64,
            CompletionItemKind::Reference => CompletionItemKind::Reference as i64,
            CompletionItemKind::Folder => CompletionItemKind::Folder as i64,
            CompletionItemKind::EnumMember => CompletionItemKind::EnumMember as i64,
            CompletionItemKind::Constant => CompletionItemKind::Constant as i64,
            CompletionItemKind::Struct => CompletionItemKind::Struct as i64,
            CompletionItemKind::Event => CompletionItemKind::Event as i64,
            CompletionItemKind::Operator => CompletionItemKind::Operator as i64,
            CompletionItemKind::TypeParameter => CompletionItemKind::TypeParameter as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for CompletionItemKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Text: i64 = CompletionItemKind::Text as i64;
            const Method: i64 = CompletionItemKind::Method as i64;
            const Function: i64 = CompletionItemKind::Function as i64;
            const Constructor: i64 = CompletionItemKind::Constructor as i64;
            const Field: i64 = CompletionItemKind::Field as i64;
            const Variable: i64 = CompletionItemKind::Variable as i64;
            const Class: i64 = CompletionItemKind::Class as i64;
            const Interface: i64 = CompletionItemKind::Interface as i64;
            const Module: i64 = CompletionItemKind::Module as i64;
            const Property: i64 = CompletionItemKind::Property as i64;
            const Unit: i64 = CompletionItemKind::Unit as i64;
            const Value: i64 = CompletionItemKind::Value as i64;
            const Enum: i64 = CompletionItemKind::Enum as i64;
            const Keyword: i64 = CompletionItemKind::Keyword as i64;
            const Snippet: i64 = CompletionItemKind::Snippet as i64;
            const Color: i64 = CompletionItemKind::Color as i64;
            const File: i64 = CompletionItemKind::File as i64;
            const Reference: i64 = CompletionItemKind::Reference as i64;
            const Folder: i64 = CompletionItemKind::Folder as i64;
            const EnumMember: i64 = CompletionItemKind::EnumMember as i64;
            const Constant: i64 = CompletionItemKind::Constant as i64;
            const Struct: i64 = CompletionItemKind::Struct as i64;
            const Event: i64 = CompletionItemKind::Event as i64;
            const Operator: i64 = CompletionItemKind::Operator as i64;
            const TypeParameter: i64 = CompletionItemKind::TypeParameter as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Text => core::result::Result::Ok(CompletionItemKind::Text),
            discriminant::Method => core::result::Result::Ok(CompletionItemKind::Method),
            discriminant::Function => {
                core::result::Result::Ok(CompletionItemKind::Function)
            }
            discriminant::Constructor => {
                core::result::Result::Ok(CompletionItemKind::Constructor)
            }
            discriminant::Field => core::result::Result::Ok(CompletionItemKind::Field),
            discriminant::Variable => {
                core::result::Result::Ok(CompletionItemKind::Variable)
            }
            discriminant::Class => core::result::Result::Ok(CompletionItemKind::Class),
            discriminant::Interface => {
                core::result::Result::Ok(CompletionItemKind::Interface)
            }
            discriminant::Module => core::result::Result::Ok(CompletionItemKind::Module),
            discriminant::Property => {
                core::result::Result::Ok(CompletionItemKind::Property)
            }
            discriminant::Unit => core::result::Result::Ok(CompletionItemKind::Unit),
            discriminant::Value => core::result::Result::Ok(CompletionItemKind::Value),
            discriminant::Enum => core::result::Result::Ok(CompletionItemKind::Enum),
            discriminant::Keyword => {
                core::result::Result::Ok(CompletionItemKind::Keyword)
            }
            discriminant::Snippet => {
                core::result::Result::Ok(CompletionItemKind::Snippet)
            }
            discriminant::Color => core::result::Result::Ok(CompletionItemKind::Color),
            discriminant::File => core::result::Result::Ok(CompletionItemKind::File),
            discriminant::Reference => {
                core::result::Result::Ok(CompletionItemKind::Reference)
            }
            discriminant::Folder => core::result::Result::Ok(CompletionItemKind::Folder),
            discriminant::EnumMember => {
                core::result::Result::Ok(CompletionItemKind::EnumMember)
            }
            discriminant::Constant => {
                core::result::Result::Ok(CompletionItemKind::Constant)
            }
            discriminant::Struct => core::result::Result::Ok(CompletionItemKind::Struct),
            discriminant::Event => core::result::Result::Ok(CompletionItemKind::Event),
            discriminant::Operator => {
                core::result::Result::Ok(CompletionItemKind::Operator)
            }
            discriminant::TypeParameter => {
                core::result::Result::Ok(CompletionItemKind::TypeParameter)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "invalid value: ",
                                ", expected one of: ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                                ", ",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Text),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Method),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Function,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Constructor,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Field),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Variable,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Class),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Interface,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Module),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Property,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Unit),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Value),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Enum),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Keyword,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Snippet,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Color),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::File),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Reference,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Folder),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::EnumMember,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Constant,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Struct),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Event),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Operator,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::TypeParameter,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CompletionItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            CompletionItemKind::Text => ::core::fmt::Formatter::write_str(f, "Text"),
            CompletionItemKind::Method => ::core::fmt::Formatter::write_str(f, "Method"),
            CompletionItemKind::Function => {
                ::core::fmt::Formatter::write_str(f, "Function")
            }
            CompletionItemKind::Constructor => {
                ::core::fmt::Formatter::write_str(f, "Constructor")
            }
            CompletionItemKind::Field => ::core::fmt::Formatter::write_str(f, "Field"),
            CompletionItemKind::Variable => {
                ::core::fmt::Formatter::write_str(f, "Variable")
            }
            CompletionItemKind::Class => ::core::fmt::Formatter::write_str(f, "Class"),
            CompletionItemKind::Interface => {
                ::core::fmt::Formatter::write_str(f, "Interface")
            }
            CompletionItemKind::Module => ::core::fmt::Formatter::write_str(f, "Module"),
            CompletionItemKind::Property => {
                ::core::fmt::Formatter::write_str(f, "Property")
            }
            CompletionItemKind::Unit => ::core::fmt::Formatter::write_str(f, "Unit"),
            CompletionItemKind::Value => ::core::fmt::Formatter::write_str(f, "Value"),
            CompletionItemKind::Enum => ::core::fmt::Formatter::write_str(f, "Enum"),
            CompletionItemKind::Keyword => {
                ::core::fmt::Formatter::write_str(f, "Keyword")
            }
            CompletionItemKind::Snippet => {
                ::core::fmt::Formatter::write_str(f, "Snippet")
            }
            CompletionItemKind::Color => ::core::fmt::Formatter::write_str(f, "Color"),
            CompletionItemKind::File => ::core::fmt::Formatter::write_str(f, "File"),
            CompletionItemKind::Reference => {
                ::core::fmt::Formatter::write_str(f, "Reference")
            }
            CompletionItemKind::Folder => ::core::fmt::Formatter::write_str(f, "Folder"),
            CompletionItemKind::EnumMember => {
                ::core::fmt::Formatter::write_str(f, "EnumMember")
            }
            CompletionItemKind::Constant => {
                ::core::fmt::Formatter::write_str(f, "Constant")
            }
            CompletionItemKind::Struct => ::core::fmt::Formatter::write_str(f, "Struct"),
            CompletionItemKind::Event => ::core::fmt::Formatter::write_str(f, "Event"),
            CompletionItemKind::Operator => {
                ::core::fmt::Formatter::write_str(f, "Operator")
            }
            CompletionItemKind::TypeParameter => {
                ::core::fmt::Formatter::write_str(f, "TypeParameter")
            }
        }
    }
}
#[repr(i64)]
enum CompletionItemTag {
    Deprecated = 1i64,
}
impl serde::Serialize for CompletionItemTag {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            CompletionItemTag::Deprecated => CompletionItemTag::Deprecated as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for CompletionItemTag {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Deprecated: i64 = CompletionItemTag::Deprecated as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Deprecated => {
                core::result::Result::Ok(CompletionItemTag::Deprecated)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Deprecated,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CompletionItemTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Deprecated")
    }
}
#[repr(i64)]
enum InsertTextFormat {
    PlainText = 1i64,
    Snippet = 2i64,
}
impl serde::Serialize for InsertTextFormat {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            InsertTextFormat::PlainText => InsertTextFormat::PlainText as i64,
            InsertTextFormat::Snippet => InsertTextFormat::Snippet as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for InsertTextFormat {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const PlainText: i64 = InsertTextFormat::PlainText as i64;
            const Snippet: i64 = InsertTextFormat::Snippet as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::PlainText => {
                core::result::Result::Ok(InsertTextFormat::PlainText)
            }
            discriminant::Snippet => core::result::Result::Ok(InsertTextFormat::Snippet),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected ", " or "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::PlainText,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Snippet),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for InsertTextFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            InsertTextFormat::PlainText => {
                ::core::fmt::Formatter::write_str(f, "PlainText")
            }
            InsertTextFormat::Snippet => ::core::fmt::Formatter::write_str(f, "Snippet"),
        }
    }
}
#[repr(i64)]
enum InsertTextMode {
    asIs = 1i64,
    adjustIndentation = 2i64,
}
impl serde::Serialize for InsertTextMode {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            InsertTextMode::asIs => InsertTextMode::asIs as i64,
            InsertTextMode::adjustIndentation => InsertTextMode::adjustIndentation as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for InsertTextMode {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const asIs: i64 = InsertTextMode::asIs as i64;
            const adjustIndentation: i64 = InsertTextMode::adjustIndentation as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::asIs => core::result::Result::Ok(InsertTextMode::asIs),
            discriminant::adjustIndentation => {
                core::result::Result::Ok(InsertTextMode::adjustIndentation)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected ", " or "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::asIs),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::adjustIndentation,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for InsertTextMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            InsertTextMode::asIs => ::core::fmt::Formatter::write_str(f, "asIs"),
            InsertTextMode::adjustIndentation => {
                ::core::fmt::Formatter::write_str(f, "adjustIndentation")
            }
        }
    }
}
#[repr(i64)]
enum DocumentHighlightKind {
    Text = 1i64,
    Read = 2i64,
    Write = 3i64,
}
impl serde::Serialize for DocumentHighlightKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            DocumentHighlightKind::Text => DocumentHighlightKind::Text as i64,
            DocumentHighlightKind::Read => DocumentHighlightKind::Read as i64,
            DocumentHighlightKind::Write => DocumentHighlightKind::Write as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for DocumentHighlightKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Text: i64 = DocumentHighlightKind::Text as i64;
            const Read: i64 = DocumentHighlightKind::Read as i64;
            const Write: i64 = DocumentHighlightKind::Write as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Text => core::result::Result::Ok(DocumentHighlightKind::Text),
            discriminant::Read => core::result::Result::Ok(DocumentHighlightKind::Read),
            discriminant::Write => core::result::Result::Ok(DocumentHighlightKind::Write),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Text),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Read),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Write),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for DocumentHighlightKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DocumentHighlightKind::Text => ::core::fmt::Formatter::write_str(f, "Text"),
            DocumentHighlightKind::Read => ::core::fmt::Formatter::write_str(f, "Read"),
            DocumentHighlightKind::Write => ::core::fmt::Formatter::write_str(f, "Write"),
        }
    }
}
enum CodeActionKind {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "quickfix")]
    QuickFix,
    #[serde(rename = "refactor")]
    Refactor,
    #[serde(rename = "refactor.extract")]
    RefactorExtract,
    #[serde(rename = "refactor.inline")]
    RefactorInline,
    #[serde(rename = "refactor.rewrite")]
    RefactorRewrite,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "source.organizeImports")]
    SourceOrganizeImports,
    #[serde(rename = "source.fixAll")]
    SourceFixAll,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CodeActionKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                CodeActionKind::Empty => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        0u32,
                        "",
                    )
                }
                CodeActionKind::QuickFix => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        1u32,
                        "quickfix",
                    )
                }
                CodeActionKind::Refactor => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        2u32,
                        "refactor",
                    )
                }
                CodeActionKind::RefactorExtract => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        3u32,
                        "refactor.extract",
                    )
                }
                CodeActionKind::RefactorInline => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        4u32,
                        "refactor.inline",
                    )
                }
                CodeActionKind::RefactorRewrite => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        5u32,
                        "refactor.rewrite",
                    )
                }
                CodeActionKind::Source => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        6u32,
                        "source",
                    )
                }
                CodeActionKind::SourceOrganizeImports => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        7u32,
                        "source.organizeImports",
                    )
                }
                CodeActionKind::SourceFixAll => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "CodeActionKind",
                        8u32,
                        "source.fixAll",
                    )
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
    impl<'de> _serde::Deserialize<'de> for CodeActionKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 9",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "" => _serde::__private::Ok(__Field::__field0),
                        "quickfix" => _serde::__private::Ok(__Field::__field1),
                        "refactor" => _serde::__private::Ok(__Field::__field2),
                        "refactor.extract" => _serde::__private::Ok(__Field::__field3),
                        "refactor.inline" => _serde::__private::Ok(__Field::__field4),
                        "refactor.rewrite" => _serde::__private::Ok(__Field::__field5),
                        "source" => _serde::__private::Ok(__Field::__field6),
                        "source.organizeImports" => {
                            _serde::__private::Ok(__Field::__field7)
                        }
                        "source.fixAll" => _serde::__private::Ok(__Field::__field8),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"" => _serde::__private::Ok(__Field::__field0),
                        b"quickfix" => _serde::__private::Ok(__Field::__field1),
                        b"refactor" => _serde::__private::Ok(__Field::__field2),
                        b"refactor.extract" => _serde::__private::Ok(__Field::__field3),
                        b"refactor.inline" => _serde::__private::Ok(__Field::__field4),
                        b"refactor.rewrite" => _serde::__private::Ok(__Field::__field5),
                        b"source" => _serde::__private::Ok(__Field::__field6),
                        b"source.organizeImports" => {
                            _serde::__private::Ok(__Field::__field7)
                        }
                        b"source.fixAll" => _serde::__private::Ok(__Field::__field8),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CodeActionKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CodeActionKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#CodeActionKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::Empty)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::QuickFix)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::Refactor)
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::RefactorExtract)
                        }
                        (__Field::__field4, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::RefactorInline)
                        }
                        (__Field::__field5, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::RefactorRewrite)
                        }
                        (__Field::__field6, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::Source)
                        }
                        (__Field::__field7, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::SourceOrganizeImports)
                        }
                        (__Field::__field8, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(CodeActionKind::SourceFixAll)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &[
                "",
                "quickfix",
                "refactor",
                "refactor.extract",
                "refactor.inline",
                "refactor.rewrite",
                "source",
                "source.organizeImports",
                "source.fixAll",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CodeActionKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CodeActionKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for CodeActionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            CodeActionKind::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
            CodeActionKind::QuickFix => ::core::fmt::Formatter::write_str(f, "QuickFix"),
            CodeActionKind::Refactor => ::core::fmt::Formatter::write_str(f, "Refactor"),
            CodeActionKind::RefactorExtract => {
                ::core::fmt::Formatter::write_str(f, "RefactorExtract")
            }
            CodeActionKind::RefactorInline => {
                ::core::fmt::Formatter::write_str(f, "RefactorInline")
            }
            CodeActionKind::RefactorRewrite => {
                ::core::fmt::Formatter::write_str(f, "RefactorRewrite")
            }
            CodeActionKind::Source => ::core::fmt::Formatter::write_str(f, "Source"),
            CodeActionKind::SourceOrganizeImports => {
                ::core::fmt::Formatter::write_str(f, "SourceOrganizeImports")
            }
            CodeActionKind::SourceFixAll => {
                ::core::fmt::Formatter::write_str(f, "SourceFixAll")
            }
        }
    }
}
enum TraceValues {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "messages")]
    Messages,
    #[serde(rename = "verbose")]
    Verbose,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for TraceValues {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                TraceValues::Off => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TraceValues",
                        0u32,
                        "off",
                    )
                }
                TraceValues::Messages => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TraceValues",
                        1u32,
                        "messages",
                    )
                }
                TraceValues::Verbose => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TraceValues",
                        2u32,
                        "verbose",
                    )
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
    impl<'de> _serde::Deserialize<'de> for TraceValues {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "off" => _serde::__private::Ok(__Field::__field0),
                        "messages" => _serde::__private::Ok(__Field::__field1),
                        "verbose" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"off" => _serde::__private::Ok(__Field::__field0),
                        b"messages" => _serde::__private::Ok(__Field::__field1),
                        b"verbose" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TraceValues>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TraceValues;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#TraceValues",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(TraceValues::Off)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(TraceValues::Messages)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(TraceValues::Verbose)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["off", "messages", "verbose"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TraceValues",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TraceValues>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for TraceValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TraceValues::Off => ::core::fmt::Formatter::write_str(f, "Off"),
            TraceValues::Messages => ::core::fmt::Formatter::write_str(f, "Messages"),
            TraceValues::Verbose => ::core::fmt::Formatter::write_str(f, "Verbose"),
        }
    }
}
enum MarkupKind {
    #[serde(rename = "plaintext")]
    PlainText,
    #[serde(rename = "markdown")]
    Markdown,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for MarkupKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                MarkupKind::PlainText => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "MarkupKind",
                        0u32,
                        "plaintext",
                    )
                }
                MarkupKind::Markdown => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "MarkupKind",
                        1u32,
                        "markdown",
                    )
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
    impl<'de> _serde::Deserialize<'de> for MarkupKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "plaintext" => _serde::__private::Ok(__Field::__field0),
                        "markdown" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"plaintext" => _serde::__private::Ok(__Field::__field0),
                        b"markdown" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MarkupKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MarkupKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#MarkupKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(MarkupKind::PlainText)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(MarkupKind::Markdown)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["plaintext", "markdown"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MarkupKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MarkupKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for MarkupKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MarkupKind::PlainText => ::core::fmt::Formatter::write_str(f, "PlainText"),
            MarkupKind::Markdown => ::core::fmt::Formatter::write_str(f, "Markdown"),
        }
    }
}
enum PositionEncodingKind {
    #[serde(rename = "utf-8")]
    UTF8,
    #[serde(rename = "utf-16")]
    UTF16,
    #[serde(rename = "utf-32")]
    UTF32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for PositionEncodingKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                PositionEncodingKind::UTF8 => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PositionEncodingKind",
                        0u32,
                        "utf-8",
                    )
                }
                PositionEncodingKind::UTF16 => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PositionEncodingKind",
                        1u32,
                        "utf-16",
                    )
                }
                PositionEncodingKind::UTF32 => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PositionEncodingKind",
                        2u32,
                        "utf-32",
                    )
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
    impl<'de> _serde::Deserialize<'de> for PositionEncodingKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "utf-8" => _serde::__private::Ok(__Field::__field0),
                        "utf-16" => _serde::__private::Ok(__Field::__field1),
                        "utf-32" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"utf-8" => _serde::__private::Ok(__Field::__field0),
                        b"utf-16" => _serde::__private::Ok(__Field::__field1),
                        b"utf-32" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PositionEncodingKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PositionEncodingKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#PositionEncodingKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(PositionEncodingKind::UTF8)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(PositionEncodingKind::UTF16)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(PositionEncodingKind::UTF32)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["utf-8", "utf-16", "utf-32"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "PositionEncodingKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PositionEncodingKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for PositionEncodingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            PositionEncodingKind::UTF8 => ::core::fmt::Formatter::write_str(f, "UTF8"),
            PositionEncodingKind::UTF16 => ::core::fmt::Formatter::write_str(f, "UTF16"),
            PositionEncodingKind::UTF32 => ::core::fmt::Formatter::write_str(f, "UTF32"),
        }
    }
}
#[repr(i64)]
enum FileChangeType {
    Created = 1i64,
    Changed = 2i64,
    Deleted = 3i64,
}
impl serde::Serialize for FileChangeType {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            FileChangeType::Created => FileChangeType::Created as i64,
            FileChangeType::Changed => FileChangeType::Changed as i64,
            FileChangeType::Deleted => FileChangeType::Deleted as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for FileChangeType {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Created: i64 = FileChangeType::Created as i64;
            const Changed: i64 = FileChangeType::Changed as i64;
            const Deleted: i64 = FileChangeType::Deleted as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Created => core::result::Result::Ok(FileChangeType::Created),
            discriminant::Changed => core::result::Result::Ok(FileChangeType::Changed),
            discriminant::Deleted => core::result::Result::Ok(FileChangeType::Deleted),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Created,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Changed,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Deleted),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for FileChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            FileChangeType::Created => ::core::fmt::Formatter::write_str(f, "Created"),
            FileChangeType::Changed => ::core::fmt::Formatter::write_str(f, "Changed"),
            FileChangeType::Deleted => ::core::fmt::Formatter::write_str(f, "Deleted"),
        }
    }
}
#[repr(i64)]
enum WatchKind {
    Create = 1i64,
    Change = 2i64,
    Delete = 4i64,
}
impl serde::Serialize for WatchKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            WatchKind::Create => WatchKind::Create as i64,
            WatchKind::Change => WatchKind::Change as i64,
            WatchKind::Delete => WatchKind::Delete as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for WatchKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Create: i64 = WatchKind::Create as i64;
            const Change: i64 = WatchKind::Change as i64;
            const Delete: i64 = WatchKind::Delete as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Create => core::result::Result::Ok(WatchKind::Create),
            discriminant::Change => core::result::Result::Ok(WatchKind::Change),
            discriminant::Delete => core::result::Result::Ok(WatchKind::Delete),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Create),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Change),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Delete),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for WatchKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            WatchKind::Create => ::core::fmt::Formatter::write_str(f, "Create"),
            WatchKind::Change => ::core::fmt::Formatter::write_str(f, "Change"),
            WatchKind::Delete => ::core::fmt::Formatter::write_str(f, "Delete"),
        }
    }
}
#[repr(i64)]
enum DiagnosticSeverity {
    Error = 1i64,
    Warning = 2i64,
    Information = 3i64,
    Hint = 4i64,
}
impl serde::Serialize for DiagnosticSeverity {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            DiagnosticSeverity::Error => DiagnosticSeverity::Error as i64,
            DiagnosticSeverity::Warning => DiagnosticSeverity::Warning as i64,
            DiagnosticSeverity::Information => DiagnosticSeverity::Information as i64,
            DiagnosticSeverity::Hint => DiagnosticSeverity::Hint as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for DiagnosticSeverity {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Error: i64 = DiagnosticSeverity::Error as i64;
            const Warning: i64 = DiagnosticSeverity::Warning as i64;
            const Information: i64 = DiagnosticSeverity::Information as i64;
            const Hint: i64 = DiagnosticSeverity::Hint as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Error => core::result::Result::Ok(DiagnosticSeverity::Error),
            discriminant::Warning => {
                core::result::Result::Ok(DiagnosticSeverity::Warning)
            }
            discriminant::Information => {
                core::result::Result::Ok(DiagnosticSeverity::Information)
            }
            discriminant::Hint => core::result::Result::Ok(DiagnosticSeverity::Hint),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "invalid value: ",
                                ", expected one of: ",
                                ", ",
                                ", ",
                                ", ",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Error),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Warning,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Information,
                                ),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Hint),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for DiagnosticSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DiagnosticSeverity::Error => ::core::fmt::Formatter::write_str(f, "Error"),
            DiagnosticSeverity::Warning => {
                ::core::fmt::Formatter::write_str(f, "Warning")
            }
            DiagnosticSeverity::Information => {
                ::core::fmt::Formatter::write_str(f, "Information")
            }
            DiagnosticSeverity::Hint => ::core::fmt::Formatter::write_str(f, "Hint"),
        }
    }
}
#[repr(i64)]
enum DiagnosticTag {
    Unnecessary = 1i64,
    Deprecated = 2i64,
}
impl serde::Serialize for DiagnosticTag {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            DiagnosticTag::Unnecessary => DiagnosticTag::Unnecessary as i64,
            DiagnosticTag::Deprecated => DiagnosticTag::Deprecated as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for DiagnosticTag {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Unnecessary: i64 = DiagnosticTag::Unnecessary as i64;
            const Deprecated: i64 = DiagnosticTag::Deprecated as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Unnecessary => {
                core::result::Result::Ok(DiagnosticTag::Unnecessary)
            }
            discriminant::Deprecated => {
                core::result::Result::Ok(DiagnosticTag::Deprecated)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected ", " or "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Unnecessary,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Deprecated,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for DiagnosticTag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DiagnosticTag::Unnecessary => {
                ::core::fmt::Formatter::write_str(f, "Unnecessary")
            }
            DiagnosticTag::Deprecated => {
                ::core::fmt::Formatter::write_str(f, "Deprecated")
            }
        }
    }
}
#[repr(i64)]
enum CompletionTriggerKind {
    Invoked = 1i64,
    TriggerCharacter = 2i64,
    TriggerForIncompleteCompletions = 3i64,
}
impl serde::Serialize for CompletionTriggerKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            CompletionTriggerKind::Invoked => CompletionTriggerKind::Invoked as i64,
            CompletionTriggerKind::TriggerCharacter => {
                CompletionTriggerKind::TriggerCharacter as i64
            }
            CompletionTriggerKind::TriggerForIncompleteCompletions => {
                CompletionTriggerKind::TriggerForIncompleteCompletions as i64
            }
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for CompletionTriggerKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Invoked: i64 = CompletionTriggerKind::Invoked as i64;
            const TriggerCharacter: i64 = CompletionTriggerKind::TriggerCharacter as i64;
            const TriggerForIncompleteCompletions: i64 = CompletionTriggerKind::TriggerForIncompleteCompletions
                as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Invoked => {
                core::result::Result::Ok(CompletionTriggerKind::Invoked)
            }
            discriminant::TriggerCharacter => {
                core::result::Result::Ok(CompletionTriggerKind::TriggerCharacter)
            }
            discriminant::TriggerForIncompleteCompletions => {
                core::result::Result::Ok(
                    CompletionTriggerKind::TriggerForIncompleteCompletions,
                )
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Invoked,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::TriggerCharacter,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::TriggerForIncompleteCompletions,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CompletionTriggerKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            CompletionTriggerKind::Invoked => {
                ::core::fmt::Formatter::write_str(f, "Invoked")
            }
            CompletionTriggerKind::TriggerCharacter => {
                ::core::fmt::Formatter::write_str(f, "TriggerCharacter")
            }
            CompletionTriggerKind::TriggerForIncompleteCompletions => {
                ::core::fmt::Formatter::write_str(f, "TriggerForIncompleteCompletions")
            }
        }
    }
}
#[repr(i64)]
enum SignatureHelpTriggerKind {
    Invoked = 1i64,
    TriggerCharacter = 2i64,
    ContentChange = 3i64,
}
impl serde::Serialize for SignatureHelpTriggerKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            SignatureHelpTriggerKind::Invoked => SignatureHelpTriggerKind::Invoked as i64,
            SignatureHelpTriggerKind::TriggerCharacter => {
                SignatureHelpTriggerKind::TriggerCharacter as i64
            }
            SignatureHelpTriggerKind::ContentChange => {
                SignatureHelpTriggerKind::ContentChange as i64
            }
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for SignatureHelpTriggerKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Invoked: i64 = SignatureHelpTriggerKind::Invoked as i64;
            const TriggerCharacter: i64 = SignatureHelpTriggerKind::TriggerCharacter
                as i64;
            const ContentChange: i64 = SignatureHelpTriggerKind::ContentChange as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Invoked => {
                core::result::Result::Ok(SignatureHelpTriggerKind::Invoked)
            }
            discriminant::TriggerCharacter => {
                core::result::Result::Ok(SignatureHelpTriggerKind::TriggerCharacter)
            }
            discriminant::ContentChange => {
                core::result::Result::Ok(SignatureHelpTriggerKind::ContentChange)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected one of: ", ", ", ", "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Invoked,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::TriggerCharacter,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::ContentChange,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for SignatureHelpTriggerKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            SignatureHelpTriggerKind::Invoked => {
                ::core::fmt::Formatter::write_str(f, "Invoked")
            }
            SignatureHelpTriggerKind::TriggerCharacter => {
                ::core::fmt::Formatter::write_str(f, "TriggerCharacter")
            }
            SignatureHelpTriggerKind::ContentChange => {
                ::core::fmt::Formatter::write_str(f, "ContentChange")
            }
        }
    }
}
#[repr(i64)]
enum CodeActionTriggerKind {
    Invoked = 1i64,
    Automatic = 2i64,
}
impl serde::Serialize for CodeActionTriggerKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            CodeActionTriggerKind::Invoked => CodeActionTriggerKind::Invoked as i64,
            CodeActionTriggerKind::Automatic => CodeActionTriggerKind::Automatic as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for CodeActionTriggerKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Invoked: i64 = CodeActionTriggerKind::Invoked as i64;
            const Automatic: i64 = CodeActionTriggerKind::Automatic as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Invoked => {
                core::result::Result::Ok(CodeActionTriggerKind::Invoked)
            }
            discriminant::Automatic => {
                core::result::Result::Ok(CodeActionTriggerKind::Automatic)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected ", " or "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Invoked,
                                ),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Automatic,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CodeActionTriggerKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            CodeActionTriggerKind::Invoked => {
                ::core::fmt::Formatter::write_str(f, "Invoked")
            }
            CodeActionTriggerKind::Automatic => {
                ::core::fmt::Formatter::write_str(f, "Automatic")
            }
        }
    }
}
enum FileOperationPatternKind {
    #[serde(rename = "file")]
    file,
    #[serde(rename = "folder")]
    folder,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for FileOperationPatternKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                FileOperationPatternKind::file => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FileOperationPatternKind",
                        0u32,
                        "file",
                    )
                }
                FileOperationPatternKind::folder => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FileOperationPatternKind",
                        1u32,
                        "folder",
                    )
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
    impl<'de> _serde::Deserialize<'de> for FileOperationPatternKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "file" => _serde::__private::Ok(__Field::__field0),
                        "folder" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"file" => _serde::__private::Ok(__Field::__field0),
                        b"folder" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FileOperationPatternKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FileOperationPatternKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#FileOperationPatternKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FileOperationPatternKind::file)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FileOperationPatternKind::folder)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["file", "folder"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "FileOperationPatternKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FileOperationPatternKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for FileOperationPatternKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            FileOperationPatternKind::file => {
                ::core::fmt::Formatter::write_str(f, "file")
            }
            FileOperationPatternKind::folder => {
                ::core::fmt::Formatter::write_str(f, "folder")
            }
        }
    }
}
#[repr(i64)]
enum NotebookCellKind {
    Markup = 1i64,
    Code = 2i64,
}
impl serde::Serialize for NotebookCellKind {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            NotebookCellKind::Markup => NotebookCellKind::Markup as i64,
            NotebookCellKind::Code => NotebookCellKind::Code as i64,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for NotebookCellKind {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Markup: i64 = NotebookCellKind::Markup as i64;
            const Code: i64 = NotebookCellKind::Code as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Markup => core::result::Result::Ok(NotebookCellKind::Markup),
            discriminant::Code => core::result::Result::Ok(NotebookCellKind::Code),
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected ", " or "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Markup),
                                ::core::fmt::ArgumentV1::new_display(&discriminant::Code),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for NotebookCellKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            NotebookCellKind::Markup => ::core::fmt::Formatter::write_str(f, "Markup"),
            NotebookCellKind::Code => ::core::fmt::Formatter::write_str(f, "Code"),
        }
    }
}
enum ResourceOperationKind {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "rename")]
    Rename,
    #[serde(rename = "delete")]
    Delete,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ResourceOperationKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                ResourceOperationKind::Create => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ResourceOperationKind",
                        0u32,
                        "create",
                    )
                }
                ResourceOperationKind::Rename => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ResourceOperationKind",
                        1u32,
                        "rename",
                    )
                }
                ResourceOperationKind::Delete => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ResourceOperationKind",
                        2u32,
                        "delete",
                    )
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
    impl<'de> _serde::Deserialize<'de> for ResourceOperationKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "create" => _serde::__private::Ok(__Field::__field0),
                        "rename" => _serde::__private::Ok(__Field::__field1),
                        "delete" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"create" => _serde::__private::Ok(__Field::__field0),
                        b"rename" => _serde::__private::Ok(__Field::__field1),
                        b"delete" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ResourceOperationKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ResourceOperationKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#ResourceOperationKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(ResourceOperationKind::Create)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(ResourceOperationKind::Rename)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(ResourceOperationKind::Delete)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["create", "rename", "delete"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ResourceOperationKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ResourceOperationKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for ResourceOperationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ResourceOperationKind::Create => {
                ::core::fmt::Formatter::write_str(f, "Create")
            }
            ResourceOperationKind::Rename => {
                ::core::fmt::Formatter::write_str(f, "Rename")
            }
            ResourceOperationKind::Delete => {
                ::core::fmt::Formatter::write_str(f, "Delete")
            }
        }
    }
}
enum FailureHandlingKind {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "transactional")]
    Transactional,
    #[serde(rename = "textOnlyTransactional")]
    TextOnlyTransactional,
    #[serde(rename = "undo")]
    Undo,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for FailureHandlingKind {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                FailureHandlingKind::Abort => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FailureHandlingKind",
                        0u32,
                        "abort",
                    )
                }
                FailureHandlingKind::Transactional => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FailureHandlingKind",
                        1u32,
                        "transactional",
                    )
                }
                FailureHandlingKind::TextOnlyTransactional => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FailureHandlingKind",
                        2u32,
                        "textOnlyTransactional",
                    )
                }
                FailureHandlingKind::Undo => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "FailureHandlingKind",
                        3u32,
                        "undo",
                    )
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
    impl<'de> _serde::Deserialize<'de> for FailureHandlingKind {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 4",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "abort" => _serde::__private::Ok(__Field::__field0),
                        "transactional" => _serde::__private::Ok(__Field::__field1),
                        "textOnlyTransactional" => {
                            _serde::__private::Ok(__Field::__field2)
                        }
                        "undo" => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"abort" => _serde::__private::Ok(__Field::__field0),
                        b"transactional" => _serde::__private::Ok(__Field::__field1),
                        b"textOnlyTransactional" => {
                            _serde::__private::Ok(__Field::__field2)
                        }
                        b"undo" => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FailureHandlingKind>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FailureHandlingKind;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#FailureHandlingKind",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FailureHandlingKind::Abort)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FailureHandlingKind::Transactional)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(
                                FailureHandlingKind::TextOnlyTransactional,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(FailureHandlingKind::Undo)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &[
                "abort",
                "transactional",
                "textOnlyTransactional",
                "undo",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "FailureHandlingKind",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FailureHandlingKind>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for FailureHandlingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            FailureHandlingKind::Abort => ::core::fmt::Formatter::write_str(f, "Abort"),
            FailureHandlingKind::Transactional => {
                ::core::fmt::Formatter::write_str(f, "Transactional")
            }
            FailureHandlingKind::TextOnlyTransactional => {
                ::core::fmt::Formatter::write_str(f, "TextOnlyTransactional")
            }
            FailureHandlingKind::Undo => ::core::fmt::Formatter::write_str(f, "Undo"),
        }
    }
}
#[repr(i64)]
enum PrepareSupportDefaultBehavior {
    Identifier = 1i64,
}
impl serde::Serialize for PrepareSupportDefaultBehavior {
    #[allow(clippy::use_self)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: i64 = match *self {
            PrepareSupportDefaultBehavior::Identifier => {
                PrepareSupportDefaultBehavior::Identifier as i64
            }
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for PrepareSupportDefaultBehavior {
    #[allow(clippy::use_self)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct discriminant;
        #[allow(non_upper_case_globals)]
        impl discriminant {
            const Identifier: i64 = PrepareSupportDefaultBehavior::Identifier as i64;
        }
        match <i64 as serde::Deserialize>::deserialize(deserializer)? {
            discriminant::Identifier => {
                core::result::Result::Ok(PrepareSupportDefaultBehavior::Identifier)
            }
            other => {
                core::result::Result::Err(
                    serde::de::Error::custom(
                        ::core::fmt::Arguments::new_v1(
                            &["invalid value: ", ", expected "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&other),
                                ::core::fmt::ArgumentV1::new_display(
                                    &discriminant::Identifier,
                                ),
                            ],
                        ),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for PrepareSupportDefaultBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Identifier")
    }
}
enum TokenFormat {
    #[serde(rename = "relative")]
    Relative,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for TokenFormat {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                TokenFormat::Relative => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TokenFormat",
                        0u32,
                        "relative",
                    )
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
    impl<'de> _serde::Deserialize<'de> for TokenFormat {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "relative" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"relative" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TokenFormat>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TokenFormat;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum r#TokenFormat",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(TokenFormat::Relative)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["relative"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TokenFormat",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TokenFormat>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for TokenFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Relative")
    }
}
type Definition = ();
type DefinitionLink = LocationLink;
type LSPArray = Vec<LSPAny>;
type LSPAny = ();
type Declaration = ();
type DeclarationLink = LocationLink;
type InlineValue = ();
type DocumentDiagnosticReport = ();
type PrepareRenameResult = ();
type ProgressToken = ();
type DocumentSelector = Vec<DocumentFilter>;
type ChangeAnnotationIdentifier = String;
type WorkspaceDocumentDiagnosticReport = ();
type TextDocumentContentChangeEvent = ();
type MarkedString = ();
type DocumentFilter = ();
type GlobPattern = ();
type TextDocumentFilter = ();
type NotebookDocumentFilter = ();
type Pattern = String;
struct _a5bc90b4e274fd1dfe6c37f2977a1724fad6dab81ddb997d93176e830c732244314f26ceb5df30335a18650ced74c2785b8a725ac0eaa63822e80a46bfb14714 {
    name: String,
    version: String,
}
struct _7ae7e55a8a79b0971e84ecd7062ed3de19819bafca1fac6bd4d9685d50550ed78eb68d6b22deca9ca6b224119d6b7136f3f896284da13f1b95e1921b9580514f {
    commitCharacters: Vec<String>,
    editRange: (),
    insertTextFormat: InsertTextFormat,
    insertTextMode: InsertTextMode,
    data: LSPAny,
}
struct _ce1090e415d966ccb27f2a997f901efccd17ac40366663a71ce3bb09a5a4e9a63df27ffac45793458022ddeb4b1795ba091724b324e25383fa228f38463258c1 {
    reason: String,
}
struct _1ebbbe894fc68ac5cd2e39a59705aba06f8b8d811e54864460b0a724e3e5d7e2a6a777d1d5ddd22ae3f22f366cc7423997521a2b9da799e8ea90b2f0d65472c6 {
    structure: _0a10a87b5f105469277bb5c4b6c06e2449801e7ab514985adaf9c6cf29d66596b01075fe579af4d7370f28ef7a63e1a70a505de2a163a116f49ea000764265f7,
    data: Vec<NotebookCell>,
    textContent: Vec<
        _549adf68a3ceae636fe0ea94c83fc7a9441bf20b1966d85163009dbf29672f97b569b1883e2d6aaf9bfa47c2196ead593e2c3328f02979d633f26d263a0fb4e9,
    >,
}
struct _0a10a87b5f105469277bb5c4b6c06e2449801e7ab514985adaf9c6cf29d66596b01075fe579af4d7370f28ef7a63e1a70a505de2a163a116f49ea000764265f7 {
    array: NotebookCellArrayChange,
    didOpen: Vec<TextDocumentItem>,
    didClose: Vec<TextDocumentIdentifier>,
}
struct _549adf68a3ceae636fe0ea94c83fc7a9441bf20b1966d85163009dbf29672f97b569b1883e2d6aaf9bfa47c2196ead593e2c3328f02979d633f26d263a0fb4e9 {
    document: VersionedTextDocumentIdentifier,
    changes: Vec<TextDocumentContentChangeEvent>,
}
struct _8e9eec0a047ad6b4edd461f2ca17d65d05c6a610f60dc85839911e9b7b66f8139928ed36fcc97dde7ba973cf5c09af6cf11b1e6f9e24851eefd7bcf7ec577cff {
    name: String,
    version: String,
}
struct _36de340c982b6a081a5bdff8d29d6d8938a1dae81bd312be301a0591a65917dbd16cfc8ef79dfc87d0922ab425f43d23abc8b57ee6d96c3d51e460c825c5c3df {
    workspaceFolders: WorkspaceFoldersServerCapabilities,
    fileOperations: FileOperationOptions,
}
struct _b5e527483acb8945bd5eca8784653bb555de57482d0df5a3748e6b72861e2a5aa154f350c5da05e8b027b51b7f02c0b1f0a11b8f3a138852a8dc032b45499dd6 {
    labelDetailsSupport: bool,
}
struct _7fcf6aa68293a2b4614dc94d58ca257915ce4e0538d72651edf5c458023826a5db214355b4f0dbed7127f6da8a8a5a367f16c009fdd98fde9d2275d229a2c62b {
    cancel: bool,
    retryOnContentModified: Vec<String>,
}
struct _fdffbb0f38f59a1d10cfb8c2569663f1805bfa4fe53b6ef608ad0426abd91bd895266c8b7e777b70e72d17475fd75204e58d3641a56bb349e71c72a36cfcbd56 {
    groupsOnLabel: bool,
}
struct _21d303f52f0333abd47205a7887a5324b848d6157926fd601929f7f72b13919d40c9248962648c13fd146a2e564b6bf4b2972b456bfb77ee25860604f46c39a6 {
    valueSet: Vec<SymbolKind>,
}
struct _806faf2a8ba70a755f5b5e2e06ecb7114b9fdb5cabaf8ad4011da6c3c8b8713c6ce50ae916a542fa24b89270d37b20afe5d495364e786893b612156b5ee71f42 {
    valueSet: Vec<SymbolTag>,
}
struct _e67ced71b750eb19e9db7df593ee2cfb82565f084c734396e196585999a93a93629d24439c0e22b5a315d93888f7026d67ef3e25e00f8ffca289a2502e928d10 {
    properties: Vec<String>,
}
struct _7f1acadde6489bc941a252e04a12b385814bdb081ccd4efb8f8c45529b871eaf7345858db7cee396e5c5d04ba260bcd60dece33e2d38cbf64d1fd31c90611d12 {
    snippetSupport: bool,
    commitCharactersSupport: bool,
    documentationFormat: Vec<MarkupKind>,
    deprecatedSupport: bool,
    preselectSupport: bool,
    tagSupport: _c6fe7fb3b1a0f6b4acee0359636273ac25c3307ee210ddfbb0287966401bf3946bf74c12673d49e44cc9dc49963e80103b412ad180ca25a2ca6704aaf5292000,
    insertReplaceSupport: bool,
    resolveSupport: _4877d657d7e10474af0457c2cb0ce6d0ef7ecc25741ef24dcb9aedb47ac558c548956d56d11ea5977e15b1322216edc4e20e3a14b714c35be41fda84cc6658b1,
    insertTextModeSupport: _36fa2e6aa2c98805a7b99d679ca9f8b5d717f843506f3fd85364a860334e74a5127820fb4e3e76b0e98a893aecadfc6883143c31157e3292eb1a3e34f9b07f09,
    labelDetailsSupport: bool,
}
struct _c6fe7fb3b1a0f6b4acee0359636273ac25c3307ee210ddfbb0287966401bf3946bf74c12673d49e44cc9dc49963e80103b412ad180ca25a2ca6704aaf5292000 {
    valueSet: Vec<CompletionItemTag>,
}
struct _4877d657d7e10474af0457c2cb0ce6d0ef7ecc25741ef24dcb9aedb47ac558c548956d56d11ea5977e15b1322216edc4e20e3a14b714c35be41fda84cc6658b1 {
    properties: Vec<String>,
}
struct _36fa2e6aa2c98805a7b99d679ca9f8b5d717f843506f3fd85364a860334e74a5127820fb4e3e76b0e98a893aecadfc6883143c31157e3292eb1a3e34f9b07f09 {
    valueSet: Vec<InsertTextMode>,
}
struct _112ee96493dbd1af80c741be48287de36e748dfe2787be1d12031a724aa62510ed5c84f8a425273051c0ba775ad9dfae16fad8ab56a9c893c37087ffd0687da1 {
    valueSet: Vec<CompletionItemKind>,
}
struct _f09316013831cd16817cdc4e93ff7742daeb134af0593e2e7528b180c256b0d533dcb2ce0313517b4d0f676069c8ac75cedffde11905e714ff9078f22f314896 {
    itemDefaults: Vec<String>,
}
struct _27c4c3e44bc0d3be9515ede577d8651e493629884f1b2f33bda7070d9f2d262059e94c50b4b86b4e8a2db815e5a42b7c18510dbb588c68a34e8b8d6297bf5322 {
    documentationFormat: Vec<MarkupKind>,
    parameterInformation: _9915f0f4dcc5133298f4f17846804130e16f3a7dff01967426ca041d16f505fd39f6fc39fa5f84f7d05b28e2ad5df632c11243af088a0abfff7343c507514dae,
    activeParameterSupport: bool,
}
struct _9915f0f4dcc5133298f4f17846804130e16f3a7dff01967426ca041d16f505fd39f6fc39fa5f84f7d05b28e2ad5df632c11243af088a0abfff7343c507514dae {
    labelOffsetSupport: bool,
}
struct _b105da1b775754f6969b9af37f32d5dfcbac5f1d88aa9c9676e1d43dc11e35f896acb31001dd77192b857d38bf141d8e161bea90348ff51ef2b60dd8a68fc494 {
    valueSet: Vec<SymbolKind>,
}
struct _162dc8b47d41b280e6a2c8cf5e6cd8d1c17fc12a36fda2fa40caaf9550ec03086fefebd9997dc602f43950f18d1cb4bbe9825758bf69e9d6d1866f4061667ec0 {
    valueSet: Vec<SymbolTag>,
}
struct _f42244e16477cb41bd4d88a591a848939ec6da820b03065797359e167f83d2bf94c32671ebb9f7c5e0afba632c811de68f9fd446e2dcc8798386568af5679bf1 {
    codeActionKind: _77b2044b975da398f7bb419a83cb266711604995a9acf57de863c474567be9d7f3c1f191b617a5531b81257b49ebfcbddcf09416f97e478c604a5794ff11d792,
}
struct _77b2044b975da398f7bb419a83cb266711604995a9acf57de863c474567be9d7f3c1f191b617a5531b81257b49ebfcbddcf09416f97e478c604a5794ff11d792 {
    valueSet: Vec<CodeActionKind>,
}
struct _72613928c93e66504aa7e63a79f7510b3b923b17d0f0536f814cc744ddbceb90335b271a67d869b11d2c01dee698f512d84575c963cbb9bb1ca258dcf50d480f {
    properties: Vec<String>,
}
struct _e7443b70f8dd8f53ef9e6c7718eaedc077237826481a5bba027f2e363e6342685da8048c6e80d804d4468bcc42a0da1626b2810e0f0eee7a7825d009c9292330 {
    valueSet: Vec<FoldingRangeKind>,
}
struct _14a067214339c724729317fe39539073e349f3e1bdc1eb89930d34087aee869e0224609bb296082db201a675d87c185ad9ea577c805f575424de48d9b468995c {
    collapsedText: bool,
}
struct _59b898e0c0c5afe9493233c28c4cf6ab41107db22f1363849c443415161b5c9656b52bc4bd0d5ddd1f647709eb3f16afb99679cf98de4af217ec4c640c8b0ee0 {
    valueSet: Vec<DiagnosticTag>,
}
struct _b1f0972623f5caad1fb4a9450e63197ec12a1f68de46c61d83b7bee46cd04a5ee516229a814a38227093abfb2a162abf4a2cf9fa21de4894cda2d90c50c538f1 {
    range: (),
    full: (),
}
struct _a9cb216a72ad09a7b75a04b96f0336300b7b138f8cf648f1e556d786e1cd9dc607822ebe7132d714e44a651f08f3a9dc28838f68803c1b8eddfc703fdbac1111 {
    properties: Vec<String>,
}
struct _d6d88b24c8a62fc4f6708f29adaefd6e16fa2715626a0a276635eff7cfcefae41123c9efc024620be428440319d37882a201533f77c717722f3a7cb55222fa08 {
    additionalPropertiesSupport: bool,
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
