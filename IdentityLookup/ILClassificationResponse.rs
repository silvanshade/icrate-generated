//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILClassificationResponse")]
    pub struct ILClassificationResponse;

    #[cfg(feature = "IdentityLookup_ILClassificationResponse")]
    unsafe impl ClassType for ILClassificationResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
unsafe impl NSCoding for ILClassificationResponse {}

#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
unsafe impl NSObjectProtocol for ILClassificationResponse {}

#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
unsafe impl NSSecureCoding for ILClassificationResponse {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILClassificationResponse")]
    unsafe impl ILClassificationResponse {
        #[method(action)]
        pub unsafe fn action(&self) -> ILClassificationAction;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userString)]
        pub unsafe fn userString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUserString:)]
        pub unsafe fn setUserString(&self, user_string: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, Object>>);

        #[method_id(@__retain_semantics Init initWithClassificationAction:)]
        pub unsafe fn initWithClassificationAction(
            this: Option<Allocated<Self>>,
            action: ILClassificationAction,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
