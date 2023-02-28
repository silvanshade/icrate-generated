//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

typed_enum!(
    pub type SKCloudServiceSetupOptionsKey = NSString;
);

typed_enum!(
    pub type SKCloudServiceSetupAction = NSString;
);

typed_enum!(
    pub type SKCloudServiceSetupMessageIdentifier = NSString;
);

#[objc2::interface(
    unsafe super = NSViewController,
    unsafe inherits = [
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type SKCloudServiceSetupViewController;
}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSCoding for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSEditor for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSObjectProtocol for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSSeguePerforming for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSUserInterfaceItemIdentification for SKCloudServiceSetupViewController {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    pub type SKCloudServiceSetupViewController;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(
        &self,
    ) -> Option<Id<ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(
        &self,
        delegate: Option<&ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>,
    );

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSError"))]
    #[objc2::method(sel = "loadWithOptions:completionHandler:")]
    pub unsafe fn loadWithOptions_completionHandler(
        &self,
        options: &NSDictionary<SKCloudServiceSetupOptionsKey, Object>,
        completion_handler: Option<&Block<(Bool, *mut NSError), ()>>,
    );
}

#[objc2::protocol]
pub unsafe trait SKCloudServiceSetupViewControllerDelegate: NSObjectProtocol {
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    #[objc2::method(optional, sel = "cloudServiceSetupViewControllerDidDismiss:")]
    unsafe fn cloudServiceSetupViewControllerDidDismiss(
        &self,
        cloud_service_setup_view_controller: &SKCloudServiceSetupViewController,
    );
}

extern_static!(SKCloudServiceSetupOptionsActionKey: &'static SKCloudServiceSetupOptionsKey);

extern_static!(
    SKCloudServiceSetupOptionsITunesItemIdentifierKey: &'static SKCloudServiceSetupOptionsKey
);

extern_static!(SKCloudServiceSetupOptionsAffiliateTokenKey: &'static SKCloudServiceSetupOptionsKey);

extern_static!(SKCloudServiceSetupOptionsCampaignTokenKey: &'static SKCloudServiceSetupOptionsKey);

extern_static!(
    SKCloudServiceSetupOptionsMessageIdentifierKey: &'static SKCloudServiceSetupOptionsKey
);

extern_static!(SKCloudServiceSetupActionSubscribe: &'static SKCloudServiceSetupAction);

extern_static!(
    SKCloudServiceSetupMessageIdentifierJoin: &'static SKCloudServiceSetupMessageIdentifier
);

extern_static!(
    SKCloudServiceSetupMessageIdentifierConnect: &'static SKCloudServiceSetupMessageIdentifier
);

extern_static!(
    SKCloudServiceSetupMessageIdentifierAddMusic: &'static SKCloudServiceSetupMessageIdentifier
);

extern_static!(
    SKCloudServiceSetupMessageIdentifierPlayMusic: &'static SKCloudServiceSetupMessageIdentifier
);

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    pub type SKCloudServiceSetupViewController;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "initWithNibName:bundle:", managed = "Init")]
    pub unsafe fn initWithNibName_bundle(
        this: Option<Allocated<Self>>,
        nib_name_or_nil: Option<&NSNibName>,
        nib_bundle_or_nil: Option<&NSBundle>,
    ) -> Id<Self>;
}
