//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ILClassificationRequest;
}

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSCoding for ILClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSObjectProtocol for ILClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSSecureCoding for ILClassificationRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    pub type ILClassificationRequest;
}
