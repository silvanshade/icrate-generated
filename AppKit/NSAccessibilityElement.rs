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
    #[cfg(feature = "AppKit_NSAccessibilityElement")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSAccessibilityElement;
}

#[cfg(feature = "AppKit_NSAccessibilityElement")]
unsafe impl NSAccessibility for NSAccessibilityElement {}

#[cfg(feature = "AppKit_NSAccessibilityElement")]
unsafe impl NSObjectProtocol for NSAccessibilityElement {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSAccessibilityElement")]
    pub type NSAccessibilityElement;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(
        sel = "accessibilityElementWithRole:frame:label:parent:",
        managed = "Other"
    )]
    pub unsafe fn accessibilityElementWithRole_frame_label_parent(
        role: &NSAccessibilityRole,
        frame: NSRect,
        label: Option<&NSString>,
        parent: Option<&Object>,
    ) -> Id<Object>;

    #[objc2::method(sel = "accessibilityAddChildElement:")]
    pub unsafe fn accessibilityAddChildElement(&self, child_element: &NSAccessibilityElement);

    #[objc2::method(sel = "accessibilityFrameInParentSpace")]
    pub unsafe fn accessibilityFrameInParentSpace(&self) -> NSRect;

    #[objc2::method(sel = "setAccessibilityFrameInParentSpace:")]
    pub unsafe fn setAccessibilityFrameInParentSpace(
        &self,
        accessibility_frame_in_parent_space: NSRect,
    );
}
