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
    #[deprecated]
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type GKFriendRequestComposeViewController;
}

#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
unsafe impl GKViewController for GKFriendRequestComposeViewController {}

#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
unsafe impl NSCoding for GKFriendRequestComposeViewController {}

#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
unsafe impl NSEditor for GKFriendRequestComposeViewController {}

#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
unsafe impl NSObjectProtocol for GKFriendRequestComposeViewController {}

#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
unsafe impl NSSeguePerforming for GKFriendRequestComposeViewController {}

#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKFriendRequestComposeViewController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    #[deprecated]
    pub type GKFriendRequestComposeViewController;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    #[deprecated]
    pub type GKFriendRequestComposeViewController;

    #[objc2::method(sel = "maxNumberOfRecipients")]
    pub unsafe fn maxNumberOfRecipients() -> NSUInteger;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setMessage:")]
    pub unsafe fn setMessage(&self, message: Option<&NSString>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
    #[objc2::method(sel = "addRecipientPlayers:")]
    pub unsafe fn addRecipientPlayers(&self, players: &NSArray<GKPlayer>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[deprecated = "use addRecipientPlayers:"]
    #[objc2::method(sel = "addRecipientsWithPlayerIDs:")]
    pub unsafe fn addRecipientsWithPlayerIDs(&self, player_i_ds: &NSArray<NSString>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "addRecipientsWithEmailAddresses:")]
    pub unsafe fn addRecipientsWithEmailAddresses(&self, email_addresses: &NSArray<NSString>);

    #[deprecated]
    #[objc2::method(sel = "composeViewDelegate", managed = "Other")]
    pub unsafe fn composeViewDelegate(
        &self,
    ) -> Option<Id<ProtocolObject<dyn GKFriendRequestComposeViewControllerDelegate>>>;

    #[deprecated]
    #[objc2::method(sel = "setComposeViewDelegate:")]
    pub unsafe fn setComposeViewDelegate(
        &self,
        compose_view_delegate: Option<
            &ProtocolObject<dyn GKFriendRequestComposeViewControllerDelegate>,
        >,
    );
}

#[objc2::protocol]
#[deprecated]
pub unsafe trait GKFriendRequestComposeViewControllerDelegate {
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    #[deprecated]
    #[objc2::method(sel = "friendRequestComposeViewControllerDidFinish:")]
    unsafe fn friendRequestComposeViewControllerDidFinish(
        &self,
        view_controller: &GKFriendRequestComposeViewController,
    );
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    pub type GKFriendRequestComposeViewController;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "initWithNibName:bundle:", managed = "Init")]
    pub unsafe fn initWithNibName_bundle(
        this: Option<Allocated<Self>>,
        nib_name_or_nil: Option<&NSNibName>,
        nib_bundle_or_nil: Option<&NSBundle>,
    ) -> Id<Self>;
}
