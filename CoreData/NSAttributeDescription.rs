//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAttributeType {
        NSUndefinedAttributeType = 0,
        NSInteger16AttributeType = 100,
        NSInteger32AttributeType = 200,
        NSInteger64AttributeType = 300,
        NSDecimalAttributeType = 400,
        NSDoubleAttributeType = 500,
        NSFloatAttributeType = 600,
        NSStringAttributeType = 700,
        NSBooleanAttributeType = 800,
        NSDateAttributeType = 900,
        NSBinaryDataAttributeType = 1000,
        NSUUIDAttributeType = 1100,
        NSURIAttributeType = 1200,
        NSTransformableAttributeType = 1800,
        NSObjectIDAttributeType = 2000,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributeDescription;

    unsafe impl ClassType for NSAttributeDescription {
        #[inherits(NSObject)]
        type Super = CoreData::NSPropertyDescription;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSAttributeDescription")]
    unsafe impl NSAttributeDescription {
        #[method(attributeType)]
        pub unsafe fn attributeType(&self) -> CoreData::NSAttributeType;

        #[method(setAttributeType:)]
        pub unsafe fn setAttributeType(&self, attributeType: CoreData::NSAttributeType);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attributeValueClassName)]
        pub unsafe fn attributeValueClassName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAttributeValueClassName:)]
        pub unsafe fn setAttributeValueClassName(
            &self,
            attributeValueClassName: Option<&Foundation::NSString>,
        );

        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, defaultValue: Option<&Object>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Id<Foundation::NSData, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueTransformerName)]
        pub unsafe fn valueTransformerName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValueTransformerName:)]
        pub unsafe fn setValueTransformerName(
            &self,
            valueTransformerName: Option<&Foundation::NSString>,
        );

        #[method(allowsExternalBinaryDataStorage)]
        pub unsafe fn allowsExternalBinaryDataStorage(&self) -> bool;

        #[method(setAllowsExternalBinaryDataStorage:)]
        pub unsafe fn setAllowsExternalBinaryDataStorage(
            &self,
            allowsExternalBinaryDataStorage: bool,
        );

        #[method(preservesValueInHistoryOnDeletion)]
        pub unsafe fn preservesValueInHistoryOnDeletion(&self) -> bool;

        #[method(setPreservesValueInHistoryOnDeletion:)]
        pub unsafe fn setPreservesValueInHistoryOnDeletion(
            &self,
            preservesValueInHistoryOnDeletion: bool,
        );

        #[method(allowsCloudEncryption)]
        pub unsafe fn allowsCloudEncryption(&self) -> bool;

        #[method(setAllowsCloudEncryption:)]
        pub unsafe fn setAllowsCloudEncryption(&self, allowsCloudEncryption: bool);
    }
);
