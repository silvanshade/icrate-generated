//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ExceptionHandling::*;
use crate::Foundation::*;

extern_static!(NSUncaughtSystemExceptionException: Option<&'static NSString>);

extern_static!(NSUncaughtRuntimeErrorException: Option<&'static NSString>);

extern_static!(NSStackTraceKey: Option<&'static NSString>);

extern_fn!(
    pub unsafe fn NSExceptionHandlerResume();
);

#[extern_enum]
#[underlying(c_uint)]
pub enum __anonymous__ {
    NSLogUncaughtExceptionMask = 1 << 0,
    NSHandleUncaughtExceptionMask = 1 << 1,
    NSLogUncaughtSystemExceptionMask = 1 << 2,
    NSHandleUncaughtSystemExceptionMask = 1 << 3,
    NSLogUncaughtRuntimeErrorMask = 1 << 4,
    NSHandleUncaughtRuntimeErrorMask = 1 << 5,
    NSLogTopLevelExceptionMask = 1 << 6,
    NSHandleTopLevelExceptionMask = 1 << 7,
    NSLogOtherExceptionMask = 1 << 8,
    NSHandleOtherExceptionMask = 1 << 9,
}

#[extern_enum]
#[underlying(c_uint)]
pub enum __anonymous__ {
    NSHangOnUncaughtExceptionMask = 1 << 0,
    NSHangOnUncaughtSystemExceptionMask = 1 << 1,
    NSHangOnUncaughtRuntimeErrorMask = 1 << 2,
    NSHangOnTopLevelExceptionMask = 1 << 3,
    NSHangOnOtherExceptionMask = 1 << 4,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSExceptionHandler;
}

#[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
unsafe impl NSObjectProtocol for NSExceptionHandler {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
    pub type NSExceptionHandler;

    #[objc2::method(sel = "defaultExceptionHandler", managed = "Other")]
    pub unsafe fn defaultExceptionHandler() -> Option<Id<NSExceptionHandler>>;

    #[objc2::method(sel = "setExceptionHandlingMask:")]
    pub unsafe fn setExceptionHandlingMask(&self, a_mask: NSUInteger);

    #[objc2::method(sel = "exceptionHandlingMask")]
    pub unsafe fn exceptionHandlingMask(&self) -> NSUInteger;

    #[objc2::method(sel = "setExceptionHangingMask:")]
    pub unsafe fn setExceptionHangingMask(&self, a_mask: NSUInteger);

    #[objc2::method(sel = "exceptionHangingMask")]
    pub unsafe fn exceptionHangingMask(&self) -> NSUInteger;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, an_object: Option<&Object>);

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<Object>>;
}
