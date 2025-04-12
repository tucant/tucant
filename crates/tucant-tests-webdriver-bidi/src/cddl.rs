pub struct Command {}
pub struct CommandData {
    pub todo: TODO,
}
pub struct EmptyParams {}
pub struct Message {}
pub struct CommandResponse {}
pub struct ErrorResponse {}
pub struct ResultData {}
pub struct EmptyResult {}
pub struct Event {}
pub struct EventData {
    pub todo: TODO,
}
pub struct Extensible {
    pub TODO1: TODO,
}
pub struct JsInt {}
pub struct JsUint {}
pub struct ErrorCode {}
pub struct SessionCommand {
    pub todo: TODO,
}
pub struct SessionResult {}
pub struct SessionCapabilitiesRequest {}
pub struct SessionCapabilityRequest {}
pub struct SessionProxyConfiguration {}
pub struct SessionAutodetectProxyConfiguration {
    pub proxyType: TODO,
    pub NONE: TODO,
}
pub struct SessionDirectProxyConfiguration {
    pub proxyType: TODO,
    pub NONE: TODO,
}
pub struct SessionManualProxyConfiguration {
    pub proxyType: TODO,
    pub ftpProxy: TODO,
    pub httpProxy: TODO,
    pub sslProxy: TODO,
    pub NONE: TODO,
    pub noProxy: TODO,
    pub NONE: TODO,
}
pub struct SessionSocksProxyConfiguration {
    pub socksProxy: TODO,
    pub socksVersion: TODO,
}
pub struct SessionPacProxyConfiguration {
    pub proxyType: TODO,
    pub proxyAutoconfigUrl: TODO,
    pub NONE: TODO,
}
pub struct SessionSystemProxyConfiguration {
    pub proxyType: TODO,
    pub NONE: TODO,
}
pub struct SessionUserPromptHandler {}
pub struct SessionUserPromptHandlerType {}
pub struct SessionSubscription {}
pub struct SessionSubscriptionRequest {}
pub struct SessionUnsubscribeByIdRequest {}
pub struct SessionUnsubscribeByAttributesRequest {}
pub struct SessionStatus {
    pub method: TODO,
    pub params: TODO,
}
pub struct SessionStatusResult {}
pub struct SessionNew {
    pub method: TODO,
    pub params: TODO,
}
pub struct SessionNewParameters {}
pub struct SessionNewResult {}
pub struct SessionEnd {
    pub method: TODO,
    pub params: TODO,
}
pub struct SessionSubscribe {
    pub method: TODO,
    pub params: TODO,
}
pub struct SessionSubscribeResult {}
pub struct SessionUnsubscribe {
    pub method: TODO,
    pub params: TODO,
}
pub struct SessionUnsubscribeParameters {}
pub struct BrowserCommand {
    pub todo: TODO,
}
pub struct BrowserResult {}
pub struct BrowserClientWindow {}
pub struct BrowserClientWindowInfo {}
pub struct BrowserUserContext {}
pub struct BrowserUserContextInfo {}
pub struct BrowserClose {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowserCreateUserContext {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowserCreateUserContextResult {}
pub struct BrowserGetClientWindows {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowserGetClientWindowsResult {}
pub struct BrowserGetUserContexts {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowserGetUserContextsResult {}
pub struct BrowserRemoveUserContext {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowserRemoveUserContextParameters {}
pub struct BrowserSetClientWindowState {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowserSetClientWindowStateParameters {}
pub struct BrowserClientWindowNamedState {
    pub state: TODO,
}
pub struct BrowserClientWindowRectState {
    pub state: TODO,
    pub width: TODO,
    pub height: TODO,
    pub x: TODO,
    pub y: TODO,
}
pub struct BrowsingContextCommand {
    pub todo: TODO,
}
pub struct BrowsingContextResult {}
pub struct BrowsingContextEvent {
    pub todo: TODO,
}
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
    pub context: TODO,
    pub navigation: TODO,
    pub timestamp: TODO,
    pub url: TODO,
}
pub struct BrowsingContextNavigationInfo {}
pub struct BrowsingContextReadinessState {}
pub struct BrowsingContextUserPromptType {}
pub struct BrowsingContextActivate {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextActivateParameters {}
pub struct BrowsingContextCaptureScreenshot {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextCaptureScreenshotParameters {}
pub struct BrowsingContextImageFormat {}
pub struct BrowsingContextClipRectangle {}
pub struct BrowsingContextElementClipRectangle {}
pub struct BrowsingContextBoxClipRectangle {}
pub struct BrowsingContextCaptureScreenshotResult {}
pub struct BrowsingContextClose {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextCloseParameters {}
pub struct BrowsingContextCreate {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextCreateType {}
pub struct BrowsingContextCreateParameters {}
pub struct BrowsingContextCreateResult {}
pub struct BrowsingContextGetTree {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextGetTreeParameters {}
pub struct BrowsingContextGetTreeResult {}
pub struct BrowsingContextHandleUserPrompt {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextHandleUserPromptParameters {}
pub struct BrowsingContextLocateNodes {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextLocateNodesParameters {}
pub struct BrowsingContextLocateNodesResult {}
pub struct BrowsingContextNavigate {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextNavigateParameters {}
pub struct BrowsingContextNavigateResult {}
pub struct BrowsingContextPrint {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextPrintParameters {}
pub struct BrowsingContextPrintMarginParameters {}
pub struct BrowsingContextPrintPageParameters {}
pub struct BrowsingContextPrintResult {}
pub struct BrowsingContextReload {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextReloadParameters {}
pub struct BrowsingContextSetViewport {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextSetViewportParameters {}
pub struct BrowsingContextViewport {}
pub struct BrowsingContextTraverseHistory {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextTraverseHistoryParameters {}
pub struct BrowsingContextTraverseHistoryResult {}
pub struct BrowsingContextContextCreated {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextContextDestroyed {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextNavigationStarted {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextFragmentNavigated {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextHistoryUpdated {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextHistoryUpdatedParameters {}
pub struct BrowsingContextDomContentLoaded {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextLoad {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextDownloadWillBegin {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextDownloadWillBeginParams {}
pub struct BrowsingContextNavigationAborted {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextNavigationCommitted {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextNavigationFailed {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextUserPromptClosed {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextUserPromptClosedParameters {}
pub struct BrowsingContextUserPromptOpened {
    pub method: TODO,
    pub params: TODO,
}
pub struct BrowsingContextUserPromptOpenedParameters {}
pub struct EmulationCommand {
    pub NONE: TODO,
    pub NONE: TODO,
}
pub struct EmulationSetGeolocationOverride {
    pub method: TODO,
    pub params: TODO,
    pub NONE: TODO,
}
pub struct EmulationSetGeolocationOverrideParameters {}
pub struct EmulationGeolocationCoordinates {}
pub struct NetworkCommand {
    pub todo: TODO,
}
pub struct NetworkResult {}
pub struct NetworkEvent {
    pub todo: TODO,
}
pub struct NetworkAuthChallenge {}
pub struct NetworkAuthCredentials {}
pub struct NetworkBaseParameters {
    pub context: TODO,
    pub isBlocked: TODO,
    pub navigation: TODO,
    pub redirectCount: TODO,
    pub request: TODO,
    pub timestamp: TODO,
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
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkAddInterceptParameters {}
pub struct NetworkInterceptPhase {}
pub struct NetworkAddInterceptResult {}
pub struct NetworkContinueRequest {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkContinueRequestParameters {}
pub struct NetworkContinueResponse {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkContinueResponseParameters {}
pub struct NetworkContinueWithAuth {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkContinueWithAuthParameters {}
pub struct NetworkContinueWithAuthCredentials {
    pub action: TODO,
    pub credentials: TODO,
}
pub struct NetworkContinueWithAuthNoCredentials {
    pub action: TODO,
}
pub struct NetworkFailRequest {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkFailRequestParameters {}
pub struct NetworkProvideResponse {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkProvideResponseParameters {}
pub struct NetworkRemoveIntercept {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkRemoveInterceptParameters {}
pub struct NetworkSetCacheBehavior {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkSetCacheBehaviorParameters {}
pub struct NetworkAuthRequired {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkAuthRequiredParameters {}
pub struct NetworkBeforeRequestSent {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkBeforeRequestSentParameters {}
pub struct NetworkFetchError {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkFetchErrorParameters {}
pub struct NetworkResponseCompleted {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkResponseCompletedParameters {}
pub struct NetworkResponseStarted {
    pub method: TODO,
    pub params: TODO,
}
pub struct NetworkResponseStartedParameters {}
pub struct ScriptCommand {
    pub todo: TODO,
}
pub struct ScriptResult {}
pub struct ScriptEvent {
    pub todo: TODO,
}
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
    pub realm: TODO,
    pub origin: TODO,
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
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptAddPreloadScriptParameters {}
pub struct ScriptAddPreloadScriptResult {}
pub struct ScriptDisown {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptDisownParameters {}
pub struct ScriptCallFunction {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptCallFunctionParameters {}
pub struct ScriptEvaluate {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptEvaluateParameters {}
pub struct ScriptGetRealms {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptGetRealmsParameters {}
pub struct ScriptGetRealmsResult {}
pub struct ScriptRemovePreloadScript {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptRemovePreloadScriptParameters {}
pub struct ScriptMessage {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptMessageParameters {}
pub struct ScriptRealmCreated {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptRealmDestroyed {
    pub method: TODO,
    pub params: TODO,
}
pub struct ScriptRealmDestroyedParameters {}
pub struct StorageCommand {
    pub todo: TODO,
}
pub struct StorageResult {}
pub struct StoragePartitionKey {}
pub struct StorageGetCookies {
    pub method: TODO,
    pub params: TODO,
}
pub struct StorageCookieFilter {}
pub struct StorageBrowsingContextPartitionDescriptor {}
pub struct StorageStorageKeyPartitionDescriptor {}
pub struct StoragePartitionDescriptor {}
pub struct StorageGetCookiesParameters {}
pub struct StorageGetCookiesResult {}
pub struct StorageSetCookie {
    pub method: TODO,
    pub params: TODO,
}
pub struct StoragePartialCookie {}
pub struct StorageSetCookieParameters {}
pub struct StorageSetCookieResult {}
pub struct StorageDeleteCookies {
    pub method: TODO,
    pub params: TODO,
}
pub struct StorageDeleteCookiesParameters {}
pub struct StorageDeleteCookiesResult {}
pub struct LogEvent {}
pub struct LogLevel {}
pub struct LogEntry {}
pub struct LogBaseLogEntry {
    pub level: TODO,
    pub source: TODO,
    pub text: TODO,
    pub timestamp: TODO,
    pub stackTrace: TODO,
}
pub struct LogGenericLogEntry {}
pub struct LogConsoleLogEntry {}
pub struct LogJavascriptLogEntry {}
pub struct LogEntryAdded {
    pub method: TODO,
    pub params: TODO,
}
pub struct InputCommand {
    pub todo: TODO,
}
pub struct InputEvent {}
pub struct InputElementOrigin {}
pub struct InputPerformActions {
    pub method: TODO,
    pub params: TODO,
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
    pub method: TODO,
    pub params: TODO,
}
pub struct InputReleaseActionsParameters {}
pub struct InputSetFiles {
    pub method: TODO,
    pub params: TODO,
}
pub struct InputSetFilesParameters {}
pub struct InputFileDialogOpened {
    pub method: TODO,
    pub params: TODO,
}
pub struct InputFileDialogInfo {}
pub struct WebExtensionCommand {
    pub todo: TODO,
}
pub struct WebExtensionResult {}
pub struct WebExtensionExtension {}
pub struct WebExtensionInstall {
    pub method: TODO,
    pub params: TODO,
}
pub struct WebExtensionInstallParameters {}
pub struct WebExtensionExtensionData {}
pub struct WebExtensionExtensionPath {}
pub struct WebExtensionExtensionArchivePath {}
pub struct WebExtensionExtensionBase64Encoded {}
pub struct WebExtensionInstallResult {}
pub struct WebExtensionUninstall {
    pub method: TODO,
    pub params: TODO,
}
pub struct WebExtensionUninstallParameters {}
