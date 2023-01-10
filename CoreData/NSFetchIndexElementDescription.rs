//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFetchIndexElementType {
        NSFetchIndexElementTypeBinary = 0,
        NSFetchIndexElementTypeRTree = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchIndexElementDescription;

    unsafe impl ClassType for NSFetchIndexElementDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
    unsafe impl NSFetchIndexElementDescription {
        #[cfg(feature = "CoreData_NSPropertyDescription")]
        #[method_id(@__retain_semantics Init initWithProperty:collationType:)]
        pub unsafe fn initWithProperty_collationType(
            this: Option<Allocated<Self>>,
            property: &CoreData::NSPropertyDescription,
            collationType: CoreData::NSFetchIndexElementType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CoreData_NSPropertyDescription")]
        #[method_id(@__retain_semantics Other property)]
        pub unsafe fn property(&self) -> Option<Id<CoreData::NSPropertyDescription, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other propertyName)]
        pub unsafe fn propertyName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[method(collationType)]
        pub unsafe fn collationType(&self) -> CoreData::NSFetchIndexElementType;

        #[method(setCollationType:)]
        pub unsafe fn setCollationType(&self, collationType: CoreData::NSFetchIndexElementType);

        #[method(isAscending)]
        pub unsafe fn isAscending(&self) -> bool;

        #[method(setAscending:)]
        pub unsafe fn setAscending(&self, ascending: bool);

        #[cfg(feature = "CoreData_NSFetchIndexDescription")]
        #[method_id(@__retain_semantics Other indexDescription)]
        pub unsafe fn indexDescription(
            &self,
        ) -> Option<Id<CoreData::NSFetchIndexDescription, Shared>>;
    }
);
