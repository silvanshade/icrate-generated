//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WKErrorDomain: &'static NSString);

#[ns_enum]
#[underlying(NSInteger)]
pub enum WKErrorCode {
    WKErrorUnknown = 1,
    WKErrorWebContentProcessTerminated = 2,
    WKErrorWebViewInvalidated = 3,
    WKErrorJavaScriptExceptionOccurred = 4,
    WKErrorJavaScriptResultTypeIsUnsupported = 5,
    WKErrorContentRuleListStoreCompileFailed = 6,
    WKErrorContentRuleListStoreLookUpFailed = 7,
    WKErrorContentRuleListStoreRemoveFailed = 8,
    WKErrorContentRuleListStoreVersionMismatch = 9,
    WKErrorAttributedStringContentFailedToLoad = 10,
    WKErrorAttributedStringContentLoadTimedOut = 11,
    WKErrorJavaScriptInvalidFrameTarget = 12,
    WKErrorNavigationAppBoundDomain = 13,
    WKErrorJavaScriptAppBoundDomain = 14,
    WKErrorDuplicateCredential = 15,
    WKErrorMalformedCredential = 16,
    WKErrorCredentialNotFound = 17,
}
