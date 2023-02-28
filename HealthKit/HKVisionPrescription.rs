//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[ns_enum]
#[underlying(NSUInteger)]
pub enum HKVisionPrescriptionType {
    HKVisionPrescriptionTypeGlasses = 1,
    HKVisionPrescriptionTypeContacts = 2,
}

#[objc2::interface(
    unsafe super = HKSample,
    unsafe inherits = [
        HKObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKVisionPrescription")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKVisionPrescription;
}

#[cfg(feature = "HealthKit_HKVisionPrescription")]
unsafe impl NSCoding for HKVisionPrescription {}

#[cfg(feature = "HealthKit_HKVisionPrescription")]
unsafe impl NSObjectProtocol for HKVisionPrescription {}

#[cfg(feature = "HealthKit_HKVisionPrescription")]
unsafe impl NSSecureCoding for HKVisionPrescription {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKVisionPrescription")]
    pub type HKVisionPrescription;

    #[objc2::method(sel = "prescriptionType")]
    pub unsafe fn prescriptionType(&self) -> HKVisionPrescriptionType;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "dateIssued", managed = "Other")]
    pub unsafe fn dateIssued(&self) -> Id<NSDate>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "expirationDate", managed = "Other")]
    pub unsafe fn expirationDate(&self) -> Option<Id<NSDate>>;

    #[cfg(all(
        feature = "Foundation_NSDate",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "HealthKit_HKDevice"
    ))]
    #[objc2::method(
        sel = "prescriptionWithType:dateIssued:expirationDate:device:metadata:",
        managed = "Other"
    )]
    pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
        r#type: HKVisionPrescriptionType,
        date_issued: &NSDate,
        expiration_date: Option<&NSDate>,
        device: Option<&HKDevice>,
        metadata: Option<&NSDictionary<NSString, Object>>,
    ) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;
}
