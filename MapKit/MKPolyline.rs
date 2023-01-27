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
    #[cfg(feature = "MapKit_MKPolyline")]
    pub struct MKPolyline;

    #[cfg(feature = "MapKit_MKPolyline")]
    unsafe impl ClassType for MKPolyline {
        #[inherits(MKShape, NSObject)]
        type Super = MKMultiPoint;
    }
);

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl MKAnnotation for MKPolyline {}

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl MKOverlay for MKPolyline {}

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl NSObjectProtocol for MKPolyline {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPolyline")]
    unsafe impl MKPolyline {
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
