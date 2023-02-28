//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum UNAuthorizationStatus {
    UNAuthorizationStatusNotDetermined = 0,
    UNAuthorizationStatusDenied = 1,
    UNAuthorizationStatusAuthorized = 2,
    UNAuthorizationStatusProvisional = 3,
    UNAuthorizationStatusEphemeral = 4,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum UNShowPreviewsSetting {
    UNShowPreviewsSettingAlways = 0,
    UNShowPreviewsSettingWhenAuthenticated = 1,
    UNShowPreviewsSettingNever = 2,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum UNNotificationSetting {
    UNNotificationSettingNotSupported = 0,
    UNNotificationSettingDisabled = 1,
    UNNotificationSettingEnabled = 2,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum UNAlertStyle {
    UNAlertStyleNone = 0,
    UNAlertStyleBanner = 1,
    UNAlertStyleAlert = 2,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "UserNotifications_UNNotificationSettings")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type UNNotificationSettings;
}

#[cfg(feature = "UserNotifications_UNNotificationSettings")]
unsafe impl NSCoding for UNNotificationSettings {}

#[cfg(feature = "UserNotifications_UNNotificationSettings")]
unsafe impl NSObjectProtocol for UNNotificationSettings {}

#[cfg(feature = "UserNotifications_UNNotificationSettings")]
unsafe impl NSSecureCoding for UNNotificationSettings {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "UserNotifications_UNNotificationSettings")]
    pub type UNNotificationSettings;

    #[objc2::method(sel = "authorizationStatus")]
    pub unsafe fn authorizationStatus(&self) -> UNAuthorizationStatus;

    #[objc2::method(sel = "soundSetting")]
    pub unsafe fn soundSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "badgeSetting")]
    pub unsafe fn badgeSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "alertSetting")]
    pub unsafe fn alertSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "notificationCenterSetting")]
    pub unsafe fn notificationCenterSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "lockScreenSetting")]
    pub unsafe fn lockScreenSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "carPlaySetting")]
    pub unsafe fn carPlaySetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "alertStyle")]
    pub unsafe fn alertStyle(&self) -> UNAlertStyle;

    #[objc2::method(sel = "showPreviewsSetting")]
    pub unsafe fn showPreviewsSetting(&self) -> UNShowPreviewsSetting;

    #[objc2::method(sel = "criticalAlertSetting")]
    pub unsafe fn criticalAlertSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "providesAppNotificationSettings")]
    pub unsafe fn providesAppNotificationSettings(&self) -> bool;

    #[objc2::method(sel = "announcementSetting")]
    pub unsafe fn announcementSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "timeSensitiveSetting")]
    pub unsafe fn timeSensitiveSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "scheduledDeliverySetting")]
    pub unsafe fn scheduledDeliverySetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "directMessagesSetting")]
    pub unsafe fn directMessagesSetting(&self) -> UNNotificationSetting;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}
