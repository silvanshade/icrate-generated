//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSMappingModel")]
    pub struct NSMappingModel;

    #[cfg(feature = "CoreData_NSMappingModel")]
    unsafe impl ClassType for NSMappingModel {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSMappingModel")]
    unsafe impl NSMappingModel {
        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSBundle"
        ))]
        #[method_id(@__retain_semantics Other mappingModelFromBundles:forSourceModel:destinationModel:)]
        pub unsafe fn mappingModelFromBundles_forSourceModel_destinationModel(
            bundles: Option<&NSArray<NSBundle>>,
            source_model: Option<&NSManagedObjectModel>,
            destination_model: Option<&NSManagedObjectModel>,
        ) -> Option<Id<NSMappingModel, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other inferredMappingModelForSourceModel:destinationModel:error:_)]
        pub unsafe fn inferredMappingModelForSourceModel_destinationModel_error(
            source_model: &NSManagedObjectModel,
            destination_model: &NSManagedObjectModel,
        ) -> Result<Id<NSMappingModel, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "CoreData_NSEntityMapping", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other entityMappings)]
        pub unsafe fn entityMappings(&self) -> Option<Id<NSArray<NSEntityMapping>, Shared>>;

        #[cfg(all(feature = "CoreData_NSEntityMapping", feature = "Foundation_NSArray"))]
        #[method(setEntityMappings:)]
        pub unsafe fn setEntityMappings(&self, entity_mappings: Option<&NSArray<NSEntityMapping>>);

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other entityMappingsByName)]
        pub unsafe fn entityMappingsByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSEntityMapping>, Shared>;
    }
);
