//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCompoundPredicateType {
        NSNotPredicateType = 0,
        NSAndPredicateType = 1,
        NSOrPredicateType = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCompoundPredicate")]
    pub struct NSCompoundPredicate;

    #[cfg(feature = "Foundation_NSCompoundPredicate")]
    unsafe impl ClassType for NSCompoundPredicate {
        #[inherits(NSObject)]
        type Super = NSPredicate;
    }
);

#[cfg(feature = "Foundation_NSCompoundPredicate")]
unsafe impl NSCoding for NSCompoundPredicate {}

#[cfg(feature = "Foundation_NSCompoundPredicate")]
unsafe impl NSObjectProtocol for NSCompoundPredicate {}

#[cfg(feature = "Foundation_NSCompoundPredicate")]
unsafe impl NSSecureCoding for NSCompoundPredicate {}

extern_methods!(
    #[cfg(feature = "Foundation_NSCompoundPredicate")]
    unsafe impl NSCompoundPredicate {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithType:subpredicates:)]
        pub unsafe fn initWithType_subpredicates(
            this: Option<Allocated<Self>>,
            r#type: NSCompoundPredicateType,
            subpredicates: &NSArray<NSPredicate>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(compoundPredicateType)]
        pub unsafe fn compoundPredicateType(&self) -> NSCompoundPredicateType;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subpredicates)]
        pub unsafe fn subpredicates(&self) -> Id<NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other andPredicateWithSubpredicates:)]
        pub unsafe fn andPredicateWithSubpredicates(
            subpredicates: &NSArray<NSPredicate>,
        ) -> Id<NSCompoundPredicate, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other orPredicateWithSubpredicates:)]
        pub unsafe fn orPredicateWithSubpredicates(
            subpredicates: &NSArray<NSPredicate>,
        ) -> Id<NSCompoundPredicate, Shared>;

        #[method_id(@__retain_semantics Other notPredicateWithSubpredicate:)]
        pub unsafe fn notPredicateWithSubpredicate(
            predicate: &NSPredicate,
        ) -> Id<NSCompoundPredicate, Shared>;
    }
);
