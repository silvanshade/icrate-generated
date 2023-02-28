//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = DOMDocument,
    unsafe inherits = [
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLDocument")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMHTMLDocument;
}

#[cfg(feature = "WebKit_DOMHTMLDocument")]
unsafe impl DOMEventTarget for DOMHTMLDocument {}

#[cfg(feature = "WebKit_DOMHTMLDocument")]
unsafe impl NSObjectProtocol for DOMHTMLDocument {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLDocument")]
    #[deprecated]
    pub type DOMHTMLDocument;

    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[objc2::method(sel = "embeds", managed = "Other")]
    pub unsafe fn embeds(&self) -> Option<Id<DOMHTMLCollection>>;

    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[objc2::method(sel = "plugins", managed = "Other")]
    pub unsafe fn plugins(&self) -> Option<Id<DOMHTMLCollection>>;

    #[cfg(feature = "WebKit_DOMHTMLCollection")]
    #[objc2::method(sel = "scripts", managed = "Other")]
    pub unsafe fn scripts(&self) -> Option<Id<DOMHTMLCollection>>;

    #[objc2::method(sel = "width")]
    pub unsafe fn width(&self) -> c_int;

    #[objc2::method(sel = "height")]
    pub unsafe fn height(&self) -> c_int;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "dir", managed = "Other")]
    pub unsafe fn dir(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setDir:")]
    pub unsafe fn setDir(&self, dir: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "designMode", managed = "Other")]
    pub unsafe fn designMode(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setDesignMode:")]
    pub unsafe fn setDesignMode(&self, design_mode: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "compatMode", managed = "Other")]
    pub unsafe fn compatMode(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "bgColor", managed = "Other")]
    pub unsafe fn bgColor(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setBgColor:")]
    pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "fgColor", managed = "Other")]
    pub unsafe fn fgColor(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setFgColor:")]
    pub unsafe fn setFgColor(&self, fg_color: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "alinkColor", managed = "Other")]
    pub unsafe fn alinkColor(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setAlinkColor:")]
    pub unsafe fn setAlinkColor(&self, alink_color: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "linkColor", managed = "Other")]
    pub unsafe fn linkColor(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setLinkColor:")]
    pub unsafe fn setLinkColor(&self, link_color: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "vlinkColor", managed = "Other")]
    pub unsafe fn vlinkColor(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setVlinkColor:")]
    pub unsafe fn setVlinkColor(&self, vlink_color: Option<&NSString>);

    #[objc2::method(sel = "open")]
    pub unsafe fn open(&self);

    #[objc2::method(sel = "close")]
    pub unsafe fn close(&self);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "write:")]
    pub unsafe fn write(&self, text: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "writeln:")]
    pub unsafe fn writeln(&self, text: Option<&NSString>);

    #[objc2::method(sel = "clear")]
    pub unsafe fn clear(&self);

    #[objc2::method(sel = "captureEvents")]
    pub unsafe fn captureEvents(&self);

    #[objc2::method(sel = "releaseEvents")]
    pub unsafe fn releaseEvents(&self);
}
