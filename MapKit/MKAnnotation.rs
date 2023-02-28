//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[objc2::protocol]
pub unsafe trait MKAnnotation: NSObjectProtocol {
    #[objc2::method(sel = "coordinate")]
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(optional, sel = "title", managed = "Other")]
    unsafe fn title(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(optional, sel = "subtitle", managed = "Other")]
    unsafe fn subtitle(&self) -> Option<Id<NSString>>;

    #[objc2::method(optional, sel = "setCoordinate:")]
    unsafe fn setCoordinate(&self, new_coordinate: CLLocationCoordinate2D);
}
