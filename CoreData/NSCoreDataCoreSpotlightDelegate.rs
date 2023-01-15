//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(
    NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification: &'static NSNotificationName
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
    pub struct NSCoreDataCoreSpotlightDelegate;

    #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
    unsafe impl ClassType for NSCoreDataCoreSpotlightDelegate {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
    unsafe impl NSCoreDataCoreSpotlightDelegate {
        #[method(isIndexingEnabled)]
        pub unsafe fn isIndexingEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other domainIdentifier)]
        pub unsafe fn domainIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other indexName)]
        pub unsafe fn indexName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreCoordinator",
            feature = "CoreData_NSPersistentStoreDescription"
        ))]
        #[method_id(@__retain_semantics Init initForStoreWithDescription:coordinator:)]
        pub unsafe fn initForStoreWithDescription_coordinator(
            this: Option<Allocated<Self>>,
            description: &NSPersistentStoreDescription,
            psc: &NSPersistentStoreCoordinator,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "CoreData_NSPersistentStoreDescription"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initForStoreWithDescription:model:)]
        pub unsafe fn initForStoreWithDescription_model(
            this: Option<Allocated<Self>>,
            description: &NSPersistentStoreDescription,
            model: &NSManagedObjectModel,
        ) -> Id<Self, Shared>;

        #[method(startSpotlightIndexing)]
        pub unsafe fn startSpotlightIndexing(&self);

        #[method(stopSpotlightIndexing)]
        pub unsafe fn stopSpotlightIndexing(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[method(deleteSpotlightIndexWithCompletionHandler:)]
        pub unsafe fn deleteSpotlightIndexWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);
