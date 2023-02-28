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
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKAchievementViewController;
}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl GKViewController for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSCoding for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSEditor for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSObjectProtocol for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSSeguePerforming for GKAchievementViewController {}

#[cfg(feature = "GameKit_GKAchievementViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKAchievementViewController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    #[deprecated = "Use GKGameCenterViewController instead"]
    pub type GKAchievementViewController;

    #[objc2::method(sel = "achievementDelegate", managed = "Other")]
    pub unsafe fn achievementDelegate(
        &self,
    ) -> Option<Id<ProtocolObject<dyn GKAchievementViewControllerDelegate>>>;

    #[objc2::method(sel = "setAchievementDelegate:")]
    pub unsafe fn setAchievementDelegate(
        &self,
        achievement_delegate: Option<&ProtocolObject<dyn GKAchievementViewControllerDelegate>>,
    );
}

#[objc2::protocol]
#[deprecated = "Use GKGameCenterViewController instead"]
pub unsafe trait GKAchievementViewControllerDelegate: NSObjectProtocol {
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    #[objc2::method(sel = "achievementViewControllerDidFinish:")]
    unsafe fn achievementViewControllerDidFinish(
        &self,
        view_controller: Option<&GKAchievementViewController>,
    );
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `GKGameCenterViewController`
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    pub type GKAchievementViewController;

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
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKAchievementViewController")]
    pub type GKAchievementViewController;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "initWithNibName:bundle:", managed = "Init")]
    pub unsafe fn initWithNibName_bundle(
        this: Option<Allocated<Self>>,
        nib_name_or_nil: Option<&NSNibName>,
        nib_bundle_or_nil: Option<&NSBundle>,
    ) -> Id<Self>;
}
