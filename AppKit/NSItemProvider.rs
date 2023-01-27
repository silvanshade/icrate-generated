//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_methods!(
    /// NSItemSourceInfo
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[method(sourceFrame)]
        pub unsafe fn sourceFrame(&self) -> NSRect;

        #[method(containerFrame)]
        pub unsafe fn containerFrame(&self) -> NSRect;

        #[method(preferredPresentationSize)]
        pub unsafe fn preferredPresentationSize(&self) -> NSSize;
    }
);

extern_static!(NSTypeIdentifierDateText: &'static NSString);

extern_static!(NSTypeIdentifierAddressText: &'static NSString);

extern_static!(NSTypeIdentifierPhoneNumberText: &'static NSString);

extern_static!(NSTypeIdentifierTransitInformationText: &'static NSString);
