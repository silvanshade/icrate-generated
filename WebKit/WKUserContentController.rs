//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_WKUserContentController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type WKUserContentController;
}

#[cfg(feature = "WebKit_WKUserContentController")]
unsafe impl NSCoding for WKUserContentController {}

#[cfg(feature = "WebKit_WKUserContentController")]
unsafe impl NSObjectProtocol for WKUserContentController {}

#[cfg(feature = "WebKit_WKUserContentController")]
unsafe impl NSSecureCoding for WKUserContentController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_WKUserContentController")]
    pub type WKUserContentController;

    #[cfg(all(feature = "Foundation_NSArray", feature = "WebKit_WKUserScript"))]
    #[objc2::method(sel = "userScripts", managed = "Other")]
    pub unsafe fn userScripts(&self) -> Id<NSArray<WKUserScript>>;

    #[cfg(feature = "WebKit_WKUserScript")]
    #[objc2::method(sel = "addUserScript:")]
    pub unsafe fn addUserScript(&self, user_script: &WKUserScript);

    #[objc2::method(sel = "removeAllUserScripts")]
    pub unsafe fn removeAllUserScripts(&self);

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WKContentWorld"))]
    #[objc2::method(sel = "addScriptMessageHandler:contentWorld:name:")]
    pub unsafe fn addScriptMessageHandler_contentWorld_name(
        &self,
        script_message_handler: &ProtocolObject<dyn WKScriptMessageHandler>,
        world: &WKContentWorld,
        name: &NSString,
    );

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WKContentWorld"))]
    #[objc2::method(sel = "addScriptMessageHandlerWithReply:contentWorld:name:")]
    pub unsafe fn addScriptMessageHandlerWithReply_contentWorld_name(
        &self,
        script_message_handler_with_reply: &ProtocolObject<dyn WKScriptMessageHandlerWithReply>,
        content_world: &WKContentWorld,
        name: &NSString,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "addScriptMessageHandler:name:")]
    pub unsafe fn addScriptMessageHandler_name(
        &self,
        script_message_handler: &ProtocolObject<dyn WKScriptMessageHandler>,
        name: &NSString,
    );

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_WKContentWorld"))]
    #[objc2::method(sel = "removeScriptMessageHandlerForName:contentWorld:")]
    pub unsafe fn removeScriptMessageHandlerForName_contentWorld(
        &self,
        name: &NSString,
        content_world: &WKContentWorld,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "removeScriptMessageHandlerForName:")]
    pub unsafe fn removeScriptMessageHandlerForName(&self, name: &NSString);

    #[cfg(feature = "WebKit_WKContentWorld")]
    #[objc2::method(sel = "removeAllScriptMessageHandlersFromContentWorld:")]
    pub unsafe fn removeAllScriptMessageHandlersFromContentWorld(
        &self,
        content_world: &WKContentWorld,
    );

    #[objc2::method(sel = "removeAllScriptMessageHandlers")]
    pub unsafe fn removeAllScriptMessageHandlers(&self);

    #[cfg(feature = "WebKit_WKContentRuleList")]
    #[objc2::method(sel = "addContentRuleList:")]
    pub unsafe fn addContentRuleList(&self, content_rule_list: &WKContentRuleList);

    #[cfg(feature = "WebKit_WKContentRuleList")]
    #[objc2::method(sel = "removeContentRuleList:")]
    pub unsafe fn removeContentRuleList(&self, content_rule_list: &WKContentRuleList);

    #[objc2::method(sel = "removeAllContentRuleLists")]
    pub unsafe fn removeAllContentRuleLists(&self);
}
