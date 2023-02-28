//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

#[objc2::protocol]
pub unsafe trait SKOverlayDelegate: NSObjectProtocol {
    #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKOverlay"))]
    #[objc2::method(optional, sel = "storeOverlay:didFailToLoadWithError:")]
    unsafe fn storeOverlay_didFailToLoadWithError(&self, overlay: &SKOverlay, error: &NSError);

    #[cfg(all(
        feature = "StoreKit_SKOverlay",
        feature = "StoreKit_SKOverlayTransitionContext"
    ))]
    #[objc2::method(optional, sel = "storeOverlay:willStartPresentation:")]
    unsafe fn storeOverlay_willStartPresentation(
        &self,
        overlay: &SKOverlay,
        transition_context: &SKOverlayTransitionContext,
    );

    #[cfg(all(
        feature = "StoreKit_SKOverlay",
        feature = "StoreKit_SKOverlayTransitionContext"
    ))]
    #[objc2::method(optional, sel = "storeOverlay:didFinishPresentation:")]
    unsafe fn storeOverlay_didFinishPresentation(
        &self,
        overlay: &SKOverlay,
        transition_context: &SKOverlayTransitionContext,
    );

    #[cfg(all(
        feature = "StoreKit_SKOverlay",
        feature = "StoreKit_SKOverlayTransitionContext"
    ))]
    #[objc2::method(optional, sel = "storeOverlay:willStartDismissal:")]
    unsafe fn storeOverlay_willStartDismissal(
        &self,
        overlay: &SKOverlay,
        transition_context: &SKOverlayTransitionContext,
    );

    #[cfg(all(
        feature = "StoreKit_SKOverlay",
        feature = "StoreKit_SKOverlayTransitionContext"
    ))]
    #[objc2::method(optional, sel = "storeOverlay:didFinishDismissal:")]
    unsafe fn storeOverlay_didFinishDismissal(
        &self,
        overlay: &SKOverlay,
        transition_context: &SKOverlayTransitionContext,
    );
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "StoreKit_SKOverlay")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type SKOverlay;
}

#[cfg(feature = "StoreKit_SKOverlay")]
unsafe impl NSObjectProtocol for SKOverlay {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "StoreKit_SKOverlay")]
    pub type SKOverlay;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
    #[objc2::method(sel = "initWithConfiguration:", managed = "Init")]
    pub unsafe fn initWithConfiguration(
        this: Option<Allocated<Self>>,
        configuration: &SKOverlayConfiguration,
    ) -> Id<Self>;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn SKOverlayDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn SKOverlayDelegate>>);

    #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
    #[objc2::method(sel = "configuration", managed = "Other")]
    pub unsafe fn configuration(&self) -> Id<SKOverlayConfiguration>;
}
