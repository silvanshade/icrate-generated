//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

#[objc2::interface(
    unsafe super = NSViewController,
    unsafe inherits = [
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKTurnBasedMatchmakerViewController;
}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl GKViewController for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSCoding for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSEditor for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSObjectProtocol for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSSeguePerforming for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKTurnBasedMatchmakerViewController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    pub type GKTurnBasedMatchmakerViewController;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    pub type GKTurnBasedMatchmakerViewController;

    #[objc2::method(sel = "turnBasedMatchmakerDelegate", managed = "Other")]
    pub unsafe fn turnBasedMatchmakerDelegate(
        &self,
    ) -> Option<Id<ProtocolObject<dyn GKTurnBasedMatchmakerViewControllerDelegate>>>;

    #[objc2::method(sel = "setTurnBasedMatchmakerDelegate:")]
    pub unsafe fn setTurnBasedMatchmakerDelegate(
        &self,
        turn_based_matchmaker_delegate: Option<
            &ProtocolObject<dyn GKTurnBasedMatchmakerViewControllerDelegate>,
        >,
    );

    #[objc2::method(sel = "showExistingMatches")]
    pub unsafe fn showExistingMatches(&self) -> bool;

    #[objc2::method(sel = "setShowExistingMatches:")]
    pub unsafe fn setShowExistingMatches(&self, show_existing_matches: bool);

    #[objc2::method(sel = "matchmakingMode")]
    pub unsafe fn matchmakingMode(&self) -> GKMatchmakingMode;

    #[objc2::method(sel = "setMatchmakingMode:")]
    pub unsafe fn setMatchmakingMode(&self, matchmaking_mode: GKMatchmakingMode);

    #[cfg(feature = "GameKit_GKMatchRequest")]
    #[objc2::method(sel = "initWithMatchRequest:", managed = "Init")]
    pub unsafe fn initWithMatchRequest(
        this: Option<Allocated<Self>>,
        request: &GKMatchRequest,
    ) -> Id<Self>;
}

#[objc2::protocol]
pub unsafe trait GKTurnBasedMatchmakerViewControllerDelegate: NSObjectProtocol {
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    #[objc2::method(sel = "turnBasedMatchmakerViewControllerWasCancelled:")]
    unsafe fn turnBasedMatchmakerViewControllerWasCancelled(
        &self,
        view_controller: &GKTurnBasedMatchmakerViewController,
    );

    #[cfg(all(
        feature = "Foundation_NSError",
        feature = "GameKit_GKTurnBasedMatchmakerViewController"
    ))]
    #[objc2::method(sel = "turnBasedMatchmakerViewController:didFailWithError:")]
    unsafe fn turnBasedMatchmakerViewController_didFailWithError(
        &self,
        view_controller: &GKTurnBasedMatchmakerViewController,
        error: &NSError,
    );

    #[cfg(all(
        feature = "GameKit_GKTurnBasedMatch",
        feature = "GameKit_GKTurnBasedMatchmakerViewController"
    ))]
    #[deprecated = "use GKTurnBasedEventListener player:receivedTurnEventForMatch:didBecomeActive:"]
    #[objc2::method(optional, sel = "turnBasedMatchmakerViewController:didFindMatch:")]
    unsafe fn turnBasedMatchmakerViewController_didFindMatch(
        &self,
        view_controller: &GKTurnBasedMatchmakerViewController,
        r#match: &GKTurnBasedMatch,
    );

    #[cfg(all(
        feature = "GameKit_GKTurnBasedMatch",
        feature = "GameKit_GKTurnBasedMatchmakerViewController"
    ))]
    #[deprecated = "use GKTurnBasedEventListener player:wantsToQuitMatch:"]
    #[objc2::method(
        optional,
        sel = "turnBasedMatchmakerViewController:playerQuitForMatch:"
    )]
    unsafe fn turnBasedMatchmakerViewController_playerQuitForMatch(
        &self,
        view_controller: &GKTurnBasedMatchmakerViewController,
        r#match: &GKTurnBasedMatch,
    );
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    pub type GKTurnBasedMatchmakerViewController;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "initWithNibName:bundle:", managed = "Init")]
    pub unsafe fn initWithNibName_bundle(
        this: Option<Allocated<Self>>,
        nib_name_or_nil: Option<&NSNibName>,
        nib_bundle_or_nil: Option<&NSBundle>,
    ) -> Id<Self>;
}
