/// https://www.rfc-editor.org/rfc/rfc8610#appendix-D
pub struct TODO;
pub type Text = String;
pub type Any = serde_json::Value;
pub struct r#Command {
    pub r#id: r#JsUint,
    #[serde(flatten)]
    pub r#command_data: r#CommandData,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub enum r#CommandData {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub struct r#EmptyParams {
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub type r#Message = TODO;
pub struct r#CommandResponse {
    pub r#type: String,
    pub r#id: r#JsUint,
    pub r#result: r#ResultData,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#ErrorResponse {
    pub r#type: String,
    pub r#id: TODO,
    pub r#error: r#ErrorCode,
    pub r#message: r#Text,
    pub r#stacktrace: r#Text,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub type r#ResultData = TODO;
pub struct r#EmptyResult {
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#Event {
    pub r#type: String,
    #[serde(flatten)]
    pub r#event_data: r#EventData,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub enum r#EventData {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub struct r#Extensible {
    pub TODO: r#Any,
}
pub type r#JsInt = TODO;
pub type r#JsUint = TODO;
pub type r#ErrorCode = TODO;
pub enum r#SessionCommand {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub type r#SessionResult = TODO;
pub struct r#SessionCapabilitiesRequest {
    pub r#alwaysMatch: r#SessionCapabilityRequest,
    pub r#firstMatch: TODO,
}
pub struct r#SessionCapabilityRequest {
    pub r#acceptInsecureCerts: r#Bool,
    pub r#browserName: r#Text,
    pub r#browserVersion: r#Text,
    pub r#platformName: r#Text,
    pub r#proxy: r#SessionProxyConfiguration,
    pub r#unhandledPromptBehavior: r#SessionUserPromptHandler,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub enum r#SessionProxyConfiguration {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub struct r#SessionAutodetectProxyConfiguration {
    pub r#proxyType: String,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#SessionDirectProxyConfiguration {
    pub r#proxyType: String,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#SessionManualProxyConfiguration {
    pub r#proxyType: String,
    pub r#ftpProxy: r#Text,
    pub r#httpProxy: r#Text,
    pub r#sslProxy: r#Text,
    #[serde(flatten)]
    pub r#session_socks_proxy_configuration: r#SessionSocksProxyConfiguration,
    pub r#noProxy: TODO,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#SessionSocksProxyConfiguration {
    pub r#socksProxy: r#Text,
    pub r#socksVersion: TODO,
}
pub struct r#SessionPacProxyConfiguration {
    pub r#proxyType: String,
    pub r#proxyAutoconfigUrl: r#Text,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#SessionSystemProxyConfiguration {
    pub r#proxyType: String,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#SessionUserPromptHandler {
    pub r#alert: r#SessionUserPromptHandlerType,
    pub r#beforeUnload: r#SessionUserPromptHandlerType,
    pub r#confirm: r#SessionUserPromptHandlerType,
    pub r#default: r#SessionUserPromptHandlerType,
    pub r#file: r#SessionUserPromptHandlerType,
    pub r#prompt: r#SessionUserPromptHandlerType,
}
pub type r#SessionUserPromptHandlerType = TODO;
pub type r#SessionSubscription = TODO;
pub struct r#SessionSubscriptionRequest {
    pub r#events: TODO,
    pub r#contexts: TODO,
    pub r#userContexts: TODO,
}
pub struct r#SessionUnsubscribeByIdRequest {
    pub r#subscriptions: TODO,
}
pub struct r#SessionUnsubscribeByAttributesRequest {
    pub r#events: TODO,
    pub r#contexts: TODO,
}
pub struct r#SessionStatus {
    pub r#method: String,
    pub r#params: r#EmptyParams,
}
pub struct r#SessionStatusResult {
    pub r#ready: r#Bool,
    pub r#message: r#Text,
}
pub struct r#SessionNew {
    pub r#method: String,
    pub r#params: r#SessionNewParameters,
}
pub struct r#SessionNewParameters {
    pub r#capabilities: r#SessionCapabilitiesRequest,
}
pub struct r#SessionNewResult {
    pub r#sessionId: r#Text,
    pub r#capabilities: TODO,
}
pub struct r#SessionEnd {
    pub r#method: String,
    pub r#params: r#EmptyParams,
}
pub struct r#SessionSubscribe {
    pub r#method: String,
    pub r#params: r#SessionSubscriptionRequest,
}
pub struct r#SessionSubscribeResult {
    pub r#subscription: r#SessionSubscription,
}
pub struct r#SessionUnsubscribe {
    pub r#method: String,
    pub r#params: r#SessionUnsubscribeParameters,
}
pub type r#SessionUnsubscribeParameters = TODO;
pub enum r#BrowserCommand {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub type r#BrowserResult = TODO;
pub type r#BrowserClientWindow = TODO;
pub struct r#BrowserClientWindowInfo {
    pub r#active: r#Bool,
    pub r#clientWindow: r#BrowserClientWindow,
    pub r#height: r#JsUint,
    pub r#state: TODO,
    pub r#width: r#JsUint,
    pub r#x: r#JsInt,
    pub r#y: r#JsInt,
}
pub type r#BrowserUserContext = TODO;
pub struct r#BrowserUserContextInfo {
    pub r#userContext: r#BrowserUserContext,
}
pub struct r#BrowserClose {
    pub r#method: String,
    pub r#params: r#EmptyParams,
}
pub struct r#BrowserCreateUserContext {
    pub r#method: String,
    pub r#params: r#EmptyParams,
}
pub type r#BrowserCreateUserContextResult = TODO;
pub struct r#BrowserGetClientWindows {
    pub r#method: String,
    pub r#params: r#EmptyParams,
}
pub struct r#BrowserGetClientWindowsResult {
    pub r#clientWindows: TODO,
}
pub struct r#BrowserGetUserContexts {
    pub r#method: String,
    pub r#params: r#EmptyParams,
}
pub struct r#BrowserGetUserContextsResult {
    pub r#userContexts: TODO,
}
pub struct r#BrowserRemoveUserContext {
    pub r#method: String,
    pub r#params: r#BrowserRemoveUserContextParameters,
}
pub struct r#BrowserRemoveUserContextParameters {
    pub r#userContext: r#BrowserUserContext,
}
pub struct r#BrowserSetClientWindowState {
    pub r#method: String,
    pub r#params: r#BrowserSetClientWindowStateParameters,
}
pub struct r#BrowserSetClientWindowStateParameters {
    pub r#clientWindow: r#BrowserClientWindow,
    pub todo: TODO,
}
pub struct r#BrowserClientWindowNamedState {
    pub r#state: TODO,
}
pub struct r#BrowserClientWindowRectState {
    pub r#state: String,
    pub r#width: r#JsUint,
    pub r#height: r#JsUint,
    pub r#x: r#JsInt,
    pub r#y: r#JsInt,
}
pub enum r#BrowsingContextCommand {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub type r#BrowsingContextResult = TODO;
pub enum r#BrowsingContextEvent {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub type r#BrowsingContextBrowsingContext = TODO;
pub struct r#BrowsingContextInfoList {
    #[serde(flatten)]
    pub r#browsing_context_info: r#BrowsingContextInfo,
}
pub struct r#BrowsingContextInfo {
    pub r#children: TODO,
    pub r#clientWindow: r#BrowserClientWindow,
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#originalOpener: TODO,
    pub r#url: r#Text,
    pub r#userContext: r#BrowserUserContext,
    pub r#parent: TODO,
}
pub type r#BrowsingContextLocator = TODO;
pub struct r#BrowsingContextAccessibilityLocator {
    pub r#type: String,
    pub r#value: TODO,
}
pub struct r#BrowsingContextCssLocator {
    pub r#type: String,
    pub r#value: r#Text,
}
pub struct r#BrowsingContextContextLocator {
    pub r#type: String,
    pub r#value: TODO,
}
pub struct r#BrowsingContextInnerTextLocator {
    pub r#type: String,
    pub r#value: r#Text,
    pub r#ignoreCase: r#Bool,
    pub r#matchType: TODO,
    pub r#maxDepth: r#JsUint,
}
pub struct r#BrowsingContextXPathLocator {
    pub r#type: String,
    pub r#value: r#Text,
}
pub type r#BrowsingContextNavigation = TODO;
pub struct r#BrowsingContextBaseNavigationInfo {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#navigation: TODO,
    pub r#timestamp: r#JsUint,
    pub r#url: r#Text,
}
pub struct r#BrowsingContextNavigationInfo {
    #[serde(flatten)]
    pub r#browsing_context_base_navigation_info: r#BrowsingContextBaseNavigationInfo,
}
pub type r#BrowsingContextReadinessState = TODO;
pub type r#BrowsingContextUserPromptType = TODO;
pub struct r#BrowsingContextActivate {
    pub r#method: String,
    pub r#params: r#BrowsingContextActivateParameters,
}
pub struct r#BrowsingContextActivateParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
}
pub struct r#BrowsingContextCaptureScreenshot {
    pub r#method: String,
    pub r#params: r#BrowsingContextCaptureScreenshotParameters,
}
pub struct r#BrowsingContextCaptureScreenshotParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#origin: TODO,
    pub r#format: r#BrowsingContextImageFormat,
    pub r#clip: r#BrowsingContextClipRectangle,
}
pub struct r#BrowsingContextImageFormat {
    pub r#type: r#Text,
    pub r#quality: TODO,
}
pub type r#BrowsingContextClipRectangle = TODO;
pub struct r#BrowsingContextElementClipRectangle {
    pub r#type: String,
    pub r#element: r#ScriptSharedReference,
}
pub struct r#BrowsingContextBoxClipRectangle {
    pub r#type: String,
    pub r#x: r#Float,
    pub r#y: r#Float,
    pub r#width: r#Float,
    pub r#height: r#Float,
}
pub struct r#BrowsingContextCaptureScreenshotResult {
    pub r#data: r#Text,
}
pub struct r#BrowsingContextClose {
    pub r#method: String,
    pub r#params: r#BrowsingContextCloseParameters,
}
pub struct r#BrowsingContextCloseParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#promptUnload: TODO,
}
pub struct r#BrowsingContextCreate {
    pub r#method: String,
    pub r#params: r#BrowsingContextCreateParameters,
}
pub type r#BrowsingContextCreateType = TODO;
pub struct r#BrowsingContextCreateParameters {
    pub r#type: r#BrowsingContextCreateType,
    pub r#referenceContext: r#BrowsingContextBrowsingContext,
    pub r#background: TODO,
    pub r#userContext: r#BrowserUserContext,
}
pub struct r#BrowsingContextCreateResult {
    pub r#context: r#BrowsingContextBrowsingContext,
}
pub struct r#BrowsingContextGetTree {
    pub r#method: String,
    pub r#params: r#BrowsingContextGetTreeParameters,
}
pub struct r#BrowsingContextGetTreeParameters {
    pub r#maxDepth: r#JsUint,
    pub r#root: r#BrowsingContextBrowsingContext,
}
pub struct r#BrowsingContextGetTreeResult {
    pub r#contexts: r#BrowsingContextInfoList,
}
pub struct r#BrowsingContextHandleUserPrompt {
    pub r#method: String,
    pub r#params: r#BrowsingContextHandleUserPromptParameters,
}
pub struct r#BrowsingContextHandleUserPromptParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#accept: r#Bool,
    pub r#userText: r#Text,
}
pub struct r#BrowsingContextLocateNodes {
    pub r#method: String,
    pub r#params: r#BrowsingContextLocateNodesParameters,
}
pub struct r#BrowsingContextLocateNodesParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#locator: r#BrowsingContextLocator,
    pub r#maxNodeCount: TODO,
    pub r#serializationOptions: r#ScriptSerializationOptions,
    pub r#startNodes: TODO,
}
pub struct r#BrowsingContextLocateNodesResult {
    pub r#nodes: TODO,
}
pub struct r#BrowsingContextNavigate {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigateParameters,
}
pub struct r#BrowsingContextNavigateParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#url: r#Text,
    pub r#wait: r#BrowsingContextReadinessState,
}
pub struct r#BrowsingContextNavigateResult {
    pub r#navigation: TODO,
    pub r#url: r#Text,
}
pub struct r#BrowsingContextPrint {
    pub r#method: String,
    pub r#params: r#BrowsingContextPrintParameters,
}
pub struct r#BrowsingContextPrintParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#background: TODO,
    pub r#margin: r#BrowsingContextPrintMarginParameters,
    pub r#orientation: TODO,
    pub r#page: r#BrowsingContextPrintPageParameters,
    pub r#pageRanges: TODO,
    pub r#scale: TODO,
    pub r#shrinkToFit: TODO,
}
pub struct r#BrowsingContextPrintMarginParameters {
    pub r#bottom: TODO,
    pub r#left: TODO,
    pub r#right: TODO,
    pub r#top: TODO,
}
pub struct r#BrowsingContextPrintPageParameters {
    pub r#height: TODO,
    pub r#width: TODO,
}
pub struct r#BrowsingContextPrintResult {
    pub r#data: r#Text,
}
pub struct r#BrowsingContextReload {
    pub r#method: String,
    pub r#params: r#BrowsingContextReloadParameters,
}
pub struct r#BrowsingContextReloadParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#ignoreCache: r#Bool,
    pub r#wait: r#BrowsingContextReadinessState,
}
pub struct r#BrowsingContextSetViewport {
    pub r#method: String,
    pub r#params: r#BrowsingContextSetViewportParameters,
}
pub struct r#BrowsingContextSetViewportParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#viewport: TODO,
    pub r#devicePixelRatio: TODO,
    pub r#userContexts: TODO,
}
pub struct r#BrowsingContextViewport {
    pub r#width: r#JsUint,
    pub r#height: r#JsUint,
}
pub struct r#BrowsingContextTraverseHistory {
    pub r#method: String,
    pub r#params: r#BrowsingContextTraverseHistoryParameters,
}
pub struct r#BrowsingContextTraverseHistoryParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#delta: r#JsInt,
}
pub struct r#BrowsingContextTraverseHistoryResult {}
pub struct r#BrowsingContextContextCreated {
    pub r#method: String,
    pub r#params: r#BrowsingContextInfo,
}
pub struct r#BrowsingContextContextDestroyed {
    pub r#method: String,
    pub r#params: r#BrowsingContextInfo,
}
pub struct r#BrowsingContextNavigationStarted {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextFragmentNavigated {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextHistoryUpdated {
    pub r#method: String,
    pub r#params: r#BrowsingContextHistoryUpdatedParameters,
}
pub struct r#BrowsingContextHistoryUpdatedParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#url: r#Text,
}
pub struct r#BrowsingContextDomContentLoaded {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextLoad {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextDownloadWillBegin {
    pub r#method: String,
    pub r#params: r#BrowsingContextDownloadWillBeginParams,
}
pub struct r#BrowsingContextDownloadWillBeginParams {
    pub r#suggestedFilename: r#Text,
    #[serde(flatten)]
    pub r#browsing_context_base_navigation_info: r#BrowsingContextBaseNavigationInfo,
}
pub struct r#BrowsingContextNavigationAborted {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextNavigationCommitted {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextNavigationFailed {
    pub r#method: String,
    pub r#params: r#BrowsingContextNavigationInfo,
}
pub struct r#BrowsingContextUserPromptClosed {
    pub r#method: String,
    pub r#params: r#BrowsingContextUserPromptClosedParameters,
}
pub struct r#BrowsingContextUserPromptClosedParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#accepted: r#Bool,
    pub r#type: r#BrowsingContextUserPromptType,
    pub r#userText: r#Text,
}
pub struct r#BrowsingContextUserPromptOpened {
    pub r#method: String,
    pub r#params: r#BrowsingContextUserPromptOpenedParameters,
}
pub struct r#BrowsingContextUserPromptOpenedParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#handler: r#SessionUserPromptHandlerType,
    pub r#message: r#Text,
    pub r#type: r#BrowsingContextUserPromptType,
    pub r#defaultValue: r#Text,
}
pub struct r#EmulationCommand {
    pub NO_KEY: TODO,
    #[serde(flatten)]
    pub r#mulation_set_geolocation_override: r#MulationSetGeolocationOverride,
}
pub struct r#EmulationSetGeolocationOverride {
    pub r#method: String,
    pub r#params: TODO,
    #[serde(flatten)]
    pub r#mulation_set_geolocation_override_parameters: r#MulationSetGeolocationOverrideParameters,
}
pub struct r#EmulationSetGeolocationOverrideParameters {
    pub r#coordinates: TODO,
    pub NO_KEY: TODO,
    pub r#contexts: TODO,
    pub r#userContexts: TODO,
}
pub struct r#EmulationGeolocationCoordinates {
    pub r#latitude: r#Float,
    pub r#longitude: r#Float,
    pub r#accuracy: TODO,
    pub r#altitude: TODO,
    pub r#altitudeAccuracy: TODO,
    pub r#heading: TODO,
    pub r#speed: TODO,
}
pub enum r#NetworkCommand {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub type r#NetworkResult = TODO;
pub enum r#NetworkEvent {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub struct r#NetworkAuthChallenge {
    pub r#scheme: r#Text,
    pub r#realm: r#Text,
}
pub struct r#NetworkAuthCredentials {
    pub r#type: String,
    pub r#username: r#Text,
    pub r#password: r#Text,
}
pub struct r#NetworkBaseParameters {
    pub r#context: TODO,
    pub r#isBlocked: r#Bool,
    pub r#navigation: TODO,
    pub r#redirectCount: r#JsUint,
    pub r#request: r#NetworkRequestData,
    pub r#timestamp: r#JsUint,
    pub r#intercepts: TODO,
}
pub type r#NetworkBytesValue = TODO;
pub struct r#NetworkStringValue {
    pub r#type: String,
    pub r#value: r#Text,
}
pub struct r#NetworkBase64Value {
    pub r#type: String,
    pub r#value: r#Text,
}
pub type r#NetworkSameSite = TODO;
pub struct r#NetworkCookie {
    pub r#name: r#Text,
    pub r#value: r#NetworkBytesValue,
    pub r#domain: r#Text,
    pub r#path: r#Text,
    pub r#size: r#JsUint,
    pub r#httpOnly: r#Bool,
    pub r#secure: r#Bool,
    pub r#sameSite: r#NetworkSameSite,
    pub r#expiry: r#JsUint,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#NetworkCookieHeader {
    pub r#name: r#Text,
    pub r#value: r#NetworkBytesValue,
}
pub struct r#NetworkFetchTimingInfo {
    pub r#timeOrigin: r#Float,
    pub r#requestTime: r#Float,
    pub r#redirectStart: r#Float,
    pub r#redirectEnd: r#Float,
    pub r#fetchStart: r#Float,
    pub r#dnsStart: r#Float,
    pub r#dnsEnd: r#Float,
    pub r#connectStart: r#Float,
    pub r#connectEnd: r#Float,
    pub r#tlsStart: r#Float,
    pub r#requestStart: r#Float,
    pub r#responseStart: r#Float,
    pub r#responseEnd: r#Float,
}
pub struct r#NetworkHeader {
    pub r#name: r#Text,
    pub r#value: r#NetworkBytesValue,
}
pub struct r#NetworkInitiator {
    pub r#columnNumber: r#JsUint,
    pub r#lineNumber: r#JsUint,
    pub r#request: r#NetworkRequest,
    pub r#stackTrace: r#ScriptStackTrace,
    pub r#type: TODO,
}
pub type r#NetworkIntercept = TODO;
pub type r#NetworkRequest = TODO;
pub struct r#NetworkRequestData {
    pub r#request: r#NetworkRequest,
    pub r#url: r#Text,
    pub r#method: r#Text,
    pub r#headers: TODO,
    pub r#cookies: TODO,
    pub r#headersSize: r#JsUint,
    pub r#bodySize: TODO,
    pub r#destination: r#Text,
    pub r#initiatorType: TODO,
    pub r#timings: r#NetworkFetchTimingInfo,
}
pub struct r#NetworkResponseContent {
    pub r#size: r#JsUint,
}
pub struct r#NetworkResponseData {
    pub r#url: r#Text,
    pub r#protocol: r#Text,
    pub r#status: r#JsUint,
    pub r#statusText: r#Text,
    pub r#fromCache: r#Bool,
    pub r#headers: TODO,
    pub r#mimeType: r#Text,
    pub r#bytesReceived: r#JsUint,
    pub r#headersSize: TODO,
    pub r#bodySize: TODO,
    pub r#content: r#NetworkResponseContent,
    pub r#authChallenges: TODO,
}
pub struct r#NetworkSetCookieHeader {
    pub r#name: r#Text,
    pub r#value: r#NetworkBytesValue,
    pub r#domain: r#Text,
    pub r#httpOnly: r#Bool,
    pub r#expiry: r#Text,
    pub r#maxAge: r#JsInt,
    pub r#path: r#Text,
    pub r#sameSite: r#NetworkSameSite,
    pub r#secure: r#Bool,
}
pub type r#NetworkUrlPattern = TODO;
pub struct r#NetworkUrlPatternPattern {
    pub r#type: String,
    pub r#protocol: r#Text,
    pub r#hostname: r#Text,
    pub r#port: r#Text,
    pub r#pathname: r#Text,
    pub r#search: r#Text,
}
pub struct r#NetworkUrlPatternString {
    pub r#type: String,
    pub r#pattern: r#Text,
}
pub struct r#NetworkAddIntercept {
    pub r#method: String,
    pub r#params: r#NetworkAddInterceptParameters,
}
pub struct r#NetworkAddInterceptParameters {
    pub r#phases: TODO,
    pub r#contexts: TODO,
    pub r#urlPatterns: TODO,
}
pub type r#NetworkInterceptPhase = TODO;
pub struct r#NetworkAddInterceptResult {
    pub r#intercept: r#NetworkIntercept,
}
pub struct r#NetworkContinueRequest {
    pub r#method: String,
    pub r#params: r#NetworkContinueRequestParameters,
}
pub struct r#NetworkContinueRequestParameters {
    pub r#request: r#NetworkRequest,
    pub r#body: r#NetworkBytesValue,
    pub r#cookies: TODO,
    pub r#headers: TODO,
    pub r#method: r#Text,
    pub r#url: r#Text,
}
pub struct r#NetworkContinueResponse {
    pub r#method: String,
    pub r#params: r#NetworkContinueResponseParameters,
}
pub struct r#NetworkContinueResponseParameters {
    pub r#request: r#NetworkRequest,
    pub r#cookies: TODO,
    pub r#credentials: r#NetworkAuthCredentials,
    pub r#headers: TODO,
    pub r#reasonPhrase: r#Text,
    pub r#statusCode: r#JsUint,
}
pub struct r#NetworkContinueWithAuth {
    pub r#method: String,
    pub r#params: r#NetworkContinueWithAuthParameters,
}
pub struct r#NetworkContinueWithAuthParameters {
    pub r#request: r#NetworkRequest,
    pub todo: TODO,
}
pub struct r#NetworkContinueWithAuthCredentials {
    pub r#action: String,
    pub r#credentials: r#NetworkAuthCredentials,
}
pub struct r#NetworkContinueWithAuthNoCredentials {
    pub r#action: TODO,
}
pub struct r#NetworkFailRequest {
    pub r#method: String,
    pub r#params: r#NetworkFailRequestParameters,
}
pub struct r#NetworkFailRequestParameters {
    pub r#request: r#NetworkRequest,
}
pub struct r#NetworkProvideResponse {
    pub r#method: String,
    pub r#params: r#NetworkProvideResponseParameters,
}
pub struct r#NetworkProvideResponseParameters {
    pub r#request: r#NetworkRequest,
    pub r#body: r#NetworkBytesValue,
    pub r#cookies: TODO,
    pub r#headers: TODO,
    pub r#reasonPhrase: r#Text,
    pub r#statusCode: r#JsUint,
}
pub struct r#NetworkRemoveIntercept {
    pub r#method: String,
    pub r#params: r#NetworkRemoveInterceptParameters,
}
pub struct r#NetworkRemoveInterceptParameters {
    pub r#intercept: r#NetworkIntercept,
}
pub struct r#NetworkSetCacheBehavior {
    pub r#method: String,
    pub r#params: r#NetworkSetCacheBehaviorParameters,
}
pub struct r#NetworkSetCacheBehaviorParameters {
    pub r#cacheBehavior: TODO,
    pub r#contexts: TODO,
}
pub struct r#NetworkAuthRequired {
    pub r#method: String,
    pub r#params: r#NetworkAuthRequiredParameters,
}
pub struct r#NetworkAuthRequiredParameters {
    #[serde(flatten)]
    pub r#network_base_parameters: r#NetworkBaseParameters,
    pub r#response: r#NetworkResponseData,
}
pub struct r#NetworkBeforeRequestSent {
    pub r#method: String,
    pub r#params: r#NetworkBeforeRequestSentParameters,
}
pub struct r#NetworkBeforeRequestSentParameters {
    #[serde(flatten)]
    pub r#network_base_parameters: r#NetworkBaseParameters,
    pub r#initiator: r#NetworkInitiator,
}
pub struct r#NetworkFetchError {
    pub r#method: String,
    pub r#params: r#NetworkFetchErrorParameters,
}
pub struct r#NetworkFetchErrorParameters {
    #[serde(flatten)]
    pub r#network_base_parameters: r#NetworkBaseParameters,
    pub r#errorText: r#Text,
}
pub struct r#NetworkResponseCompleted {
    pub r#method: String,
    pub r#params: r#NetworkResponseCompletedParameters,
}
pub struct r#NetworkResponseCompletedParameters {
    #[serde(flatten)]
    pub r#network_base_parameters: r#NetworkBaseParameters,
    pub r#response: r#NetworkResponseData,
}
pub struct r#NetworkResponseStarted {
    pub r#method: String,
    pub r#params: r#NetworkResponseStartedParameters,
}
pub struct r#NetworkResponseStartedParameters {
    #[serde(flatten)]
    pub r#network_base_parameters: r#NetworkBaseParameters,
    pub r#response: r#NetworkResponseData,
}
pub enum r#ScriptCommand {
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
    Todo,
}
pub type r#ScriptResult = TODO;
pub enum r#ScriptEvent {
    Todo,
    Todo,
    Todo,
}
pub type r#ScriptChannel = TODO;
pub struct r#ScriptChannelValue {
    pub r#type: String,
    pub r#value: r#ScriptChannelProperties,
}
pub struct r#ScriptChannelProperties {
    pub r#channel: r#ScriptChannel,
    pub r#serializationOptions: r#ScriptSerializationOptions,
    pub r#ownership: r#ScriptResultOwnership,
}
pub type r#ScriptEvaluateResult = TODO;
pub struct r#ScriptEvaluateResultSuccess {
    pub r#type: String,
    pub r#result: r#ScriptRemoteValue,
    pub r#realm: r#ScriptRealm,
}
pub struct r#ScriptEvaluateResultException {
    pub r#type: String,
    pub r#exceptionDetails: r#ScriptExceptionDetails,
    pub r#realm: r#ScriptRealm,
}
pub struct r#ScriptExceptionDetails {
    pub r#columnNumber: r#JsUint,
    pub r#exception: r#ScriptRemoteValue,
    pub r#lineNumber: r#JsUint,
    pub r#stackTrace: r#ScriptStackTrace,
    pub r#text: r#Text,
}
pub type r#ScriptHandle = TODO;
pub type r#ScriptInternalId = TODO;
pub type r#ScriptLocalValue = TODO;
pub struct r#ScriptListLocalValue {
    #[serde(flatten)]
    pub r#script_local_value: r#ScriptLocalValue,
}
pub struct r#ScriptArrayLocalValue {
    pub r#type: String,
    pub r#value: r#ScriptListLocalValue,
}
pub struct r#ScriptDateLocalValue {
    pub r#type: String,
    pub r#value: r#Text,
}
pub struct r#ScriptMappingLocalValue {
    pub NO_KEY: TODO,
}
pub struct r#ScriptMapLocalValue {
    pub r#type: String,
    pub r#value: r#ScriptMappingLocalValue,
}
pub struct r#ScriptObjectLocalValue {
    pub r#type: String,
    pub r#value: r#ScriptMappingLocalValue,
}
pub struct r#ScriptRegExpValue {
    pub r#pattern: r#Text,
    pub r#flags: r#Text,
}
pub struct r#ScriptRegExpLocalValue {
    pub r#type: String,
    pub r#value: r#ScriptRegExpValue,
}
pub struct r#ScriptSetLocalValue {
    pub r#type: String,
    pub r#value: r#ScriptListLocalValue,
}
pub type r#ScriptPreloadScript = TODO;
pub type r#ScriptRealm = TODO;
pub type r#ScriptPrimitiveProtocolValue = TODO;
pub struct r#ScriptUndefinedValue {
    pub r#type: String,
}
pub struct r#ScriptNullValue {
    pub r#type: String,
}
pub struct r#ScriptStringValue {
    pub r#type: String,
    pub r#value: r#Text,
}
pub type r#ScriptSpecialNumber = TODO;
pub struct r#ScriptNumberValue {
    pub r#type: String,
    pub r#value: TODO,
}
pub struct r#ScriptBooleanValue {
    pub r#type: String,
    pub r#value: r#Bool,
}
pub struct r#ScriptBigIntValue {
    pub r#type: String,
    pub r#value: r#Text,
}
pub type r#ScriptRealmInfo = TODO;
pub struct r#ScriptBaseRealmInfo {
    pub r#realm: r#ScriptRealm,
    pub r#origin: r#Text,
}
pub struct r#ScriptWindowRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#sandbox: r#Text,
}
pub struct r#ScriptDedicatedWorkerRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
    pub r#owners: TODO,
}
pub struct r#ScriptSharedWorkerRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
}
pub struct r#ScriptServiceWorkerRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
}
pub struct r#ScriptWorkerRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
}
pub struct r#ScriptPaintWorkletRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
}
pub struct r#ScriptAudioWorkletRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
}
pub struct r#ScriptWorkletRealmInfo {
    #[serde(flatten)]
    pub r#script_base_realm_info: r#ScriptBaseRealmInfo,
    pub r#type: String,
}
pub type r#ScriptRealmType = TODO;
pub type r#ScriptRemoteReference = TODO;
pub struct r#ScriptSharedReference {
    pub r#sharedId: r#ScriptSharedId,
    pub r#handle: r#ScriptHandle,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#ScriptRemoteObjectReference {
    pub r#handle: r#ScriptHandle,
    pub r#sharedId: r#ScriptSharedId,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub type r#ScriptRemoteValue = TODO;
