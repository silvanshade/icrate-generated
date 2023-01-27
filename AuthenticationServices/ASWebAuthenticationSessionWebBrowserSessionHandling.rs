//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait ASWebAuthenticationSessionWebBrowserSessionHandling {
        #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionRequest")]
        #[method(beginHandlingWebAuthenticationSessionRequest:)]
        unsafe fn beginHandlingWebAuthenticationSessionRequest(
            &self,
            request: Option<&ASWebAuthenticationSessionRequest>,
        );

        #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionRequest")]
        #[method(cancelWebAuthenticationSessionRequest:)]
        unsafe fn cancelWebAuthenticationSessionRequest(
            &self,
            request: Option<&ASWebAuthenticationSessionRequest>,
        );
    }

    unsafe impl ProtocolType for dyn ASWebAuthenticationSessionWebBrowserSessionHandling {}
);
