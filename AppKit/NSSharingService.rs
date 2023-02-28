//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSSharingServiceName = NSString;
);

extern_static!(NSSharingServiceNameComposeEmail: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameComposeMessage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameSendViaAirDrop: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToSafariReadingList: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToIPhoto: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToAperture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsDesktopPicture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnFacebook: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTwitter: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnSinaWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTencentWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnLinkedIn: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsTwitterProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsFacebookProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsLinkedInProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostImageOnFlickr: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnVimeo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnYouku: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnTudou: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameCloudSharing: &'static NSSharingServiceName);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSSharingService")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSSharingService;
}

#[cfg(feature = "AppKit_NSSharingService")]
unsafe impl NSObjectProtocol for NSSharingService {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSSharingService")]
    pub type NSSharingService;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSharingServiceDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(
        &self,
        delegate: Option<&ProtocolObject<dyn NSSharingServiceDelegate>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "title", managed = "Other")]
    pub unsafe fn title(&self) -> Id<NSString>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "image", managed = "Other")]
    pub unsafe fn image(&self) -> Id<NSImage>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "alternateImage", managed = "Other")]
    pub unsafe fn alternateImage(&self) -> Option<Id<NSImage>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "menuItemTitle", managed = "Other")]
    pub unsafe fn menuItemTitle(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setMenuItemTitle:")]
    pub unsafe fn setMenuItemTitle(&self, menu_item_title: &NSString);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "recipients", managed = "Other")]
    pub unsafe fn recipients(&self) -> Option<Id<NSArray<NSString>>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setRecipients:")]
    pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<NSString>>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "subject", managed = "Other")]
    pub unsafe fn subject(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setSubject:")]
    pub unsafe fn setSubject(&self, subject: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "messageBody", managed = "Other")]
    pub unsafe fn messageBody(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "permanentLink", managed = "Other")]
    pub unsafe fn permanentLink(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "accountName", managed = "Other")]
    pub unsafe fn accountName(&self) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "attachmentFileURLs", managed = "Other")]
    pub unsafe fn attachmentFileURLs(&self) -> Option<Id<NSArray<NSURL>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[deprecated = "Use -[NSSharingServicePicker standardShareMenuItem] instead."]
    #[objc2::method(sel = "sharingServicesForItems:", managed = "Other")]
    pub unsafe fn sharingServicesForItems(items: &NSArray) -> Id<NSArray<NSSharingService>>;

    #[objc2::method(sel = "sharingServiceNamed:", managed = "Other")]
    pub unsafe fn sharingServiceNamed(
        service_name: &NSSharingServiceName,
    ) -> Option<Id<NSSharingService>>;

    #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithTitle:image:alternateImage:handler:", managed = "Init")]
    pub unsafe fn initWithTitle_image_alternateImage_handler(
        this: Option<Allocated<Self>>,
        title: &NSString,
        image: &NSImage,
        alternate_image: Option<&NSImage>,
        block: &Block<(), ()>,
    ) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "canPerformWithItems:")]
    pub unsafe fn canPerformWithItems(&self, items: Option<&NSArray>) -> bool;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "performWithItems:")]
    pub unsafe fn performWithItems(&self, items: &NSArray);
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum NSSharingContentScope {
    NSSharingContentScopeItem = 0,
    NSSharingContentScopePartial = 1,
    NSSharingContentScopeFull = 2,
}

#[objc2::protocol]
pub unsafe trait NSSharingServiceDelegate: NSObjectProtocol {
    #[cfg(all(feature = "AppKit_NSSharingService", feature = "Foundation_NSArray"))]
    #[objc2::method(optional, sel = "sharingService:willShareItems:")]
    unsafe fn sharingService_willShareItems(
        &self,
        sharing_service: &NSSharingService,
        items: &NSArray,
    );

    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(optional, sel = "sharingService:didFailToShareItems:error:")]
    unsafe fn sharingService_didFailToShareItems_error(
        &self,
        sharing_service: &NSSharingService,
        items: &NSArray,
        error: &NSError,
    );

    #[cfg(all(feature = "AppKit_NSSharingService", feature = "Foundation_NSArray"))]
    #[objc2::method(optional, sel = "sharingService:didShareItems:")]
    unsafe fn sharingService_didShareItems(
        &self,
        sharing_service: &NSSharingService,
        items: &NSArray,
    );

    #[cfg(feature = "AppKit_NSSharingService")]
    #[objc2::method(optional, sel = "sharingService:sourceFrameOnScreenForShareItem:")]
    unsafe fn sharingService_sourceFrameOnScreenForShareItem(
        &self,
        sharing_service: &NSSharingService,
        item: &Object,
    ) -> NSRect;

    #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSSharingService"))]
    #[objc2::method(
        optional,
        sel = "sharingService:transitionImageForShareItem:contentRect:",
        managed = "Other"
    )]
    unsafe fn sharingService_transitionImageForShareItem_contentRect(
        &self,
        sharing_service: &NSSharingService,
        item: &Object,
        content_rect: NonNull<NSRect>,
    ) -> Option<Id<NSImage>>;

    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "AppKit_NSWindow",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(
        optional,
        sel = "sharingService:sourceWindowForShareItems:sharingContentScope:",
        managed = "Other"
    )]
    unsafe fn sharingService_sourceWindowForShareItems_sharingContentScope(
        &self,
        sharing_service: &NSSharingService,
        items: &NSArray,
        sharing_content_scope: NonNull<NSSharingContentScope>,
    ) -> Option<Id<NSWindow>>;

    #[cfg(all(feature = "AppKit_NSSharingService", feature = "AppKit_NSView"))]
    #[objc2::method(
        optional,
        sel = "anchoringViewForSharingService:showRelativeToRect:preferredEdge:",
        managed = "Other"
    )]
    unsafe fn anchoringViewForSharingService_showRelativeToRect_preferredEdge(
        &self,
        sharing_service: &NSSharingService,
        positioning_rect: NonNull<NSRect>,
        preferred_edge: NonNull<NSRectEdge>,
    ) -> Option<Id<NSView>>;
}

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSCloudKitSharingServiceOptions {
    NSCloudKitSharingServiceStandard = 0,
    NSCloudKitSharingServiceAllowPublic = 1 << 0,
    NSCloudKitSharingServiceAllowPrivate = 1 << 1,
    NSCloudKitSharingServiceAllowReadOnly = 1 << 4,
    NSCloudKitSharingServiceAllowReadWrite = 1 << 5,
}