pub struct r#ScriptListRemoteValue {
    #[serde(flatten)]
    pub r#script_remote_value: r#ScriptRemoteValue,
}
pub struct r#ScriptMappingRemoteValue {
    pub NO_KEY: TODO,
}
pub struct r#ScriptSymbolRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptArrayRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptListRemoteValue,
}
pub struct r#ScriptObjectRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptMappingRemoteValue,
}
pub struct r#ScriptFunctionRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub type r#ScriptRegExpRemoteValue = TODO;
pub type r#ScriptDateRemoteValue = TODO;
pub struct r#ScriptMapRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptMappingRemoteValue,
}
pub struct r#ScriptSetRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptListRemoteValue,
}
pub struct r#ScriptWeakMapRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptWeakSetRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptGeneratorRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptErrorRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptProxyRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptPromiseRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptTypedArrayRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptArrayBufferRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptNodeListRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptListRemoteValue,
}
pub struct r#ScriptHtmlCollectionRemoteValue {
    pub r#type: String,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptListRemoteValue,
}
pub struct r#ScriptNodeRemoteValue {
    pub r#type: String,
    pub r#sharedId: r#ScriptSharedId,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
    pub r#value: r#ScriptNodeProperties,
}
pub struct r#ScriptNodeProperties {
    pub r#nodeType: r#JsUint,
    pub r#childNodeCount: r#JsUint,
    pub r#attributes: TODO,
    pub r#children: TODO,
    pub r#localName: r#Text,
    pub r#mode: TODO,
    pub r#namespaceURI: r#Text,
    pub r#nodeValue: r#Text,
    pub r#shadowRoot: TODO,
}
pub struct r#ScriptWindowProxyRemoteValue {
    pub r#type: String,
    pub r#value: r#ScriptWindowProxyProperties,
    pub r#handle: r#ScriptHandle,
    pub r#internalId: r#ScriptInternalId,
}
pub struct r#ScriptWindowProxyProperties {
    pub r#context: r#BrowsingContextBrowsingContext,
}
pub type r#ScriptResultOwnership = TODO;
pub struct r#ScriptSerializationOptions {
    pub r#maxDomDepth: TODO,
    pub r#maxObjectDepth: TODO,
    pub r#includeShadowTree: TODO,
}
pub type r#ScriptSharedId = TODO;
pub struct r#ScriptStackFrame {
    pub r#columnNumber: r#JsUint,
    pub r#functionName: r#Text,
    pub r#lineNumber: r#JsUint,
    pub r#url: r#Text,
}
pub struct r#ScriptStackTrace {
    pub r#callFrames: TODO,
}
pub struct r#ScriptSource {
    pub r#realm: r#ScriptRealm,
    pub r#context: r#BrowsingContextBrowsingContext,
}
pub struct r#ScriptRealmTarget {
    pub r#realm: r#ScriptRealm,
}
pub struct r#ScriptContextTarget {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#sandbox: r#Text,
}
pub type r#ScriptTarget = TODO;
pub struct r#ScriptAddPreloadScript {
    pub r#method: String,
    pub r#params: r#ScriptAddPreloadScriptParameters,
}
pub struct r#ScriptAddPreloadScriptParameters {
    pub r#functionDeclaration: r#Text,
    pub r#arguments: TODO,
    pub r#contexts: TODO,
    pub r#userContexts: TODO,
    pub r#sandbox: r#Text,
}
pub struct r#ScriptAddPreloadScriptResult {
    pub r#script: r#ScriptPreloadScript,
}
pub struct r#ScriptDisown {
    pub r#method: String,
    pub r#params: r#ScriptDisownParameters,
}
pub struct r#ScriptDisownParameters {
    pub r#handles: TODO,
    pub r#target: r#ScriptTarget,
}
pub struct r#ScriptCallFunction {
    pub r#method: String,
    pub r#params: r#ScriptCallFunctionParameters,
}
pub struct r#ScriptCallFunctionParameters {
    pub r#functionDeclaration: r#Text,
    pub r#awaitPromise: r#Bool,
    pub r#target: r#ScriptTarget,
    pub r#arguments: TODO,
    pub r#resultOwnership: r#ScriptResultOwnership,
    pub r#serializationOptions: r#ScriptSerializationOptions,
    pub r#this: r#ScriptLocalValue,
    pub r#userActivation: TODO,
}
pub struct r#ScriptEvaluate {
    pub r#method: String,
    pub r#params: r#ScriptEvaluateParameters,
}
pub struct r#ScriptEvaluateParameters {
    pub r#expression: r#Text,
    pub r#target: r#ScriptTarget,
    pub r#awaitPromise: r#Bool,
    pub r#resultOwnership: r#ScriptResultOwnership,
    pub r#serializationOptions: r#ScriptSerializationOptions,
    pub r#userActivation: TODO,
}
pub struct r#ScriptGetRealms {
    pub r#method: String,
    pub r#params: r#ScriptGetRealmsParameters,
}
pub struct r#ScriptGetRealmsParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#type: r#ScriptRealmType,
}
pub struct r#ScriptGetRealmsResult {
    pub r#realms: TODO,
}
pub struct r#ScriptRemovePreloadScript {
    pub r#method: String,
    pub r#params: r#ScriptRemovePreloadScriptParameters,
}
pub struct r#ScriptRemovePreloadScriptParameters {
    pub r#script: r#ScriptPreloadScript,
}
pub struct r#ScriptMessage {
    pub r#method: String,
    pub r#params: r#ScriptMessageParameters,
}
pub struct r#ScriptMessageParameters {
    pub r#channel: r#ScriptChannel,
    pub r#data: r#ScriptRemoteValue,
    pub r#source: r#ScriptSource,
}
pub struct r#ScriptRealmCreated {
    pub r#method: String,
    pub r#params: r#ScriptRealmInfo,
}
pub struct r#ScriptRealmDestroyed {
    pub r#method: String,
    pub r#params: r#ScriptRealmDestroyedParameters,
}
pub struct r#ScriptRealmDestroyedParameters {
    pub r#realm: r#ScriptRealm,
}
pub enum r#StorageCommand {
    Todo,
    Todo,
    Todo,
}
pub type r#StorageResult = TODO;
pub struct r#StoragePartitionKey {
    pub r#userContext: r#Text,
    pub r#sourceOrigin: r#Text,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#StorageGetCookies {
    pub r#method: String,
    pub r#params: r#StorageGetCookiesParameters,
}
pub struct r#StorageCookieFilter {
    pub r#name: r#Text,
    pub r#value: r#NetworkBytesValue,
    pub r#domain: r#Text,
    pub r#path: r#Text,
    pub r#size: r#JsUint,
    pub r#httpOnly: r#Bool,
    pub r#secure: r#Bool,
    pub r#sameSite: r#NetworkSameSite,
    pub r#expiry: r#JsUint,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#StorageBrowsingContextPartitionDescriptor {
    pub r#type: String,
    pub r#context: r#BrowsingContextBrowsingContext,
}
pub struct r#StorageStorageKeyPartitionDescriptor {
    pub r#type: String,
    pub r#userContext: r#Text,
    pub r#sourceOrigin: r#Text,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub type r#StoragePartitionDescriptor = TODO;
