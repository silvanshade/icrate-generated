//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSTouchPhase {
    NSTouchPhaseBegan = 1 << 0,
    NSTouchPhaseMoved = 1 << 1,
    NSTouchPhaseStationary = 1 << 2,
    NSTouchPhaseEnded = 1 << 3,
    NSTouchPhaseCancelled = 1 << 4,
    NSTouchPhaseTouching = NSTouchPhaseBegan | NSTouchPhaseMoved | NSTouchPhaseStationary,
    NSTouchPhaseAny = NSUIntegerMax as _,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum NSTouchType {
    NSTouchTypeDirect = 0,
    NSTouchTypeIndirect = 1,
}

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSTouchTypeMask {
    NSTouchTypeMaskDirect = 1 << NSTouchTypeDirect,
    NSTouchTypeMaskIndirect = 1 << NSTouchTypeIndirect,
}

inline_fn!(
    pub unsafe fn NSTouchTypeMaskFromType(r#type: NSTouchType) -> NSTouchTypeMask {
        todo!()
    }
);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTouch")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSTouch;
}

#[cfg(feature = "AppKit_NSTouch")]
unsafe impl NSObjectProtocol for NSTouch {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTouch")]
    pub type NSTouch;

    #[objc2::method(sel = "identity", managed = "Other")]
    pub unsafe fn identity(&self) -> Id<TodoProtocols>;

    #[objc2::method(sel = "phase")]
    pub unsafe fn phase(&self) -> NSTouchPhase;

    #[objc2::method(sel = "normalizedPosition")]
    pub unsafe fn normalizedPosition(&self) -> NSPoint;

    #[objc2::method(sel = "isResting")]
    pub unsafe fn isResting(&self) -> bool;

    #[objc2::method(sel = "device", managed = "Other")]
    pub unsafe fn device(&self) -> Option<Id<Object>>;

    #[objc2::method(sel = "deviceSize")]
    pub unsafe fn deviceSize(&self) -> NSSize;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTouch")]
    pub type NSTouch;

    #[objc2::method(sel = "type")]
    pub unsafe fn r#type(&self) -> NSTouchType;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "locationInView:")]
    pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "previousLocationInView:")]
    pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
}
