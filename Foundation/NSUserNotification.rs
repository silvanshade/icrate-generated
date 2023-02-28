//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSInteger)]
#[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
pub enum NSUserNotificationActivationType {
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    NSUserNotificationActivationTypeNone = 0,
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    NSUserNotificationActivationTypeContentsClicked = 1,
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    NSUserNotificationActivationTypeActionButtonClicked = 2,
    NSUserNotificationActivationTypeReplied = 3,
    NSUserNotificationActivationTypeAdditionalActionClicked = 4,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[cfg(feature = "Foundation_NSUserNotification")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserNotification;
}

#[cfg(feature = "Foundation_NSUserNotification")]
unsafe impl NSObjectProtocol for NSUserNotification {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserNotification")]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub type NSUserNotification;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "title", managed = "Other")]
    pub unsafe fn title(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTitle:")]
    pub unsafe fn setTitle(&self, title: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "subtitle", managed = "Other")]
    pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setSubtitle:")]
    pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "informativeText", managed = "Other")]
    pub unsafe fn informativeText(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setInformativeText:")]
    pub unsafe fn setInformativeText(&self, informative_text: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "actionButtonTitle", managed = "Other")]
    pub unsafe fn actionButtonTitle(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setActionButtonTitle:")]
    pub unsafe fn setActionButtonTitle(&self, action_button_title: &NSString);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "userInfo", managed = "Other")]
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setUserInfo:")]
    pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, Object>>);

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "deliveryDate", managed = "Other")]
    pub unsafe fn deliveryDate(&self) -> Option<Id<NSDate>>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "setDeliveryDate:")]
    pub unsafe fn setDeliveryDate(&self, delivery_date: Option<&NSDate>);

    #[cfg(feature = "Foundation_NSTimeZone")]
    #[objc2::method(sel = "deliveryTimeZone", managed = "Other")]
    pub unsafe fn deliveryTimeZone(&self) -> Option<Id<NSTimeZone>>;

    #[cfg(feature = "Foundation_NSTimeZone")]
    #[objc2::method(sel = "setDeliveryTimeZone:")]
    pub unsafe fn setDeliveryTimeZone(&self, delivery_time_zone: Option<&NSTimeZone>);

    #[cfg(feature = "Foundation_NSDateComponents")]
    #[objc2::method(sel = "deliveryRepeatInterval", managed = "Other")]
    pub unsafe fn deliveryRepeatInterval(&self) -> Option<Id<NSDateComponents>>;

    #[cfg(feature = "Foundation_NSDateComponents")]
    #[objc2::method(sel = "setDeliveryRepeatInterval:")]
    pub unsafe fn setDeliveryRepeatInterval(
        &self,
        delivery_repeat_interval: Option<&NSDateComponents>,
    );

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "actualDeliveryDate", managed = "Other")]
    pub unsafe fn actualDeliveryDate(&self) -> Option<Id<NSDate>>;

    #[objc2::method(sel = "isPresented")]
    pub unsafe fn isPresented(&self) -> bool;

    #[objc2::method(sel = "isRemote")]
    pub unsafe fn isRemote(&self) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "soundName", managed = "Other")]
    pub unsafe fn soundName(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setSoundName:")]
    pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

    #[objc2::method(sel = "hasActionButton")]
    pub unsafe fn hasActionButton(&self) -> bool;

    #[objc2::method(sel = "setHasActionButton:")]
    pub unsafe fn setHasActionButton(&self, has_action_button: bool);

    #[objc2::method(sel = "activationType")]
    pub unsafe fn activationType(&self) -> NSUserNotificationActivationType;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "otherButtonTitle", managed = "Other")]
    pub unsafe fn otherButtonTitle(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setOtherButtonTitle:")]
    pub unsafe fn setOtherButtonTitle(&self, other_button_title: &NSString);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setIdentifier:")]
    pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

    #[objc2::method(sel = "hasReplyButton")]
    pub unsafe fn hasReplyButton(&self) -> bool;

    #[objc2::method(sel = "setHasReplyButton:")]
    pub unsafe fn setHasReplyButton(&self, has_reply_button: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "responsePlaceholder", managed = "Other")]
    pub unsafe fn responsePlaceholder(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setResponsePlaceholder:")]
    pub unsafe fn setResponsePlaceholder(&self, response_placeholder: Option<&NSString>);

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "response", managed = "Other")]
    pub unsafe fn response(&self) -> Option<Id<NSAttributedString>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSUserNotificationAction"
    ))]
    #[objc2::method(sel = "additionalActions", managed = "Other")]
    pub unsafe fn additionalActions(&self) -> Option<Id<NSArray<NSUserNotificationAction>>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSUserNotificationAction"
    ))]
    #[objc2::method(sel = "setAdditionalActions:")]
    pub unsafe fn setAdditionalActions(
        &self,
        additional_actions: Option<&NSArray<NSUserNotificationAction>>,
    );

    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    #[objc2::method(sel = "additionalActivationAction", managed = "Other")]
    pub unsafe fn additionalActivationAction(&self) -> Option<Id<NSUserNotificationAction>>;
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserNotificationAction;
}

