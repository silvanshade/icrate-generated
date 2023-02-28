//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[objc2::protocol]
pub unsafe trait ASAuthorizationPublicKeyCredentialRegistration:
    ASPublicKeyCredential
{
    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "rawAttestationObject", managed = "Other")]
    unsafe fn rawAttestationObject(&self) -> Option<Id<NSData>>;
}
