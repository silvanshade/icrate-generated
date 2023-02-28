//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSImageAlignment {
    NSImageAlignCenter = 0,
    NSImageAlignTop = 1,
    NSImageAlignTopLeft = 2,
    NSImageAlignTopRight = 3,
    NSImageAlignLeft = 4,
    NSImageAlignBottom = 5,
    NSImageAlignBottomLeft = 6,
    NSImageAlignBottomRight = 7,
    NSImageAlignRight = 8,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSImageFrameStyle {
    NSImageFrameNone = 0,
    NSImageFramePhoto = 1,
    NSImageFrameGrayBezel = 2,
    NSImageFrameGroove = 3,
    NSImageFrameButton = 4,
}

#[objc2::interface(
    unsafe super = NSCell,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImageCell")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSImageCell;
}

#[cfg(feature = "AppKit_NSImageCell")]
unsafe impl NSAccessibility for NSImageCell {}

#[cfg(feature = "AppKit_NSImageCell")]
unsafe impl NSAccessibilityElementProtocol for NSImageCell {}

#[cfg(feature = "AppKit_NSImageCell")]
unsafe impl NSCoding for NSImageCell {}

#[cfg(feature = "AppKit_NSImageCell")]
unsafe impl NSObjectProtocol for NSImageCell {}

#[cfg(feature = "AppKit_NSImageCell")]
unsafe impl NSUserInterfaceItemIdentification for NSImageCell {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImageCell")]
    pub type NSImageCell;

    #[objc2::method(sel = "imageAlignment")]
    pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

    #[objc2::method(sel = "setImageAlignment:")]
    pub unsafe fn setImageAlignment(&self, image_alignment: NSImageAlignment);

    #[objc2::method(sel = "imageScaling")]
    pub unsafe fn imageScaling(&self) -> NSImageScaling;

    #[objc2::method(sel = "setImageScaling:")]
    pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

    #[objc2::method(sel = "imageFrameStyle")]
    pub unsafe fn imageFrameStyle(&self) -> NSImageFrameStyle;

    #[objc2::method(sel = "setImageFrameStyle:")]
    pub unsafe fn setImageFrameStyle(&self, image_frame_style: NSImageFrameStyle);
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSImageCell")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImageCell")]
    pub type NSImageCell;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initTextCell:", managed = "Init")]
    pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "initImageCell:", managed = "Init")]
    pub unsafe fn initImageCell(this: Option<Allocated<Self>>, image: Option<&NSImage>)
        -> Id<Self>;
}
