//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKSharingParticipantAccessOption {
        CKSharingParticipantAccessOptionAnyoneWithLink = 1 << 0,
        CKSharingParticipantAccessOptionSpecifiedRecipientsOnly = 1 << 1,
        CKSharingParticipantAccessOptionAny = CKSharingParticipantAccessOptionAnyoneWithLink
            | CKSharingParticipantAccessOptionSpecifiedRecipientsOnly,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKSharingParticipantPermissionOption {
        CKSharingParticipantPermissionOptionReadOnly = 1 << 0,
        CKSharingParticipantPermissionOptionReadWrite = 1 << 1,
        CKSharingParticipantPermissionOptionAny = CKSharingParticipantPermissionOptionReadOnly
            | CKSharingParticipantPermissionOptionReadWrite,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    pub struct CKAllowedSharingOptions;

    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    unsafe impl ClassType for CKAllowedSharingOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
unsafe impl NSCoding for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
unsafe impl NSObjectProtocol for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
unsafe impl NSSecureCoding for CKAllowedSharingOptions {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    unsafe impl CKAllowedSharingOptions {
        #[method_id(@__retain_semantics Init initWithAllowedParticipantPermissionOptions:allowedParticipantAccessOptions:)]
        pub unsafe fn initWithAllowedParticipantPermissionOptions_allowedParticipantAccessOptions(
            this: Option<Allocated<Self>>,
            allowed_participant_permission_options: CKSharingParticipantPermissionOption,
            allowed_participant_access_options: CKSharingParticipantAccessOption,
        ) -> Id<Self>;

        #[method(allowedParticipantPermissionOptions)]
        pub unsafe fn allowedParticipantPermissionOptions(
            &self,
        ) -> CKSharingParticipantPermissionOption;

        #[method(setAllowedParticipantPermissionOptions:)]
        pub unsafe fn setAllowedParticipantPermissionOptions(
            &self,
            allowed_participant_permission_options: CKSharingParticipantPermissionOption,
        );

        #[method(allowedParticipantAccessOptions)]
        pub unsafe fn allowedParticipantAccessOptions(&self) -> CKSharingParticipantAccessOption;

        #[method(setAllowedParticipantAccessOptions:)]
        pub unsafe fn setAllowedParticipantAccessOptions(
            &self,
            allowed_participant_access_options: CKSharingParticipantAccessOption,
        );

        #[method_id(@__retain_semantics Other standardOptions)]
        pub unsafe fn standardOptions() -> Id<CKAllowedSharingOptions>;
    }
);
