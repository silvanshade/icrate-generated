//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreFoundation::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_protocol!(
    pub unsafe trait MKAnnotation: NSObjectProtocol {
        #[method(coordinate)]
        unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other title)]
        unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other subtitle)]
        unsafe fn subtitle(&self) -> Option<Id<NSString, Shared>>;

        #[optional]
        #[method(setCoordinate:)]
        unsafe fn setCoordinate(&self, new_coordinate: CLLocationCoordinate2D);
    }

    unsafe impl ProtocolType for dyn MKAnnotation {}
);
