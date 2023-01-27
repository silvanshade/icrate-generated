//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSLayoutGuide")]
    pub struct NSLayoutGuide;

    #[cfg(feature = "AppKit_NSLayoutGuide")]
    unsafe impl ClassType for NSLayoutGuide {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSLayoutGuide")]
unsafe impl NSCoding for NSLayoutGuide {}

#[cfg(feature = "AppKit_NSLayoutGuide")]
unsafe impl NSObjectProtocol for NSLayoutGuide {}

#[cfg(feature = "AppKit_NSLayoutGuide")]
unsafe impl NSUserInterfaceItemIdentification for NSLayoutGuide {}

extern_methods!(
    #[cfg(feature = "AppKit_NSLayoutGuide")]
    unsafe impl NSLayoutGuide {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other owningView)]
        pub unsafe fn owningView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setOwningView:)]
        pub unsafe fn setOwningView(&self, owning_view: Option<&NSView>);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUserInterfaceItemIdentifier, Shared>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutDimension")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Id<NSLayoutDimension, Shared>;

        #[cfg(feature = "AppKit_NSLayoutDimension")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Id<NSLayoutDimension, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Id<NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Id<NSLayoutYAxisAnchor, Shared>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<NSArray<NSLayoutConstraint>, Shared>;
    }
);

extern_methods!(
    /// NSLayoutGuideSupport
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSLayoutGuide")]
        #[method(addLayoutGuide:)]
        pub unsafe fn addLayoutGuide(&self, guide: &NSLayoutGuide);

        #[cfg(feature = "AppKit_NSLayoutGuide")]
        #[method(removeLayoutGuide:)]
        pub unsafe fn removeLayoutGuide(&self, guide: &NSLayoutGuide);

        #[cfg(all(feature = "AppKit_NSLayoutGuide", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other layoutGuides)]
        pub unsafe fn layoutGuides(&self) -> Id<NSArray<NSLayoutGuide>, Shared>;
    }
);
