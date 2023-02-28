//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKDeletedObject")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKDeletedObject;
}

#[cfg(feature = "HealthKit_HKDeletedObject")]
unsafe impl NSCoding for HKDeletedObject {}

#[cfg(feature = "HealthKit_HKDeletedObject")]
unsafe impl NSObjectProtocol for HKDeletedObject {}

#[cfg(feature = "HealthKit_HKDeletedObject")]
unsafe impl NSSecureCoding for HKDeletedObject {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKDeletedObject")]
    pub type HKDeletedObject;

    #[cfg(feature = "Foundation_NSUUID")]
    #[objc2::method(sel = "UUID", managed = "Other")]
    pub unsafe fn UUID(&self) -> Id<NSUUID>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "metadata", managed = "Other")]
    pub unsafe fn metadata(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}
