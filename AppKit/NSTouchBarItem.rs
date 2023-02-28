//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSTouchBarItemIdentifier = NSString;
);

typed_extensible_enum!(
    pub type NSTouchBarItemPriority = c_float;
);

extern_static!(NSTouchBarItemPriorityHigh: NSTouchBarItemPriority = 1000);

extern_static!(NSTouchBarItemPriorityNormal: NSTouchBarItemPriority = 0);

extern_static!(NSTouchBarItemPriorityLow: NSTouchBarItemPriority = -1000);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSTouchBarItem;
}

#[cfg(feature = "AppKit_NSTouchBarItem")]
unsafe impl NSCoding for NSTouchBarItem {}

#[cfg(feature = "AppKit_NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSTouchBarItem {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    pub type NSTouchBarItem;

    #[objc2::method(sel = "initWithIdentifier:", managed = "Init")]
    pub unsafe fn initWithIdentifier(
        this: Option<Allocated<Self>>,
        identifier: &NSTouchBarItemIdentifier,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder)
        -> Option<Id<Self>>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Id<NSTouchBarItemIdentifier>;

    #[objc2::method(sel = "visibilityPriority")]
    pub unsafe fn visibilityPriority(&self) -> NSTouchBarItemPriority;

    #[objc2::method(sel = "setVisibilityPriority:")]
    pub unsafe fn setVisibilityPriority(&self, visibility_priority: NSTouchBarItemPriority);

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "view", managed = "Other")]
    pub unsafe fn view(&self) -> Option<Id<NSView>>;

    #[cfg(feature = "AppKit_NSViewController")]
    #[objc2::method(sel = "viewController", managed = "Other")]
    pub unsafe fn viewController(&self) -> Option<Id<NSViewController>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "customizationLabel", managed = "Other")]
    pub unsafe fn customizationLabel(&self) -> Id<NSString>;

    #[objc2::method(sel = "isVisible")]
    pub unsafe fn isVisible(&self) -> bool;
}

extern_static!(NSTouchBarItemIdentifierFixedSpaceSmall: &'static NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierFixedSpaceLarge: &'static NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierFlexibleSpace: &'static NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierOtherItemsProxy: &'static NSTouchBarItemIdentifier);
