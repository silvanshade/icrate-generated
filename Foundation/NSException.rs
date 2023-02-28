//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSGenericException: &'static NSExceptionName);

extern_static!(NSRangeException: &'static NSExceptionName);

extern_static!(NSInvalidArgumentException: &'static NSExceptionName);

extern_static!(NSInternalInconsistencyException: &'static NSExceptionName);

extern_static!(NSMallocException: &'static NSExceptionName);

extern_static!(NSObjectInaccessibleException: &'static NSExceptionName);

extern_static!(NSObjectNotAvailableException: &'static NSExceptionName);

extern_static!(NSDestinationInvalidException: &'static NSExceptionName);

extern_static!(NSPortTimeoutException: &'static NSExceptionName);

extern_static!(NSInvalidSendPortException: &'static NSExceptionName);

extern_static!(NSInvalidReceivePortException: &'static NSExceptionName);

extern_static!(NSPortSendException: &'static NSExceptionName);

extern_static!(NSPortReceiveException: &'static NSExceptionName);

extern_static!(NSOldStyleException: &'static NSExceptionName);

extern_static!(NSInconsistentArchiveException: &'static NSExceptionName);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSException")]
    #[derive(PartialEq, Eq, Hash)]
    pub type NSException;
}

#[cfg(feature = "Foundation_NSException")]
unsafe impl NSCoding for NSException {}

#[cfg(feature = "Foundation_NSException")]
unsafe impl NSObjectProtocol for NSException {}

#[cfg(feature = "Foundation_NSException")]
unsafe impl NSSecureCoding for NSException {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSException")]
    pub type NSException;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "exceptionWithName:reason:userInfo:", managed = "Other")]
    pub unsafe fn exceptionWithName_reason_userInfo(
        name: &NSExceptionName,
        reason: Option<&NSString>,
        user_info: Option<&NSDictionary>,
    ) -> Id<NSException>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithName:reason:userInfo:", managed = "Init")]
    pub unsafe fn initWithName_reason_userInfo(
        this: Option<Allocated<Self>>,
        a_name: &NSExceptionName,
        a_reason: Option<&NSString>,
        a_user_info: Option<&NSDictionary>,
    ) -> Id<Self>;

    #[objc2::method(sel = "name", managed = "Other")]
    pub fn name(&self) -> Id<NSExceptionName>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "reason", managed = "Other")]
    pub fn reason(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "userInfo", managed = "Other")]
    pub fn userInfo(&self) -> Option<Id<NSDictionary>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
    #[objc2::method(sel = "callStackReturnAddresses", managed = "Other")]
    pub unsafe fn callStackReturnAddresses(&self) -> Id<NSArray<NSNumber>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "callStackSymbols", managed = "Other")]
    pub unsafe fn callStackSymbols(&self) -> Id<NSArray<NSString>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSException")]
    pub type NSException;
}

pub type NSUncaughtExceptionHandler = TodoFunction;

extern_fn!(
    pub unsafe fn NSGetUncaughtExceptionHandler() -> *mut NSUncaughtExceptionHandler;
);

extern_fn!(
    pub unsafe fn NSSetUncaughtExceptionHandler(_: *mut NSUncaughtExceptionHandler);
);

extern_static!(NSAssertionHandlerKey: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSAssertionHandler")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSAssertionHandler;
}

#[cfg(feature = "Foundation_NSAssertionHandler")]
unsafe impl NSObjectProtocol for NSAssertionHandler {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSAssertionHandler")]
    pub type NSAssertionHandler;

    #[objc2::method(sel = "currentHandler", managed = "Other")]
    pub unsafe fn currentHandler() -> Id<NSAssertionHandler>;
}
