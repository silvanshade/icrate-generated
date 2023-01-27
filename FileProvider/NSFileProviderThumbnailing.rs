//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_methods!(
    /// NSFileProviderThumbnailing
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress"
        ))]
        #[method_id(@__retain_semantics Other fetchThumbnailsForItemIdentifiers:requestedSize:perThumbnailCompletionHandler:completionHandler:)]
        pub unsafe fn fetchThumbnailsForItemIdentifiers_requestedSize_perThumbnailCompletionHandler_completionHandler(
            &self,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
            size: CGSize,
            per_thumbnail_completion_handler: &Block<
                (
                    NonNull<NSFileProviderItemIdentifier>,
                    *mut NSData,
                    *mut NSError,
                ),
                (),
            >,
            completion_handler: &Block<(*mut NSError,), ()>,
        ) -> Id<NSProgress, Shared>;
    }
);
