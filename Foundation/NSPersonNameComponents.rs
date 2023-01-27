//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPersonNameComponents")]
    pub struct NSPersonNameComponents;

    #[cfg(feature = "Foundation_NSPersonNameComponents")]
    unsafe impl ClassType for NSPersonNameComponents {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSPersonNameComponents")]
unsafe impl NSCoding for NSPersonNameComponents {}

#[cfg(feature = "Foundation_NSPersonNameComponents")]
unsafe impl NSObjectProtocol for NSPersonNameComponents {}

#[cfg(feature = "Foundation_NSPersonNameComponents")]
unsafe impl NSSecureCoding for NSPersonNameComponents {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPersonNameComponents")]
    unsafe impl NSPersonNameComponents {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other namePrefix)]
        pub unsafe fn namePrefix(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNamePrefix:)]
        pub unsafe fn setNamePrefix(&self, name_prefix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other givenName)]
        pub unsafe fn givenName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setGivenName:)]
        pub unsafe fn setGivenName(&self, given_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other middleName)]
        pub unsafe fn middleName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMiddleName:)]
        pub unsafe fn setMiddleName(&self, middle_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFamilyName:)]
        pub unsafe fn setFamilyName(&self, family_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameSuffix)]
        pub unsafe fn nameSuffix(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNameSuffix:)]
        pub unsafe fn setNameSuffix(&self, name_suffix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nickname)]
        pub unsafe fn nickname(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNickname:)]
        pub unsafe fn setNickname(&self, nickname: Option<&NSString>);

        #[method_id(@__retain_semantics Other phoneticRepresentation)]
        pub unsafe fn phoneticRepresentation(&self) -> Option<Id<NSPersonNameComponents, Shared>>;

        #[method(setPhoneticRepresentation:)]
        pub unsafe fn setPhoneticRepresentation(
            &self,
            phonetic_representation: Option<&NSPersonNameComponents>,
        );
    }
);
