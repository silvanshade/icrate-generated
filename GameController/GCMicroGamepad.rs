//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_static!(GCInputMicroGamepadDpad: &'static NSString);

extern_static!(GCInputMicroGamepadButtonA: &'static NSString);

extern_static!(GCInputMicroGamepadButtonX: &'static NSString);

extern_static!(GCInputMicroGamepadButtonMenu: &'static NSString);

pub type GCMicroGamepadValueChangedHandler =
    *mut Block<(NonNull<GCMicroGamepad>, NonNull<GCControllerElement>), ()>;

#[objc2::interface(
    unsafe super = GCPhysicalInputProfile,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCMicroGamepad")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GCMicroGamepad;
}

#[cfg(feature = "GameController_GCMicroGamepad")]
unsafe impl NSObjectProtocol for GCMicroGamepad {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCMicroGamepad")]
    pub type GCMicroGamepad;

    #[cfg(feature = "GameController_GCController")]
    #[objc2::method(sel = "controller", managed = "Other")]
    pub unsafe fn controller(&self) -> Option<Id<GCController>>;

    #[objc2::method(sel = "valueChangedHandler")]
    pub unsafe fn valueChangedHandler(&self) -> GCMicroGamepadValueChangedHandler;

    #[objc2::method(sel = "setValueChangedHandler:")]
    pub unsafe fn setValueChangedHandler(
        &self,
        value_changed_handler: GCMicroGamepadValueChangedHandler,
    );

    #[cfg(feature = "GameController_GCMicroGamepadSnapshot")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController capture] instead"]
    #[objc2::method(sel = "saveSnapshot", managed = "Other")]
    pub unsafe fn saveSnapshot(&self) -> Id<GCMicroGamepadSnapshot>;

    #[cfg(feature = "GameController_GCControllerDirectionPad")]
    #[objc2::method(sel = "dpad", managed = "Other")]
    pub unsafe fn dpad(&self) -> Id<GCControllerDirectionPad>;

    #[cfg(feature = "GameController_GCControllerButtonInput")]
    #[objc2::method(sel = "buttonA", managed = "Other")]
    pub unsafe fn buttonA(&self) -> Id<GCControllerButtonInput>;

    #[cfg(feature = "GameController_GCControllerButtonInput")]
    #[objc2::method(sel = "buttonX", managed = "Other")]
    pub unsafe fn buttonX(&self) -> Id<GCControllerButtonInput>;

    #[cfg(feature = "GameController_GCControllerButtonInput")]
    #[objc2::method(sel = "buttonMenu", managed = "Other")]
    pub unsafe fn buttonMenu(&self) -> Id<GCControllerButtonInput>;

    #[objc2::method(sel = "reportsAbsoluteDpadValues")]
    pub unsafe fn reportsAbsoluteDpadValues(&self) -> bool;

    #[objc2::method(sel = "setReportsAbsoluteDpadValues:")]
    pub unsafe fn setReportsAbsoluteDpadValues(&self, reports_absolute_dpad_values: bool);

    #[objc2::method(sel = "allowsRotation")]
    pub unsafe fn allowsRotation(&self) -> bool;

    #[objc2::method(sel = "setAllowsRotation:")]
    pub unsafe fn setAllowsRotation(&self, allows_rotation: bool);

    #[objc2::method(sel = "setStateFromMicroGamepad:")]
    pub unsafe fn setStateFromMicroGamepad(&self, micro_gamepad: &GCMicroGamepad);
}
