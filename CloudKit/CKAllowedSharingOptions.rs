//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

#[ns_options]
#[underlying(NSUInteger)]
pub enum CKSharingParticipantAccessOption {
    CKSharingParticipantAccessOptionAnyoneWithLink = 1 << 0,
    CKSharingParticipantAccessOptionSpecifiedRecipientsOnly = 1 << 1,
    CKSharingParticipantAccessOptionAny = CKSharingParticipantAccessOptionAnyoneWithLink
        | CKSharingParticipantAccessOptionSpecifiedRecipientsOnly,
}

#[ns_options]
#[underlying(NSUInteger)]
pub enum CKSharingParticipantPermissionOption {
    CKSharingParticipantPermissionOptionReadOnly = 1 << 0,
    CKSharingParticipantPermissionOptionReadWrite = 1 << 1,
    CKSharingParticipantPermissionOptionAny = CKSharingParticipantPermissionOptionReadOnly
        | CKSharingParticipantPermissionOptionReadWrite,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CKAllowedSharingOptions;
}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
unsafe impl NSCoding for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
unsafe impl NSObjectProtocol for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
unsafe impl NSSecureCoding for CKAllowedSharingOptions {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    pub type CKAllowedSharingOptions;

    #[objc2::method(
        sel = "initWithAllowedParticipantPermissionOptions:allowedParticipantAccessOptions:",
        managed = "Init"
    )]
    pub unsafe fn initWithAllowedParticipantPermissionOptions_allowedParticipantAccessOptions(
        this: Option<Allocated<Self>>,
        allowed_participant_permission_options: CKSharingParticipantPermissionOption,
        allowed_participant_access_options: CKSharingParticipantAccessOption,
    ) -> Id<Self>;

    #[objc2::method(sel = "allowedParticipantPermissionOptions")]
    pub unsafe fn allowedParticipantPermissionOptions(
        &self,
    ) -> CKSharingParticipantPermissionOption;

    #[objc2::method(sel = "setAllowedParticipantPermissionOptions:")]
    pub unsafe fn setAllowedParticipantPermissionOptions(
        &self,
        allowed_participant_permission_options: CKSharingParticipantPermissionOption,
    );

    #[objc2::method(sel = "allowedParticipantAccessOptions")]
    pub unsafe fn allowedParticipantAccessOptions(&self) -> CKSharingParticipantAccessOption;

    #[objc2::method(sel = "setAllowedParticipantAccessOptions:")]
    pub unsafe fn setAllowedParticipantAccessOptions(
        &self,
        allowed_participant_access_options: CKSharingParticipantAccessOption,
    );

    #[objc2::method(sel = "standardOptions", managed = "Other")]
    pub unsafe fn standardOptions() -> Id<CKAllowedSharingOptions>;
}
