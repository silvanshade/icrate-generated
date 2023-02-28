//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = DOMObject,
    unsafe inherits = [
        WebScriptObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMHTMLCollection;
}

#[cfg(feature = "WebKit_DOMHTMLCollection")]
unsafe impl NSObjectProtocol for DOMHTMLCollection {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[deprecated]
    pub type DOMHTMLCollection;

    #[objc2::method(sel = "length")]
    pub unsafe fn length(&self) -> c_uint;

    #[cfg(feature = "WebKit_DOMNode")]
    #[objc2::method(sel = "item:", managed = "Other")]
    pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
    #[objc2::method(sel = "namedItem:", managed = "Other")]
    pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNodeList"))]
    #[objc2::method(sel = "tags:", managed = "Other")]
    pub unsafe fn tags(&self, name: Option<&NSString>) -> Option<Id<DOMNodeList>>;
}
