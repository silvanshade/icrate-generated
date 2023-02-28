//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSSortDescriptor,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CKLocationSortDescriptor;
}

#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
unsafe impl NSCoding for CKLocationSortDescriptor {}

#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
unsafe impl NSObjectProtocol for CKLocationSortDescriptor {}

#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
unsafe impl NSSecureCoding for CKLocationSortDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    pub type CKLocationSortDescriptor;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[cfg(all(feature = "CoreLocation_CLLocation", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithKey:relativeLocation:", managed = "Init")]
    pub unsafe fn initWithKey_relativeLocation(
        this: Option<Allocated<Self>>,
        key: &NSString,
        relative_location: &CLLocation,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, a_decoder: &NSCoder) -> Id<Self>;

    #[cfg(feature = "CoreLocation_CLLocation")]
    #[objc2::method(sel = "relativeLocation", managed = "Other")]
    pub unsafe fn relativeLocation(&self) -> Id<CLLocation>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSSortDescriptor`
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    pub type CKLocationSortDescriptor;

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
}
