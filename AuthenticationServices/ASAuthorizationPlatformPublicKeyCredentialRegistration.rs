//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration"
    )]
    pub struct ASAuthorizationPlatformPublicKeyCredentialRegistration;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration"
    )]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialRegistration {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration")]
unsafe impl ASAuthorizationCredential for ASAuthorizationPlatformPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration")]
unsafe impl ASAuthorizationPublicKeyCredentialRegistration
    for ASAuthorizationPlatformPublicKeyCredentialRegistration
{
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration")]
unsafe impl ASPublicKeyCredential for ASAuthorizationPlatformPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration")]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration")]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration")]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialRegistration {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialRegistration"
    )]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialRegistration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
