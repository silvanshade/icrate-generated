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
    #[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CKDiscoverAllUserIdentitiesOperation;
}

#[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
unsafe impl NSObjectProtocol for CKDiscoverAllUserIdentitiesOperation {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
    pub type CKDiscoverAllUserIdentitiesOperation;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "CloudKit_CKUserIdentity")]
    #[objc2::method(sel = "userIdentityDiscoveredBlock")]
    pub unsafe fn userIdentityDiscoveredBlock(&self) -> *mut Block<(NonNull<CKUserIdentity>,), ()>;

    #[cfg(feature = "CloudKit_CKUserIdentity")]
    #[objc2::method(sel = "setUserIdentityDiscoveredBlock:")]
    pub unsafe fn setUserIdentityDiscoveredBlock(
        &self,
        user_identity_discovered_block: Option<&Block<(NonNull<CKUserIdentity>,), ()>>,
    );

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "discoverAllUserIdentitiesCompletionBlock")]
    pub unsafe fn discoverAllUserIdentitiesCompletionBlock(
        &self,
    ) -> *mut Block<(*mut NSError,), ()>;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "setDiscoverAllUserIdentitiesCompletionBlock:")]
    pub unsafe fn setDiscoverAllUserIdentitiesCompletionBlock(
        &self,
        discover_all_user_identities_completion_block: Option<&Block<(*mut NSError,), ()>>,
    );
}
