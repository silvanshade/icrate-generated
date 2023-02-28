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
    #[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MXNetworkTransferMetric;
}

#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
unsafe impl NSCoding for MXNetworkTransferMetric {}

#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
unsafe impl NSObjectProtocol for MXNetworkTransferMetric {}

#[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
unsafe impl NSSecureCoding for MXNetworkTransferMetric {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
    pub type MXNetworkTransferMetric;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitInformationStorage"
    ))]
    #[objc2::method(sel = "cumulativeWifiUpload", managed = "Other")]
    pub unsafe fn cumulativeWifiUpload(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitInformationStorage"
    ))]
    #[objc2::method(sel = "cumulativeWifiDownload", managed = "Other")]
    pub unsafe fn cumulativeWifiDownload(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitInformationStorage"
    ))]
    #[objc2::method(sel = "cumulativeCellularUpload", managed = "Other")]
    pub unsafe fn cumulativeCellularUpload(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;

    #[cfg(all(
        feature = "Foundation_NSMeasurement",
        feature = "Foundation_NSUnitInformationStorage"
    ))]
    #[objc2::method(sel = "cumulativeCellularDownload", managed = "Other")]
    pub unsafe fn cumulativeCellularDownload(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;
}
