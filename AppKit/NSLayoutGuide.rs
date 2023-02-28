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
    #[cfg(feature = "AppKit_NSLayoutGuide")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSLayoutGuide;
}

#[cfg(feature = "AppKit_NSLayoutGuide")]
unsafe impl NSCoding for NSLayoutGuide {}

#[cfg(feature = "AppKit_NSLayoutGuide")]
unsafe impl NSObjectProtocol for NSLayoutGuide {}

#[cfg(feature = "AppKit_NSLayoutGuide")]
unsafe impl NSUserInterfaceItemIdentification for NSLayoutGuide {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSLayoutGuide")]
    pub type NSLayoutGuide;

    #[objc2::method(sel = "frame")]
    pub unsafe fn frame(&self) -> NSRect;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "owningView", managed = "Other")]
    pub unsafe fn owningView(&self) -> Option<Id<NSView>>;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "setOwningView:")]
    pub unsafe fn setOwningView(&self, owning_view: Option<&NSView>);

    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier>;

    #[objc2::method(sel = "setIdentifier:")]
    pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

    #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
    #[objc2::method(sel = "leadingAnchor", managed = "Other")]
    pub unsafe fn leadingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
    #[objc2::method(sel = "trailingAnchor", managed = "Other")]
    pub unsafe fn trailingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
    #[objc2::method(sel = "leftAnchor", managed = "Other")]
    pub unsafe fn leftAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
    #[objc2::method(sel = "rightAnchor", managed = "Other")]
    pub unsafe fn rightAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
    #[objc2::method(sel = "topAnchor", managed = "Other")]
    pub unsafe fn topAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
    #[objc2::method(sel = "bottomAnchor", managed = "Other")]
    pub unsafe fn bottomAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutDimension")]
    #[objc2::method(sel = "widthAnchor", managed = "Other")]
    pub unsafe fn widthAnchor(&self) -> Id<NSLayoutDimension>;

    #[cfg(feature = "AppKit_NSLayoutDimension")]
    #[objc2::method(sel = "heightAnchor", managed = "Other")]
    pub unsafe fn heightAnchor(&self) -> Id<NSLayoutDimension>;

    #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
    #[objc2::method(sel = "centerXAnchor", managed = "Other")]
    pub unsafe fn centerXAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

    #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
    #[objc2::method(sel = "centerYAnchor", managed = "Other")]
    pub unsafe fn centerYAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

    #[objc2::method(sel = "hasAmbiguousLayout")]
    pub unsafe fn hasAmbiguousLayout(&self) -> bool;

    #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "constraintsAffectingLayoutForOrientation:", managed = "Other")]
    pub unsafe fn constraintsAffectingLayoutForOrientation(
        &self,
        orientation: NSLayoutConstraintOrientation,
    ) -> Id<NSArray<NSLayoutConstraint>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSView")]
    pub type NSView;

    #[cfg(feature = "AppKit_NSLayoutGuide")]
    #[objc2::method(sel = "addLayoutGuide:")]
    pub unsafe fn addLayoutGuide(&self, guide: &NSLayoutGuide);

    #[cfg(feature = "AppKit_NSLayoutGuide")]
    #[objc2::method(sel = "removeLayoutGuide:")]
    pub unsafe fn removeLayoutGuide(&self, guide: &NSLayoutGuide);

    #[cfg(all(feature = "AppKit_NSLayoutGuide", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "layoutGuides", managed = "Other")]
    pub unsafe fn layoutGuides(&self) -> Id<NSArray<NSLayoutGuide>>;
}
