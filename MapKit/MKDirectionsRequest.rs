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
pub enum MKDirectionsRoutePreference {
    MKDirectionsRoutePreferenceAny = 0,
    MKDirectionsRoutePreferenceAvoid = 1,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKDirectionsRequest;
}

#[cfg(feature = "MapKit_MKDirectionsRequest")]
unsafe impl NSObjectProtocol for MKDirectionsRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    pub type MKDirectionsRequest;

    #[cfg(feature = "MapKit_MKMapItem")]
    #[objc2::method(sel = "source", managed = "Other")]
    pub unsafe fn source(&self) -> Option<Id<MKMapItem>>;

    #[cfg(feature = "MapKit_MKMapItem")]
    #[objc2::method(sel = "setSource:")]
    pub unsafe fn setSource(&self, source: Option<&MKMapItem>);

    #[cfg(feature = "MapKit_MKMapItem")]
    #[objc2::method(sel = "destination", managed = "Other")]
    pub unsafe fn destination(&self) -> Option<Id<MKMapItem>>;

    #[cfg(feature = "MapKit_MKMapItem")]
    #[objc2::method(sel = "setDestination:")]
    pub unsafe fn setDestination(&self, destination: Option<&MKMapItem>);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    pub type MKDirectionsRequest;

    #[objc2::method(sel = "transportType")]
    pub unsafe fn transportType(&self) -> MKDirectionsTransportType;

    #[objc2::method(sel = "setTransportType:")]
    pub unsafe fn setTransportType(&self, transport_type: MKDirectionsTransportType);

    #[objc2::method(sel = "requestsAlternateRoutes")]
    pub unsafe fn requestsAlternateRoutes(&self) -> bool;

    #[objc2::method(sel = "setRequestsAlternateRoutes:")]
    pub unsafe fn setRequestsAlternateRoutes(&self, requests_alternate_routes: bool);

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "departureDate", managed = "Other")]
    pub unsafe fn departureDate(&self) -> Option<Id<NSDate>>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "setDepartureDate:")]
    pub unsafe fn setDepartureDate(&self, departure_date: Option<&NSDate>);

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "arrivalDate", managed = "Other")]
    pub unsafe fn arrivalDate(&self) -> Option<Id<NSDate>>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "setArrivalDate:")]
    pub unsafe fn setArrivalDate(&self, arrival_date: Option<&NSDate>);

    #[objc2::method(sel = "tollPreference")]
    pub unsafe fn tollPreference(&self) -> MKDirectionsRoutePreference;

    #[objc2::method(sel = "setTollPreference:")]
    pub unsafe fn setTollPreference(&self, toll_preference: MKDirectionsRoutePreference);

    #[objc2::method(sel = "highwayPreference")]
    pub unsafe fn highwayPreference(&self) -> MKDirectionsRoutePreference;

    #[objc2::method(sel = "setHighwayPreference:")]
    pub unsafe fn setHighwayPreference(&self, highway_preference: MKDirectionsRoutePreference);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    pub type MKDirectionsRequest;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "initWithContentsOfURL:", managed = "Init")]
    pub unsafe fn initWithContentsOfURL(this: Option<Allocated<Self>>, url: &NSURL) -> Id<Self>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "isDirectionsRequestURL:")]
    pub unsafe fn isDirectionsRequestURL(url: &NSURL) -> bool;
}
