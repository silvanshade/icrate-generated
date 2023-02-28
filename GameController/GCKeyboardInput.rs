//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCKeyboardValueChangedHandler = *mut Block<
    (
        NonNull<GCKeyboardInput>,
        NonNull<GCControllerButtonInput>,
        GCKeyCode,
        Bool,
    ),
    (),
>;

#[objc2::interface(
    unsafe super = GCPhysicalInputProfile,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCKeyboardInput")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GCKeyboardInput;
}

#[cfg(feature = "GameController_GCKeyboardInput")]
unsafe impl NSObjectProtocol for GCKeyboardInput {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCKeyboardInput")]
    pub type GCKeyboardInput;

    #[objc2::method(sel = "keyChangedHandler")]
    pub unsafe fn keyChangedHandler(&self) -> GCKeyboardValueChangedHandler;

    #[objc2::method(sel = "setKeyChangedHandler:")]
    pub unsafe fn setKeyChangedHandler(&self, key_changed_handler: GCKeyboardValueChangedHandler);

    #[objc2::method(sel = "isAnyKeyPressed")]
    pub unsafe fn isAnyKeyPressed(&self) -> bool;

    #[cfg(feature = "GameController_GCControllerButtonInput")]
    #[objc2::method(sel = "buttonForKeyCode:", managed = "Other")]
    pub unsafe fn buttonForKeyCode(&self, code: GCKeyCode) -> Option<Id<GCControllerButtonInput>>;
}
