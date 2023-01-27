//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOrthography")]
    pub struct NSOrthography;

    #[cfg(feature = "Foundation_NSOrthography")]
    unsafe impl ClassType for NSOrthography {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSOrthography")]
unsafe impl NSCoding for NSOrthography {}

#[cfg(feature = "Foundation_NSOrthography")]
unsafe impl NSObjectProtocol for NSOrthography {}

#[cfg(feature = "Foundation_NSOrthography")]
unsafe impl NSSecureCoding for NSOrthography {}

extern_methods!(
    #[cfg(feature = "Foundation_NSOrthography")]
    unsafe impl NSOrthography {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dominantScript)]
        pub unsafe fn dominantScript(&self) -> Id<NSString, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other languageMap)]
        pub unsafe fn languageMap(&self) -> Id<NSDictionary<NSString, NSArray<NSString>>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithDominantScript:languageMap:)]
        pub unsafe fn initWithDominantScript_languageMap(
            this: Option<Allocated<Self>>,
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSOrthographyExtended
    #[cfg(feature = "Foundation_NSOrthography")]
    unsafe impl NSOrthography {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other languagesForScript:)]
        pub unsafe fn languagesForScript(
            &self,
            script: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dominantLanguageForScript:)]
        pub unsafe fn dominantLanguageForScript(
            &self,
            script: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allScripts)]
        pub unsafe fn allScripts(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allLanguages)]
        pub unsafe fn allLanguages(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultOrthographyForLanguage:)]
        pub unsafe fn defaultOrthographyForLanguage(language: &NSString) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSOrthographyCreation
    #[cfg(feature = "Foundation_NSOrthography")]
    unsafe impl NSOrthography {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other orthographyWithDominantScript:languageMap:)]
        pub unsafe fn orthographyWithDominantScript_languageMap(
            script: &NSString,
            map: &NSDictionary<NSString, NSArray<NSString>>,
        ) -> Id<Self, Shared>;
    }
);