#[cfg(feature = "Foundation_NSUserNotificationAction")]
unsafe impl NSObjectProtocol for NSUserNotificationAction {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserNotificationAction")]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub type NSUserNotificationAction;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "actionWithIdentifier:title:", managed = "Other")]
    pub unsafe fn actionWithIdentifier_title(
        identifier: Option<&NSString>,
        title: Option<&NSString>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "title", managed = "Other")]
    pub unsafe fn title(&self) -> Option<Id<NSString>>;
}

extern_static!(NSUserNotificationDefaultSoundName: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[cfg(feature = "Foundation_NSUserNotificationCenter")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserNotificationCenter;
}

#[cfg(feature = "Foundation_NSUserNotificationCenter")]
unsafe impl NSObjectProtocol for NSUserNotificationCenter {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserNotificationCenter")]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub type NSUserNotificationCenter;

    #[objc2::method(sel = "defaultUserNotificationCenter", managed = "Other")]
    pub unsafe fn defaultUserNotificationCenter() -> Id<NSUserNotificationCenter>;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(
        &self,
    ) -> Option<Id<ProtocolObject<dyn NSUserNotificationCenterDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(
        &self,
        delegate: Option<&ProtocolObject<dyn NSUserNotificationCenterDelegate>>,
    );

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSUserNotification"
    ))]
    #[objc2::method(sel = "scheduledNotifications", managed = "Other")]
    pub unsafe fn scheduledNotifications(&self) -> Id<NSArray<NSUserNotification>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSUserNotification"
    ))]
    #[objc2::method(sel = "setScheduledNotifications:")]
    pub unsafe fn setScheduledNotifications(
        &self,
        scheduled_notifications: &NSArray<NSUserNotification>,
    );

    #[cfg(feature = "Foundation_NSUserNotification")]
    #[objc2::method(sel = "scheduleNotification:")]
    pub unsafe fn scheduleNotification(&self, notification: &NSUserNotification);

    #[cfg(feature = "Foundation_NSUserNotification")]
    #[objc2::method(sel = "removeScheduledNotification:")]
    pub unsafe fn removeScheduledNotification(&self, notification: &NSUserNotification);

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSUserNotification"
    ))]
    #[objc2::method(sel = "deliveredNotifications", managed = "Other")]
    pub unsafe fn deliveredNotifications(&self) -> Id<NSArray<NSUserNotification>>;

    #[cfg(feature = "Foundation_NSUserNotification")]
    #[objc2::method(sel = "deliverNotification:")]
    pub unsafe fn deliverNotification(&self, notification: &NSUserNotification);

    #[cfg(feature = "Foundation_NSUserNotification")]
    #[objc2::method(sel = "removeDeliveredNotification:")]
    pub unsafe fn removeDeliveredNotification(&self, notification: &NSUserNotification);

    #[objc2::method(sel = "removeAllDeliveredNotifications")]
    pub unsafe fn removeAllDeliveredNotifications(&self);
}

#[objc2::protocol]
pub unsafe trait NSUserNotificationCenterDelegate: NSObjectProtocol {
    #[cfg(all(
        feature = "Foundation_NSUserNotification",
        feature = "Foundation_NSUserNotificationCenter"
    ))]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[objc2::method(optional, sel = "userNotificationCenter:didDeliverNotification:")]
    unsafe fn userNotificationCenter_didDeliverNotification(
        &self,
        center: &NSUserNotificationCenter,
        notification: &NSUserNotification,
    );

    #[cfg(all(
        feature = "Foundation_NSUserNotification",
        feature = "Foundation_NSUserNotificationCenter"
    ))]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[objc2::method(optional, sel = "userNotificationCenter:didActivateNotification:")]
    unsafe fn userNotificationCenter_didActivateNotification(
        &self,
        center: &NSUserNotificationCenter,
        notification: &NSUserNotification,
    );

    #[cfg(all(
        feature = "Foundation_NSUserNotification",
        feature = "Foundation_NSUserNotificationCenter"
    ))]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[objc2::method(optional, sel = "userNotificationCenter:shouldPresentNotification:")]
    unsafe fn userNotificationCenter_shouldPresentNotification(
        &self,
        center: &NSUserNotificationCenter,
        notification: &NSUserNotification,
    ) -> bool;
}
