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
    #[cfg(feature = "WebKit_DOMHTMLDListElement")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMHTMLDListElement;
}

#[cfg(feature = "WebKit_DOMHTMLDListElement")]
unsafe impl DOMEventTarget for DOMHTMLDListElement {}

#[cfg(feature = "WebKit_DOMHTMLDListElement")]
unsafe impl NSObjectProtocol for DOMHTMLDListElement {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLDListElement")]
    #[deprecated]
    pub type DOMHTMLDListElement;

    #[objc2::method(sel = "compact")]
    pub unsafe fn compact(&self) -> bool;

    #[objc2::method(sel = "setCompact:")]
    pub unsafe fn setCompact(&self, compact: bool);
}
