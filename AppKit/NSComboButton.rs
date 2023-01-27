//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSComboButtonStyle {
        NSComboButtonStyleSplit = 0,
        NSComboButtonStyleUnified = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSComboButton")]
    pub struct NSComboButton;

    #[cfg(feature = "AppKit_NSComboButton")]
    unsafe impl ClassType for NSComboButton {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSAccessibility for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSAccessibilityElementProtocol for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSAnimatablePropertyContainer for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSAppearanceCustomization for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSCoding for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSDraggingDestination for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSObjectProtocol for NSComboButton {}

#[cfg(feature = "AppKit_NSComboButton")]
unsafe impl NSUserInterfaceItemIdentification for NSComboButton {}

extern_methods!(
    #[cfg(feature = "AppKit_NSComboButton")]
    unsafe impl NSComboButton {
        #[cfg(all(feature = "AppKit_NSMenu", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other comboButtonWithTitle:menu:target:action:)]
        pub unsafe fn comboButtonWithTitle_menu_target_action(
            title: &NSString,
            menu: Option<&NSMenu>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSMenu"))]
        #[method_id(@__retain_semantics Other comboButtonWithImage:menu:target:action:)]
        pub unsafe fn comboButtonWithImage_menu_target_action(
            image: &NSImage,
            menu: Option<&NSMenu>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "AppKit_NSMenu",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other comboButtonWithTitle:image:menu:target:action:)]
        pub unsafe fn comboButtonWithTitle_image_menu_target_action(
            title: &NSString,
            image: &NSImage,
            menu: Option<&NSMenu>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Id<NSMenu, Shared>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: &NSMenu);

        #[method(style)]
        pub unsafe fn style(&self) -> NSComboButtonStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSComboButtonStyle);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSComboButton")]
    unsafe impl NSComboButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
