//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSWindowTab")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSWindowTab;
}

#[cfg(feature = "AppKit_NSWindowTab")]
unsafe impl NSObjectProtocol for NSWindowTab {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSWindowTab")]
    pub type NSWindowTab;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "title", managed = "Other")]
    pub unsafe fn title(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTitle:")]
    pub unsafe fn setTitle(&self, title: Option<&NSString>);

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "attributedTitle", managed = "Other")]
    pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString>>;

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "setAttributedTitle:")]
    pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "toolTip", managed = "Other")]
    pub unsafe fn toolTip(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setToolTip:")]
    pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "accessoryView", managed = "Other")]
    pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "setAccessoryView:")]
    pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);
}
