//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_static!(GKPlayerIDNoLongerAvailable: &'static NSString);

#[objc2::interface(
    unsafe super = GKBasePlayer,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKPlayer")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKPlayer;
}

#[cfg(feature = "GameKit_GKPlayer")]
unsafe impl NSObjectProtocol for GKPlayer {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKPlayer")]
    pub type GKPlayer;

    #[objc2::method(sel = "scopedIDsArePersistent")]
    pub unsafe fn scopedIDsArePersistent(&self) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "gamePlayerID", managed = "Other")]
    pub unsafe fn gamePlayerID(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "teamPlayerID", managed = "Other")]
    pub unsafe fn teamPlayerID(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "displayName", managed = "Other")]
    pub unsafe fn displayName(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "alias", managed = "Other")]
    pub unsafe fn alias(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "anonymousGuestPlayerWithIdentifier:", managed = "Other")]
    pub unsafe fn anonymousGuestPlayerWithIdentifier(guest_identifier: &NSString) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "guestIdentifier", managed = "Other")]
    pub unsafe fn guestIdentifier(&self) -> Option<Id<NSString>>;

    #[objc2::method(sel = "isInvitable")]
    pub unsafe fn isInvitable(&self) -> bool;
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum GKPhotoSize {
    GKPhotoSizeSmall = 0,
    GKPhotoSizeNormal = 1,
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKPlayer")]
    pub type GKPlayer;

    #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
    #[objc2::method(sel = "loadPhotoForSize:withCompletionHandler:")]
    pub unsafe fn loadPhotoForSize_withCompletionHandler(
        &self,
        size: GKPhotoSize,
        completion_handler: Option<&Block<(*mut NSImage, *mut NSError), ()>>,
    );
}

extern_static!(GKPlayerDidChangeNotificationName: &'static NSNotificationName);

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKPlayer")]
    pub type GKPlayer;

    #[deprecated = "use -[GKLocalPlayer loadFriendPlayers...]"]
    #[objc2::method(sel = "isFriend")]
    pub unsafe fn isFriend(&self) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated = "use the teamPlayerID property to identify a player"]
    #[objc2::method(sel = "playerID", managed = "Other")]
    pub unsafe fn playerID(&self) -> Id<NSString>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "Foundation_NSString"
    ))]
    #[deprecated = "use GKLocalPlayer.loadFriendsWithIdentifiers to load a friend's GKPlayer object."]
    #[objc2::method(sel = "loadPlayersForIdentifiers:withCompletionHandler:")]
    pub unsafe fn loadPlayersForIdentifiers_withCompletionHandler(
        identifiers: &NSArray<NSString>,
        completion_handler: Option<&Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>>,
    );
}
