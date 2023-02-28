//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum HKElectrocardiogramLead {
    HKElectrocardiogramLeadAppleWatchSimilarToLeadI = 1,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum HKElectrocardiogramClassification {
    HKElectrocardiogramClassificationNotSet = 0,
    HKElectrocardiogramClassificationSinusRhythm = 1,
    HKElectrocardiogramClassificationAtrialFibrillation = 2,
    HKElectrocardiogramClassificationInconclusiveLowHeartRate = 3,
    HKElectrocardiogramClassificationInconclusiveHighHeartRate = 4,
    HKElectrocardiogramClassificationInconclusivePoorReading = 5,
    HKElectrocardiogramClassificationInconclusiveOther = 6,
    HKElectrocardiogramClassificationUnrecognized = 100,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum HKElectrocardiogramSymptomsStatus {
    HKElectrocardiogramSymptomsStatusNotSet = 0,
    HKElectrocardiogramSymptomsStatusNone = 1,
    HKElectrocardiogramSymptomsStatusPresent = 2,
}

#[objc2::interface(
    unsafe super = HKSample,
    unsafe inherits = [
        HKObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKElectrocardiogram;
}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSCoding for HKElectrocardiogram {}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSObjectProtocol for HKElectrocardiogram {}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSSecureCoding for HKElectrocardiogram {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    pub type HKElectrocardiogram;

    #[objc2::method(sel = "numberOfVoltageMeasurements")]
    pub unsafe fn numberOfVoltageMeasurements(&self) -> NSInteger;

    #[cfg(feature = "HealthKit_HKQuantity")]
    #[objc2::method(sel = "samplingFrequency", managed = "Other")]
    pub unsafe fn samplingFrequency(&self) -> Option<Id<HKQuantity>>;

    #[objc2::method(sel = "classification")]
    pub unsafe fn classification(&self) -> HKElectrocardiogramClassification;

    #[cfg(feature = "HealthKit_HKQuantity")]
    #[objc2::method(sel = "averageHeartRate", managed = "Other")]
    pub unsafe fn averageHeartRate(&self) -> Option<Id<HKQuantity>>;

    #[objc2::method(sel = "symptomsStatus")]
    pub unsafe fn symptomsStatus(&self) -> HKElectrocardiogramSymptomsStatus;
}

extern_static!(HKPredicateKeyPathAverageHeartRate: &'static NSString);

extern_static!(HKPredicateKeyPathECGClassification: &'static NSString);

extern_static!(HKPredicateKeyPathECGSymptomsStatus: &'static NSString);
