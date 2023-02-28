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
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type ASAuthorizationProviderExtensionAuthorizationResult;
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionAuthorizationResult {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult")]
    pub type ASAuthorizationProviderExtensionAuthorizationResult;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithHTTPAuthorizationHeaders:", managed = "Init")]
    pub unsafe fn initWithHTTPAuthorizationHeaders(
        this: Option<Allocated<Self>>,
        http_authorization_headers: &NSDictionary<NSString, NSString>,
    ) -> Id<Self>;

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "Foundation_NSHTTPURLResponse"
    ))]
    #[objc2::method(sel = "initWithHTTPResponse:httpBody:", managed = "Init")]
    pub unsafe fn initWithHTTPResponse_httpBody(
        this: Option<Allocated<Self>>,
        http_response: &NSHTTPURLResponse,
        http_body: Option<&NSData>,
    ) -> Id<Self>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "httpAuthorizationHeaders", managed = "Other")]
    pub unsafe fn httpAuthorizationHeaders(&self) -> Option<Id<NSDictionary<NSString, NSString>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setHttpAuthorizationHeaders:")]
    pub unsafe fn setHttpAuthorizationHeaders(
        &self,
        http_authorization_headers: Option<&NSDictionary<NSString, NSString>>,
    );

    #[cfg(feature = "Foundation_NSHTTPURLResponse")]
    #[objc2::method(sel = "httpResponse", managed = "Other")]
    pub unsafe fn httpResponse(&self) -> Option<Id<NSHTTPURLResponse>>;

    #[cfg(feature = "Foundation_NSHTTPURLResponse")]
    #[objc2::method(sel = "setHttpResponse:")]
    pub unsafe fn setHttpResponse(&self, http_response: Option<&NSHTTPURLResponse>);

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "httpBody", managed = "Other")]
    pub unsafe fn httpBody(&self) -> Option<Id<NSData>>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "setHttpBody:")]
    pub unsafe fn setHttpBody(&self, http_body: Option<&NSData>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "privateKeys", managed = "Other")]
    pub unsafe fn privateKeys(&self) -> Id<NSArray>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setPrivateKeys:")]
    pub unsafe fn setPrivateKeys(&self, private_keys: &NSArray);
}
