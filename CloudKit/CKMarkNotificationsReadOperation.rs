//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = CKOperation,
    unsafe inherits = [
        NSOperation,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
    #[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CKMarkNotificationsReadOperation;
}

#[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
unsafe impl NSObjectProtocol for CKMarkNotificationsReadOperation {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
    #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
    pub type CKMarkNotificationsReadOperation;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(all(feature = "CloudKit_CKNotificationID", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "initWithNotificationIDsToMarkRead:", managed = "Init")]
    pub unsafe fn initWithNotificationIDsToMarkRead(
        this: Option<Allocated<Self>>,
        notification_i_ds: &NSArray<CKNotificationID>,
    ) -> Id<Self>;

    #[cfg(all(feature = "CloudKit_CKNotificationID", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "notificationIDs", managed = "Other")]
    pub unsafe fn notificationIDs(&self) -> Option<Id<NSArray<CKNotificationID>>>;

    #[cfg(all(feature = "CloudKit_CKNotificationID", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "setNotificationIDs:")]
    pub unsafe fn setNotificationIDs(&self, notification_i_ds: Option<&NSArray<CKNotificationID>>);

    #[cfg(all(
        feature = "CloudKit_CKNotificationID",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(sel = "markNotificationsReadCompletionBlock")]
    pub unsafe fn markNotificationsReadCompletionBlock(
        &self,
    ) -> *mut Block<(*mut NSArray<CKNotificationID>, *mut NSError), ()>;

    #[cfg(all(
        feature = "CloudKit_CKNotificationID",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(sel = "setMarkNotificationsReadCompletionBlock:")]
    pub unsafe fn setMarkNotificationsReadCompletionBlock(
        &self,
        mark_notifications_read_completion_block: Option<
            &Block<(*mut NSArray<CKNotificationID>, *mut NSError), ()>,
        >,
    );
}
