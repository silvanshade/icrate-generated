//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKSavedGame")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKSavedGame;
}

#[cfg(feature = "GameKit_GKSavedGame")]
unsafe impl NSObjectProtocol for GKSavedGame {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKSavedGame")]
    pub type GKSavedGame;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "deviceName", managed = "Other")]
    pub unsafe fn deviceName(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "modificationDate", managed = "Other")]
    pub unsafe fn modificationDate(&self) -> Option<Id<NSDate>>;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
    #[objc2::method(sel = "loadDataWithCompletionHandler:")]
    pub unsafe fn loadDataWithCompletionHandler(
        &self,
        handler: Option<&Block<(*mut NSData, *mut NSError), ()>>,
    );
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKLocalPlayer")]
    pub type GKLocalPlayer;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "GameKit_GKSavedGame"
    ))]
    #[objc2::method(sel = "fetchSavedGamesWithCompletionHandler:")]
    pub unsafe fn fetchSavedGamesWithCompletionHandler(
        &self,
        handler: Option<&Block<(*mut NSArray<GKSavedGame>, *mut NSError), ()>>,
    );

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "Foundation_NSError",
        feature = "Foundation_NSString",
        feature = "GameKit_GKSavedGame"
    ))]
    #[objc2::method(sel = "saveGameData:withName:completionHandler:")]
    pub unsafe fn saveGameData_withName_completionHandler(
        &self,
        data: &NSData,
        name: &NSString,
        handler: Option<&Block<(*mut GKSavedGame, *mut NSError), ()>>,
    );

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "deleteSavedGamesWithName:completionHandler:")]
    pub unsafe fn deleteSavedGamesWithName_completionHandler(
        &self,
        name: &NSString,
        handler: Option<&Block<(*mut NSError,), ()>>,
    );

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSData",
        feature = "Foundation_NSError",
        feature = "GameKit_GKSavedGame"
    ))]
    #[objc2::method(sel = "resolveConflictingSavedGames:withData:completionHandler:")]
    pub unsafe fn resolveConflictingSavedGames_withData_completionHandler(
        &self,
        conflicting_saved_games: &NSArray<GKSavedGame>,
        data: &NSData,
        handler: Option<&Block<(*mut NSArray<GKSavedGame>, *mut NSError), ()>>,
    );
}

#[cfg(feature = "GameKit_GKLocalPlayer")]
unsafe impl GKSavedGameListener for GKLocalPlayer {}
