//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKModifyBadgeOperation")]
    #[deprecated = "No longer supported, will cease working at some point in the future"]
    pub struct CKModifyBadgeOperation;

    #[cfg(feature = "CloudKit_CKModifyBadgeOperation")]
    unsafe impl ClassType for CKModifyBadgeOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

#[cfg(feature = "CloudKit_CKModifyBadgeOperation")]
unsafe impl NSObjectProtocol for CKModifyBadgeOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKModifyBadgeOperation")]
    unsafe impl CKModifyBadgeOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBadgeValue:)]
        pub unsafe fn initWithBadgeValue(
            this: Option<Allocated<Self>>,
            badge_value: NSUInteger,
        ) -> Id<Self>;

        #[method(badgeValue)]
        pub unsafe fn badgeValue(&self) -> NSUInteger;

        #[method(setBadgeValue:)]
        pub unsafe fn setBadgeValue(&self, badge_value: NSUInteger);

        #[cfg(feature = "Foundation_NSError")]
        #[method(modifyBadgeCompletionBlock)]
        pub unsafe fn modifyBadgeCompletionBlock(&self) -> *mut Block<(*mut NSError,), ()>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(setModifyBadgeCompletionBlock:)]
        pub unsafe fn setModifyBadgeCompletionBlock(
            &self,
            modify_badge_completion_block: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);
