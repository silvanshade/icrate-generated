//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSView,
    unsafe inherits = [
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScrubberArrangedView;
}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAccessibility for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSAppearanceCustomization for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSCoding for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSDraggingDestination for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSObjectProtocol for NSScrubberArrangedView {}

#[cfg(feature = "AppKit_NSScrubberArrangedView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberArrangedView {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    pub type NSScrubberArrangedView;

    #[objc2::method(sel = "isSelected")]
    pub unsafe fn isSelected(&self) -> bool;

    #[objc2::method(sel = "setSelected:")]
    pub unsafe fn setSelected(&self, selected: bool);

    #[objc2::method(sel = "isHighlighted")]
    pub unsafe fn isHighlighted(&self) -> bool;

    #[objc2::method(sel = "setHighlighted:")]
    pub unsafe fn setHighlighted(&self, highlighted: bool);

    #[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
    #[objc2::method(sel = "applyLayoutAttributes:")]
    pub unsafe fn applyLayoutAttributes(&self, layout_attributes: &NSScrubberLayoutAttributes);
}

#[objc2::interface(
    unsafe super = NSScrubberArrangedView,
    unsafe inherits = [
        NSView,
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScrubberSelectionView;
}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAccessibility for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSAppearanceCustomization for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSCoding for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSDraggingDestination for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSObjectProtocol for NSScrubberSelectionView {}

#[cfg(feature = "AppKit_NSScrubberSelectionView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberSelectionView {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    pub type NSScrubberSelectionView;
}

#[objc2::interface(
    unsafe super = NSScrubberArrangedView,
    unsafe inherits = [
        NSView,
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScrubberItemView;
}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAccessibility for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSAppearanceCustomization for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSCoding for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSDraggingDestination for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSObjectProtocol for NSScrubberItemView {}

#[cfg(feature = "AppKit_NSScrubberItemView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberItemView {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    pub type NSScrubberItemView;
}

#[objc2::interface(
    unsafe super = NSScrubberItemView,
    unsafe inherits = [
        NSScrubberArrangedView,
        NSView,
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScrubberTextItemView;
}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAccessibility for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSAppearanceCustomization for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSCoding for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSDraggingDestination for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSObjectProtocol for NSScrubberTextItemView {}

#[cfg(feature = "AppKit_NSScrubberTextItemView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberTextItemView {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    pub type NSScrubberTextItemView;

    #[cfg(feature = "AppKit_NSTextField")]
    #[objc2::method(sel = "textField", managed = "Other")]
    pub unsafe fn textField(&self) -> Id<NSTextField>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "title", managed = "Other")]
    pub unsafe fn title(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTitle:")]
    pub unsafe fn setTitle(&self, title: &NSString);
}

#[objc2::interface(
    unsafe super = NSScrubberItemView,
    unsafe inherits = [
        NSScrubberArrangedView,
        NSView,
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScrubberImageItemView;
}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAccessibility for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAccessibilityElementProtocol for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAnimatablePropertyContainer for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSAppearanceCustomization for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSCoding for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSDraggingDestination for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSObjectProtocol for NSScrubberImageItemView {}

#[cfg(feature = "AppKit_NSScrubberImageItemView")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubberImageItemView {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    pub type NSScrubberImageItemView;

    #[cfg(feature = "AppKit_NSImageView")]
    #[objc2::method(sel = "imageView", managed = "Other")]
    pub unsafe fn imageView(&self) -> Id<NSImageView>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "image", managed = "Other")]
    pub unsafe fn image(&self) -> Id<NSImage>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "setImage:")]
    pub unsafe fn setImage(&self, image: &NSImage);

    #[objc2::method(sel = "imageAlignment")]
    pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

    #[objc2::method(sel = "setImageAlignment:")]
    pub unsafe fn setImageAlignment(&self, image_alignment: NSImageAlignment);
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberArrangedView")]
    pub type NSScrubberArrangedView;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberSelectionView")]
    pub type NSScrubberSelectionView;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberItemView")]
    pub type NSScrubberItemView;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberTextItemView")]
    pub type NSScrubberTextItemView;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSScrubberImageItemView")]
    pub type NSScrubberImageItemView;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}
