//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKFindResult")]
    pub struct WKFindResult;

    #[cfg(feature = "WebKit_WKFindResult")]
    unsafe impl ClassType for WKFindResult {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKFindResult")]
unsafe impl NSObjectProtocol for WKFindResult {}

extern_methods!(
    #[cfg(feature = "WebKit_WKFindResult")]
    unsafe impl WKFindResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(matchFound)]
        pub unsafe fn matchFound(&self) -> bool;
    }
);
