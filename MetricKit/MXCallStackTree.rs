//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MetricKit_MXCallStackTree")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MXCallStackTree;
}

#[cfg(feature = "MetricKit_MXCallStackTree")]
unsafe impl NSCoding for MXCallStackTree {}

#[cfg(feature = "MetricKit_MXCallStackTree")]
unsafe impl NSObjectProtocol for MXCallStackTree {}

#[cfg(feature = "MetricKit_MXCallStackTree")]
unsafe impl NSSecureCoding for MXCallStackTree {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MetricKit_MXCallStackTree")]
    pub type MXCallStackTree;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "JSONRepresentation", managed = "Other")]
    pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;
}
