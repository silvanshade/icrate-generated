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
    #[cfg(feature = "HealthKit_HKQueryAnchor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKQueryAnchor;
}

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSCoding for HKQueryAnchor {}

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSObjectProtocol for HKQueryAnchor {}

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSSecureCoding for HKQueryAnchor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKQueryAnchor")]
    pub type HKQueryAnchor;

    #[objc2::method(sel = "anchorFromValue:", managed = "Other")]
    pub unsafe fn anchorFromValue(value: NSUInteger) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}
