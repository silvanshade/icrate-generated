//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = ASAccountAuthenticationModificationRequest,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
    )]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest;
}

#[cfg(
    feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
)]
unsafe impl NSObjectProtocol
    for ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest
{
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest"
    )]
    pub type ASAccountAuthenticationModificationReplacePasswordWithSignInWithAppleRequest;

    #[cfg(all(
        feature = "AuthenticationServices_ASCredentialServiceIdentifier",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "initWithUser:serviceIdentifier:userInfo:", managed = "Init")]
    pub unsafe fn initWithUser_serviceIdentifier_userInfo(
        this: Option<Allocated<Self>>,
        user: &NSString,
        service_identifier: &ASCredentialServiceIdentifier,
        user_info: Option<&NSDictionary>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "user", managed = "Other")]
    pub unsafe fn user(&self) -> Id<NSString>;

    #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
    #[objc2::method(sel = "serviceIdentifier", managed = "Other")]
    pub unsafe fn serviceIdentifier(&self) -> Id<ASCredentialServiceIdentifier>;

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "userInfo", managed = "Other")]
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;
}
