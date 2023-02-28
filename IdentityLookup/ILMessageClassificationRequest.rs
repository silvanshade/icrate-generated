//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

#[objc2::interface(
    unsafe super = ILClassificationRequest,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ILMessageClassificationRequest;
}

#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
unsafe impl NSCoding for ILMessageClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
unsafe impl NSObjectProtocol for ILMessageClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
unsafe impl NSSecureCoding for ILMessageClassificationRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
    pub type ILMessageClassificationRequest;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "IdentityLookup_ILMessageCommunication"
    ))]
    #[objc2::method(sel = "messageCommunications", managed = "Other")]
    pub unsafe fn messageCommunications(&self) -> Id<NSArray<ILMessageCommunication>>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}
