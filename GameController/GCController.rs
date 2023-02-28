//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_static!(GCControllerDidConnectNotification: &'static NSString);

extern_static!(GCControllerDidDisconnectNotification: &'static NSString);

extern_static!(GCControllerDidBecomeCurrentNotification: &'static NSString);

extern_static!(GCControllerDidStopBeingCurrentNotification: &'static NSString);

extern_static!(GCControllerUserCustomizationsDidChangeNotification: &'static NSString);

#[ns_enum]
#[underlying(NSInteger)]
pub enum GCControllerPlayerIndex {
    GCControllerPlayerIndexUnset = -1,
    GCControllerPlayerIndex1 = 0,
    GCControllerPlayerIndex2 = 1,
    GCControllerPlayerIndex3 = 2,
    GCControllerPlayerIndex4 = 3,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GCController;
}

#[cfg(feature = "GameController_GCController")]
unsafe impl GCDevice for GCController {}

#[cfg(feature = "GameController_GCController")]
unsafe impl NSObjectProtocol for GCController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameController_GCController")]
    pub type GCController;

    #[deprecated = "controllerPausedHandler has been deprecated. Use the Menu button found on the controller's profile, if it exists."]
    #[objc2::method(sel = "controllerPausedHandler")]
    pub unsafe fn controllerPausedHandler(&self) -> *mut Block<(NonNull<GCController>,), ()>;

    #[deprecated = "controllerPausedHandler has been deprecated. Use the Menu button found on the controller's profile, if it exists."]
    #[objc2::method(sel = "setControllerPausedHandler:")]
    pub unsafe fn setControllerPausedHandler(
        &self,
        controller_paused_handler: Option<&Block<(NonNull<GCController>,), ()>>,
    );

    #[objc2::method(sel = "current", managed = "Other")]
    pub unsafe fn current() -> Option<Id<GCController>>;

    #[objc2::method(sel = "shouldMonitorBackgroundEvents")]
    pub unsafe fn shouldMonitorBackgroundEvents() -> bool;

    #[objc2::method(sel = "setShouldMonitorBackgroundEvents:")]
    pub unsafe fn setShouldMonitorBackgroundEvents(should_monitor_background_events: bool);

    #[objc2::method(sel = "isAttachedToDevice")]
    pub unsafe fn isAttachedToDevice(&self) -> bool;

    #[objc2::method(sel = "isSnapshot")]
    pub unsafe fn isSnapshot(&self) -> bool;

    #[objc2::method(sel = "playerIndex")]
    pub unsafe fn playerIndex(&self) -> GCControllerPlayerIndex;

    #[objc2::method(sel = "setPlayerIndex:")]
    pub unsafe fn setPlayerIndex(&self, player_index: GCControllerPlayerIndex);

    #[cfg(feature = "GameController_GCDeviceBattery")]
    #[objc2::method(sel = "battery", managed = "Other")]
    pub unsafe fn battery(&self) -> Option<Id<GCDeviceBattery>>;

    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    #[objc2::method(sel = "physicalInputProfile", managed = "Other")]
    pub unsafe fn physicalInputProfile(&self) -> Id<GCPhysicalInputProfile>;

    #[cfg(feature = "GameController_GCGamepad")]
    #[deprecated]
    #[objc2::method(sel = "gamepad", managed = "Other")]
    pub unsafe fn gamepad(&self) -> Option<Id<GCGamepad>>;

    #[cfg(feature = "GameController_GCMicroGamepad")]
    #[objc2::method(sel = "microGamepad", managed = "Other")]
    pub unsafe fn microGamepad(&self) -> Option<Id<GCMicroGamepad>>;

    #[cfg(feature = "GameController_GCExtendedGamepad")]
    #[objc2::method(sel = "extendedGamepad", managed = "Other")]
    pub unsafe fn extendedGamepad(&self) -> Option<Id<GCExtendedGamepad>>;

    #[cfg(feature = "GameController_GCMotion")]
    #[objc2::method(sel = "motion", managed = "Other")]
    pub unsafe fn motion(&self) -> Option<Id<GCMotion>>;

    #[cfg(feature = "GameController_GCDeviceLight")]
    #[objc2::method(sel = "light", managed = "Other")]
    pub unsafe fn light(&self) -> Option<Id<GCDeviceLight>>;

    #[cfg(feature = "GameController_GCDeviceHaptics")]
    #[objc2::method(sel = "haptics", managed = "Other")]
    pub unsafe fn haptics(&self) -> Option<Id<GCDeviceHaptics>>;

    #[objc2::method(sel = "capture", managed = "Other")]
    pub unsafe fn capture(&self) -> Id<GCController>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "controllers", managed = "Other")]
    pub unsafe fn controllers() -> Id<NSArray<GCController>>;

    #[objc2::method(sel = "startWirelessControllerDiscoveryWithCompletionHandler:")]
    pub unsafe fn startWirelessControllerDiscoveryWithCompletionHandler(
        completion_handler: Option<&Block<(), ()>>,
    );

    #[objc2::method(sel = "stopWirelessControllerDiscovery")]
    pub unsafe fn stopWirelessControllerDiscovery();

    #[objc2::method(sel = "controllerWithMicroGamepad", managed = "Other")]
    pub unsafe fn controllerWithMicroGamepad() -> Id<GCController>;

    #[objc2::method(sel = "controllerWithExtendedGamepad", managed = "Other")]
    pub unsafe fn controllerWithExtendedGamepad() -> Id<GCController>;
}
