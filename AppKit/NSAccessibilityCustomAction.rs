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
    #[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSAccessibilityCustomAction;
}

#[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
unsafe impl NSObjectProtocol for NSAccessibilityCustomAction {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
    pub type NSAccessibilityCustomAction;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithName:handler:", managed = "Init")]
    pub unsafe fn initWithName_handler(
        this: Option<Allocated<Self>>,
        name: &NSString,
        handler: Option<&Block<(), Bool>>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithName:target:selector:", managed = "Init")]
    pub unsafe fn initWithName_target_selector(
        this: Option<Allocated<Self>>,
        name: &NSString,
        target: &NSObject,
        selector: Sel,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setName:")]
    pub unsafe fn setName(&self, name: &NSString);

    #[objc2::method(sel = "handler")]
    pub unsafe fn handler(&self) -> *mut Block<(), Bool>;

    #[objc2::method(sel = "setHandler:")]
    pub unsafe fn setHandler(&self, handler: Option<&Block<(), Bool>>);

    #[objc2::method(sel = "target", managed = "Other")]
    pub unsafe fn target(&self) -> Option<Id<NSObject>>;

    #[objc2::method(sel = "setTarget:")]
    pub unsafe fn setTarget(&self, target: Option<&NSObject>);

    #[objc2::method(sel = "selector")]
    pub unsafe fn selector(&self) -> Option<Sel>;

    #[objc2::method(sel = "setSelector:")]
    pub unsafe fn setSelector(&self, selector: Option<Sel>);
}
