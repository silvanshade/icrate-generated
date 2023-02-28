//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = CNFetchRequest,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Contacts_CNContactFetchRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CNContactFetchRequest;
}

#[cfg(feature = "Contacts_CNContactFetchRequest")]
unsafe impl NSCoding for CNContactFetchRequest {}

#[cfg(feature = "Contacts_CNContactFetchRequest")]
unsafe impl NSObjectProtocol for CNContactFetchRequest {}

#[cfg(feature = "Contacts_CNContactFetchRequest")]
unsafe impl NSSecureCoding for CNContactFetchRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Contacts_CNContactFetchRequest")]
    pub type CNContactFetchRequest;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "initWithKeysToFetch:", managed = "Init")]
    pub unsafe fn initWithKeysToFetch(
        this: Option<Allocated<Self>>,
        keys_to_fetch: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "predicate", managed = "Other")]
    pub unsafe fn predicate(&self) -> Option<Id<NSPredicate>>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "setPredicate:")]
    pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "keysToFetch", managed = "Other")]
    pub unsafe fn keysToFetch(&self) -> Id<NSArray<ProtocolObject<dyn CNKeyDescriptor>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setKeysToFetch:")]
    pub unsafe fn setKeysToFetch(
        &self,
        keys_to_fetch: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
    );

    #[objc2::method(sel = "mutableObjects")]
    pub unsafe fn mutableObjects(&self) -> bool;

    #[objc2::method(sel = "setMutableObjects:")]
    pub unsafe fn setMutableObjects(&self, mutable_objects: bool);

    #[objc2::method(sel = "unifyResults")]
    pub unsafe fn unifyResults(&self) -> bool;

    #[objc2::method(sel = "setUnifyResults:")]
    pub unsafe fn setUnifyResults(&self, unify_results: bool);

    #[objc2::method(sel = "sortOrder")]
    pub unsafe fn sortOrder(&self) -> CNContactSortOrder;

    #[objc2::method(sel = "setSortOrder:")]
    pub unsafe fn setSortOrder(&self, sort_order: CNContactSortOrder);
}
