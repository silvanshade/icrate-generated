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
    pub enum MKDirectionsRoutePreference {
        MKDirectionsRoutePreferenceAny = 0,
        MKDirectionsRoutePreferenceAvoid = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    pub struct MKDirectionsRequest;

    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    unsafe impl ClassType for MKDirectionsRequest {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKDirectionsRequest")]
unsafe impl NSObjectProtocol for MKDirectionsRequest {}

extern_methods!(
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    unsafe impl MKDirectionsRequest {
        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Id<MKMapItem>>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&MKMapItem>);

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other destination)]
        pub unsafe fn destination(&self) -> Option<Id<MKMapItem>>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method(setDestination:)]
        pub unsafe fn setDestination(&self, destination: Option<&MKMapItem>);
    }
);

extern_methods!(
    /// MKRequestOptions
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    unsafe impl MKDirectionsRequest {
        #[method(transportType)]
        pub unsafe fn transportType(&self) -> MKDirectionsTransportType;

        #[method(setTransportType:)]
        pub unsafe fn setTransportType(&self, transport_type: MKDirectionsTransportType);

        #[method(requestsAlternateRoutes)]
        pub unsafe fn requestsAlternateRoutes(&self) -> bool;

        #[method(setRequestsAlternateRoutes:)]
        pub unsafe fn setRequestsAlternateRoutes(&self, requests_alternate_routes: bool);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other departureDate)]
        pub unsafe fn departureDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setDepartureDate:)]
        pub unsafe fn setDepartureDate(&self, departure_date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other arrivalDate)]
        pub unsafe fn arrivalDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setArrivalDate:)]
        pub unsafe fn setArrivalDate(&self, arrival_date: Option<&NSDate>);

        #[method(tollPreference)]
        pub unsafe fn tollPreference(&self) -> MKDirectionsRoutePreference;

        #[method(setTollPreference:)]
        pub unsafe fn setTollPreference(&self, toll_preference: MKDirectionsRoutePreference);

        #[method(highwayPreference)]
        pub unsafe fn highwayPreference(&self) -> MKDirectionsRoutePreference;

        #[method(setHighwayPreference:)]
        pub unsafe fn setHighwayPreference(&self, highway_preference: MKDirectionsRoutePreference);
    }
);

extern_methods!(
    /// MKDirectionsURL
    #[cfg(feature = "MapKit_MKDirectionsRequest")]
    unsafe impl MKDirectionsRequest {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Option<Allocated<Self>>, url: &NSURL)
            -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(isDirectionsRequestURL:)]
        pub unsafe fn isDirectionsRequestURL(url: &NSURL) -> bool;
    }
);
