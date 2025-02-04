//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAccessibilityElement")]
    pub struct NSAccessibilityElement;

    #[cfg(feature = "AppKit_NSAccessibilityElement")]
    unsafe impl ClassType for NSAccessibilityElement {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSAccessibilityElement")]
unsafe impl NSAccessibility for NSAccessibilityElement {}

#[cfg(feature = "AppKit_NSAccessibilityElement")]
unsafe impl NSObjectProtocol for NSAccessibilityElement {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAccessibilityElement")]
    unsafe impl NSAccessibilityElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accessibilityElementWithRole:frame:label:parent:)]
        pub unsafe fn accessibilityElementWithRole_frame_label_parent(
            role: &NSAccessibilityRole,
            frame: NSRect,
            label: Option<&NSString>,
            parent: Option<&Object>,
        ) -> Id<Object>;

        #[method(accessibilityAddChildElement:)]
        pub unsafe fn accessibilityAddChildElement(&self, child_element: &NSAccessibilityElement);

        #[method(accessibilityFrameInParentSpace)]
        pub unsafe fn accessibilityFrameInParentSpace(&self) -> NSRect;

        #[method(setAccessibilityFrameInParentSpace:)]
        pub unsafe fn setAccessibilityFrameInParentSpace(
            &self,
            accessibility_frame_in_parent_space: NSRect,
        );
    }
);
