//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSTextFieldCell,
    unsafe inherits = [
        NSActionCell,
        NSCell,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSTableHeaderCell;
}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSAccessibility for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSAccessibilityElementProtocol for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSCoding for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSObjectProtocol for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSUserInterfaceItemIdentification for NSTableHeaderCell {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    pub type NSTableHeaderCell;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "drawSortIndicatorWithFrame:inView:ascending:priority:")]
    pub unsafe fn drawSortIndicatorWithFrame_inView_ascending_priority(
        &self,
        cell_frame: NSRect,
        control_view: &NSView,
        ascending: bool,
        priority: NSInteger,
    );

    #[objc2::method(sel = "sortIndicatorRectForBounds:")]
    pub unsafe fn sortIndicatorRectForBounds(&self, rect: NSRect) -> NSRect;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    pub type NSTableHeaderCell;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initTextCell:", managed = "Init")]
    pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "initImageCell:", managed = "Init")]
    pub unsafe fn initImageCell(this: Option<Allocated<Self>>, image: Option<&NSImage>)
        -> Id<Self>;
}
