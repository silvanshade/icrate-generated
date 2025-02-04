//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCursor")]
    pub struct NSCursor;

    #[cfg(feature = "AppKit_NSCursor")]
    unsafe impl ClassType for NSCursor {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSCursor")]
unsafe impl NSCoding for NSCursor {}

#[cfg(feature = "AppKit_NSCursor")]
unsafe impl NSObjectProtocol for NSCursor {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCursor")]
    unsafe impl NSCursor {
        #[method_id(@__retain_semantics Other currentCursor)]
        pub unsafe fn currentCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other currentSystemCursor)]
        pub unsafe fn currentSystemCursor() -> Option<Id<NSCursor>>;

        #[method_id(@__retain_semantics Other arrowCursor)]
        pub unsafe fn arrowCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other IBeamCursor)]
        pub unsafe fn IBeamCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other pointingHandCursor)]
        pub unsafe fn pointingHandCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other closedHandCursor)]
        pub unsafe fn closedHandCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other openHandCursor)]
        pub unsafe fn openHandCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other resizeLeftCursor)]
        pub unsafe fn resizeLeftCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other resizeRightCursor)]
        pub unsafe fn resizeRightCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other resizeLeftRightCursor)]
        pub unsafe fn resizeLeftRightCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other resizeUpCursor)]
        pub unsafe fn resizeUpCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other resizeDownCursor)]
        pub unsafe fn resizeDownCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other resizeUpDownCursor)]
        pub unsafe fn resizeUpDownCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other crosshairCursor)]
        pub unsafe fn crosshairCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other disappearingItemCursor)]
        pub unsafe fn disappearingItemCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other operationNotAllowedCursor)]
        pub unsafe fn operationNotAllowedCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other dragLinkCursor)]
        pub unsafe fn dragLinkCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other dragCopyCursor)]
        pub unsafe fn dragCopyCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other contextualMenuCursor)]
        pub unsafe fn contextualMenuCursor() -> Id<NSCursor>;

        #[method_id(@__retain_semantics Other IBeamCursorForVerticalLayout)]
        pub unsafe fn IBeamCursorForVerticalLayout() -> Id<NSCursor>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initWithImage:hotSpot:)]
        pub unsafe fn initWithImage_hotSpot(
            this: Option<Allocated<Self>>,
            new_image: &NSImage,
            point: NSPoint,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method(hide)]
        pub unsafe fn hide();

        #[method(unhide)]
        pub unsafe fn unhide();

        #[method(setHiddenUntilMouseMoves:)]
        pub unsafe fn setHiddenUntilMouseMoves(flag: bool);

        #[method(pop)]
        pub unsafe fn pop_class();

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;

        #[method(hotSpot)]
        pub unsafe fn hotSpot(&self) -> NSPoint;

        #[method(push)]
        pub unsafe fn push(&self);

        #[method(pop)]
        pub unsafe fn pop(&self);

        #[method(set)]
        pub unsafe fn set(&self);
    }
);

extern_static!(NSAppKitVersionNumberWithCursorSizeSupport: NSAppKitVersion = 682.0);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSCursor")]
    unsafe impl NSCursor {
        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSImage"))]
        #[deprecated = "Color hints are ignored. Use -initWithImage:hotSpot: instead"]
        #[method_id(@__retain_semantics Init initWithImage:foregroundColorHint:backgroundColorHint:hotSpot:)]
        pub unsafe fn initWithImage_foregroundColorHint_backgroundColorHint_hotSpot(
            this: Option<Allocated<Self>>,
            new_image: &NSImage,
            fg: Option<&NSColor>,
            bg: Option<&NSColor>,
            hot_spot: NSPoint,
        ) -> Id<Self>;

        #[deprecated = "setOnMouseExited is unused and should not be called"]
        #[method(setOnMouseExited:)]
        pub unsafe fn setOnMouseExited(&self, flag: bool);

        #[deprecated = "setOnMouseEntered is unused and should not be called"]
        #[method(setOnMouseEntered:)]
        pub unsafe fn setOnMouseEntered(&self, flag: bool);

        #[deprecated = "isSetOnMouseExited is unused"]
        #[method(isSetOnMouseExited)]
        pub unsafe fn isSetOnMouseExited(&self) -> bool;

        #[deprecated = "isSetOnMouseEntered is unused"]
        #[method(isSetOnMouseEntered)]
        pub unsafe fn isSetOnMouseEntered(&self) -> bool;

        #[cfg(feature = "AppKit_NSEvent")]
        #[deprecated = "mouseEntered: is unused and should not be called"]
        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[deprecated = "mouseExited: is unused and should not be called"]
        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);
    }
);
