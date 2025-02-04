//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
    )]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
    )]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
    }
);

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
)]
unsafe impl ASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest
{
}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
)]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
)]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

#[cfg(
    feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
)]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest"
    )]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowed_credentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );
    }
);
