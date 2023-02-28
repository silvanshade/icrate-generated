//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

#[objc2::protocol]
#[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
pub unsafe trait GKGameSessionEventListener: NSObjectProtocol {
    #[cfg(all(feature = "GameKit_GKCloudPlayer", feature = "GameKit_GKGameSession"))]
    #[objc2::method(optional, sel = "session:didAddPlayer:")]
    unsafe fn session_didAddPlayer(&self, session: &GKGameSession, player: &GKCloudPlayer);

    #[cfg(all(feature = "GameKit_GKCloudPlayer", feature = "GameKit_GKGameSession"))]
    #[objc2::method(optional, sel = "session:didRemovePlayer:")]
    unsafe fn session_didRemovePlayer(&self, session: &GKGameSession, player: &GKCloudPlayer);

    #[cfg(all(feature = "GameKit_GKCloudPlayer", feature = "GameKit_GKGameSession"))]
    #[objc2::method(optional, sel = "session:player:didChangeConnectionState:")]
    unsafe fn session_player_didChangeConnectionState(
        &self,
        session: &GKGameSession,
        player: &GKCloudPlayer,
        new_state: GKConnectionState,
    );

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "GameKit_GKCloudPlayer",
        feature = "GameKit_GKGameSession"
    ))]
    #[objc2::method(optional, sel = "session:player:didSaveData:")]
    unsafe fn session_player_didSaveData(
        &self,
        session: &GKGameSession,
        player: &GKCloudPlayer,
        data: &NSData,
    );

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "GameKit_GKCloudPlayer",
        feature = "GameKit_GKGameSession"
    ))]
    #[objc2::method(optional, sel = "session:didReceiveData:fromPlayer:")]
    unsafe fn session_didReceiveData_fromPlayer(
        &self,
        session: &GKGameSession,
        data: &NSData,
        player: &GKCloudPlayer,
    );

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "Foundation_NSString",
        feature = "GameKit_GKCloudPlayer",
        feature = "GameKit_GKGameSession"
    ))]
    #[objc2::method(optional, sel = "session:didReceiveMessage:withData:fromPlayer:")]
    unsafe fn session_didReceiveMessage_withData_fromPlayer(
        &self,
        session: &GKGameSession,
        message: &NSString,
        data: &NSData,
        player: &GKCloudPlayer,
    );
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKGameSession")]
    pub type GKGameSession;

    #[deprecated = "Use GKLocalPlayer's registerListener: to register for GKLocalPlayerListener event notifications."]
    #[objc2::method(sel = "addEventListener:")]
    pub unsafe fn addEventListener(listener: &NSObject);

    #[deprecated = "Use GKLocalPlayer's unregisterListener: or unregisterAllListeners to unregister from GKLocalPlayerListener event notifications."]
    #[objc2::method(sel = "removeEventListener:")]
    pub unsafe fn removeEventListener(listener: &NSObject);
}
