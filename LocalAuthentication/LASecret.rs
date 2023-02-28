//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "LocalAuthentication_LASecret")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type LASecret;
}

#[cfg(feature = "LocalAuthentication_LASecret")]
unsafe impl NSObjectProtocol for LASecret {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "LocalAuthentication_LASecret")]
    pub type LASecret;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
    #[objc2::method(sel = "loadDataWithCompletion:")]
    pub unsafe fn loadDataWithCompletion(&self, handler: &Block<(*mut NSData, *mut NSError), ()>);

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}
