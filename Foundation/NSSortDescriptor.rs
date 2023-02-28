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
    #[cfg(feature = "Foundation_NSSortDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSSortDescriptor;
}

#[cfg(feature = "Foundation_NSSortDescriptor")]
unsafe impl NSCoding for NSSortDescriptor {}

#[cfg(feature = "Foundation_NSSortDescriptor")]
unsafe impl NSObjectProtocol for NSSortDescriptor {}

#[cfg(feature = "Foundation_NSSortDescriptor")]
unsafe impl NSSecureCoding for NSSortDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSSortDescriptor")]
    pub type NSSortDescriptor;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "sortDescriptorWithKey:ascending:", managed = "Other")]
    pub unsafe fn sortDescriptorWithKey_ascending(
        key: Option<&NSString>,
        ascending: bool,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "sortDescriptorWithKey:ascending:selector:", managed = "Other")]
    pub unsafe fn sortDescriptorWithKey_ascending_selector(
        key: Option<&NSString>,
        ascending: bool,
        selector: Option<Sel>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithKey:ascending:", managed = "Init")]
    pub unsafe fn initWithKey_ascending(
        this: Option<Allocated<Self>>,
        key: Option<&NSString>,
        ascending: bool,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithKey:ascending:selector:", managed = "Init")]
    pub unsafe fn initWithKey_ascending_selector(
        this: Option<Allocated<Self>>,
        key: Option<&NSString>,
        ascending: bool,
        selector: Option<Sel>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder)
        -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "key", managed = "Other")]
    pub unsafe fn key(&self) -> Option<Id<NSString>>;

    #[objc2::method(sel = "ascending")]
    pub unsafe fn ascending(&self) -> bool;

    #[objc2::method(sel = "selector")]
    pub unsafe fn selector(&self) -> Option<Sel>;

    #[objc2::method(sel = "allowEvaluation")]
    pub unsafe fn allowEvaluation(&self);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "sortDescriptorWithKey:ascending:comparator:", managed = "Other")]
    pub unsafe fn sortDescriptorWithKey_ascending_comparator(
        key: Option<&NSString>,
        ascending: bool,
        cmptr: NSComparator,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithKey:ascending:comparator:", managed = "Init")]
    pub unsafe fn initWithKey_ascending_comparator(
        this: Option<Allocated<Self>>,
        key: Option<&NSString>,
        ascending: bool,
        cmptr: NSComparator,
    ) -> Id<Self>;

    #[objc2::method(sel = "comparator")]
    pub unsafe fn comparator(&self) -> NSComparator;

    #[objc2::method(sel = "compareObject:toObject:")]
    pub unsafe fn compareObject_toObject(
        &self,
        object1: &Object,
        object2: &Object,
    ) -> NSComparisonResult;

    #[objc2::method(sel = "reversedSortDescriptor", managed = "Other")]
    pub unsafe fn reversedSortDescriptor(&self) -> Id<Object>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSSet")]
    pub type NSSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSSortDescriptor"
    ))]
    #[objc2::method(sel = "sortedArrayUsingDescriptors:", managed = "Other")]
    pub unsafe fn sortedArrayUsingDescriptors(
        &self,
        sort_descriptors: &NSArray<NSSortDescriptor>,
    ) -> Id<NSArray<ObjectType>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSArray")]
    pub type NSArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(feature = "Foundation_NSSortDescriptor")]
    #[objc2::method(sel = "sortedArrayUsingDescriptors:", managed = "Other")]
    pub unsafe fn sortedArrayUsingDescriptors(
        &self,
        sort_descriptors: &NSArray<NSSortDescriptor>,
    ) -> Id<NSArray<ObjectType>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMutableArray")]
    pub type NSMutableArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSSortDescriptor"
    ))]
    #[objc2::method(sel = "sortUsingDescriptors:")]
    pub unsafe fn sortUsingDescriptors(&self, sort_descriptors: &NSArray<NSSortDescriptor>);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSOrderedSet")]
    pub type NSOrderedSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSSortDescriptor"
    ))]
    #[objc2::method(sel = "sortedArrayUsingDescriptors:", managed = "Other")]
    pub unsafe fn sortedArrayUsingDescriptors(
        &self,
        sort_descriptors: &NSArray<NSSortDescriptor>,
    ) -> Id<NSArray<ObjectType>>;
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

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSSortDescriptor"
    ))]
    #[objc2::method(sel = "sortUsingDescriptors:")]
    pub unsafe fn sortUsingDescriptors(&self, sort_descriptors: &NSArray<NSSortDescriptor>);
}
