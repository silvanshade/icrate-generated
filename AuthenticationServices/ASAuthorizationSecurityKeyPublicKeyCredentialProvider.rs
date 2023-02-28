//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ASAuthorizationSecurityKeyPublicKeyCredentialProvider;
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
    pub type ASAuthorizationSecurityKeyPublicKeyCredentialProvider;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithRelyingPartyIdentifier:", managed = "Init")]
    pub unsafe fn initWithRelyingPartyIdentifier(
        this: Option<Allocated<Self>>,
        relying_party_identifier: &NSString,
    ) -> Id<Self>;

    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest",
        feature = "Foundation_NSData",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(
        sel = "createCredentialRegistrationRequestWithChallenge:displayName:name:userID:",
        managed = "Other"
    )]
    pub unsafe fn createCredentialRegistrationRequestWithChallenge_displayName_name_userID(
        &self,
        challenge: &NSData,
        display_name: &NSString,
        name: &NSString,
        user_id: &NSData,
    ) -> Id<ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest>;

    #[cfg(all(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest",
        feature = "Foundation_NSData"
    ))]
    #[objc2::method(
        sel = "createCredentialAssertionRequestWithChallenge:",
        managed = "Other"
    )]
    pub unsafe fn createCredentialAssertionRequestWithChallenge(
        &self,
        challenge: &NSData,
    ) -> Id<ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "relyingPartyIdentifier", managed = "Other")]
    pub unsafe fn relyingPartyIdentifier(&self) -> Id<NSString>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;
}
