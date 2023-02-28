//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::protocol]
pub unsafe trait WKScriptMessageHandlerWithReply: NSObjectProtocol {
    #[cfg(all(
        feature = "Foundation_NSString",
        feature = "WebKit_WKScriptMessage",
        feature = "WebKit_WKUserContentController"
    ))]
    #[objc2::method(sel = "userContentController:didReceiveScriptMessage:replyHandler:")]
    unsafe fn userContentController_didReceiveScriptMessage_replyHandler(
        &self,
        user_content_controller: &WKUserContentController,
        message: &WKScriptMessage,
        reply_handler: &Block<(*mut Object, *mut NSString), ()>,
    );
}
