//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSCoder,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated = "Use NSXPCConnection instead"]
    #[cfg(feature = "Foundation_NSPortCoder")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSPortCoder;
}

#[cfg(feature = "Foundation_NSPortCoder")]
unsafe impl NSObjectProtocol for NSPortCoder {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSPortCoder")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub type NSPortCoder;

    #[objc2::method(sel = "isBycopy")]
    pub unsafe fn isBycopy(&self) -> bool;

    #[objc2::method(sel = "isByref")]
    pub unsafe fn isByref(&self) -> bool;

    #[cfg(feature = "Foundation_NSPort")]
    #[objc2::method(sel = "encodePortObject:")]
    pub unsafe fn encodePortObject(&self, aport: &NSPort);

    #[cfg(feature = "Foundation_NSPort")]
    #[objc2::method(sel = "decodePortObject", managed = "Other")]
    pub unsafe fn decodePortObject(&self) -> Option<Id<NSPort>>;

    #[cfg(feature = "Foundation_NSConnection")]
    #[deprecated]
    #[objc2::method(sel = "connection", managed = "Other")]
    pub unsafe fn connection(&self) -> Option<Id<NSConnection>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPort"))]
    #[deprecated]
    #[objc2::method(
        sel = "portCoderWithReceivePort:sendPort:components:",
        managed = "Other"
    )]
    pub unsafe fn portCoderWithReceivePort_sendPort_components(
        rcv_port: Option<&NSPort>,
        snd_port: Option<&NSPort>,
        comps: Option<&NSArray>,
    ) -> Id<Object>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPort"))]
    #[deprecated]
    #[objc2::method(sel = "initWithReceivePort:sendPort:components:", managed = "Init")]
    pub unsafe fn initWithReceivePort_sendPort_components(
        this: Option<Allocated<Self>>,
        rcv_port: Option<&NSPort>,
        snd_port: Option<&NSPort>,
        comps: Option<&NSArray>,
    ) -> Id<Self>;

    #[deprecated]
    #[objc2::method(sel = "dispatch")]
    pub unsafe fn dispatch(&self);
}
