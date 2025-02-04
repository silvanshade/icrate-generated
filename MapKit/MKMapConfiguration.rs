//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MKMapElevationStyle {
        MKMapElevationStyleFlat = 0,
        MKMapElevationStyleRealistic = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMapConfiguration")]
    pub struct MKMapConfiguration;

    #[cfg(feature = "MapKit_MKMapConfiguration")]
    unsafe impl ClassType for MKMapConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKMapConfiguration")]
unsafe impl NSCoding for MKMapConfiguration {}

#[cfg(feature = "MapKit_MKMapConfiguration")]
unsafe impl NSObjectProtocol for MKMapConfiguration {}

#[cfg(feature = "MapKit_MKMapConfiguration")]
unsafe impl NSSecureCoding for MKMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMapConfiguration")]
    unsafe impl MKMapConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(elevationStyle)]
        pub unsafe fn elevationStyle(&self) -> MKMapElevationStyle;

        #[method(setElevationStyle:)]
        pub unsafe fn setElevationStyle(&self, elevation_style: MKMapElevationStyle);
    }
);
