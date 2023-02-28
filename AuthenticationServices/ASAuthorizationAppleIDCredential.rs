//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum ASUserDetectionStatus {
    ASUserDetectionStatusUnsupported = 0,
    ASUserDetectionStatusUnknown = 1,
    ASUserDetectionStatusLikelyReal = 2,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ASAuthorizationAppleIDCredential;
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl ASAuthorizationCredential for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSCoding for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
unsafe impl NSSecureCoding for ASAuthorizationAppleIDCredential {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDCredential")]
    pub type ASAuthorizationAppleIDCredential;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "user", managed = "Other")]
    pub unsafe fn user(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "state", managed = "Other")]
    pub unsafe fn state(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "authorizedScopes", managed = "Other")]
    pub unsafe fn authorizedScopes(&self) -> Id<NSArray<ASAuthorizationScope>>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "authorizationCode", managed = "Other")]
    pub unsafe fn authorizationCode(&self) -> Option<Id<NSData>>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "identityToken", managed = "Other")]
    pub unsafe fn identityToken(&self) -> Option<Id<NSData>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "email", managed = "Other")]
    pub unsafe fn email(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSPersonNameComponents")]
    #[objc2::method(sel = "fullName", managed = "Other")]
    pub unsafe fn fullName(&self) -> Option<Id<NSPersonNameComponents>>;

    #[objc2::method(sel = "realUserStatus")]
    pub unsafe fn realUserStatus(&self) -> ASUserDetectionStatus;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}
