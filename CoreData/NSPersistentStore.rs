//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentStore")]
    pub struct NSPersistentStore;

    #[cfg(feature = "CoreData_NSPersistentStore")]
    unsafe impl ClassType for NSPersistentStore {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStore")]
    unsafe impl NSPersistentStore {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other metadataForPersistentStoreWithURL:error:_)]
        pub unsafe fn metadataForPersistentStoreWithURL_error(
            url: &NSURL,
        ) -> Result<Id<NSDictionary<NSString, Object>, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(setMetadata:forPersistentStoreWithURL:error:_)]
        pub unsafe fn setMetadata_forPersistentStoreWithURL_error(
            metadata: Option<&NSDictionary<NSString, Object>>,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(migrationManagerClass)]
        pub unsafe fn migrationManagerClass() -> &'static Class;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreCoordinator",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Option<Allocated<Self>>,
            root: Option<&NSPersistentStoreCoordinator>,
            name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(loadMetadata:_)]
        pub unsafe fn loadMetadata(&self) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Other persistentStoreCoordinator)]
        pub unsafe fn persistentStoreCoordinator(
            &self,
        ) -> Option<Id<NSPersistentStoreCoordinator, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other configurationName)]
        pub unsafe fn configurationName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString, Shared>;

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setMetadata:)]
        pub unsafe fn setMetadata(&self, metadata: Option<&NSDictionary<NSString, Object>>);

        #[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
        #[method(didAddToPersistentStoreCoordinator:)]
        pub unsafe fn didAddToPersistentStoreCoordinator(
            &self,
            coordinator: &NSPersistentStoreCoordinator,
        );

        #[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
        #[method(willRemoveFromPersistentStoreCoordinator:)]
        pub unsafe fn willRemoveFromPersistentStoreCoordinator(
            &self,
            coordinator: Option<&NSPersistentStoreCoordinator>,
        );

        #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
        #[method_id(@__retain_semantics Other coreSpotlightExporter)]
        pub unsafe fn coreSpotlightExporter(&self) -> Id<NSCoreDataCoreSpotlightDelegate, Shared>;
    }
);
