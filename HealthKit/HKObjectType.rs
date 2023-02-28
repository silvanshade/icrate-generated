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
    #[cfg(feature = "HealthKit_HKObjectType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKObjectType;
}

#[cfg(feature = "HealthKit_HKObjectType")]
unsafe impl NSCoding for HKObjectType {}

#[cfg(feature = "HealthKit_HKObjectType")]
unsafe impl NSObjectProtocol for HKObjectType {}

#[cfg(feature = "HealthKit_HKObjectType")]
unsafe impl NSSecureCoding for HKObjectType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKObjectType")]
    pub type HKObjectType;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Id<NSString>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "HealthKit_HKQuantityType")]
    #[objc2::method(sel = "quantityTypeForIdentifier:", managed = "Other")]
    pub unsafe fn quantityTypeForIdentifier(
        identifier: &HKQuantityTypeIdentifier,
    ) -> Option<Id<HKQuantityType>>;

    #[cfg(feature = "HealthKit_HKCategoryType")]
    #[objc2::method(sel = "categoryTypeForIdentifier:", managed = "Other")]
    pub unsafe fn categoryTypeForIdentifier(
        identifier: &HKCategoryTypeIdentifier,
    ) -> Option<Id<HKCategoryType>>;

    #[cfg(feature = "HealthKit_HKCharacteristicType")]
    #[objc2::method(sel = "characteristicTypeForIdentifier:", managed = "Other")]
    pub unsafe fn characteristicTypeForIdentifier(
        identifier: &HKCharacteristicTypeIdentifier,
    ) -> Option<Id<HKCharacteristicType>>;

    #[cfg(feature = "HealthKit_HKCorrelationType")]
    #[objc2::method(sel = "correlationTypeForIdentifier:", managed = "Other")]
    pub unsafe fn correlationTypeForIdentifier(
        identifier: &HKCorrelationTypeIdentifier,
    ) -> Option<Id<HKCorrelationType>>;

    #[cfg(feature = "HealthKit_HKDocumentType")]
    #[objc2::method(sel = "documentTypeForIdentifier:", managed = "Other")]
    pub unsafe fn documentTypeForIdentifier(
        identifier: &HKDocumentTypeIdentifier,
    ) -> Option<Id<HKDocumentType>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "HealthKit_HKSeriesType"))]
    #[objc2::method(sel = "seriesTypeForIdentifier:", managed = "Other")]
    pub unsafe fn seriesTypeForIdentifier(identifier: &NSString) -> Option<Id<HKSeriesType>>;

    #[cfg(feature = "HealthKit_HKWorkoutType")]
    #[objc2::method(sel = "workoutType", managed = "Other")]
    pub unsafe fn workoutType() -> Id<HKWorkoutType>;

    #[cfg(feature = "HealthKit_HKActivitySummaryType")]
    #[objc2::method(sel = "activitySummaryType", managed = "Other")]
    pub unsafe fn activitySummaryType() -> Id<HKActivitySummaryType>;

    #[cfg(feature = "HealthKit_HKAudiogramSampleType")]
    #[objc2::method(sel = "audiogramSampleType", managed = "Other")]
    pub unsafe fn audiogramSampleType() -> Id<HKAudiogramSampleType>;

    #[cfg(feature = "HealthKit_HKElectrocardiogramType")]
    #[objc2::method(sel = "electrocardiogramType", managed = "Other")]
    pub unsafe fn electrocardiogramType() -> Id<HKElectrocardiogramType>;

    #[cfg(feature = "HealthKit_HKPrescriptionType")]
    #[objc2::method(sel = "visionPrescriptionType", managed = "Other")]
    pub unsafe fn visionPrescriptionType() -> Id<HKPrescriptionType>;

    #[objc2::method(sel = "requiresPerObjectAuthorization")]
    pub unsafe fn requiresPerObjectAuthorization(&self) -> bool;
}

