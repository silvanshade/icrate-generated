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
    #[cfg(feature = "MetricKit_MXDisplayMetric")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MXDisplayMetric;
}

#[cfg(feature = "MetricKit_MXDisplayMetric")]
unsafe impl NSCoding for MXDisplayMetric {}

#[cfg(feature = "MetricKit_MXDisplayMetric")]
unsafe impl NSObjectProtocol for MXDisplayMetric {}

#[cfg(feature = "MetricKit_MXDisplayMetric")]
unsafe impl NSSecureCoding for MXDisplayMetric {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MetricKit_MXDisplayMetric")]
    pub type MXDisplayMetric;

    #[cfg(all(
        feature = "MetricKit_MXAverage",
        feature = "MetricKit_MXUnitAveragePixelLuminance"
    ))]
    #[objc2::method(sel = "averagePixelLuminance", managed = "Other")]
    pub unsafe fn averagePixelLuminance(
        &self,
    ) -> Option<Id<MXAverage<MXUnitAveragePixelLuminance>>>;
}
