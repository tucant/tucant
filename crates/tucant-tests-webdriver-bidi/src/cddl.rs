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
    pub TODO: Any,
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
    pub NONE: Extensible,
}
pub struct SessionDirectProxyConfiguration {
    pub proxyType: TODO,
    pub NONE: Extensible,
}
pub struct SessionManualProxyConfiguration {
    pub proxyType: TODO,
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
    pub proxyType: TODO,
    pub proxyAutoconfigUrl: Text,
    pub NONE: Extensible,
}
pub struct SessionSystemProxyConfiguration {
    pub proxyType: TODO,
    pub NONE: Extensible,
}
pub struct SessionUserPromptHandler {}
pub struct SessionUserPromptHandlerType {}
pub struct SessionSubscription {}
pub struct SessionSubscriptionRequest {}
pub struct SessionUnsubscribeByIdRequest {}
pub struct SessionUnsubscribeByAttributesRequest {}
pub struct SessionStatus {
    pub method: TODO,
    pub params: EmptyParams,
}
pub struct SessionStatusResult {}
pub struct SessionNew {
    pub method: TODO,
    pub params: SessionNewParameters,
}
pub struct SessionNewParameters {}
pub struct SessionNewResult {}
pub struct SessionEnd {
    pub method: TODO,
    pub params: EmptyParams,
}
pub struct SessionSubscribe {
    pub method: TODO,
    pub params: SessionSubscriptionRequest,
}
pub struct SessionSubscribeResult {}
pub struct SessionUnsubscribe {
    pub method: TODO,
    pub params: SessionUnsubscribeParameters,
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
    pub params: EmptyParams,
}
pub struct BrowserCreateUserContext {
    pub method: TODO,
    pub params: EmptyParams,
}
pub struct BrowserCreateUserContextResult {}
pub struct BrowserGetClientWindows {
    pub method: TODO,
    pub params: EmptyParams,
}
pub struct BrowserGetClientWindowsResult {}
pub struct BrowserGetUserContexts {
    pub method: TODO,
    pub params: EmptyParams,
}
pub struct BrowserGetUserContextsResult {}
pub struct BrowserRemoveUserContext {
    pub method: TODO,
    pub params: BrowserRemoveUserContextParameters,
}
pub struct BrowserRemoveUserContextParameters {}
pub struct BrowserSetClientWindowState {
    pub method: TODO,
    pub params: BrowserSetClientWindowStateParameters,
}
pub struct BrowserSetClientWindowStateParameters {}
pub struct BrowserClientWindowNamedState {
    pub state: TODO,
}
pub struct BrowserClientWindowRectState {
    pub state: TODO,
    pub width: JsUint,
    pub height: JsUint,
    pub x: JsInt,
    pub y: JsInt,
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
    pub context: BrowsingContextBrowsingContext,
    pub navigation: TODO,
    pub timestamp: JsUint,
    pub url: Text,
}
pub struct BrowsingContextNavigationInfo {}
pub struct BrowsingContextReadinessState {}
pub struct BrowsingContextUserPromptType {}
pub struct BrowsingContextActivate {
    pub method: TODO,
    pub params: BrowsingContextActivateParameters,
}
pub struct BrowsingContextActivateParameters {}
pub struct BrowsingContextCaptureScreenshot {
    pub method: TODO,
    pub params: BrowsingContextCaptureScreenshotParameters,
}
pub struct BrowsingContextCaptureScreenshotParameters {}
pub struct BrowsingContextImageFormat {}
pub struct BrowsingContextClipRectangle {}
pub struct BrowsingContextElementClipRectangle {}
pub struct BrowsingContextBoxClipRectangle {}
pub struct BrowsingContextCaptureScreenshotResult {}
pub struct BrowsingContextClose {
    pub method: TODO,
    pub params: BrowsingContextCloseParameters,
}
pub struct BrowsingContextCloseParameters {}
pub struct BrowsingContextCreate {
    pub method: TODO,
    pub params: BrowsingContextCreateParameters,
}
pub struct BrowsingContextCreateType {}
pub struct BrowsingContextCreateParameters {}
pub struct BrowsingContextCreateResult {}
pub struct BrowsingContextGetTree {
    pub method: TODO,
    pub params: BrowsingContextGetTreeParameters,
}
pub struct BrowsingContextGetTreeParameters {}
pub struct BrowsingContextGetTreeResult {}
pub struct BrowsingContextHandleUserPrompt {
    pub method: TODO,
    pub params: BrowsingContextHandleUserPromptParameters,
}
pub struct BrowsingContextHandleUserPromptParameters {}
pub struct BrowsingContextLocateNodes {
    pub method: TODO,
    pub params: BrowsingContextLocateNodesParameters,
}
pub struct BrowsingContextLocateNodesParameters {}
pub struct BrowsingContextLocateNodesResult {}
pub struct BrowsingContextNavigate {
    pub method: TODO,
    pub params: BrowsingContextNavigateParameters,
}
pub struct BrowsingContextNavigateParameters {}
pub struct BrowsingContextNavigateResult {}
pub struct BrowsingContextPrint {
    pub method: TODO,
    pub params: BrowsingContextPrintParameters,
}
pub struct BrowsingContextPrintParameters {}
pub struct BrowsingContextPrintMarginParameters {}
pub struct BrowsingContextPrintPageParameters {}
pub struct BrowsingContextPrintResult {}
pub struct BrowsingContextReload {
    pub method: TODO,
    pub params: BrowsingContextReloadParameters,
}
pub struct BrowsingContextReloadParameters {}
pub struct BrowsingContextSetViewport {
    pub method: TODO,
    pub params: BrowsingContextSetViewportParameters,
}
pub struct BrowsingContextSetViewportParameters {}
pub struct BrowsingContextViewport {}
pub struct BrowsingContextTraverseHistory {
    pub method: TODO,
    pub params: BrowsingContextTraverseHistoryParameters,
}
pub struct BrowsingContextTraverseHistoryParameters {}
pub struct BrowsingContextTraverseHistoryResult {}
pub struct BrowsingContextContextCreated {
    pub method: TODO,
    pub params: BrowsingContextInfo,
}
pub struct BrowsingContextContextDestroyed {
    pub method: TODO,
    pub params: BrowsingContextInfo,
}
pub struct BrowsingContextNavigationStarted {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextFragmentNavigated {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextHistoryUpdated {
    pub method: TODO,
    pub params: BrowsingContextHistoryUpdatedParameters,
}
pub struct BrowsingContextHistoryUpdatedParameters {}
pub struct BrowsingContextDomContentLoaded {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextLoad {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextDownloadWillBegin {
    pub method: TODO,
    pub params: BrowsingContextDownloadWillBeginParams,
}
pub struct BrowsingContextDownloadWillBeginParams {}
pub struct BrowsingContextNavigationAborted {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextNavigationCommitted {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextNavigationFailed {
    pub method: TODO,
    pub params: BrowsingContextNavigationInfo,
}
pub struct BrowsingContextUserPromptClosed {
    pub method: TODO,
    pub params: BrowsingContextUserPromptClosedParameters,
}
pub struct BrowsingContextUserPromptClosedParameters {}
pub struct BrowsingContextUserPromptOpened {
    pub method: TODO,
    pub params: BrowsingContextUserPromptOpenedParameters,
}
pub struct BrowsingContextUserPromptOpenedParameters {}
pub struct EmulationCommand {
    pub NONE: TODO,
    pub NONE: MulationSetGeolocationOverride,
}
pub struct EmulationSetGeolocationOverride {
    pub method: TODO,
    pub params: TODO,
    pub NONE: MulationSetGeolocationOverrideParameters,
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
    pub method: TODO,
    pub params: NetworkAddInterceptParameters,
}
pub struct NetworkAddInterceptParameters {}
pub struct NetworkInterceptPhase {}
pub struct NetworkAddInterceptResult {}
pub struct NetworkContinueRequest {
    pub method: TODO,
    pub params: NetworkContinueRequestParameters,
}
pub struct NetworkContinueRequestParameters {}
pub struct NetworkContinueResponse {
    pub method: TODO,
    pub params: NetworkContinueResponseParameters,
}
pub struct NetworkContinueResponseParameters {}
pub struct NetworkContinueWithAuth {
    pub method: TODO,
    pub params: NetworkContinueWithAuthParameters,
}
pub struct NetworkContinueWithAuthParameters {}
pub struct NetworkContinueWithAuthCredentials {
    pub action: TODO,
    pub credentials: NetworkAuthCredentials,
}
pub struct NetworkContinueWithAuthNoCredentials {
    pub action: TODO,
}
pub struct NetworkFailRequest {
    pub method: TODO,
    pub params: NetworkFailRequestParameters,
}
pub struct NetworkFailRequestParameters {}
pub struct NetworkProvideResponse {
    pub method: TODO,
    pub params: NetworkProvideResponseParameters,
}
pub struct NetworkProvideResponseParameters {}
pub struct NetworkRemoveIntercept {
    pub method: TODO,
    pub params: NetworkRemoveInterceptParameters,
}
pub struct NetworkRemoveInterceptParameters {}
pub struct NetworkSetCacheBehavior {
    pub method: TODO,
    pub params: NetworkSetCacheBehaviorParameters,
}
pub struct NetworkSetCacheBehaviorParameters {}
pub struct NetworkAuthRequired {
    pub method: TODO,
    pub params: NetworkAuthRequiredParameters,
}
pub struct NetworkAuthRequiredParameters {}
pub struct NetworkBeforeRequestSent {
    pub method: TODO,
    pub params: NetworkBeforeRequestSentParameters,
}
pub struct NetworkBeforeRequestSentParameters {}
pub struct NetworkFetchError {
    pub method: TODO,
    pub params: NetworkFetchErrorParameters,
}
pub struct NetworkFetchErrorParameters {}
pub struct NetworkResponseCompleted {
    pub method: TODO,
    pub params: NetworkResponseCompletedParameters,
}
pub struct NetworkResponseCompletedParameters {}
pub struct NetworkResponseStarted {
    pub method: TODO,
    pub params: NetworkResponseStartedParameters,
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
    pub method: TODO,
    pub params: ScriptAddPreloadScriptParameters,
}
pub struct ScriptAddPreloadScriptParameters {}
pub struct ScriptAddPreloadScriptResult {}
pub struct ScriptDisown {
    pub method: TODO,
    pub params: ScriptDisownParameters,
}
pub struct ScriptDisownParameters {}
pub struct ScriptCallFunction {
    pub method: TODO,
    pub params: ScriptCallFunctionParameters,
}
pub struct ScriptCallFunctionParameters {}
pub struct ScriptEvaluate {
    pub method: TODO,
    pub params: ScriptEvaluateParameters,
}
pub struct ScriptEvaluateParameters {}
pub struct ScriptGetRealms {
    pub method: TODO,
    pub params: ScriptGetRealmsParameters,
}
pub struct ScriptGetRealmsParameters {}
pub struct ScriptGetRealmsResult {}
pub struct ScriptRemovePreloadScript {
    pub method: TODO,
    pub params: ScriptRemovePreloadScriptParameters,
}
pub struct ScriptRemovePreloadScriptParameters {}
pub struct ScriptMessage {
    pub method: TODO,
    pub params: ScriptMessageParameters,
}
pub struct ScriptMessageParameters {}
pub struct ScriptRealmCreated {
    pub method: TODO,
    pub params: ScriptRealmInfo,
}
pub struct ScriptRealmDestroyed {
    pub method: TODO,
    pub params: ScriptRealmDestroyedParameters,
}
pub struct ScriptRealmDestroyedParameters {}
pub struct StorageCommand {
    pub todo: TODO,
}
pub struct StorageResult {}
pub struct StoragePartitionKey {}
pub struct StorageGetCookies {
    pub method: TODO,
    pub params: StorageGetCookiesParameters,
}
pub struct StorageCookieFilter {}
pub struct StorageBrowsingContextPartitionDescriptor {}
pub struct StorageStorageKeyPartitionDescriptor {}
pub struct StoragePartitionDescriptor {}
pub struct StorageGetCookiesParameters {}
pub struct StorageGetCookiesResult {}
pub struct StorageSetCookie {
    pub method: TODO,
    pub params: StorageSetCookieParameters,
}
pub struct StoragePartialCookie {}
pub struct StorageSetCookieParameters {}
pub struct StorageSetCookieResult {}
pub struct StorageDeleteCookies {
    pub method: TODO,
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
    pub method: TODO,
    pub params: LogEntry,
}
pub struct InputCommand {
    pub todo: TODO,
}
pub struct InputEvent {}
pub struct InputElementOrigin {}
pub struct InputPerformActions {
    pub method: TODO,
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
    pub method: TODO,
    pub params: InputReleaseActionsParameters,
}
pub struct InputReleaseActionsParameters {}
pub struct InputSetFiles {
    pub method: TODO,
    pub params: InputSetFilesParameters,
}
pub struct InputSetFilesParameters {}
pub struct InputFileDialogOpened {
    pub method: TODO,
    pub params: InputFileDialogInfo,
}
pub struct InputFileDialogInfo {}
pub struct WebExtensionCommand {
    pub todo: TODO,
}
pub struct WebExtensionResult {}
pub struct WebExtensionExtension {}
pub struct WebExtensionInstall {
    pub method: TODO,
    pub params: WebExtensionInstallParameters,
}
pub struct WebExtensionInstallParameters {}
pub struct WebExtensionExtensionData {}
pub struct WebExtensionExtensionPath {}
pub struct WebExtensionExtensionArchivePath {}
pub struct WebExtensionExtensionBase64Encoded {}
pub struct WebExtensionInstallResult {}
pub struct WebExtensionUninstall {
    pub method: TODO,
    pub params: WebExtensionUninstallParameters,
}
pub struct WebExtensionUninstallParameters {}
