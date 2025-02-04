//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMapSnapshot")]
    pub struct MKMapSnapshot;

    #[cfg(feature = "MapKit_MKMapSnapshot")]
    unsafe impl ClassType for MKMapSnapshot {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKMapSnapshot")]
unsafe impl NSObjectProtocol for MKMapSnapshot {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMapSnapshot")]
    unsafe impl MKMapSnapshot {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;

        #[cfg(feature = "AppKit_NSAppearance")]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Id<NSAppearance>;

        #[method(pointForCoordinate:)]
        pub unsafe fn pointForCoordinate(&self, coordinate: CLLocationCoordinate2D) -> NSPoint;
    }
);
