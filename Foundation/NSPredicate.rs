//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPredicate")]
    pub struct NSPredicate;

    #[cfg(feature = "Foundation_NSPredicate")]
    unsafe impl ClassType for NSPredicate {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSPredicate")]
    unsafe impl NSPredicate {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateWithFormat:argumentArray:)]
        pub unsafe fn predicateWithFormat_argumentArray(
            predicate_format: &NSString,
            arguments: Option<&NSArray>,
        ) -> Id<NSPredicate, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predicateFromMetadataQueryString:)]
        pub unsafe fn predicateFromMetadataQueryString(
            query_string: &NSString,
        ) -> Option<Id<NSPredicate, Shared>>;

        #[method_id(@__retain_semantics Other predicateWithValue:)]
        pub unsafe fn predicateWithValue(value: bool) -> Id<NSPredicate, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateWithBlock:)]
        pub unsafe fn predicateWithBlock(
            block: &Block<(*mut Object, *mut NSDictionary<NSString, Object>), Bool>,
        ) -> Id<NSPredicate, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predicateFormat)]
        pub unsafe fn predicateFormat(&self) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other predicateWithSubstitutionVariables:)]
        pub unsafe fn predicateWithSubstitutionVariables(
            &self,
            variables: &NSDictionary<NSString, Object>,
        ) -> Id<Self, Shared>;

        #[method(evaluateWithObject:)]
        pub unsafe fn evaluateWithObject(&self, object: Option<&Object>) -> bool;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(evaluateWithObject:substitutionVariables:)]
        pub unsafe fn evaluateWithObject_substitutionVariables(
            &self,
            object: Option<&Object>,
            bindings: Option<&NSDictionary<NSString, Object>>,
        ) -> bool;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other filteredArrayUsingPredicate:)]
        pub unsafe fn filteredArrayUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSArray<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other filteredSetUsingPredicate:)]
        pub unsafe fn filteredSetUsingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Id<NSSet<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSMutableSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&self, predicate: &NSPredicate);
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSOrderedSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other filteredOrderedSetUsingPredicate:)]
        pub unsafe fn filteredOrderedSetUsingPredicate(
            &self,
            p: &NSPredicate,
        ) -> Id<NSOrderedSet<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSPredicateSupport
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableOrderedSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(filterUsingPredicate:)]
        pub unsafe fn filterUsingPredicate(&self, p: &NSPredicate);
    }
);
