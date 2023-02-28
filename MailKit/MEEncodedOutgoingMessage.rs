//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MEEncodedOutgoingMessage;
}

#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
unsafe impl NSCoding for MEEncodedOutgoingMessage {}

#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
unsafe impl NSObjectProtocol for MEEncodedOutgoingMessage {}

#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
unsafe impl NSSecureCoding for MEEncodedOutgoingMessage {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
    pub type MEEncodedOutgoingMessage;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "initWithRawData:isSigned:isEncrypted:", managed = "Init")]
    pub unsafe fn initWithRawData_isSigned_isEncrypted(
        this: Option<Allocated<Self>>,
        raw_data: &NSData,
        is_signed: bool,
        is_encrypted: bool,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "rawData", managed = "Other")]
    pub unsafe fn rawData(&self) -> Id<NSData>;

    #[objc2::method(sel = "isSigned")]
    pub unsafe fn isSigned(&self) -> bool;

    #[objc2::method(sel = "isEncrypted")]
    pub unsafe fn isEncrypted(&self) -> bool;
}
