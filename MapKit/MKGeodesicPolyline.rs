//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreFoundation::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKGeodesicPolyline")]
    pub struct MKGeodesicPolyline;

    #[cfg(feature = "MapKit_MKGeodesicPolyline")]
    unsafe impl ClassType for MKGeodesicPolyline {
        #[inherits(MKMultiPoint, MKShape, NSObject)]
        type Super = MKPolyline;
    }
);

#[cfg(feature = "MapKit_MKGeodesicPolyline")]
unsafe impl MKAnnotation for MKGeodesicPolyline {}

#[cfg(feature = "MapKit_MKGeodesicPolyline")]
unsafe impl MKOverlay for MKGeodesicPolyline {}

#[cfg(feature = "MapKit_MKGeodesicPolyline")]
unsafe impl NSObjectProtocol for MKGeodesicPolyline {}

extern_methods!(
    #[cfg(feature = "MapKit_MKGeodesicPolyline")]
    unsafe impl MKGeodesicPolyline {
        #[method_id(@__retain_semantics Other polylineWithPoints:count:)]
        pub unsafe fn polylineWithPoints_count(
            points: NonNull<MKMapPoint>,
            count: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other polylineWithCoordinates:count:)]
        pub unsafe fn polylineWithCoordinates_count(
            coords: NonNull<CLLocationCoordinate2D>,
            count: NSUInteger,
        ) -> Id<Self, Shared>;
    }
);
