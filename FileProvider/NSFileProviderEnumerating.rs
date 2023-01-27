//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

typed_extensible_enum!(
    pub type NSFileProviderSyncAnchor = NSData;
);

typed_extensible_enum!(
    pub type NSFileProviderPage = NSData;
);

extern_static!(NSFileProviderInitialPageSortedByDate: &'static NSFileProviderPage);

extern_static!(NSFileProviderInitialPageSortedByName: &'static NSFileProviderPage);

extern_protocol!(
    pub unsafe trait NSFileProviderEnumerationObserver: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSArray")]
        #[method(didEnumerateItems:)]
        unsafe fn didEnumerateItems(
            &self,
            updated_items: &NSArray<ProtocolObject<dyn NSFileProviderItemProtocol>>,
        );

        #[method(finishEnumeratingUpToPage:)]
        unsafe fn finishEnumeratingUpToPage(&self, next_page: Option<&NSFileProviderPage>);

        #[cfg(feature = "Foundation_NSError")]
        #[method(finishEnumeratingWithError:)]
        unsafe fn finishEnumeratingWithError(&self, error: &NSError);

        #[optional]
        #[method(suggestedPageSize)]
        unsafe fn suggestedPageSize(&self) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn NSFileProviderEnumerationObserver {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderChangeObserver: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSArray")]
        #[method(didUpdateItems:)]
        unsafe fn didUpdateItems(
            &self,
            updated_items: &NSArray<ProtocolObject<dyn NSFileProviderItemProtocol>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(didDeleteItemsWithIdentifiers:)]
        unsafe fn didDeleteItemsWithIdentifiers(
            &self,
            deleted_item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
        );

        #[method(finishEnumeratingChangesUpToSyncAnchor:moreComing:)]
        unsafe fn finishEnumeratingChangesUpToSyncAnchor_moreComing(
            &self,
            anchor: &NSFileProviderSyncAnchor,
            more_coming: bool,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(finishEnumeratingWithError:)]
        unsafe fn finishEnumeratingWithError(&self, error: &NSError);

        #[optional]
        #[method(suggestedBatchSize)]
        unsafe fn suggestedBatchSize(&self) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn NSFileProviderChangeObserver {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderEnumerator: NSObjectProtocol {
        #[method(invalidate)]
        unsafe fn invalidate(&self);

        #[method(enumerateItemsForObserver:startingAtPage:)]
        unsafe fn enumerateItemsForObserver_startingAtPage(
            &self,
            observer: &ProtocolObject<dyn NSFileProviderEnumerationObserver>,
            page: &NSFileProviderPage,
        );

        #[optional]
        #[method(enumerateChangesForObserver:fromSyncAnchor:)]
        unsafe fn enumerateChangesForObserver_fromSyncAnchor(
            &self,
            observer: &ProtocolObject<dyn NSFileProviderChangeObserver>,
            sync_anchor: &NSFileProviderSyncAnchor,
        );

        #[optional]
        #[method(currentSyncAnchorWithCompletionHandler:)]
        unsafe fn currentSyncAnchorWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSFileProviderSyncAnchor,), ()>,
        );
    }

    unsafe impl ProtocolType for dyn NSFileProviderEnumerator {}
);

extern_methods!(
    /// NSFileProviderEnumeration
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other enumeratorForContainerItemIdentifier:error:_)]
        pub unsafe fn enumeratorForContainerItemIdentifier_error(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Id<ProtocolObject<dyn NSFileProviderEnumerator>, Shared>, Id<NSError, Shared>>;
    }
);
