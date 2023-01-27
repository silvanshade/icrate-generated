//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_methods!(
    /// Scripting
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other attributeRuns)]
        pub unsafe fn attributeRuns(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAttributeRuns:)]
        pub unsafe fn setAttributeRuns(&self, attribute_runs: &NSArray<NSTextStorage>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other paragraphs)]
        pub unsafe fn paragraphs(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setParagraphs:)]
        pub unsafe fn setParagraphs(&self, paragraphs: &NSArray<NSTextStorage>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other words)]
        pub unsafe fn words(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setWords:)]
        pub unsafe fn setWords(&self, words: &NSArray<NSTextStorage>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Id<NSArray<NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCharacters:)]
        pub unsafe fn setCharacters(&self, characters: &NSArray<NSTextStorage>);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other foregroundColor)]
        pub unsafe fn foregroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setForegroundColor:)]
        pub unsafe fn setForegroundColor(&self, foreground_color: Option<&NSColor>);
    }
);
