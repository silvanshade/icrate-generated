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
    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    pub struct MKPointOfInterestFilter;

    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    unsafe impl ClassType for MKPointOfInterestFilter {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
unsafe impl NSCoding for MKPointOfInterestFilter {}

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
unsafe impl NSObjectProtocol for MKPointOfInterestFilter {}

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
unsafe impl NSSecureCoding for MKPointOfInterestFilter {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    unsafe impl MKPointOfInterestFilter {
        #[method_id(@__retain_semantics Other filterIncludingAllCategories)]
        pub unsafe fn filterIncludingAllCategories() -> Id<MKPointOfInterestFilter, Shared>;

        #[method_id(@__retain_semantics Other filterExcludingAllCategories)]
        pub unsafe fn filterExcludingAllCategories() -> Id<MKPointOfInterestFilter, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initIncludingCategories:)]
        pub unsafe fn initIncludingCategories(
            this: Option<Allocated<Self>>,
            categories: &NSArray<MKPointOfInterestCategory>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initExcludingCategories:)]
        pub unsafe fn initExcludingCategories(
            this: Option<Allocated<Self>>,
            categories: &NSArray<MKPointOfInterestCategory>,
        ) -> Id<Self, Shared>;

        #[method(includesCategory:)]
        pub unsafe fn includesCategory(&self, category: &MKPointOfInterestCategory) -> bool;

        #[method(excludesCategory:)]
        pub unsafe fn excludesCategory(&self, category: &MKPointOfInterestCategory) -> bool;
    }
);
