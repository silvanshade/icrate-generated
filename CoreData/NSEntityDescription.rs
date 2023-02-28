//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "CoreData_NSEntityDescription")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSEntityDescription;
}

#[cfg(feature = "CoreData_NSEntityDescription")]
unsafe impl NSCoding for NSEntityDescription {}

#[cfg(feature = "CoreData_NSEntityDescription")]
unsafe impl NSFastEnumeration for NSEntityDescription {}

#[cfg(feature = "CoreData_NSEntityDescription")]
unsafe impl NSObjectProtocol for NSEntityDescription {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CoreData_NSEntityDescription")]
    pub type NSEntityDescription;

    #[cfg(all(
        feature = "CoreData_NSManagedObjectContext",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "entityForName:inManagedObjectContext:", managed = "Other")]
    pub unsafe fn entityForName_inManagedObjectContext(
        entity_name: &NSString,
        context: &NSManagedObjectContext,
    ) -> Option<Id<NSEntityDescription>>;

    #[cfg(all(
        feature = "CoreData_NSManagedObject",
        feature = "CoreData_NSManagedObjectContext",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(
        sel = "insertNewObjectForEntityForName:inManagedObjectContext:",
        managed = "Other"
    )]
    pub unsafe fn insertNewObjectForEntityForName_inManagedObjectContext(
        entity_name: &NSString,
        context: &NSManagedObjectContext,
    ) -> Id<NSManagedObject>;

    #[cfg(feature = "CoreData_NSManagedObjectModel")]
    #[objc2::method(sel = "managedObjectModel", managed = "Other")]
    pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "managedObjectClassName", managed = "Other")]
    pub unsafe fn managedObjectClassName(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setManagedObjectClassName:")]
    pub unsafe fn setManagedObjectClassName(&self, managed_object_class_name: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setName:")]
    pub unsafe fn setName(&self, name: Option<&NSString>);

    #[objc2::method(sel = "isAbstract")]
    pub unsafe fn isAbstract(&self) -> bool;

    #[objc2::method(sel = "setAbstract:")]
    pub unsafe fn setAbstract(&self, r#abstract: bool);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "subentitiesByName", managed = "Other")]
    pub unsafe fn subentitiesByName(&self) -> Id<NSDictionary<NSString, NSEntityDescription>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "subentities", managed = "Other")]
    pub unsafe fn subentities(&self) -> Id<NSArray<NSEntityDescription>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setSubentities:")]
    pub unsafe fn setSubentities(&self, subentities: &NSArray<NSEntityDescription>);

    #[objc2::method(sel = "superentity", managed = "Other")]
    pub unsafe fn superentity(&self) -> Option<Id<NSEntityDescription>>;

    #[cfg(all(
        feature = "CoreData_NSPropertyDescription",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "propertiesByName", managed = "Other")]
    pub unsafe fn propertiesByName(&self) -> Id<NSDictionary<NSString, NSPropertyDescription>>;

    #[cfg(all(
        feature = "CoreData_NSPropertyDescription",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(sel = "properties", managed = "Other")]
    pub unsafe fn properties(&self) -> Id<NSArray<NSPropertyDescription>>;

    #[cfg(all(
        feature = "CoreData_NSPropertyDescription",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(sel = "setProperties:")]
    pub unsafe fn setProperties(&self, properties: &NSArray<NSPropertyDescription>);

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "userInfo", managed = "Other")]
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "setUserInfo:")]
    pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

    #[cfg(all(
        feature = "CoreData_NSAttributeDescription",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "attributesByName", managed = "Other")]
    pub unsafe fn attributesByName(&self) -> Id<NSDictionary<NSString, NSAttributeDescription>>;

    #[cfg(all(
        feature = "CoreData_NSRelationshipDescription",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "relationshipsByName", managed = "Other")]
    pub unsafe fn relationshipsByName(
        &self,
    ) -> Id<NSDictionary<NSString, NSRelationshipDescription>>;

    #[cfg(all(
        feature = "CoreData_NSRelationshipDescription",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(sel = "relationshipsWithDestinationEntity:", managed = "Other")]
    pub unsafe fn relationshipsWithDestinationEntity(
        &self,
        entity: &NSEntityDescription,
    ) -> Id<NSArray<NSRelationshipDescription>>;

    #[objc2::method(sel = "isKindOfEntity:")]
    pub unsafe fn isKindOfEntity(&self, entity: &NSEntityDescription) -> bool;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "versionHash", managed = "Other")]
    pub unsafe fn versionHash(&self) -> Id<NSData>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "versionHashModifier", managed = "Other")]
    pub unsafe fn versionHashModifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setVersionHashModifier:")]
    pub unsafe fn setVersionHashModifier(&self, version_hash_modifier: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "renamingIdentifier", managed = "Other")]
    pub unsafe fn renamingIdentifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setRenamingIdentifier:")]
    pub unsafe fn setRenamingIdentifier(&self, renaming_identifier: Option<&NSString>);

    #[cfg(all(
        feature = "CoreData_NSFetchIndexDescription",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(sel = "indexes", managed = "Other")]
    pub unsafe fn indexes(&self) -> Id<NSArray<NSFetchIndexDescription>>;

    #[cfg(all(
        feature = "CoreData_NSFetchIndexDescription",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(sel = "setIndexes:")]
    pub unsafe fn setIndexes(&self, indexes: &NSArray<NSFetchIndexDescription>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "uniquenessConstraints", managed = "Other")]
    pub unsafe fn uniquenessConstraints(&self) -> Id<NSArray<NSArray<Object>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setUniquenessConstraints:")]
    pub unsafe fn setUniquenessConstraints(
        &self,
        uniqueness_constraints: &NSArray<NSArray<Object>>,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[deprecated = "Use NSEntityDescription.indexes instead"]
    #[objc2::method(sel = "compoundIndexes", managed = "Other")]
    pub unsafe fn compoundIndexes(&self) -> Id<NSArray<NSArray<Object>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[deprecated = "Use NSEntityDescription.indexes instead"]
    #[objc2::method(sel = "setCompoundIndexes:")]
    pub unsafe fn setCompoundIndexes(&self, compound_indexes: &NSArray<NSArray<Object>>);

    #[cfg(feature = "Foundation_NSExpression")]
    #[objc2::method(sel = "coreSpotlightDisplayNameExpression", managed = "Other")]
    pub unsafe fn coreSpotlightDisplayNameExpression(&self) -> Id<NSExpression>;

    #[cfg(feature = "Foundation_NSExpression")]
    #[objc2::method(sel = "setCoreSpotlightDisplayNameExpression:")]
    pub unsafe fn setCoreSpotlightDisplayNameExpression(
        &self,
        core_spotlight_display_name_expression: &NSExpression,
    );
}