#[objc2::interface(
    unsafe super = HKObjectType,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKCharacteristicType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKCharacteristicType;
}

#[cfg(feature = "HealthKit_HKCharacteristicType")]
unsafe impl NSCoding for HKCharacteristicType {}

#[cfg(feature = "HealthKit_HKCharacteristicType")]
unsafe impl NSObjectProtocol for HKCharacteristicType {}

#[cfg(feature = "HealthKit_HKCharacteristicType")]
unsafe impl NSSecureCoding for HKCharacteristicType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKCharacteristicType")]
    pub type HKCharacteristicType;
}

#[objc2::interface(
    unsafe super = HKObjectType,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKSampleType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKSampleType;
}

#[cfg(feature = "HealthKit_HKSampleType")]
unsafe impl NSCoding for HKSampleType {}

#[cfg(feature = "HealthKit_HKSampleType")]
unsafe impl NSObjectProtocol for HKSampleType {}

#[cfg(feature = "HealthKit_HKSampleType")]
unsafe impl NSSecureCoding for HKSampleType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKSampleType")]
    pub type HKSampleType;

    #[objc2::method(sel = "isMaximumDurationRestricted")]
    pub unsafe fn isMaximumDurationRestricted(&self) -> bool;

    #[objc2::method(sel = "maximumAllowedDuration")]
    pub unsafe fn maximumAllowedDuration(&self) -> NSTimeInterval;

    #[objc2::method(sel = "isMinimumDurationRestricted")]
    pub unsafe fn isMinimumDurationRestricted(&self) -> bool;

    #[objc2::method(sel = "minimumAllowedDuration")]
    pub unsafe fn minimumAllowedDuration(&self) -> NSTimeInterval;

    #[objc2::method(sel = "allowsRecalibrationForEstimates")]
    pub unsafe fn allowsRecalibrationForEstimates(&self) -> bool;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKCategoryType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKCategoryType;
}

#[cfg(feature = "HealthKit_HKCategoryType")]
unsafe impl NSCoding for HKCategoryType {}

#[cfg(feature = "HealthKit_HKCategoryType")]
unsafe impl NSObjectProtocol for HKCategoryType {}

#[cfg(feature = "HealthKit_HKCategoryType")]
unsafe impl NSSecureCoding for HKCategoryType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKCategoryType")]
    pub type HKCategoryType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKCorrelationType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKCorrelationType;
}

#[cfg(feature = "HealthKit_HKCorrelationType")]
unsafe impl NSCoding for HKCorrelationType {}

#[cfg(feature = "HealthKit_HKCorrelationType")]
unsafe impl NSObjectProtocol for HKCorrelationType {}

#[cfg(feature = "HealthKit_HKCorrelationType")]
unsafe impl NSSecureCoding for HKCorrelationType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKCorrelationType")]
    pub type HKCorrelationType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKDocumentType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKDocumentType;
}

#[cfg(feature = "HealthKit_HKDocumentType")]
unsafe impl NSCoding for HKDocumentType {}

#[cfg(feature = "HealthKit_HKDocumentType")]
unsafe impl NSObjectProtocol for HKDocumentType {}

#[cfg(feature = "HealthKit_HKDocumentType")]
unsafe impl NSSecureCoding for HKDocumentType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKDocumentType")]
    pub type HKDocumentType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKQuantityType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKQuantityType;
}

#[cfg(feature = "HealthKit_HKQuantityType")]
unsafe impl NSCoding for HKQuantityType {}

#[cfg(feature = "HealthKit_HKQuantityType")]
unsafe impl NSObjectProtocol for HKQuantityType {}

#[cfg(feature = "HealthKit_HKQuantityType")]
unsafe impl NSSecureCoding for HKQuantityType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKQuantityType")]
    pub type HKQuantityType;

    #[objc2::method(sel = "aggregationStyle")]
    pub unsafe fn aggregationStyle(&self) -> HKQuantityAggregationStyle;

    #[cfg(feature = "HealthKit_HKUnit")]
    #[objc2::method(sel = "isCompatibleWithUnit:")]
    pub unsafe fn isCompatibleWithUnit(&self, unit: &HKUnit) -> bool;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKWorkoutType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKWorkoutType;
}

