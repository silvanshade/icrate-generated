//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    #[deprecated]
    pub struct GKFriendRequestComposeViewController;

    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    unsafe impl ClassType for GKFriendRequestComposeViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

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

extern_methods!(
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    unsafe impl GKFriendRequestComposeViewController {}
);

extern_methods!(
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    unsafe impl GKFriendRequestComposeViewController {
        #[method(maxNumberOfRecipients)]
        pub unsafe fn maxNumberOfRecipients() -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[method(addRecipientPlayers:)]
        pub unsafe fn addRecipientPlayers(&self, players: &NSArray<GKPlayer>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "use addRecipientPlayers:"]
        #[method(addRecipientsWithPlayerIDs:)]
        pub unsafe fn addRecipientsWithPlayerIDs(&self, player_i_ds: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(addRecipientsWithEmailAddresses:)]
        pub unsafe fn addRecipientsWithEmailAddresses(&self, email_addresses: &NSArray<NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other composeViewDelegate)]
        pub unsafe fn composeViewDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKFriendRequestComposeViewControllerDelegate>>>;

        #[deprecated]
        #[method(setComposeViewDelegate:)]
        pub unsafe fn setComposeViewDelegate(
            &self,
            compose_view_delegate: Option<
                &ProtocolObject<dyn GKFriendRequestComposeViewControllerDelegate>,
            >,
        );
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait GKFriendRequestComposeViewControllerDelegate {
        #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
        #[deprecated]
        #[method(friendRequestComposeViewControllerDidFinish:)]
        unsafe fn friendRequestComposeViewControllerDidFinish(
            &self,
            view_controller: &GKFriendRequestComposeViewController,
        );
    }

    unsafe impl ProtocolType for dyn GKFriendRequestComposeViewControllerDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
    unsafe impl GKFriendRequestComposeViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
