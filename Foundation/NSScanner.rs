//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSScanner")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScanner;
}

#[cfg(feature = "Foundation_NSScanner")]
unsafe impl NSObjectProtocol for NSScanner {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSScanner")]
    pub type NSScanner;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "string", managed = "Other")]
    pub unsafe fn string(&self) -> Id<NSString>;

    #[objc2::method(sel = "scanLocation")]
    pub unsafe fn scanLocation(&self) -> NSUInteger;

    #[objc2::method(sel = "setScanLocation:")]
    pub unsafe fn setScanLocation(&self, scan_location: NSUInteger);

    #[cfg(feature = "Foundation_NSCharacterSet")]
    #[objc2::method(sel = "charactersToBeSkipped", managed = "Other")]
    pub unsafe fn charactersToBeSkipped(&self) -> Option<Id<NSCharacterSet>>;

    #[cfg(feature = "Foundation_NSCharacterSet")]
    #[objc2::method(sel = "setCharactersToBeSkipped:")]
    pub unsafe fn setCharactersToBeSkipped(
        &self,
        characters_to_be_skipped: Option<&NSCharacterSet>,
    );

    #[objc2::method(sel = "caseSensitive")]
    pub unsafe fn caseSensitive(&self) -> bool;

    #[objc2::method(sel = "setCaseSensitive:")]
    pub unsafe fn setCaseSensitive(&self, case_sensitive: bool);

    #[objc2::method(sel = "locale", managed = "Other")]
    pub unsafe fn locale(&self) -> Option<Id<Object>>;

    #[objc2::method(sel = "setLocale:")]
    pub unsafe fn setLocale(&self, locale: Option<&Object>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithString:", managed = "Init")]
    pub unsafe fn initWithString(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSScanner")]
    pub type NSScanner;

    #[objc2::method(sel = "scanInt:")]
    pub unsafe fn scanInt(&self, result: *mut c_int) -> bool;

    #[objc2::method(sel = "scanInteger:")]
    pub unsafe fn scanInteger(&self, result: *mut NSInteger) -> bool;

    #[objc2::method(sel = "scanLongLong:")]
    pub unsafe fn scanLongLong(&self, result: *mut c_longlong) -> bool;

    #[objc2::method(sel = "scanUnsignedLongLong:")]
    pub unsafe fn scanUnsignedLongLong(&self, result: *mut c_ulonglong) -> bool;

    #[objc2::method(sel = "scanFloat:")]
    pub unsafe fn scanFloat(&self, result: *mut c_float) -> bool;

    #[objc2::method(sel = "scanDouble:")]
    pub unsafe fn scanDouble(&self, result: *mut c_double) -> bool;

    #[objc2::method(sel = "scanHexInt:")]
    pub unsafe fn scanHexInt(&self, result: *mut c_uint) -> bool;

    #[objc2::method(sel = "scanHexLongLong:")]
    pub unsafe fn scanHexLongLong(&self, result: *mut c_ulonglong) -> bool;

    #[objc2::method(sel = "scanHexFloat:")]
    pub unsafe fn scanHexFloat(&self, result: *mut c_float) -> bool;

    #[objc2::method(sel = "scanHexDouble:")]
    pub unsafe fn scanHexDouble(&self, result: *mut c_double) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "scanString:intoString:")]
    pub unsafe fn scanString_intoString(
        &self,
        string: &NSString,
        result: Option<&mut Option<Id<NSString>>>,
    ) -> bool;

    #[cfg(all(feature = "Foundation_NSCharacterSet", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "scanCharactersFromSet:intoString:")]
    pub unsafe fn scanCharactersFromSet_intoString(
        &self,
        set: &NSCharacterSet,
        result: Option<&mut Option<Id<NSString>>>,
    ) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "scanUpToString:intoString:")]
    pub unsafe fn scanUpToString_intoString(
        &self,
        string: &NSString,
        result: Option<&mut Option<Id<NSString>>>,
    ) -> bool;

    #[cfg(all(feature = "Foundation_NSCharacterSet", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "scanUpToCharactersFromSet:intoString:")]
    pub unsafe fn scanUpToCharactersFromSet_intoString(
        &self,
        set: &NSCharacterSet,
        result: Option<&mut Option<Id<NSString>>>,
    ) -> bool;

    #[objc2::method(sel = "isAtEnd")]
    pub unsafe fn isAtEnd(&self) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "scannerWithString:", managed = "Other")]
    pub unsafe fn scannerWithString(string: &NSString) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedScannerWithString:", managed = "Other")]
    pub unsafe fn localizedScannerWithString(string: &NSString) -> Id<Object>;
}
