//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum WKNavigationActionPolicy {
    WKNavigationActionPolicyCancel = 0,
    WKNavigationActionPolicyAllow = 1,
    WKNavigationActionPolicyDownload = 2,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum WKNavigationResponsePolicy {
    WKNavigationResponsePolicyCancel = 0,
    WKNavigationResponsePolicyAllow = 1,
    WKNavigationResponsePolicyDownload = 2,
}

#[objc2::protocol]
pub unsafe trait WKNavigationDelegate: NSObjectProtocol {
    #[cfg(all(feature = "WebKit_WKNavigationAction", feature = "WebKit_WKWebView"))]
    #[objc2::method(
        optional,
        sel = "webView:decidePolicyForNavigationAction:decisionHandler:"
    )]
    unsafe fn webView_decidePolicyForNavigationAction_decisionHandler(
        &self,
        web_view: &WKWebView,
        navigation_action: &WKNavigationAction,
        decision_handler: &Block<(WKNavigationActionPolicy,), ()>,
    );

    #[cfg(all(
        feature = "WebKit_WKNavigationAction",
        feature = "WebKit_WKWebView",
        feature = "WebKit_WKWebpagePreferences"
    ))]
    #[objc2::method(
        optional,
        sel = "webView:decidePolicyForNavigationAction:preferences:decisionHandler:"
    )]
    unsafe fn webView_decidePolicyForNavigationAction_preferences_decisionHandler(
        &self,
        web_view: &WKWebView,
        navigation_action: &WKNavigationAction,
        preferences: &WKWebpagePreferences,
        decision_handler: &Block<(WKNavigationActionPolicy, NonNull<WKWebpagePreferences>), ()>,
    );

    #[cfg(all(feature = "WebKit_WKNavigationResponse", feature = "WebKit_WKWebView"))]
    #[objc2::method(
        optional,
        sel = "webView:decidePolicyForNavigationResponse:decisionHandler:"
    )]
    unsafe fn webView_decidePolicyForNavigationResponse_decisionHandler(
        &self,
        web_view: &WKWebView,
        navigation_response: &WKNavigationResponse,
        decision_handler: &Block<(WKNavigationResponsePolicy,), ()>,
    );

    #[cfg(all(feature = "WebKit_WKNavigation", feature = "WebKit_WKWebView"))]
    #[objc2::method(optional, sel = "webView:didStartProvisionalNavigation:")]
    unsafe fn webView_didStartProvisionalNavigation(
        &self,
        web_view: &WKWebView,
        navigation: Option<&WKNavigation>,
    );

    #[cfg(all(feature = "WebKit_WKNavigation", feature = "WebKit_WKWebView"))]
    #[objc2::method(
        optional,
        sel = "webView:didReceiveServerRedirectForProvisionalNavigation:"
    )]
    unsafe fn webView_didReceiveServerRedirectForProvisionalNavigation(
        &self,
        web_view: &WKWebView,
        navigation: Option<&WKNavigation>,
    );

    #[cfg(all(
        feature = "Foundation_NSError",
        feature = "WebKit_WKNavigation",
        feature = "WebKit_WKWebView"
    ))]
    #[objc2::method(optional, sel = "webView:didFailProvisionalNavigation:withError:")]
    unsafe fn webView_didFailProvisionalNavigation_withError(
        &self,
        web_view: &WKWebView,
        navigation: Option<&WKNavigation>,
        error: &NSError,
    );

    #[cfg(all(feature = "WebKit_WKNavigation", feature = "WebKit_WKWebView"))]
    #[objc2::method(optional, sel = "webView:didCommitNavigation:")]
    unsafe fn webView_didCommitNavigation(
        &self,
        web_view: &WKWebView,
        navigation: Option<&WKNavigation>,
    );

    #[cfg(all(feature = "WebKit_WKNavigation", feature = "WebKit_WKWebView"))]
    #[objc2::method(optional, sel = "webView:didFinishNavigation:")]
    unsafe fn webView_didFinishNavigation(
        &self,
        web_view: &WKWebView,
        navigation: Option<&WKNavigation>,
    );

    #[cfg(all(
        feature = "Foundation_NSError",
        feature = "WebKit_WKNavigation",
        feature = "WebKit_WKWebView"
    ))]
    #[objc2::method(optional, sel = "webView:didFailNavigation:withError:")]
    unsafe fn webView_didFailNavigation_withError(
        &self,
        web_view: &WKWebView,
        navigation: Option<&WKNavigation>,
        error: &NSError,
    );

    #[cfg(all(
        feature = "Foundation_NSURLAuthenticationChallenge",
        feature = "Foundation_NSURLCredential",
        feature = "WebKit_WKWebView"
    ))]
    #[objc2::method(
        optional,
        sel = "webView:didReceiveAuthenticationChallenge:completionHandler:"
    )]
    unsafe fn webView_didReceiveAuthenticationChallenge_completionHandler(
        &self,
        web_view: &WKWebView,
        challenge: &NSURLAuthenticationChallenge,
        completion_handler: &Block<
            (NSURLSessionAuthChallengeDisposition, *mut NSURLCredential),
            (),
        >,
    );

    #[cfg(feature = "WebKit_WKWebView")]
    #[objc2::method(optional, sel = "webViewWebContentProcessDidTerminate:")]
    unsafe fn webViewWebContentProcessDidTerminate(&self, web_view: &WKWebView);

    #[cfg(all(
        feature = "Foundation_NSURLAuthenticationChallenge",
        feature = "WebKit_WKWebView"
    ))]
    #[objc2::method(
        optional,
        sel = "webView:authenticationChallenge:shouldAllowDeprecatedTLS:"
    )]
    unsafe fn webView_authenticationChallenge_shouldAllowDeprecatedTLS(
        &self,
        web_view: &WKWebView,
        challenge: &NSURLAuthenticationChallenge,
        decision_handler: &Block<(Bool,), ()>,
    );

    #[cfg(all(
        feature = "WebKit_WKDownload",
        feature = "WebKit_WKNavigationAction",
        feature = "WebKit_WKWebView"
    ))]
    #[objc2::method(optional, sel = "webView:navigationAction:didBecomeDownload:")]
    unsafe fn webView_navigationAction_didBecomeDownload(
        &self,
        web_view: &WKWebView,
        navigation_action: &WKNavigationAction,
        download: &WKDownload,
    );

    #[cfg(all(
        feature = "WebKit_WKDownload",
        feature = "WebKit_WKNavigationResponse",
        feature = "WebKit_WKWebView"
    ))]
    #[objc2::method(optional, sel = "webView:navigationResponse:didBecomeDownload:")]
    unsafe fn webView_navigationResponse_didBecomeDownload(
        &self,
        web_view: &WKWebView,
        navigation_response: &WKNavigationResponse,
        download: &WKDownload,
    );
}