#[objc2::protocol]
pub unsafe trait NSCloudSharingServiceDelegate: NSSharingServiceDelegate {
    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(optional, sel = "sharingService:didCompleteForItems:error:")]
    unsafe fn sharingService_didCompleteForItems_error(
        &self,
        sharing_service: &NSSharingService,
        items: &NSArray,
        error: Option<&NSError>,
    );

    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "Foundation_NSItemProvider"
    ))]
    #[objc2::method(optional, sel = "optionsForSharingService:shareProvider:")]
    unsafe fn optionsForSharingService_shareProvider(
        &self,
        cloud_kit_sharing_service: &NSSharingService,
        provider: &NSItemProvider,
    ) -> NSCloudKitSharingServiceOptions;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSItemProvider")]
    pub type NSItemProvider;
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSSharingServicePicker")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSSharingServicePicker;
}

#[cfg(feature = "AppKit_NSSharingServicePicker")]
unsafe impl NSObjectProtocol for NSSharingServicePicker {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSSharingServicePicker")]
    pub type NSSharingServicePicker;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self)
        -> Option<Id<ProtocolObject<dyn NSSharingServicePickerDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(
        &self,
        delegate: Option<&ProtocolObject<dyn NSSharingServicePickerDelegate>>,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "initWithItems:", managed = "Init")]
    pub unsafe fn initWithItems(this: Option<Allocated<Self>>, items: &NSArray) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(sel = "showRelativeToRect:ofView:preferredEdge:")]
    pub unsafe fn showRelativeToRect_ofView_preferredEdge(
        &self,
        rect: NSRect,
        view: &NSView,
        preferred_edge: NSRectEdge,
    );

    #[objc2::method(sel = "close")]
    pub unsafe fn close(&self);

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "standardShareMenuItem", managed = "Other")]
    pub unsafe fn standardShareMenuItem(&self) -> Id<NSMenuItem>;
}

#[objc2::protocol]
pub unsafe trait NSSharingServicePickerDelegate: NSObjectProtocol {
    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "AppKit_NSSharingServicePicker",
        feature = "Foundation_NSArray"
    ))]
    #[objc2::method(
        optional,
        sel = "sharingServicePicker:sharingServicesForItems:proposedSharingServices:",
        managed = "Other"
    )]
    unsafe fn sharingServicePicker_sharingServicesForItems_proposedSharingServices(
        &self,
        sharing_service_picker: &NSSharingServicePicker,
        items: &NSArray,
        proposed_services: &NSArray<NSSharingService>,
    ) -> Id<NSArray<NSSharingService>>;

    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "AppKit_NSSharingServicePicker"
    ))]
    #[objc2::method(
        optional,
        sel = "sharingServicePicker:delegateForSharingService:",
        managed = "Other"
    )]
    unsafe fn sharingServicePicker_delegateForSharingService(
        &self,
        sharing_service_picker: &NSSharingServicePicker,
        sharing_service: &NSSharingService,
    ) -> Option<Id<ProtocolObject<dyn NSSharingServiceDelegate>>>;

    #[cfg(all(
        feature = "AppKit_NSSharingService",
        feature = "AppKit_NSSharingServicePicker"
    ))]
    #[objc2::method(optional, sel = "sharingServicePicker:didChooseSharingService:")]
    unsafe fn sharingServicePicker_didChooseSharingService(
        &self,
        sharing_service_picker: &NSSharingServicePicker,
        service: Option<&NSSharingService>,
    );
}
