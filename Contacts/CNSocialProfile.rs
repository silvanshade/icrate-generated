//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Contacts_CNSocialProfile")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CNSocialProfile;
}

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSCoding for CNSocialProfile {}

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSObjectProtocol for CNSocialProfile {}

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSSecureCoding for CNSocialProfile {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Contacts_CNSocialProfile")]
    pub type CNSocialProfile;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "urlString", managed = "Other")]
    pub unsafe fn urlString(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "username", managed = "Other")]
    pub unsafe fn username(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "userIdentifier", managed = "Other")]
    pub unsafe fn userIdentifier(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "service", managed = "Other")]
    pub unsafe fn service(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(
        sel = "initWithUrlString:username:userIdentifier:service:",
        managed = "Init"
    )]
    pub unsafe fn initWithUrlString_username_userIdentifier_service(
        this: Option<Allocated<Self>>,
        url_string: Option<&NSString>,
        username: Option<&NSString>,
        user_identifier: Option<&NSString>,
        service: Option<&NSString>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedStringForKey:", managed = "Other")]
    pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedStringForService:", managed = "Other")]
    pub unsafe fn localizedStringForService(service: &NSString) -> Id<NSString>;
}

extern_static!(CNSocialProfileURLStringKey: &'static NSString);

extern_static!(CNSocialProfileUsernameKey: &'static NSString);

extern_static!(CNSocialProfileUserIdentifierKey: &'static NSString);

extern_static!(CNSocialProfileServiceKey: &'static NSString);

extern_static!(CNSocialProfileServiceFacebook: &'static NSString);

extern_static!(CNSocialProfileServiceFlickr: &'static NSString);

extern_static!(CNSocialProfileServiceLinkedIn: &'static NSString);

extern_static!(CNSocialProfileServiceMySpace: &'static NSString);

extern_static!(CNSocialProfileServiceSinaWeibo: &'static NSString);

extern_static!(CNSocialProfileServiceTencentWeibo: &'static NSString);

extern_static!(CNSocialProfileServiceTwitter: &'static NSString);

extern_static!(CNSocialProfileServiceYelp: &'static NSString);

extern_static!(CNSocialProfileServiceGameCenter: &'static NSString);
