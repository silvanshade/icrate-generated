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

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCControllerPlayerIndex {
        GCControllerPlayerIndexUnset = -1,
        GCControllerPlayerIndex1 = 0,
        GCControllerPlayerIndex2 = 1,
        GCControllerPlayerIndex3 = 2,
        GCControllerPlayerIndex4 = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCController")]
    pub struct GCController;

    #[cfg(feature = "GameController_GCController")]
    unsafe impl ClassType for GCController {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCController")]
unsafe impl GCDevice for GCController {}

#[cfg(feature = "GameController_GCController")]
unsafe impl NSObjectProtocol for GCController {}

extern_methods!(
    #[cfg(feature = "GameController_GCController")]
    unsafe impl GCController {
        #[deprecated = "controllerPausedHandler has been deprecated. Use the Menu button found on the controller's profile, if it exists."]
        #[method(controllerPausedHandler)]
        pub unsafe fn controllerPausedHandler(&self) -> *mut Block<(NonNull<GCController>,), ()>;

        #[deprecated = "controllerPausedHandler has been deprecated. Use the Menu button found on the controller's profile, if it exists."]
        #[method(setControllerPausedHandler:)]
        pub unsafe fn setControllerPausedHandler(
            &self,
            controller_paused_handler: Option<&Block<(NonNull<GCController>,), ()>>,
        );

        #[method_id(@__retain_semantics Other current)]
        pub unsafe fn current() -> Option<Id<GCController>>;

        #[method(shouldMonitorBackgroundEvents)]
        pub unsafe fn shouldMonitorBackgroundEvents() -> bool;

        #[method(setShouldMonitorBackgroundEvents:)]
        pub unsafe fn setShouldMonitorBackgroundEvents(should_monitor_background_events: bool);

        #[method(isAttachedToDevice)]
        pub unsafe fn isAttachedToDevice(&self) -> bool;

        #[method(isSnapshot)]
        pub unsafe fn isSnapshot(&self) -> bool;

        #[method(playerIndex)]
        pub unsafe fn playerIndex(&self) -> GCControllerPlayerIndex;

        #[method(setPlayerIndex:)]
        pub unsafe fn setPlayerIndex(&self, player_index: GCControllerPlayerIndex);

        #[cfg(feature = "GameController_GCDeviceBattery")]
        #[method_id(@__retain_semantics Other battery)]
        pub unsafe fn battery(&self) -> Option<Id<GCDeviceBattery>>;

        #[cfg(feature = "GameController_GCPhysicalInputProfile")]
        #[method_id(@__retain_semantics Other physicalInputProfile)]
        pub unsafe fn physicalInputProfile(&self) -> Id<GCPhysicalInputProfile>;

        #[cfg(feature = "GameController_GCGamepad")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gamepad)]
        pub unsafe fn gamepad(&self) -> Option<Id<GCGamepad>>;

        #[cfg(feature = "GameController_GCMicroGamepad")]
        #[method_id(@__retain_semantics Other microGamepad)]
        pub unsafe fn microGamepad(&self) -> Option<Id<GCMicroGamepad>>;

        #[cfg(feature = "GameController_GCExtendedGamepad")]
        #[method_id(@__retain_semantics Other extendedGamepad)]
        pub unsafe fn extendedGamepad(&self) -> Option<Id<GCExtendedGamepad>>;

        #[cfg(feature = "GameController_GCMotion")]
        #[method_id(@__retain_semantics Other motion)]
        pub unsafe fn motion(&self) -> Option<Id<GCMotion>>;

        #[cfg(feature = "GameController_GCDeviceLight")]
        #[method_id(@__retain_semantics Other light)]
        pub unsafe fn light(&self) -> Option<Id<GCDeviceLight>>;

        #[cfg(feature = "GameController_GCDeviceHaptics")]
        #[method_id(@__retain_semantics Other haptics)]
        pub unsafe fn haptics(&self) -> Option<Id<GCDeviceHaptics>>;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCController>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other controllers)]
        pub unsafe fn controllers() -> Id<NSArray<GCController>>;

        #[method(startWirelessControllerDiscoveryWithCompletionHandler:)]
        pub unsafe fn startWirelessControllerDiscoveryWithCompletionHandler(
            completion_handler: Option<&Block<(), ()>>,
        );

        #[method(stopWirelessControllerDiscovery)]
        pub unsafe fn stopWirelessControllerDiscovery();

        #[method_id(@__retain_semantics Other controllerWithMicroGamepad)]
        pub unsafe fn controllerWithMicroGamepad() -> Id<GCController>;

        #[method_id(@__retain_semantics Other controllerWithExtendedGamepad)]
        pub unsafe fn controllerWithExtendedGamepad() -> Id<GCController>;
    }
);
