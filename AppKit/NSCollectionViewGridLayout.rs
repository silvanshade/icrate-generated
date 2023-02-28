//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSCollectionViewLayout,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSCollectionViewGridLayout;
}

#[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
unsafe impl NSCoding for NSCollectionViewGridLayout {}

#[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewGridLayout {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
    pub type NSCollectionViewGridLayout;

    #[objc2::method(sel = "margins")]
    pub unsafe fn margins(&self) -> NSEdgeInsets;

    #[objc2::method(sel = "setMargins:")]
    pub unsafe fn setMargins(&self, margins: NSEdgeInsets);

    #[objc2::method(sel = "minimumInteritemSpacing")]
    pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

    #[objc2::method(sel = "setMinimumInteritemSpacing:")]
    pub unsafe fn setMinimumInteritemSpacing(&self, minimum_interitem_spacing: CGFloat);

    #[objc2::method(sel = "minimumLineSpacing")]
    pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

    #[objc2::method(sel = "setMinimumLineSpacing:")]
    pub unsafe fn setMinimumLineSpacing(&self, minimum_line_spacing: CGFloat);

    #[objc2::method(sel = "maximumNumberOfRows")]
    pub unsafe fn maximumNumberOfRows(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaximumNumberOfRows:")]
    pub unsafe fn setMaximumNumberOfRows(&self, maximum_number_of_rows: NSUInteger);

    #[objc2::method(sel = "maximumNumberOfColumns")]
    pub unsafe fn maximumNumberOfColumns(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaximumNumberOfColumns:")]
    pub unsafe fn setMaximumNumberOfColumns(&self, maximum_number_of_columns: NSUInteger);

    #[objc2::method(sel = "minimumItemSize")]
    pub unsafe fn minimumItemSize(&self) -> NSSize;

    #[objc2::method(sel = "setMinimumItemSize:")]
    pub unsafe fn setMinimumItemSize(&self, minimum_item_size: NSSize);

    #[objc2::method(sel = "maximumItemSize")]
    pub unsafe fn maximumItemSize(&self) -> NSSize;

    #[objc2::method(sel = "setMaximumItemSize:")]
    pub unsafe fn setMaximumItemSize(&self, maximum_item_size: NSSize);

    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "backgroundColors", managed = "Other")]
    pub unsafe fn backgroundColors(&self) -> Id<NSArray<NSColor>>;

    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "setBackgroundColors:")]
    pub unsafe fn setBackgroundColors(&self, background_colors: Option<&NSArray<NSColor>>);
}
