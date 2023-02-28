//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[objc2::interface(
    unsafe super = HKVisionPrescription,
    unsafe inherits = [
        HKSample,
        HKObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKGlassesPrescription")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKGlassesPrescription;
}

#[cfg(feature = "HealthKit_HKGlassesPrescription")]
unsafe impl NSCoding for HKGlassesPrescription {}

#[cfg(feature = "HealthKit_HKGlassesPrescription")]
unsafe impl NSObjectProtocol for HKGlassesPrescription {}

#[cfg(feature = "HealthKit_HKGlassesPrescription")]
unsafe impl NSSecureCoding for HKGlassesPrescription {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKGlassesPrescription")]
    pub type HKGlassesPrescription;

    #[cfg(feature = "HealthKit_HKGlassesLensSpecification")]
    #[objc2::method(sel = "rightEye", managed = "Other")]
    pub unsafe fn rightEye(&self) -> Option<Id<HKGlassesLensSpecification>>;

    #[cfg(feature = "HealthKit_HKGlassesLensSpecification")]
    #[objc2::method(sel = "leftEye", managed = "Other")]
    pub unsafe fn leftEye(&self) -> Option<Id<HKGlassesLensSpecification>>;

    #[cfg(all(
        feature = "Foundation_NSDate",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "HealthKit_HKDevice",
        feature = "HealthKit_HKGlassesLensSpecification"
    ))]
    #[objc2::method(
        sel = "prescriptionWithRightEyeSpecification:leftEyeSpecification:dateIssued:expirationDate:device:metadata:",
        managed = "Other"
    )]
    pub unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_dateIssued_expirationDate_device_metadata(
        right_eye_specification: Option<&HKGlassesLensSpecification>,
        left_eye_specification: Option<&HKGlassesLensSpecification>,
        date_issued: &NSDate,
        expiration_date: Option<&NSDate>,
        device: Option<&HKDevice>,
        metadata: Option<&NSDictionary<NSString, Object>>,
    ) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

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
}
