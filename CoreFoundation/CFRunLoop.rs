//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;

pub type CFRunLoopMode = CFStringRef;

pub type CFRunLoopRef = *mut c_void;

pub type CFRunLoopSourceRef = *mut c_void;

pub type CFRunLoopObserverRef = *mut c_void;

pub type CFRunLoopTimerRef = *mut c_void;

ns_enum!(
    #[underlying(i32)]
    pub enum CFRunLoopRunResult {
        kCFRunLoopRunFinished = 1,
        kCFRunLoopRunStopped = 2,
        kCFRunLoopRunTimedOut = 3,
        kCFRunLoopRunHandledSource = 4,
    }
);

ns_options!(
    #[underlying(CFOptionFlags)]
    pub enum CFRunLoopActivity {
        kCFRunLoopEntry = 1 << 0,
        kCFRunLoopBeforeTimers = 1 << 1,
        kCFRunLoopBeforeSources = 1 << 2,
        kCFRunLoopBeforeWaiting = 1 << 5,
        kCFRunLoopAfterWaiting = 1 << 6,
        kCFRunLoopExit = 1 << 7,
        kCFRunLoopAllActivities = 0x0FFFFFFF,
    }
);

extern_static!(kCFRunLoopDefaultMode: CFRunLoopMode);

extern_static!(kCFRunLoopCommonModes: CFRunLoopMode);

extern_fn!(
    pub unsafe fn CFRunLoopGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFRunLoopGetCurrent() -> CFRunLoopRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopGetMain() -> CFRunLoopRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopCopyCurrentMode(rl: CFRunLoopRef) -> CFRunLoopMode;
);

extern_fn!(
    pub unsafe fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFRunLoopMode);
);

extern_fn!(
    pub unsafe fn CFRunLoopGetNextTimerFireDate(
        rl: CFRunLoopRef,
        mode: CFRunLoopMode,
    ) -> CFAbsoluteTime;
);

extern_fn!(
    pub unsafe fn CFRunLoopRun();
);

extern_fn!(
    pub unsafe fn CFRunLoopRunInMode(
        mode: CFRunLoopMode,
        seconds: CFTimeInterval,
        return_after_source_handled: Boolean,
    ) -> CFRunLoopRunResult;
);

extern_fn!(
    pub unsafe fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopWakeUp(rl: CFRunLoopRef);
);

extern_fn!(
    pub unsafe fn CFRunLoopStop(rl: CFRunLoopRef);
);

extern_fn!(
    pub unsafe fn CFRunLoopPerformBlock(
        rl: CFRunLoopRef,
        mode: CFTypeRef,
        block: Option<&Block<(), ()>>,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopContainsSource(
        rl: CFRunLoopRef,
        source: CFRunLoopSourceRef,
        mode: CFRunLoopMode,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopAddSource(
        rl: CFRunLoopRef,
        source: CFRunLoopSourceRef,
        mode: CFRunLoopMode,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopRemoveSource(
        rl: CFRunLoopRef,
        source: CFRunLoopSourceRef,
        mode: CFRunLoopMode,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopContainsObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopAddObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopRemoveObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopContainsTimer(
        rl: CFRunLoopRef,
        timer: CFRunLoopTimerRef,
        mode: CFRunLoopMode,
    ) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopAddTimer(
        rl: CFRunLoopRef,
        timer: CFRunLoopTimerRef,
        mode: CFRunLoopMode,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopRemoveTimer(
        rl: CFRunLoopRef,
        timer: CFRunLoopTimerRef,
        mode: CFRunLoopMode,
    );
);

extern_struct!(
    #[encoding_name("?")]
    pub struct CFRunLoopSourceContext {
        pub version: CFIndex,
        pub info: *mut c_void,
        pub retain: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        pub release: Option<unsafe extern "C" fn(*mut c_void)>,
        pub copyDescription: Option<unsafe extern "C" fn(*mut c_void) -> CFStringRef>,
        pub equal: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Boolean>,
        pub hash: Option<unsafe extern "C" fn(*mut c_void) -> CFHashCode>,
        pub schedule: Option<unsafe extern "C" fn(*mut c_void, CFRunLoopRef, CFRunLoopMode)>,
        pub cancel: Option<unsafe extern "C" fn(*mut c_void, CFRunLoopRef, CFRunLoopMode)>,
        pub perform: Option<unsafe extern "C" fn(*mut c_void)>,
    }
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceCreate(
        allocator: CFAllocatorRef,
        order: CFIndex,
        context: *mut CFRunLoopSourceContext,
    ) -> CFRunLoopSourceRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceGetContext(
        source: CFRunLoopSourceRef,
        context: *mut CFRunLoopSourceContext,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);
);

extern_struct!(
    #[encoding_name("?")]
    pub struct CFRunLoopObserverContext {
        pub version: CFIndex,
        pub info: *mut c_void,
        pub retain: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        pub release: Option<unsafe extern "C" fn(*mut c_void)>,
        pub copyDescription: Option<unsafe extern "C" fn(*mut c_void) -> CFStringRef>,
    }
);

pub type CFRunLoopObserverCallBack =
    Option<unsafe extern "C" fn(CFRunLoopObserverRef, CFRunLoopActivity, *mut c_void)>;

extern_fn!(
    pub unsafe fn CFRunLoopObserverGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverCreate(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: Boolean,
        order: CFIndex,
        callout: CFRunLoopObserverCallBack,
        context: *mut CFRunLoopObserverContext,
    ) -> CFRunLoopObserverRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverCreateWithHandler(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: Boolean,
        order: CFIndex,
        block: Option<&Block<(CFRunLoopObserverRef, CFRunLoopActivity), ()>>,
    ) -> CFRunLoopObserverRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopObserverGetContext(
        observer: CFRunLoopObserverRef,
        context: *mut CFRunLoopObserverContext,
    );
);

extern_struct!(
    #[encoding_name("?")]
    pub struct CFRunLoopTimerContext {
        pub version: CFIndex,
        pub info: *mut c_void,
        pub retain: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        pub release: Option<unsafe extern "C" fn(*mut c_void)>,
        pub copyDescription: Option<unsafe extern "C" fn(*mut c_void) -> CFStringRef>,
    }
);

pub type CFRunLoopTimerCallBack = Option<unsafe extern "C" fn(CFRunLoopTimerRef, *mut c_void)>;

extern_fn!(
    pub unsafe fn CFRunLoopTimerGetTypeID() -> CFTypeID;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerCreate(
        allocator: CFAllocatorRef,
        fire_date: CFAbsoluteTime,
        interval: CFTimeInterval,
        flags: CFOptionFlags,
        order: CFIndex,
        callout: CFRunLoopTimerCallBack,
        context: *mut CFRunLoopTimerContext,
    ) -> CFRunLoopTimerRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerCreateWithHandler(
        allocator: CFAllocatorRef,
        fire_date: CFAbsoluteTime,
        interval: CFTimeInterval,
        flags: CFOptionFlags,
        order: CFIndex,
        block: Option<&Block<(CFRunLoopTimerRef,), ()>>,
    ) -> CFRunLoopTimerRef;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerSetNextFireDate(
        timer: CFRunLoopTimerRef,
        fire_date: CFAbsoluteTime,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerGetContext(
        timer: CFRunLoopTimerRef,
        context: *mut CFRunLoopTimerContext,
    );
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerGetTolerance(timer: CFRunLoopTimerRef) -> CFTimeInterval;
);

extern_fn!(
    pub unsafe fn CFRunLoopTimerSetTolerance(timer: CFRunLoopTimerRef, tolerance: CFTimeInterval);
);
