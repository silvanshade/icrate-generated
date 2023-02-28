//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSPropertyDescription,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "CoreData_NSFetchedPropertyDescription")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSFetchedPropertyDescription;
}

#[cfg(feature = "CoreData_NSFetchedPropertyDescription")]
unsafe impl NSCoding for NSFetchedPropertyDescription {}

#[cfg(feature = "CoreData_NSFetchedPropertyDescription")]
unsafe impl NSObjectProtocol for NSFetchedPropertyDescription {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CoreData_NSFetchedPropertyDescription")]
    pub type NSFetchedPropertyDescription;

    #[cfg(feature = "CoreData_NSFetchRequest")]
    #[objc2::method(sel = "fetchRequest", managed = "Other")]
    pub unsafe fn fetchRequest(&self) -> Option<Id<NSFetchRequest>>;

    #[cfg(feature = "CoreData_NSFetchRequest")]
    #[objc2::method(sel = "setFetchRequest:")]
    pub unsafe fn setFetchRequest(&self, fetch_request: Option<&NSFetchRequest>);
}
