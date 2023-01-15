//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSSaveChangesRequest")]
    pub struct NSSaveChangesRequest;

    #[cfg(feature = "CoreData_NSSaveChangesRequest")]
    unsafe impl ClassType for NSSaveChangesRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSSaveChangesRequest")]
    unsafe impl NSSaveChangesRequest {
        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Init initWithInsertedObjects:updatedObjects:deletedObjects:lockedObjects:)]
        pub unsafe fn initWithInsertedObjects_updatedObjects_deletedObjects_lockedObjects(
            this: Option<Allocated<Self>>,
            inserted_objects: Option<&NSSet<NSManagedObject>>,
            updated_objects: Option<&NSSet<NSManagedObject>>,
            deleted_objects: Option<&NSSet<NSManagedObject>>,
            locked_objects: Option<&NSSet<NSManagedObject>>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(&self) -> Option<Id<NSSet<NSManagedObject>, Shared>>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other updatedObjects)]
        pub unsafe fn updatedObjects(&self) -> Option<Id<NSSet<NSManagedObject>, Shared>>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other deletedObjects)]
        pub unsafe fn deletedObjects(&self) -> Option<Id<NSSet<NSManagedObject>, Shared>>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other lockedObjects)]
        pub unsafe fn lockedObjects(&self) -> Option<Id<NSSet<NSManagedObject>, Shared>>;
    }
);
