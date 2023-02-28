//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::InputMethodKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "InputMethodKit_IMKServer")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type IMKServer;
}

#[cfg(feature = "InputMethodKit_IMKServer")]
unsafe impl NSObjectProtocol for IMKServer {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "InputMethodKit_IMKServer")]
    pub type IMKServer;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithName:bundleIdentifier:", managed = "Init")]
    pub unsafe fn initWithName_bundleIdentifier(
        this: Option<Allocated<Self>>,
        name: Option<&NSString>,
        bundle_identifier: Option<&NSString>,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithName:controllerClass:delegateClass:", managed = "Init")]
    pub unsafe fn initWithName_controllerClass_delegateClass(
        this: Option<Allocated<Self>>,
        name: Option<&NSString>,
        controller_class_id: Option<&Class>,
        delegate_class_id: Option<&Class>,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "bundle", managed = "Other")]
    pub unsafe fn bundle(&self) -> Option<Id<NSBundle>>;

    #[objc2::method(sel = "paletteWillTerminate")]
    pub unsafe fn paletteWillTerminate(&self) -> bool;

    #[objc2::method(sel = "lastKeyEventWasDeadKey")]
    pub unsafe fn lastKeyEventWasDeadKey(&self) -> bool;
}
