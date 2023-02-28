//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[objc2::interface(
    unsafe super = MKShape,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKMultiPolygon;
}

#[cfg(feature = "MapKit_MKMultiPolygon")]
unsafe impl MKAnnotation for MKMultiPolygon {}

#[cfg(feature = "MapKit_MKMultiPolygon")]
unsafe impl MKOverlay for MKMultiPolygon {}

#[cfg(feature = "MapKit_MKMultiPolygon")]
unsafe impl NSObjectProtocol for MKMultiPolygon {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    pub type MKMultiPolygon;

    #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKPolygon"))]
    #[objc2::method(sel = "initWithPolygons:", managed = "Init")]
    pub unsafe fn initWithPolygons(
        this: Option<Allocated<Self>>,
        polygons: &NSArray<MKPolygon>,
    ) -> Id<Self>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKPolygon"))]
    #[objc2::method(sel = "polygons", managed = "Other")]
    pub unsafe fn polygons(&self) -> Id<NSArray<MKPolygon>>;
}
