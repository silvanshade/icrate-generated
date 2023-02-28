//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

#[objc2::protocol]
pub unsafe trait GCDevice: NSObjectProtocol {
    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "vendorName", managed = "Other")]
    unsafe fn vendorName(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "productCategory", managed = "Other")]
    unsafe fn productCategory(&self) -> Id<NSString>;

    #[cfg(feature = "GameController_GCPhysicalInputProfile")]
    #[deprecated = "Use the physicalInputProfile property on GCController instead.  For GCKeyboard, use the keyboardInput property.  For GCMouse, use the mouseInput property."]
    #[objc2::method(sel = "physicalInputProfile", managed = "Other")]
    unsafe fn physicalInputProfile(&self) -> Id<GCPhysicalInputProfile>;
}
