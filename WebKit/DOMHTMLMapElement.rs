//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = DOMHTMLElement,
    unsafe inherits = [
        DOMElement,
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMHTMLMapElement;
}

#[cfg(feature = "WebKit_DOMHTMLMapElement")]
unsafe impl DOMEventTarget for DOMHTMLMapElement {}

#[cfg(feature = "WebKit_DOMHTMLMapElement")]
unsafe impl NSObjectProtocol for DOMHTMLMapElement {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLMapElement")]
    #[deprecated]
    pub type DOMHTMLMapElement;

    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[objc2::method(sel = "areas", managed = "Other")]
    pub unsafe fn areas(&self) -> Option<Id<DOMHTMLCollection>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setName:")]
    pub unsafe fn setName(&self, name: Option<&NSString>);
}
