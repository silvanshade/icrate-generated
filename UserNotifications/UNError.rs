//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_static!(UNErrorDomain: Option<&'static NSString>);

#[ns_enum]
#[underlying(NSInteger)]
pub enum UNErrorCode {
    UNErrorCodeNotificationsNotAllowed = 1,
    UNErrorCodeAttachmentInvalidURL = 100,
    UNErrorCodeAttachmentUnrecognizedType = 101,
    UNErrorCodeAttachmentInvalidFileSize = 102,
    UNErrorCodeAttachmentNotInDataStore = 103,
    UNErrorCodeAttachmentMoveIntoDataStoreFailed = 104,
    UNErrorCodeAttachmentCorrupt = 105,
    UNErrorCodeNotificationInvalidNoDate = 1400,
    UNErrorCodeNotificationInvalidNoContent = 1401,
    UNErrorCodeContentProvidingObjectNotAllowed = 1500,
    UNErrorCodeContentProvidingInvalid = 1501,
    UNErrorCodeBadgeInputInvalid = 1600,
}
