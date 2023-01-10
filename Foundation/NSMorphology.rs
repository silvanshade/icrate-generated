//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalGender {
        NSGrammaticalGenderNotSet = 0,
        NSGrammaticalGenderFeminine = 1,
        NSGrammaticalGenderMasculine = 2,
        NSGrammaticalGenderNeuter = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalPartOfSpeech {
        NSGrammaticalPartOfSpeechNotSet = 0,
        NSGrammaticalPartOfSpeechDeterminer = 1,
        NSGrammaticalPartOfSpeechPronoun = 2,
        NSGrammaticalPartOfSpeechLetter = 3,
        NSGrammaticalPartOfSpeechAdverb = 4,
        NSGrammaticalPartOfSpeechParticle = 5,
        NSGrammaticalPartOfSpeechAdjective = 6,
        NSGrammaticalPartOfSpeechAdposition = 7,
        NSGrammaticalPartOfSpeechVerb = 8,
        NSGrammaticalPartOfSpeechNoun = 9,
        NSGrammaticalPartOfSpeechConjunction = 10,
        NSGrammaticalPartOfSpeechNumeral = 11,
        NSGrammaticalPartOfSpeechInterjection = 12,
        NSGrammaticalPartOfSpeechPreposition = 13,
        NSGrammaticalPartOfSpeechAbbreviation = 14,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGrammaticalNumber {
        NSGrammaticalNumberNotSet = 0,
        NSGrammaticalNumberSingular = 1,
        NSGrammaticalNumberZero = 2,
        NSGrammaticalNumberPlural = 3,
        NSGrammaticalNumberPluralTwo = 4,
        NSGrammaticalNumberPluralFew = 5,
        NSGrammaticalNumberPluralMany = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMorphology;

    unsafe impl ClassType for NSMorphology {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method(grammaticalGender)]
        pub unsafe fn grammaticalGender(&self) -> Foundation::NSGrammaticalGender;

        #[method(setGrammaticalGender:)]
        pub unsafe fn setGrammaticalGender(
            &self,
            grammaticalGender: Foundation::NSGrammaticalGender,
        );

        #[method(partOfSpeech)]
        pub unsafe fn partOfSpeech(&self) -> Foundation::NSGrammaticalPartOfSpeech;

        #[method(setPartOfSpeech:)]
        pub unsafe fn setPartOfSpeech(&self, partOfSpeech: Foundation::NSGrammaticalPartOfSpeech);

        #[method(number)]
        pub unsafe fn number(&self) -> Foundation::NSGrammaticalNumber;

        #[method(setNumber:)]
        pub unsafe fn setNumber(&self, number: Foundation::NSGrammaticalNumber);
    }
);

extern_methods!(
    /// NSCustomPronouns
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[cfg(all(
            feature = "Foundation_NSMorphologyCustomPronoun",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other customPronounForLanguage:)]
        pub unsafe fn customPronounForLanguage(
            &self,
            language: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSMorphologyCustomPronoun, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSMorphologyCustomPronoun",
            feature = "Foundation_NSString"
        ))]
        #[method(setCustomPronoun:forLanguage:error:_)]
        pub unsafe fn setCustomPronoun_forLanguage_error(
            &self,
            features: Option<&Foundation::NSMorphologyCustomPronoun>,
            language: &Foundation::NSString,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMorphologyCustomPronoun;

    unsafe impl ClassType for NSMorphologyCustomPronoun {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMorphologyCustomPronoun")]
    unsafe impl NSMorphologyCustomPronoun {
        #[cfg(feature = "Foundation_NSString")]
        #[method(isSupportedForLanguage:)]
        pub unsafe fn isSupportedForLanguage(language: &Foundation::NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other requiredKeysForLanguage:)]
        pub unsafe fn requiredKeysForLanguage(
            language: &Foundation::NSString,
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subjectForm)]
        pub unsafe fn subjectForm(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubjectForm:)]
        pub unsafe fn setSubjectForm(&self, subjectForm: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForm)]
        pub unsafe fn objectForm(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setObjectForm:)]
        pub unsafe fn setObjectForm(&self, objectForm: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other possessiveForm)]
        pub unsafe fn possessiveForm(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPossessiveForm:)]
        pub unsafe fn setPossessiveForm(&self, possessiveForm: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other possessiveAdjectiveForm)]
        pub unsafe fn possessiveAdjectiveForm(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPossessiveAdjectiveForm:)]
        pub unsafe fn setPossessiveAdjectiveForm(
            &self,
            possessiveAdjectiveForm: Option<&Foundation::NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other reflexiveForm)]
        pub unsafe fn reflexiveForm(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setReflexiveForm:)]
        pub unsafe fn setReflexiveForm(&self, reflexiveForm: Option<&Foundation::NSString>);
    }
);

extern_methods!(
    /// NSMorphologyUserSettings
    #[cfg(feature = "Foundation_NSMorphology")]
    unsafe impl NSMorphology {
        #[method(isUnspecified)]
        pub unsafe fn isUnspecified(&self) -> bool;

        #[method_id(@__retain_semantics Other userMorphology)]
        pub unsafe fn userMorphology() -> Id<Foundation::NSMorphology, Shared>;
    }
);
