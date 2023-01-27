//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLDivElement")]
    #[deprecated]
    pub struct DOMHTMLDivElement;

    #[cfg(feature = "WebKit_DOMHTMLDivElement")]
    unsafe impl ClassType for DOMHTMLDivElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLDivElement")]
unsafe impl DOMEventTarget for DOMHTMLDivElement {}

#[cfg(feature = "WebKit_DOMHTMLDivElement")]
unsafe impl NSObjectProtocol for DOMHTMLDivElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLDivElement")]
    unsafe impl DOMHTMLDivElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);
    }
);
