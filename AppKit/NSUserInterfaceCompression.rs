//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
    pub struct NSUserInterfaceCompressionOptions;

    #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
    unsafe impl ClassType for NSUserInterfaceCompressionOptions {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
    unsafe impl NSUserInterfaceCompressionOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Init initWithCompressionOptions:)]
        pub unsafe fn initWithCompressionOptions(
            this: Option<Allocated<Self>>,
            options: &NSSet<NSUserInterfaceCompressionOptions>,
        ) -> Id<Self, Shared>;

        #[method(containsOptions:)]
        pub unsafe fn containsOptions(&self, options: &NSUserInterfaceCompressionOptions) -> bool;

        #[method(intersectsOptions:)]
        pub unsafe fn intersectsOptions(&self, options: &NSUserInterfaceCompressionOptions)
            -> bool;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other optionsByAddingOptions:)]
        pub unsafe fn optionsByAddingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[method_id(@__retain_semantics Other optionsByRemovingOptions:)]
        pub unsafe fn optionsByRemovingOptions(
            &self,
            options: &NSUserInterfaceCompressionOptions,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[method_id(@__retain_semantics Other hideImagesOption)]
        pub unsafe fn hideImagesOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[method_id(@__retain_semantics Other hideTextOption)]
        pub unsafe fn hideTextOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[method_id(@__retain_semantics Other reduceMetricsOption)]
        pub unsafe fn reduceMetricsOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[method_id(@__retain_semantics Other breakEqualWidthsOption)]
        pub unsafe fn breakEqualWidthsOption() -> Id<NSUserInterfaceCompressionOptions, Shared>;

        #[method_id(@__retain_semantics Other standardOptions)]
        pub unsafe fn standardOptions() -> Id<NSUserInterfaceCompressionOptions, Shared>;
    }
);

extern_protocol!(
    pub struct NSUserInterfaceCompression;

    unsafe impl ProtocolType for NSUserInterfaceCompression {
        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(
            &self,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;
    }
);
