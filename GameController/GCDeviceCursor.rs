//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

#[objc2::interface(
    unsafe super = GCControllerDirectionPad,
    unsafe inherits = [
        GCControllerElement,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCDeviceCursor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GCDeviceCursor;
}

#[cfg(feature = "GameController_GCDeviceCursor")]
unsafe impl NSObjectProtocol for GCDeviceCursor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCDeviceCursor")]
    pub type GCDeviceCursor;
}
