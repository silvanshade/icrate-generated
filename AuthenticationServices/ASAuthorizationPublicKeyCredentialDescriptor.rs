//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASAuthorizationPublicKeyCredentialDescriptor;

    unsafe impl ProtocolType for ASAuthorizationPublicKeyCredentialDescriptor {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other credentialID)]
        pub unsafe fn credentialID(&self) -> Id<NSData, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setCredentialID:)]
        pub unsafe fn setCredentialID(&self, credential_id: &NSData);
    }
);
