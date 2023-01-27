//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_methods!(
    /// NSScripting
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other orderedDocuments)]
        pub unsafe fn orderedDocuments(&self) -> Id<NSArray<NSDocument>, Shared>;

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other orderedWindows)]
        pub unsafe fn orderedWindows(&self) -> Id<NSArray<NSWindow>, Shared>;
    }
);
