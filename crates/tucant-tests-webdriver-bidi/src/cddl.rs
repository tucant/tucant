pub struct Command {}
pub enum CommandData {}
pub struct EmptyParams {}
pub struct Message {}
pub struct CommandResponse {}
pub struct ErrorResponse {}
pub struct ResultData {}
pub struct EmptyResult {}
pub struct Event {}
pub enum EventData {}
pub struct Extensible {
    pub TODO: Any,
}
pub struct JsInt {}
pub struct JsUint {}
pub struct ErrorCode {}
pub enum SessionCommand {}
pub struct SessionResult {}
pub struct SessionCapabilitiesRequest {}
pub struct SessionCapabilityRequest {}
pub struct SessionProxyConfiguration {}
pub struct SessionAutodetectProxyConfiguration {
    pub proxyType: String,
    pub NONE: Extensible,
}
pub struct SessionDirectProxyConfiguration {
    pub proxyType: String,
    pub NONE: Extensible,
}
pub struct SessionManualProxyConfiguration {
    pub proxyType: String,
    pub ftpProxy: Text,
    pub httpProxy: Text,
    pub sslProxy: Text,
    pub NONE: SessionSocksProxyConfiguration,
    pub noProxy: TODO,
    pub NONE: Extensible,
}
pub struct SessionSocksProxyConfiguration {
    pub socksProxy: Text,
    pub socksVersion: TODO,
}
pub struct SessionPacProxyConfiguration {
    pub proxyType: String,
    pub proxyAutoconfigUrl: Text,
    pub NONE: Extensible,
}
pub struct SessionSystemProxyConfiguration {
    pub proxyType: String,
    pub NONE: Extensible,
}
pub struct SessionUserPromptHandler {}
pub struct SessionUserPromptHandlerType {}
pub struct SessionSubscription {}
pub struct SessionSubscriptionRequest {}
pub struct SessionUnsubscribeByIdRequest {}
pub struct SessionUnsubscribeByAttributesRequest {}
pub struct SessionStatus {
    pub method: String,
    pub params: EmptyParams,
}
pub struct SessionStatusResult {}
pub struct SessionNew {
    pub method: String,
    pub params: SessionNewParameters,
}
pub struct SessionNewParameters {}
pub struct SessionNewResult {}
pub struct SessionEnd {
    pub method: String,
    pub params: EmptyParams,
}
pub struct SessionSubscribe {
    pub method: String,
    pub params: SessionSubscriptionRequest,
}
pub struct SessionSubscribeResult {}
pub struct SessionUnsubscribe {
    pub method: String,
    pub params: SessionUnsubscribeParameters,
}
pub struct SessionUnsubscribeParameters {}
pub enum BrowserCommand {}
pub struct BrowserResult {}
pub struct BrowserClientWindow {}
pub struct BrowserClientWindowInfo {}
pub struct BrowserUserContext {}
pub struct BrowserUserContextInfo {}
pub struct BrowserClose {
    pub method: String,
    pub params: EmptyParams,
}
pub struct BrowserCreateUserContext {
    pub method: String,
    pub params: EmptyParams,
}
pub struct BrowserCreateUserContextResult {}
pub struct BrowserGetClientWindows {
    pub method: String,
    pub params: EmptyParams,
}
pub struct BrowserGetClientWindowsResult {}
pub struct BrowserGetUserContexts {
    pub method: String,
    pub params: EmptyParams,
}
pub struct BrowserGetUserContextsResult {}
pub struct BrowserRemoveUserContext {
    pub method: String,
    pub params: BrowserRemoveUserContextParameters,
}
pub struct BrowserRemoveUserContextParameters {}
pub struct BrowserSetClientWindowState {
    pub method: String,
    pub params: BrowserSetClientWindowStateParameters,
}
pub struct BrowserSetClientWindowStateParameters {}
pub struct BrowserClientWindowNamedState {
    pub state: TODO,
}
pub struct BrowserClientWindowRectState {
    pub state: String,
    pub width: JsUint,
    pub height: JsUint,
    pub x: JsInt,
    pub y: JsInt,
}
pub enum BrowsingContextCommand {}
pub struct BrowsingContextResult {}
pub enum BrowsingContextEvent {}
pub struct BrowsingContextBrowsingContext {}
pub struct BrowsingContextInfoList {}
pub struct BrowsingContextInfo {}
pub struct BrowsingContextLocator {}
pub struct BrowsingContextAccessibilityLocator {}
pub struct BrowsingContextCssLocator {}
pub struct BrowsingContextContextLocator {}
pub struct BrowsingContextInnerTextLocator {}
pub struct BrowsingContextXPathLocator {}
pub struct BrowsingContextNavigation {}
pub struct BrowsingContextBaseNavigationInfo {
    pub context: BrowsingContextBrowsingContext,
    pub navigation: TODO,
    pub timestamp: JsUint,
    pub url: Text,
}
pub struct BrowsingContextNavigationInfo {}
pub struct BrowsingContextReadinessState {}
pub struct BrowsingContextUserPromptType {}
pub struct BrowsingContextActivate {
    pub method: String,
    pub params: BrowsingContextActivateParameters,
}
pub struct BrowsingContextActivateParameters {}
pub struct BrowsingContextCaptureScreenshot {
    pub method: String,
    pub params: BrowsingContextCaptureScreenshotParameters,
}
pub struct BrowsingContextCaptureScreenshotParameters {}
pub struct BrowsingContextImageFormat {}
pub struct BrowsingContextClipRectangle {}
pub struct BrowsingContextElementClipRectangle {}
pub struct BrowsingContextBoxClipRectangle {}
pub struct BrowsingContextCaptureScreenshotResult {}
pub struct BrowsingContextClose {
    pub method: String,
    pub params: BrowsingContextCloseParameters,
}
pub struct BrowsingContextCloseParameters {}
pub struct BrowsingContextCreate {
    pub method: String,
    pub params: BrowsingContextCreateParameters,
}
pub struct BrowsingContextCreateType {}
pub struct BrowsingContextCreateParameters {}
pub struct BrowsingContextCreateResult {}
pub struct BrowsingContextGetTree {
    pub method: String,
    pub params: BrowsingContextGetTreeParameters,
}
pub struct BrowsingContextGetTreeParameters {}
pub struct BrowsingContextGetTreeResult {}
pub struct BrowsingContextHandleUserPrompt {
    pub method: String,
    pub params: BrowsingContextHandleUserPromptParameters,
}
pub struct BrowsingContextHandleUserPromptParameters {}
pub struct BrowsingContextLocateNodes {
    pub method: String,
    pub params: BrowsingContextLocateNodesParameters,
}
pub struct BrowsingContextLocateNodesParameters {}
pub struct BrowsingContextLocateNodesResult {}
pub struct BrowsingContextNavigate {
    pub method: String,
    pub params: BrowsingContextNavigateParameters,
}
pub struct BrowsingContextNavigateParameters {}
pub struct BrowsingContextNavigateResult {}
pub struct BrowsingContextPrint {
    pub method: String,
    pub params: BrowsingContextPrintParameters,
}
pub struct BrowsingContextPrintParameters {}
pub struct BrowsingContextPrintMarginParameters {}
pub struct BrowsingContextPrintPageParameters {}
pub struct BrowsingContextPrintResult {}
pub struct BrowsingContextReload {
    pub method: String,
    pub params: BrowsingContextReloadParameters,
}
pub struct BrowsingContextReloadParameters {}
pub struct BrowsingContextSetViewport {
    pub method: String,
    pub params: BrowsingContextSetViewportParameters,
}
pub struct BrowsingContextSetViewportParameters {}
pub struct BrowsingContextViewport {}
pub struct BrowsingContextTraverseHistory {
    pub method: String,
    pub params: BrowsingContextTraverseHistoryParameters,
}
pub struct BrowsingContextTraverseHistoryParameters {}
pub struct BrowsingContextTraverseHistoryResult {}
pub struct BrowsingContextContextCreated {
    pub method: String,
    pub params: BrowsingContextInfo,
}
pub struct BrowsingContextContextDestroyed {
    pub method: String,
    pub params: BrowsingContextInfo,
}
pub struct BrowsingContextNavigationStarted {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextFragmentNavigated {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextHistoryUpdated {
    pub method: String,
    pub params: BrowsingContextHistoryUpdatedParameters,
}
pub struct BrowsingContextHistoryUpdatedParameters {}
pub struct BrowsingContextDomContentLoaded {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextLoad {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextDownloadWillBegin {
    pub method: String,
    pub params: BrowsingContextDownloadWillBeginParams,
}
pub struct BrowsingContextDownloadWillBeginParams {}
pub struct BrowsingContextNavigationAborted {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextNavigationCommitted {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextNavigationFailed {
    pub method: String,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextUserPromptClosed {
    pub method: String,
    pub params: BrowsingContextUserPromptClosedParameters,
}
pub struct BrowsingContextUserPromptClosedParameters {}
pub struct BrowsingContextUserPromptOpened {
    pub method: String,
    pub params: BrowsingContextUserPromptOpenedParameters,
}
pub struct BrowsingContextUserPromptOpenedParameters {}
pub struct EmulationCommand {
    pub NONE: TODO,
    pub NONE: MulationSetGeolocationOverride,
}
pub struct EmulationSetGeolocationOverride {
    pub method: String,
    pub params: TODO,
    pub NONE: MulationSetGeolocationOverrideParameters,
}
pub struct EmulationSetGeolocationOverrideParameters {}
pub struct EmulationGeolocationCoordinates {}
pub enum NetworkCommand {}
pub struct NetworkResult {}
pub enum NetworkEvent {}
pub struct NetworkAuthChallenge {}
pub struct NetworkAuthCredentials {}
pub struct NetworkBaseParameters {
    pub context: TODO,
    pub isBlocked: Bool,
    pub navigation: TODO,
    pub redirectCount: JsUint,
    pub request: NetworkRequestData,
    pub timestamp: JsUint,
    pub intercepts: TODO,
}
pub struct NetworkBytesValue {}
pub struct NetworkStringValue {}
pub struct NetworkBase64Value {}
pub struct NetworkSameSite {}
pub struct NetworkCookie {}
pub struct NetworkCookieHeader {}
pub struct NetworkFetchTimingInfo {}
pub struct NetworkHeader {}
pub struct NetworkInitiator {}
pub struct NetworkIntercept {}
pub struct NetworkRequest {}
pub struct NetworkRequestData {}
pub struct NetworkResponseContent {}
pub struct NetworkResponseData {}
pub struct NetworkSetCookieHeader {}
pub struct NetworkUrlPattern {}
pub struct NetworkUrlPatternPattern {}
pub struct NetworkUrlPatternString {}
pub struct NetworkAddIntercept {
    pub method: String,
    pub params: NetworkAddInterceptParameters,
}
pub struct NetworkAddInterceptParameters {}
pub struct NetworkInterceptPhase {}
pub struct NetworkAddInterceptResult {}
pub struct NetworkContinueRequest {
    pub method: String,
    pub params: NetworkContinueRequestParameters,
}
pub struct NetworkContinueRequestParameters {}
pub struct NetworkContinueResponse {
    pub method: String,
    pub params: NetworkContinueResponseParameters,
}
pub struct NetworkContinueResponseParameters {}
pub struct NetworkContinueWithAuth {
    pub method: String,
    pub params: NetworkContinueWithAuthParameters,
}
pub struct NetworkContinueWithAuthParameters {}
pub struct NetworkContinueWithAuthCredentials {
    pub action: String,
    pub credentials: NetworkAuthCredentials,
}
pub struct NetworkContinueWithAuthNoCredentials {
    pub action: TODO,
}
pub struct NetworkFailRequest {
    pub method: String,
    pub params: NetworkFailRequestParameters,
}
pub struct NetworkFailRequestParameters {}
pub struct NetworkProvideResponse {
    pub method: String,
    pub params: NetworkProvideResponseParameters,
}
pub struct NetworkProvideResponseParameters {}
pub struct NetworkRemoveIntercept {
    pub method: String,
    pub params: NetworkRemoveInterceptParameters,
}
pub struct NetworkRemoveInterceptParameters {}
pub struct NetworkSetCacheBehavior {
    pub method: String,
    pub params: NetworkSetCacheBehaviorParameters,
}
pub struct NetworkSetCacheBehaviorParameters {}
pub struct NetworkAuthRequired {
    pub method: String,
    pub params: NetworkAuthRequiredParameters,
}
pub struct NetworkAuthRequiredParameters {}
pub struct NetworkBeforeRequestSent {
    pub method: String,
    pub params: NetworkBeforeRequestSentParameters,
}
pub struct NetworkBeforeRequestSentParameters {}
pub struct NetworkFetchError {
    pub method: String,
    pub params: NetworkFetchErrorParameters,
}
pub struct NetworkFetchErrorParameters {}
pub struct NetworkResponseCompleted {
    pub method: String,
    pub params: NetworkResponseCompletedParameters,
}
pub struct NetworkResponseCompletedParameters {}
pub struct NetworkResponseStarted {
    pub method: String,
    pub params: NetworkResponseStartedParameters,
}
pub struct NetworkResponseStartedParameters {}
pub enum ScriptCommand {}
pub struct ScriptResult {}
pub enum ScriptEvent {}
pub struct ScriptChannel {}
pub struct ScriptChannelValue {}
pub struct ScriptChannelProperties {}
pub struct ScriptEvaluateResult {}
pub struct ScriptEvaluateResultSuccess {}
pub struct ScriptEvaluateResultException {}
pub struct ScriptExceptionDetails {}
pub struct ScriptHandle {}
pub struct ScriptInternalId {}
pub struct ScriptLocalValue {}
pub struct ScriptListLocalValue {}
pub struct ScriptArrayLocalValue {}
pub struct ScriptDateLocalValue {}
pub struct ScriptMappingLocalValue {}
pub struct ScriptMapLocalValue {}
pub struct ScriptObjectLocalValue {}
pub struct ScriptRegExpValue {}
pub struct ScriptRegExpLocalValue {}
pub struct ScriptSetLocalValue {}
pub struct ScriptPreloadScript {}
pub struct ScriptRealm {}
pub struct ScriptPrimitiveProtocolValue {}
pub struct ScriptUndefinedValue {}
pub struct ScriptNullValue {}
pub struct ScriptStringValue {}
pub struct ScriptSpecialNumber {}
pub struct ScriptNumberValue {}
pub struct ScriptBooleanValue {}
pub struct ScriptBigIntValue {}
pub struct ScriptRealmInfo {}
pub struct ScriptBaseRealmInfo {
    pub realm: ScriptRealm,
    pub origin: Text,
}
pub struct ScriptWindowRealmInfo {}
pub struct ScriptDedicatedWorkerRealmInfo {}
pub struct ScriptSharedWorkerRealmInfo {}
pub struct ScriptServiceWorkerRealmInfo {}
pub struct ScriptWorkerRealmInfo {}
pub struct ScriptPaintWorkletRealmInfo {}
pub struct ScriptAudioWorkletRealmInfo {}
pub struct ScriptWorkletRealmInfo {}
pub struct ScriptRealmType {}
pub struct ScriptRemoteReference {}
pub struct ScriptSharedReference {}
pub struct ScriptRemoteObjectReference {}
pub struct ScriptRemoteValue {}
pub struct ScriptListRemoteValue {}
pub struct ScriptMappingRemoteValue {}
pub struct ScriptSymbolRemoteValue {}
pub struct ScriptArrayRemoteValue {}
pub struct ScriptObjectRemoteValue {}
pub struct ScriptFunctionRemoteValue {}
pub struct ScriptRegExpRemoteValue {}
pub struct ScriptDateRemoteValue {}
pub struct ScriptMapRemoteValue {}
pub struct ScriptSetRemoteValue {}
pub struct ScriptWeakMapRemoteValue {}
pub struct ScriptWeakSetRemoteValue {}
pub struct ScriptGeneratorRemoteValue {}
pub struct ScriptErrorRemoteValue {}
pub struct ScriptProxyRemoteValue {}
pub struct ScriptPromiseRemoteValue {}
pub struct ScriptTypedArrayRemoteValue {}
pub struct ScriptArrayBufferRemoteValue {}
pub struct ScriptNodeListRemoteValue {}
pub struct ScriptHtmlCollectionRemoteValue {}
pub struct ScriptNodeRemoteValue {}
pub struct ScriptNodeProperties {}
pub struct ScriptWindowProxyRemoteValue {}
pub struct ScriptWindowProxyProperties {}
pub struct ScriptResultOwnership {}
pub struct ScriptSerializationOptions {}
pub struct ScriptSharedId {}
pub struct ScriptStackFrame {}
pub struct ScriptStackTrace {}
pub struct ScriptSource {}
pub struct ScriptRealmTarget {}
pub struct ScriptContextTarget {}
pub struct ScriptTarget {}
pub struct ScriptAddPreloadScript {
    pub method: String,
    pub params: ScriptAddPreloadScriptParameters,
}
pub struct ScriptAddPreloadScriptParameters {}
pub struct ScriptAddPreloadScriptResult {}
pub struct ScriptDisown {
    pub method: String,
    pub params: ScriptDisownParameters,
}
pub struct ScriptDisownParameters {}
pub struct ScriptCallFunction {
    pub method: String,
    pub params: ScriptCallFunctionParameters,
}
pub struct ScriptCallFunctionParameters {}
pub struct ScriptEvaluate {
    pub method: String,
    pub params: ScriptEvaluateParameters,
}
pub struct ScriptEvaluateParameters {}
pub struct ScriptGetRealms {
    pub method: String,
    pub params: ScriptGetRealmsParameters,
}
pub struct ScriptGetRealmsParameters {}
pub struct ScriptGetRealmsResult {}
pub struct ScriptRemovePreloadScript {
    pub method: String,
    pub params: ScriptRemovePreloadScriptParameters,
}
pub struct ScriptRemovePreloadScriptParameters {}
pub struct ScriptMessage {
    pub method: String,
    pub params: ScriptMessageParameters,
}
pub struct ScriptMessageParameters {}
pub struct ScriptRealmCreated {
    pub method: String,
    pub params: ScriptRealmInfo,
}
pub struct ScriptRealmDestroyed {
    pub method: String,
    pub params: ScriptRealmDestroyedParameters,
}
pub struct ScriptRealmDestroyedParameters {}
pub enum StorageCommand {}
pub struct StorageResult {}
pub struct StoragePartitionKey {}
pub struct StorageGetCookies {
    pub method: String,
    pub params: StorageGetCookiesParameters,
}
pub struct StorageCookieFilter {}
pub struct StorageBrowsingContextPartitionDescriptor {}
pub struct StorageStorageKeyPartitionDescriptor {}
pub struct StoragePartitionDescriptor {}
pub struct StorageGetCookiesParameters {}
pub struct StorageGetCookiesResult {}
pub struct StorageSetCookie {
    pub method: String,
    pub params: StorageSetCookieParameters,
}
pub struct StoragePartialCookie {}
pub struct StorageSetCookieParameters {}
pub struct StorageSetCookieResult {}
pub struct StorageDeleteCookies {
    pub method: String,
    pub params: StorageDeleteCookiesParameters,
}
pub struct StorageDeleteCookiesParameters {}
pub struct StorageDeleteCookiesResult {}
pub struct LogEvent {}
pub struct LogLevel {}
pub struct LogEntry {}
pub struct LogBaseLogEntry {
    pub level: LogLevel,
    pub source: ScriptSource,
    pub text: TODO,
    pub timestamp: JsUint,
    pub stackTrace: ScriptStackTrace,
}
pub struct LogGenericLogEntry {}
pub struct LogConsoleLogEntry {}
pub struct LogJavascriptLogEntry {}
pub struct LogEntryAdded {
    pub method: String,
    pub params: LogEntry,
}
pub enum InputCommand {}
pub struct InputEvent {}
pub struct InputElementOrigin {}
pub struct InputPerformActions {
    pub method: String,
    pub params: InputPerformActionsParameters,
}
pub struct InputPerformActionsParameters {}
pub struct InputSourceActions {}
pub struct InputNoneSourceActions {}
pub struct InputNoneSourceAction {}
pub struct InputKeySourceActions {}
pub struct InputKeySourceAction {}
pub struct InputPointerSourceActions {}
pub struct InputPointerType {}
pub struct InputPointerParameters {}
pub struct InputPointerSourceAction {}
pub struct InputWheelSourceActions {}
pub struct InputWheelSourceAction {}
pub struct InputPauseAction {}
pub struct InputKeyDownAction {}
pub struct InputKeyUpAction {}
pub struct InputPointerUpAction {}
pub struct InputPointerDownAction {}
pub struct InputPointerMoveAction {}
pub struct InputWheelScrollAction {}
pub struct InputPointerCommonProperties {
    pub width: TODO,
    pub height: TODO,
    pub pressure: TODO,
    pub tangentialPressure: TODO,
    pub twist: TODO,
    pub altitudeAngle: TODO,
    pub azimuthAngle: TODO,
}
pub struct InputOrigin {}
pub struct InputReleaseActions {
    pub method: String,
    pub params: InputReleaseActionsParameters,
}
pub struct InputReleaseActionsParameters {}
pub struct InputSetFiles {
    pub method: String,
    pub params: InputSetFilesParameters,
}
pub struct InputSetFilesParameters {}
pub struct InputFileDialogOpened {
    pub method: String,
    pub params: InputFileDialogInfo,
}
pub struct InputFileDialogInfo {}
pub enum WebExtensionCommand {}
pub struct WebExtensionResult {}
pub struct WebExtensionExtension {}
pub struct WebExtensionInstall {
    pub method: String,
    pub params: WebExtensionInstallParameters,
}
pub struct WebExtensionInstallParameters {}
pub struct WebExtensionExtensionData {}
pub struct WebExtensionExtensionPath {}
pub struct WebExtensionExtensionArchivePath {}
pub struct WebExtensionExtensionBase64Encoded {}
pub struct WebExtensionInstallResult {}
pub struct WebExtensionUninstall {
    pub method: String,
    pub params: WebExtensionUninstallParameters,
}
pub struct WebExtensionUninstallParameters {}
