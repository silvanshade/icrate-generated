//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

extern_static!(NSFoundationVersionNumber: c_double);

typed_extensible_enum!(
    pub type NSExceptionName = Foundation::NSString;
);

typed_extensible_enum!(
    pub type NSRunLoopMode = Foundation::NSString;
);

extern_fn!(
    pub unsafe fn NSStringFromSelector(aSelector: Sel) -> NonNull<Foundation::NSString>;
);

extern_fn!(
    pub unsafe fn NSSelectorFromString(aSelectorName: &Foundation::NSString) -> Sel;
);

extern_fn!(
    pub unsafe fn NSStringFromClass(aClass: &Class) -> NonNull<Foundation::NSString>;
);

extern_fn!(
    pub unsafe fn NSClassFromString(aClassName: &Foundation::NSString) -> *const Class;
);

extern_fn!(
    pub unsafe fn NSStringFromProtocol(proto: &Protocol) -> NonNull<Foundation::NSString>;
);

extern_fn!(
    pub unsafe fn NSProtocolFromString(namestr: &Foundation::NSString) -> *mut Protocol;
);

extern_fn!(
    pub unsafe fn NSGetSizeAndAlignment(
        typePtr: NonNull<c_char>,
        sizep: *mut NSUInteger,
        alignp: *mut NSUInteger,
    ) -> NonNull<c_char>;
);

pub type NSComparator =
    *mut Block<(NonNull<Object>, NonNull<Object>), Foundation::NSComparisonResult>;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEnumerationOptions {
        NSEnumerationConcurrent = 1 << 0,
        NSEnumerationReverse = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSSortOptions {
        NSSortConcurrent = 1 << 0,
        NSSortStable = 1 << 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSQualityOfService {
        NSQualityOfServiceUserInteractive = 0x21,
        NSQualityOfServiceUserInitiated = 0x19,
        NSQualityOfServiceUtility = 0x11,
        NSQualityOfServiceBackground = 0x09,
        NSQualityOfServiceDefault = -1,
    }
);
