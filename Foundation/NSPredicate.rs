//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSPredicate")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSPredicate;
}

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSCoding for NSPredicate {}

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSObjectProtocol for NSPredicate {}

#[cfg(feature = "Foundation_NSPredicate")]
unsafe impl NSSecureCoding for NSPredicate {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSPredicate")]
    pub type NSPredicate;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "predicateWithFormat:argumentArray:", managed = "Other")]
    pub unsafe fn predicateWithFormat_argumentArray(
        predicate_format: &NSString,
        arguments: Option<&NSArray>,
    ) -> Id<NSPredicate>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "predicateFromMetadataQueryString:", managed = "Other")]
    pub unsafe fn predicateFromMetadataQueryString(
        query_string: &NSString,
    ) -> Option<Id<NSPredicate>>;

    #[objc2::method(sel = "predicateWithValue:", managed = "Other")]
    pub unsafe fn predicateWithValue(value: bool) -> Id<NSPredicate>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "predicateWithBlock:", managed = "Other")]
    pub unsafe fn predicateWithBlock(
        block: &Block<(*mut Object, *mut NSDictionary<NSString, Object>), Bool>,
    ) -> Id<NSPredicate>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "predicateFormat", managed = "Other")]
    pub unsafe fn predicateFormat(&self) -> Id<NSString>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "predicateWithSubstitutionVariables:", managed = "Other")]
    pub unsafe fn predicateWithSubstitutionVariables(
        &self,
        variables: &NSDictionary<NSString, Object>,
    ) -> Id<Self>;

    #[objc2::method(sel = "evaluateWithObject:")]
    pub unsafe fn evaluateWithObject(&self, object: Option<&Object>) -> bool;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "evaluateWithObject:substitutionVariables:")]
    pub unsafe fn evaluateWithObject_substitutionVariables(
        &self,
        object: Option<&Object>,
        bindings: Option<&NSDictionary<NSString, Object>>,
    ) -> bool;

    #[objc2::method(sel = "allowEvaluation")]
    pub unsafe fn allowEvaluation(&self);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSArray")]
    pub type NSArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "filteredArrayUsingPredicate:", managed = "Other")]
    pub unsafe fn filteredArrayUsingPredicate(
        &self,
        predicate: &NSPredicate,
    ) -> Id<NSArray<ObjectType>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMutableArray")]
    pub type NSMutableArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "filterUsingPredicate:")]
    pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSSet")]
    pub type NSSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "filteredSetUsingPredicate:", managed = "Other")]
    pub unsafe fn filteredSetUsingPredicate(
        &self,
        predicate: &NSPredicate,
    ) -> Id<NSSet<ObjectType>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMutableSet")]
    pub type NSMutableSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "filterUsingPredicate:")]
    pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSOrderedSet")]
    pub type NSOrderedSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "filteredOrderedSetUsingPredicate:", managed = "Other")]
    pub unsafe fn filteredOrderedSetUsingPredicate(
        &self,
        p: &NSPredicate,
    ) -> Id<NSOrderedSet<ObjectType>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    pub type NSMutableOrderedSet<
        ObjectType: Message = Object,
        ObjectTypeOwnership: Ownership = Shared,
    >;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "filterUsingPredicate:")]
    pub unsafe fn filterUsingPredicate(&self, p: &NSPredicate);
}