#[cfg(feature = "HealthKit_HKWorkoutType")]
unsafe impl NSCoding for HKWorkoutType {}

#[cfg(feature = "HealthKit_HKWorkoutType")]
unsafe impl NSObjectProtocol for HKWorkoutType {}

#[cfg(feature = "HealthKit_HKWorkoutType")]
unsafe impl NSSecureCoding for HKWorkoutType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKWorkoutType")]
    pub type HKWorkoutType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKSeriesType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKSeriesType;
}

#[cfg(feature = "HealthKit_HKSeriesType")]
unsafe impl NSCoding for HKSeriesType {}

#[cfg(feature = "HealthKit_HKSeriesType")]
unsafe impl NSObjectProtocol for HKSeriesType {}

#[cfg(feature = "HealthKit_HKSeriesType")]
unsafe impl NSSecureCoding for HKSeriesType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKSeriesType")]
    pub type HKSeriesType;

    #[objc2::method(sel = "workoutRouteType", managed = "Other")]
    pub unsafe fn workoutRouteType() -> Id<Self>;

    #[objc2::method(sel = "heartbeatSeriesType", managed = "Other")]
    pub unsafe fn heartbeatSeriesType() -> Id<Self>;
}

#[objc2::interface(
    unsafe super = HKObjectType,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKActivitySummaryType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKActivitySummaryType;
}

#[cfg(feature = "HealthKit_HKActivitySummaryType")]
unsafe impl NSCoding for HKActivitySummaryType {}

#[cfg(feature = "HealthKit_HKActivitySummaryType")]
unsafe impl NSObjectProtocol for HKActivitySummaryType {}

#[cfg(feature = "HealthKit_HKActivitySummaryType")]
unsafe impl NSSecureCoding for HKActivitySummaryType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKActivitySummaryType")]
    pub type HKActivitySummaryType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKAudiogramSampleType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKAudiogramSampleType;
}

#[cfg(feature = "HealthKit_HKAudiogramSampleType")]
unsafe impl NSCoding for HKAudiogramSampleType {}

#[cfg(feature = "HealthKit_HKAudiogramSampleType")]
unsafe impl NSObjectProtocol for HKAudiogramSampleType {}

#[cfg(feature = "HealthKit_HKAudiogramSampleType")]
unsafe impl NSSecureCoding for HKAudiogramSampleType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKAudiogramSampleType")]
    pub type HKAudiogramSampleType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKElectrocardiogramType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKElectrocardiogramType;
}

#[cfg(feature = "HealthKit_HKElectrocardiogramType")]
unsafe impl NSCoding for HKElectrocardiogramType {}

#[cfg(feature = "HealthKit_HKElectrocardiogramType")]
unsafe impl NSObjectProtocol for HKElectrocardiogramType {}

#[cfg(feature = "HealthKit_HKElectrocardiogramType")]
unsafe impl NSSecureCoding for HKElectrocardiogramType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKElectrocardiogramType")]
    pub type HKElectrocardiogramType;
}

#[objc2::interface(
    unsafe super = HKSampleType,
    unsafe inherits = [
        HKObjectType,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKPrescriptionType")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKPrescriptionType;
}

#[cfg(feature = "HealthKit_HKPrescriptionType")]
unsafe impl NSCoding for HKPrescriptionType {}

#[cfg(feature = "HealthKit_HKPrescriptionType")]
unsafe impl NSObjectProtocol for HKPrescriptionType {}

#[cfg(feature = "HealthKit_HKPrescriptionType")]
unsafe impl NSSecureCoding for HKPrescriptionType {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKPrescriptionType")]
    pub type HKPrescriptionType;
}
