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
    #[cfg(feature = "MapKit_MKLookAroundSnapshot")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKLookAroundSnapshot;
}

#[cfg(feature = "MapKit_MKLookAroundSnapshot")]
unsafe impl NSObjectProtocol for MKLookAroundSnapshot {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKLookAroundSnapshot")]
    pub type MKLookAroundSnapshot;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "image", managed = "Other")]
    pub unsafe fn image(&self) -> Id<NSImage>;
}
