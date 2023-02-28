//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_static!(SKErrorDomain: &'static NSString);

#[ns_enum]
#[underlying(NSInteger)]
pub enum SKErrorCode {
    SKErrorUnknown = 0,
    SKErrorClientInvalid = 1,
    SKErrorPaymentCancelled = 2,
    SKErrorPaymentInvalid = 3,
    SKErrorPaymentNotAllowed = 4,
    SKErrorStoreProductNotAvailable = 5,
    SKErrorCloudServicePermissionDenied = 6,
    SKErrorCloudServiceNetworkConnectionFailed = 7,
    SKErrorCloudServiceRevoked = 8,
    SKErrorPrivacyAcknowledgementRequired = 9,
    SKErrorUnauthorizedRequestData = 10,
    SKErrorInvalidOfferIdentifier = 11,
    SKErrorInvalidSignature = 12,
    SKErrorMissingOfferParams = 13,
    SKErrorInvalidOfferPrice = 14,
    SKErrorOverlayCancelled = 15,
    SKErrorOverlayInvalidConfiguration = 16,
    SKErrorOverlayTimeout = 17,
    SKErrorIneligibleForOffer = 18,
    SKErrorUnsupportedPlatform = 19,
    SKErrorOverlayPresentedInBackgroundScene = 20,
}
