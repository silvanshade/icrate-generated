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
    #[cfg(feature = "WebKit_DOMHTMLAreaElement")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMHTMLAreaElement;
}

#[cfg(feature = "WebKit_DOMHTMLAreaElement")]
unsafe impl DOMEventTarget for DOMHTMLAreaElement {}

#[cfg(feature = "WebKit_DOMHTMLAreaElement")]
unsafe impl NSObjectProtocol for DOMHTMLAreaElement {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLAreaElement")]
    #[deprecated]
    pub type DOMHTMLAreaElement;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "alt", managed = "Other")]
    pub unsafe fn alt(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setAlt:")]
    pub unsafe fn setAlt(&self, alt: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "coords", managed = "Other")]
    pub unsafe fn coords(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCoords:")]
    pub unsafe fn setCoords(&self, coords: Option<&NSString>);

    #[objc2::method(sel = "noHref")]
    pub unsafe fn noHref(&self) -> bool;

    #[objc2::method(sel = "setNoHref:")]
    pub unsafe fn setNoHref(&self, no_href: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "shape", managed = "Other")]
    pub unsafe fn shape(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setShape:")]
    pub unsafe fn setShape(&self, shape: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "target", managed = "Other")]
    pub unsafe fn target(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTarget:")]
    pub unsafe fn setTarget(&self, target: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated]
    #[objc2::method(sel = "accessKey", managed = "Other")]
    pub unsafe fn accessKey(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated]
    #[objc2::method(sel = "setAccessKey:")]
    pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "absoluteLinkURL", managed = "Other")]
    pub unsafe fn absoluteLinkURL(&self) -> Id<NSURL>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "href", managed = "Other")]
    pub unsafe fn href(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setHref:")]
    pub unsafe fn setHref(&self, href: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "protocol", managed = "Other")]
    pub unsafe fn protocol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "host", managed = "Other")]
    pub unsafe fn host(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "hostname", managed = "Other")]
    pub unsafe fn hostname(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "port", managed = "Other")]
    pub unsafe fn port(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathname", managed = "Other")]
    pub unsafe fn pathname(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "search", managed = "Other")]
    pub unsafe fn search(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "hashName", managed = "Other")]
    pub unsafe fn hashName(&self) -> Id<NSString>;
}
