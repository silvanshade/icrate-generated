//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

#[objc2::interface(
    unsafe super = MXMetric,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MetricKit_MXLocationActivityMetric")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MXLocationActivityMetric;
}

#[cfg(feature = "MetricKit_MXLocationActivityMetric")]
unsafe impl NSCoding for MXLocationActivityMetric {}

#[cfg(feature = "MetricKit_MXLocationActivityMetric")]
unsafe impl NSObjectProtocol for MXLocationActivityMetric {}

#[cfg(feature = "MetricKit_MXLocationActivityMetric")]
unsafe impl NSSecureCoding for MXLocationActivityMetric {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MetricKit_MXLocationActivityMetric")]
    pub type MXLocationActivityMetric;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitDuration"
    ))]
    #[objc2::method(sel = "cumulativeBestAccuracyTime", managed = "Other")]
    pub unsafe fn cumulativeBestAccuracyTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitDuration"
    ))]
    #[objc2::method(sel = "cumulativeBestAccuracyForNavigationTime", managed = "Other")]
    pub unsafe fn cumulativeBestAccuracyForNavigationTime(
        &self,
    ) -> Id<NSMeasurement<NSUnitDuration>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitDuration"
    ))]
    #[objc2::method(sel = "cumulativeNearestTenMetersAccuracyTime", managed = "Other")]
    pub unsafe fn cumulativeNearestTenMetersAccuracyTime(
        &self,
    ) -> Id<NSMeasurement<NSUnitDuration>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitDuration"
    ))]
    #[objc2::method(sel = "cumulativeHundredMetersAccuracyTime", managed = "Other")]
    pub unsafe fn cumulativeHundredMetersAccuracyTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitDuration"
    ))]
    #[objc2::method(sel = "cumulativeKilometerAccuracyTime", managed = "Other")]
    pub unsafe fn cumulativeKilometerAccuracyTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitDuration"
    ))]
    #[objc2::method(sel = "cumulativeThreeKilometersAccuracyTime", managed = "Other")]
    pub unsafe fn cumulativeThreeKilometersAccuracyTime(&self)
        -> Id<NSMeasurement<NSUnitDuration>>;
}
