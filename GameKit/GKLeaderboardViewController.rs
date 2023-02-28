//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

#[objc2::interface(
    unsafe super = GKGameCenterViewController,
    unsafe inherits = [
        NSViewController,
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated = "Use GKGameCenterViewController instead"]
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKLeaderboardViewController;
}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl GKViewController for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSCoding for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSEditor for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSObjectProtocol for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSSeguePerforming for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKLeaderboardViewController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    #[deprecated = "Use GKGameCenterViewController instead"]
    pub type GKLeaderboardViewController;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    pub type GKLeaderboardViewController;

    #[objc2::method(sel = "timeScope")]
    pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

    #[objc2::method(sel = "setTimeScope:")]
    pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "category", managed = "Other")]
    pub unsafe fn category(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCategory:")]
    pub unsafe fn setCategory(&self, category: Option<&NSString>);

    #[objc2::method(sel = "leaderboardDelegate", managed = "Other")]
    pub unsafe fn leaderboardDelegate(
        &self,
    ) -> Option<Id<ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>>;

    #[objc2::method(sel = "setLeaderboardDelegate:")]
    pub unsafe fn setLeaderboardDelegate(
        &self,
        leaderboard_delegate: Option<&ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>,
    );
}

#[objc2::protocol]
#[deprecated = "Use GKGameCenterViewController instead"]
pub unsafe trait GKLeaderboardViewControllerDelegate: NSObjectProtocol {
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    #[objc2::method(sel = "leaderboardViewControllerDidFinish:")]
    unsafe fn leaderboardViewControllerDidFinish(
        &self,
        view_controller: Option<&GKLeaderboardViewController>,
    );
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `GKGameCenterViewController`
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    pub type GKLeaderboardViewController;

    #[objc2::method(sel = "initWithState:", managed = "Init")]
    pub unsafe fn initWithState(
        this: Option<Allocated<Self>>,
        state: GKGameCenterViewControllerState,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithLeaderboardID:playerScope:timeScope:", managed = "Init")]
    pub unsafe fn initWithLeaderboardID_playerScope_timeScope(
        this: Option<Allocated<Self>>,
        leaderboard_id: &NSString,
        player_scope: GKLeaderboardPlayerScope,
        time_scope: GKLeaderboardTimeScope,
    ) -> Id<Self>;

    #[cfg(feature = "GameKit_GKLeaderboard")]
    #[objc2::method(sel = "initWithLeaderboard:playerScope:", managed = "Init")]
    pub unsafe fn initWithLeaderboard_playerScope(
        this: Option<Allocated<Self>>,
        leaderboard: &GKLeaderboard,
        player_scope: GKLeaderboardPlayerScope,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithAchievementID:", managed = "Init")]
    pub unsafe fn initWithAchievementID(
        this: Option<Allocated<Self>>,
        achievement_id: &NSString,
    ) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    pub type GKLeaderboardViewController;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "initWithNibName:bundle:", managed = "Init")]
    pub unsafe fn initWithNibName_bundle(
        this: Option<Allocated<Self>>,
        nib_name_or_nil: Option<&NSNibName>,
        nib_bundle_or_nil: Option<&NSBundle>,
    ) -> Id<Self>;
}
