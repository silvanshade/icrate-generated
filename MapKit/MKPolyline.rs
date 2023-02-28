//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[objc2::interface(
    unsafe super = MKMultiPoint,
    unsafe inherits = [
        MKShape,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKPolyline")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKPolyline;
}

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl MKAnnotation for MKPolyline {}

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl MKOverlay for MKPolyline {}

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl NSObjectProtocol for MKPolyline {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKPolyline")]
    pub type MKPolyline;

    #[objc2::method(sel = "polylineWithPoints:count:", managed = "Other")]
    pub unsafe fn polylineWithPoints_count(
        points: NonNull<MKMapPoint>,
        count: NSUInteger,
    ) -> Id<Self>;

    #[objc2::method(sel = "polylineWithCoordinates:count:", managed = "Other")]
    pub unsafe fn polylineWithCoordinates_count(
        coords: NonNull<CLLocationCoordinate2D>,
        count: NSUInteger,
    ) -> Id<Self>;
}
