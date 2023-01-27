//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;

pub type CFNotificationName = CFStringRef;

pub type CFNotificationCenterRef = *mut c_void;

pub type CFNotificationCallback = Option<
    unsafe extern "C" fn(
        CFNotificationCenterRef,
        *mut c_void,
        CFNotificationName,
        *mut c_void,
        CFDictionaryRef,
    ),
>;

ns_enum!(
    #[underlying(CFIndex)]
    pub enum CFNotificationSuspensionBehavior {
        CFNotificationSuspensionBehaviorDrop = 1,
        CFNotificationSuspensionBehaviorCoalesce = 2,
        CFNotificationSuspensionBehaviorHold = 3,
        CFNotificationSuspensionBehaviorDeliverImmediately = 4,
    }
);

extern_fn!(
    pub unsafe fn CFNotificationCenterGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFNotificationCenterGetLocalCenter() -> CFNotificationCenterRef;
);

extern_fn!(
    pub unsafe fn CFNotificationCenterGetDistributedCenter() -> CFNotificationCenterRef;
);

extern_fn!(
    pub unsafe fn CFNotificationCenterGetDarwinNotifyCenter() -> CFNotificationCenterRef;
);

extern_fn!(
    pub unsafe fn CFNotificationCenterAddObserver(
        center: CFNotificationCenterRef,
        observer: *mut c_void,
        call_back: CFNotificationCallback,
        name: CFStringRef,
        object: *mut c_void,
        suspension_behavior: CFNotificationSuspensionBehavior,
    );
);

extern_fn!(
    pub unsafe fn CFNotificationCenterRemoveObserver(
        center: CFNotificationCenterRef,
        observer: *mut c_void,
        name: CFNotificationName,
        object: *mut c_void,
    );
);

extern_fn!(
    pub unsafe fn CFNotificationCenterRemoveEveryObserver(
        center: CFNotificationCenterRef,
        observer: *mut c_void,
    );
);

extern_fn!(
    pub unsafe fn CFNotificationCenterPostNotification(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *mut c_void,
        user_info: CFDictionaryRef,
        deliver_immediately: Boolean,
    );
);

ns_enum!(
    #[underlying(CFOptionFlags)]
    pub enum __anonymous__ {
        kCFNotificationDeliverImmediately = 1 << 0,
        kCFNotificationPostToAllSessions = 1 << 1,
    }
);

extern_fn!(
    pub unsafe fn CFNotificationCenterPostNotificationWithOptions(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *mut c_void,
        user_info: CFDictionaryRef,
        options: CFOptionFlags,
    );
);
