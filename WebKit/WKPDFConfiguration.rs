//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKPDFConfiguration")]
    pub struct WKPDFConfiguration;

    #[cfg(feature = "WebKit_WKPDFConfiguration")]
    unsafe impl ClassType for WKPDFConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKPDFConfiguration")]
unsafe impl NSObjectProtocol for WKPDFConfiguration {}

extern_methods!(
    #[cfg(feature = "WebKit_WKPDFConfiguration")]
    unsafe impl WKPDFConfiguration {
        #[method(rect)]
        pub unsafe fn rect(&self) -> CGRect;

        #[method(setRect:)]
        pub unsafe fn setRect(&self, rect: CGRect);
    }
);
