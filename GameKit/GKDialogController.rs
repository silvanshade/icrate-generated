//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

#[objc2::protocol]
pub unsafe trait GKViewController {}

#[objc2::interface(
    unsafe super = NSResponder,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKDialogController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKDialogController;
}

#[cfg(feature = "GameKit_GKDialogController")]
unsafe impl NSCoding for GKDialogController {}

#[cfg(feature = "GameKit_GKDialogController")]
unsafe impl NSObjectProtocol for GKDialogController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKDialogController")]
    pub type GKDialogController;

    #[cfg(feature = "AppKit_NSWindow")]
    #[objc2::method(sel = "parentWindow", managed = "Other")]
    pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow>>;

    #[cfg(feature = "AppKit_NSWindow")]
    #[objc2::method(sel = "setParentWindow:")]
    pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

    #[cfg(feature = "AppKit_NSViewController")]
    #[objc2::method(sel = "presentViewController:")]
    pub unsafe fn presentViewController(&self, view_controller: &NSViewController) -> bool;

    #[objc2::method(sel = "dismiss:")]
    pub unsafe fn dismiss(&self, sender: &Object);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKDialogController")]
    pub type GKDialogController;

    #[objc2::method(sel = "sharedDialogController", managed = "Other")]
    pub unsafe fn sharedDialogController() -> Id<GKDialogController>;
}
