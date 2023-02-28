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
    #[cfg(feature = "WebKit_DOMBlob")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMBlob;
}

#[cfg(feature = "WebKit_DOMBlob")]
unsafe impl NSObjectProtocol for DOMBlob {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMBlob")]
    #[deprecated]
    pub type DOMBlob;

    #[objc2::method(sel = "size")]
    pub unsafe fn size(&self) -> c_ulonglong;
}
