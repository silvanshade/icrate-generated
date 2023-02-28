//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKLookAroundSceneRequest;
}

#[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
unsafe impl NSObjectProtocol for MKLookAroundSceneRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
    pub type MKLookAroundSceneRequest;

    #[objc2::method(sel = "new", managed = "New")]
    pub unsafe fn new() -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "initWithCoordinate:", managed = "Init")]
    pub unsafe fn initWithCoordinate(
        this: Option<Allocated<Self>>,
        coordinate: CLLocationCoordinate2D,
    ) -> Id<Self>;

    #[cfg(feature = "MapKit_MKMapItem")]
    #[objc2::method(sel = "initWithMapItem:", managed = "Init")]
    pub unsafe fn initWithMapItem(this: Option<Allocated<Self>>, map_item: &MKMapItem) -> Id<Self>;

    #[objc2::method(sel = "coordinate")]
    pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

    #[cfg(feature = "MapKit_MKMapItem")]
    #[objc2::method(sel = "mapItem", managed = "Other")]
    pub unsafe fn mapItem(&self) -> Option<Id<MKMapItem>>;

    #[objc2::method(sel = "isCancelled")]
    pub unsafe fn isCancelled(&self) -> bool;

    #[objc2::method(sel = "isLoading")]
    pub unsafe fn isLoading(&self) -> bool;

    #[cfg(all(feature = "Foundation_NSError", feature = "MapKit_MKLookAroundScene"))]
    #[objc2::method(sel = "getSceneWithCompletionHandler:")]
    pub unsafe fn getSceneWithCompletionHandler(
        &self,
        completion_handler: &Block<(*mut MKLookAroundScene, *mut NSError), ()>,
    );

    #[objc2::method(sel = "cancel")]
    pub unsafe fn cancel(&self);
}
