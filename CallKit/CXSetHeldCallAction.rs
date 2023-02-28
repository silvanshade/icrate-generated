//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = CXCallAction,
    unsafe inherits = [
        CXAction,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CXSetHeldCallAction;
}

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSCoding for CXSetHeldCallAction {}

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSObjectProtocol for CXSetHeldCallAction {}

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSSecureCoding for CXSetHeldCallAction {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    pub type CXSetHeldCallAction;

    #[cfg(feature = "Foundation_NSUUID")]
    #[objc2::method(sel = "initWithCallUUID:onHold:", managed = "Init")]
    pub unsafe fn initWithCallUUID_onHold(
        this: Option<Allocated<Self>>,
        call_uuid: &NSUUID,
        on_hold: bool,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(
        this: Option<Allocated<Self>>,
        a_decoder: &NSCoder,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSUUID")]
    #[objc2::method(sel = "initWithCallUUID:", managed = "Init")]
    pub unsafe fn initWithCallUUID(this: Option<Allocated<Self>>, call_uuid: &NSUUID) -> Id<Self>;

    #[objc2::method(sel = "isOnHold")]
    pub unsafe fn isOnHold(&self) -> bool;

    #[objc2::method(sel = "setOnHold:")]
    pub unsafe fn setOnHold(&self, on_hold: bool);
}
