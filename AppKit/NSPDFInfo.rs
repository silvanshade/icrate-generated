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
    #[cfg(feature = "AppKit_NSPDFInfo")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSPDFInfo;
}

#[cfg(feature = "AppKit_NSPDFInfo")]
unsafe impl NSCoding for NSPDFInfo {}

#[cfg(feature = "AppKit_NSPDFInfo")]
unsafe impl NSObjectProtocol for NSPDFInfo {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSPDFInfo")]
    pub type NSPDFInfo;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "URL", managed = "Other")]
    pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "setURL:")]
    pub unsafe fn setURL(&self, url: Option<&NSURL>);

    #[objc2::method(sel = "isFileExtensionHidden")]
    pub unsafe fn isFileExtensionHidden(&self) -> bool;

    #[objc2::method(sel = "setFileExtensionHidden:")]
    pub unsafe fn setFileExtensionHidden(&self, file_extension_hidden: bool);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "tagNames", managed = "Other")]
    pub unsafe fn tagNames(&self) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTagNames:")]
    pub unsafe fn setTagNames(&self, tag_names: &NSArray<NSString>);

    #[objc2::method(sel = "orientation")]
    pub unsafe fn orientation(&self) -> NSPaperOrientation;

    #[objc2::method(sel = "setOrientation:")]
    pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);

    #[objc2::method(sel = "paperSize")]
    pub unsafe fn paperSize(&self) -> NSSize;

    #[objc2::method(sel = "setPaperSize:")]
    pub unsafe fn setPaperSize(&self, paper_size: NSSize);

    #[cfg(feature = "Foundation_NSMutableDictionary")]
    #[objc2::method(sel = "attributes", managed = "Other")]
    pub unsafe fn attributes(
        &self,
    ) -> Id<NSMutableDictionary<NSPrintInfoAttributeKey, Object>, Owned>;
}
