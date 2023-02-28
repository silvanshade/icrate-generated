//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSProxy,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[deprecated = "Use NSXPCConnection instead"]
    #[cfg(feature = "Foundation_NSDistantObject")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSDistantObject;
}

#[cfg(feature = "Foundation_NSDistantObject")]
unsafe impl NSCoding for NSDistantObject {}

#[cfg(feature = "Foundation_NSDistantObject")]
unsafe impl NSObjectProtocol for NSDistantObject {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSDistantObject")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub type NSDistantObject;

    #[cfg(feature = "Foundation_NSConnection")]
    #[objc2::method(sel = "proxyWithTarget:connection:", managed = "Other")]
    pub unsafe fn proxyWithTarget_connection(
        target: &Object,
        connection: &NSConnection,
    ) -> Option<Id<Object>>;

    #[cfg(feature = "Foundation_NSConnection")]
    #[objc2::method(sel = "initWithTarget:connection:", managed = "Init")]
    pub unsafe fn initWithTarget_connection(
        this: Option<Allocated<Self>>,
        target: &Object,
        connection: &NSConnection,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSConnection")]
    #[objc2::method(sel = "proxyWithLocal:connection:", managed = "Other")]
    pub unsafe fn proxyWithLocal_connection(
        target: &Object,
        connection: &NSConnection,
    ) -> Id<Object>;

    #[cfg(feature = "Foundation_NSConnection")]
    #[objc2::method(sel = "initWithLocal:connection:", managed = "Init")]
    pub unsafe fn initWithLocal_connection(
        this: Option<Allocated<Self>>,
        target: &Object,
        connection: &NSConnection,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(
        this: Option<Allocated<Self>>,
        in_coder: &NSCoder,
    ) -> Option<Id<Self>>;

    #[objc2::method(sel = "setProtocolForProxy:")]
    pub unsafe fn setProtocolForProxy(&self, proto: Option<&Protocol>);

    #[cfg(feature = "Foundation_NSConnection")]
    #[objc2::method(sel = "connectionForProxy", managed = "Other")]
    pub unsafe fn connectionForProxy(&self) -> Id<NSConnection>;
}
