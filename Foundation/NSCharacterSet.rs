//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSOpenStepUnicodeReservedBase = 0xF400,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCharacterSet")]
    pub struct NSCharacterSet;

    #[cfg(feature = "Foundation_NSCharacterSet")]
    unsafe impl ClassType for NSCharacterSet {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSCharacterSet")]
    unsafe impl NSCharacterSet {
        #[method_id(@__retain_semantics Other controlCharacterSet)]
        pub unsafe fn controlCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other whitespaceCharacterSet)]
        pub unsafe fn whitespaceCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other whitespaceAndNewlineCharacterSet)]
        pub unsafe fn whitespaceAndNewlineCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other decimalDigitCharacterSet)]
        pub unsafe fn decimalDigitCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other letterCharacterSet)]
        pub unsafe fn letterCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other lowercaseLetterCharacterSet)]
        pub unsafe fn lowercaseLetterCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other uppercaseLetterCharacterSet)]
        pub unsafe fn uppercaseLetterCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other nonBaseCharacterSet)]
        pub unsafe fn nonBaseCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other alphanumericCharacterSet)]
        pub unsafe fn alphanumericCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other decomposableCharacterSet)]
        pub unsafe fn decomposableCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other illegalCharacterSet)]
        pub unsafe fn illegalCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other punctuationCharacterSet)]
        pub unsafe fn punctuationCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other capitalizedLetterCharacterSet)]
        pub unsafe fn capitalizedLetterCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other symbolCharacterSet)]
        pub unsafe fn symbolCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other newlineCharacterSet)]
        pub unsafe fn newlineCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other characterSetWithRange:)]
        pub unsafe fn characterSetWithRange(a_range: NSRange) -> Id<NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characterSetWithCharactersInString:)]
        pub unsafe fn characterSetWithCharactersInString(
            a_string: &NSString,
        ) -> Id<NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other characterSetWithBitmapRepresentation:)]
        pub unsafe fn characterSetWithBitmapRepresentation(
            data: &NSData,
        ) -> Id<NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characterSetWithContentsOfFile:)]
        pub unsafe fn characterSetWithContentsOfFile(
            f_name: &NSString,
        ) -> Option<Id<NSCharacterSet, Shared>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(characterIsMember:)]
        pub unsafe fn characterIsMember(&self, a_character: unichar) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other bitmapRepresentation)]
        pub unsafe fn bitmapRepresentation(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other invertedSet)]
        pub unsafe fn invertedSet(&self) -> Id<NSCharacterSet, Shared>;

        #[method(longCharacterIsMember:)]
        pub unsafe fn longCharacterIsMember(&self, the_long_char: UTF32Char) -> bool;

        #[method(isSupersetOfSet:)]
        pub unsafe fn isSupersetOfSet(&self, the_other_set: &NSCharacterSet) -> bool;

        #[method(hasMemberInPlane:)]
        pub unsafe fn hasMemberInPlane(&self, the_plane: u8) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMutableCharacterSet")]
    pub struct NSMutableCharacterSet;

    #[cfg(feature = "Foundation_NSMutableCharacterSet")]
    unsafe impl ClassType for NSMutableCharacterSet {
        #[inherits(NSObject)]
        type Super = NSCharacterSet;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableCharacterSet")]
    unsafe impl NSMutableCharacterSet {
        #[method(addCharactersInRange:)]
        pub unsafe fn addCharactersInRange(&self, a_range: NSRange);

        #[method(removeCharactersInRange:)]
        pub unsafe fn removeCharactersInRange(&self, a_range: NSRange);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addCharactersInString:)]
        pub unsafe fn addCharactersInString(&self, a_string: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeCharactersInString:)]
        pub unsafe fn removeCharactersInString(&self, a_string: &NSString);

        #[method(formUnionWithCharacterSet:)]
        pub unsafe fn formUnionWithCharacterSet(&self, other_set: &NSCharacterSet);

        #[method(formIntersectionWithCharacterSet:)]
        pub unsafe fn formIntersectionWithCharacterSet(&self, other_set: &NSCharacterSet);

        #[method(invert)]
        pub unsafe fn invert(&self);

        #[method_id(@__retain_semantics Other controlCharacterSet)]
        pub unsafe fn controlCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other whitespaceCharacterSet)]
        pub unsafe fn whitespaceCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other whitespaceAndNewlineCharacterSet)]
        pub unsafe fn whitespaceAndNewlineCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other decimalDigitCharacterSet)]
        pub unsafe fn decimalDigitCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other letterCharacterSet)]
        pub unsafe fn letterCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other lowercaseLetterCharacterSet)]
        pub unsafe fn lowercaseLetterCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other uppercaseLetterCharacterSet)]
        pub unsafe fn uppercaseLetterCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other nonBaseCharacterSet)]
        pub unsafe fn nonBaseCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other alphanumericCharacterSet)]
        pub unsafe fn alphanumericCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other decomposableCharacterSet)]
        pub unsafe fn decomposableCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other illegalCharacterSet)]
        pub unsafe fn illegalCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other punctuationCharacterSet)]
        pub unsafe fn punctuationCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other capitalizedLetterCharacterSet)]
        pub unsafe fn capitalizedLetterCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other symbolCharacterSet)]
        pub unsafe fn symbolCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other newlineCharacterSet)]
        pub unsafe fn newlineCharacterSet() -> Id<NSMutableCharacterSet, Owned>;

        #[method_id(@__retain_semantics Other characterSetWithRange:)]
        pub unsafe fn characterSetWithRange(a_range: NSRange) -> Id<NSMutableCharacterSet, Owned>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characterSetWithCharactersInString:)]
        pub unsafe fn characterSetWithCharactersInString(
            a_string: &NSString,
        ) -> Id<NSMutableCharacterSet, Owned>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other characterSetWithBitmapRepresentation:)]
        pub unsafe fn characterSetWithBitmapRepresentation(
            data: &NSData,
        ) -> Id<NSMutableCharacterSet, Owned>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characterSetWithContentsOfFile:)]
        pub unsafe fn characterSetWithContentsOfFile(
            f_name: &NSString,
        ) -> Option<Id<NSMutableCharacterSet, Owned>>;
    }
);