pub struct r#StorageGetCookiesParameters {
    pub r#filter: r#StorageCookieFilter,
    pub r#partition: r#StoragePartitionDescriptor,
}
pub struct r#StorageGetCookiesResult {
    pub r#cookies: TODO,
    pub r#partitionKey: r#StoragePartitionKey,
}
pub struct r#StorageSetCookie {
    pub r#method: String,
    pub r#params: r#StorageSetCookieParameters,
}
pub struct r#StoragePartialCookie {
    pub r#name: r#Text,
    pub r#value: r#NetworkBytesValue,
    pub r#domain: r#Text,
    pub r#path: r#Text,
    pub r#httpOnly: r#Bool,
    pub r#secure: r#Bool,
    pub r#sameSite: r#NetworkSameSite,
    pub r#expiry: r#JsUint,
    #[serde(flatten)]
    pub r#extensible: r#Extensible,
}
pub struct r#StorageSetCookieParameters {
    pub r#cookie: r#StoragePartialCookie,
    pub r#partition: r#StoragePartitionDescriptor,
}
pub struct r#StorageSetCookieResult {
    pub r#partitionKey: r#StoragePartitionKey,
}
pub struct r#StorageDeleteCookies {
    pub r#method: String,
    pub r#params: r#StorageDeleteCookiesParameters,
}
pub struct r#StorageDeleteCookiesParameters {
    pub r#filter: r#StorageCookieFilter,
    pub r#partition: r#StoragePartitionDescriptor,
}
pub struct r#StorageDeleteCookiesResult {
    pub r#partitionKey: r#StoragePartitionKey,
}
pub type r#LogEvent = TODO;
pub type r#LogLevel = TODO;
pub type r#LogEntry = TODO;
pub struct r#LogBaseLogEntry {
    pub r#level: r#LogLevel,
    pub r#source: r#ScriptSource,
    pub r#text: TODO,
    pub r#timestamp: r#JsUint,
    pub r#stackTrace: r#ScriptStackTrace,
}
pub struct r#LogGenericLogEntry {
    #[serde(flatten)]
    pub r#log_base_log_entry: r#LogBaseLogEntry,
    pub r#type: r#Text,
}
pub struct r#LogConsoleLogEntry {
    #[serde(flatten)]
    pub r#log_base_log_entry: r#LogBaseLogEntry,
    pub r#type: String,
    pub r#method: r#Text,
    pub r#args: TODO,
}
pub struct r#LogJavascriptLogEntry {
    #[serde(flatten)]
    pub r#log_base_log_entry: r#LogBaseLogEntry,
    pub r#type: String,
}
pub struct r#LogEntryAdded {
    pub r#method: String,
    pub r#params: r#LogEntry,
}
pub enum r#InputCommand {
    Todo,
    Todo,
    Todo,
}
pub type r#InputEvent = TODO;
pub struct r#InputElementOrigin {
    pub r#type: String,
    pub r#element: r#ScriptSharedReference,
}
pub struct r#InputPerformActions {
    pub r#method: String,
    pub r#params: r#InputPerformActionsParameters,
}
pub struct r#InputPerformActionsParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#actions: TODO,
}
pub type r#InputSourceActions = TODO;
pub struct r#InputNoneSourceActions {
    pub r#type: String,
    pub r#id: r#Text,
    pub r#actions: TODO,
}
pub type r#InputNoneSourceAction = TODO;
pub struct r#InputKeySourceActions {
    pub r#type: String,
    pub r#id: r#Text,
    pub r#actions: TODO,
}
pub type r#InputKeySourceAction = TODO;
pub struct r#InputPointerSourceActions {
    pub r#type: String,
    pub r#id: r#Text,
    pub r#parameters: r#InputPointerParameters,
    pub r#actions: TODO,
}
pub type r#InputPointerType = TODO;
pub struct r#InputPointerParameters {
    pub r#pointerType: TODO,
}
pub type r#InputPointerSourceAction = TODO;
pub struct r#InputWheelSourceActions {
    pub r#type: String,
    pub r#id: r#Text,
    pub r#actions: TODO,
}
pub type r#InputWheelSourceAction = TODO;
pub struct r#InputPauseAction {
    pub r#type: String,
    pub r#duration: r#JsUint,
}
pub struct r#InputKeyDownAction {
    pub r#type: String,
    pub r#value: r#Text,
}
pub struct r#InputKeyUpAction {
    pub r#type: String,
    pub r#value: r#Text,
}
pub struct r#InputPointerUpAction {
    pub r#type: String,
    pub r#button: r#JsUint,
}
pub struct r#InputPointerDownAction {
    pub r#type: String,
    pub r#button: r#JsUint,
    #[serde(flatten)]
    pub r#input_pointer_common_properties: r#InputPointerCommonProperties,
}
pub struct r#InputPointerMoveAction {
    pub r#type: String,
    pub r#x: r#Float,
    pub r#y: r#Float,
    pub r#duration: r#JsUint,
    pub r#origin: r#InputOrigin,
    #[serde(flatten)]
    pub r#input_pointer_common_properties: r#InputPointerCommonProperties,
}
pub struct r#InputWheelScrollAction {
    pub r#type: String,
    pub r#x: r#JsInt,
    pub r#y: r#JsInt,
    pub r#deltaX: r#JsInt,
    pub r#deltaY: r#JsInt,
    pub r#duration: r#JsUint,
    pub r#origin: TODO,
}
pub struct r#InputPointerCommonProperties {
    pub r#width: TODO,
    pub r#height: TODO,
    pub r#pressure: TODO,
    pub r#tangentialPressure: TODO,
    pub r#twist: TODO,
    pub r#altitudeAngle: TODO,
    pub r#azimuthAngle: TODO,
}
pub type r#InputOrigin = TODO;
pub struct r#InputReleaseActions {
    pub r#method: String,
    pub r#params: r#InputReleaseActionsParameters,
}
pub struct r#InputReleaseActionsParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
}
pub struct r#InputSetFiles {
    pub r#method: String,
    pub r#params: r#InputSetFilesParameters,
}
pub struct r#InputSetFilesParameters {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#element: r#ScriptSharedReference,
    pub r#files: TODO,
}
pub struct r#InputFileDialogOpened {
    pub r#method: String,
    pub r#params: r#InputFileDialogInfo,
}
pub struct r#InputFileDialogInfo {
    pub r#context: r#BrowsingContextBrowsingContext,
    pub r#element: r#ScriptSharedReference,
    pub r#multiple: r#Bool,
}
pub enum r#WebExtensionCommand {
    Todo,
    Todo,
}
pub type r#WebExtensionResult = TODO;
pub type r#WebExtensionExtension = TODO;
pub struct r#WebExtensionInstall {
    pub r#method: String,
    pub r#params: r#WebExtensionInstallParameters,
}
pub struct r#WebExtensionInstallParameters {
    pub r#extensionData: r#WebExtensionExtensionData,
}
pub type r#WebExtensionExtensionData = TODO;
pub struct r#WebExtensionExtensionPath {
    pub r#type: String,
    pub r#path: r#Text,
}
pub struct r#WebExtensionExtensionArchivePath {
    pub r#type: String,
    pub r#path: r#Text,
}
pub struct r#WebExtensionExtensionBase64Encoded {
    pub r#type: String,
    pub r#value: r#Text,
}
pub struct r#WebExtensionInstallResult {
    pub r#extension: r#WebExtensionExtension,
}
pub struct r#WebExtensionUninstall {
    pub r#method: String,
    pub r#params: r#WebExtensionUninstallParameters,
}
pub struct r#WebExtensionUninstallParameters {
    pub r#extension: r#WebExtensionExtension,
}
