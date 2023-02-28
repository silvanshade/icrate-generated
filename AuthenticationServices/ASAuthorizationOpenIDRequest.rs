//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASAuthorizationOpenIDOperation = NSString;
);

extern_static!(ASAuthorizationOperationImplicit: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationLogin: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationRefresh: &'static ASAuthorizationOpenIDOperation);

extern_static!(ASAuthorizationOperationLogout: &'static ASAuthorizationOpenIDOperation);

#[objc2::interface(
    unsafe super = ASAuthorizationRequest,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ASAuthorizationOpenIDRequest;
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSCoding for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationOpenIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
unsafe impl NSSecureCoding for ASAuthorizationOpenIDRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationOpenIDRequest")]
    pub type ASAuthorizationOpenIDRequest;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "requestedScopes", managed = "Other")]
    pub unsafe fn requestedScopes(&self) -> Option<Id<NSArray<ASAuthorizationScope>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setRequestedScopes:")]
    pub unsafe fn setRequestedScopes(
        &self,
        requested_scopes: Option<&NSArray<ASAuthorizationScope>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "state", managed = "Other")]
    pub unsafe fn state(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setState:")]
    pub unsafe fn setState(&self, state: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "nonce", managed = "Other")]
    pub unsafe fn nonce(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNonce:")]
    pub unsafe fn setNonce(&self, nonce: Option<&NSString>);

    #[objc2::method(sel = "requestedOperation", managed = "Other")]
    pub unsafe fn requestedOperation(&self) -> Id<ASAuthorizationOpenIDOperation>;

    #[objc2::method(sel = "setRequestedOperation:")]
    pub unsafe fn setRequestedOperation(
        &self,
        requested_operation: &ASAuthorizationOpenIDOperation,
    );
}
