//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum MKMapElevationStyle {
    MKMapElevationStyleFlat = 0,
    MKMapElevationStyleRealistic = 1,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMapConfiguration")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKMapConfiguration;
}

#[cfg(feature = "MapKit_MKMapConfiguration")]
unsafe impl NSCoding for MKMapConfiguration {}

#[cfg(feature = "MapKit_MKMapConfiguration")]
unsafe impl NSObjectProtocol for MKMapConfiguration {}

#[cfg(feature = "MapKit_MKMapConfiguration")]
unsafe impl NSSecureCoding for MKMapConfiguration {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMapConfiguration")]
    pub type MKMapConfiguration;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[objc2::method(sel = "elevationStyle")]
    pub unsafe fn elevationStyle(&self) -> MKMapElevationStyle;

    #[objc2::method(sel = "setElevationStyle:")]
    pub unsafe fn setElevationStyle(&self, elevation_style: MKMapElevationStyle);
}
