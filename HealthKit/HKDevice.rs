//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(HKDevicePropertyKeyName: &'static NSString);

extern_static!(HKDevicePropertyKeyManufacturer: &'static NSString);

extern_static!(HKDevicePropertyKeyModel: &'static NSString);

extern_static!(HKDevicePropertyKeyHardwareVersion: &'static NSString);

extern_static!(HKDevicePropertyKeyFirmwareVersion: &'static NSString);

extern_static!(HKDevicePropertyKeySoftwareVersion: &'static NSString);

extern_static!(HKDevicePropertyKeyLocalIdentifier: &'static NSString);

extern_static!(HKDevicePropertyKeyUDIDeviceIdentifier: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKDevice")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKDevice;
}

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSCoding for HKDevice {}

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSObjectProtocol for HKDevice {}

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSSecureCoding for HKDevice {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKDevice")]
    pub type HKDevice;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "manufacturer", managed = "Other")]
    pub unsafe fn manufacturer(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "model", managed = "Other")]
    pub unsafe fn model(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "hardwareVersion", managed = "Other")]
    pub unsafe fn hardwareVersion(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "firmwareVersion", managed = "Other")]
    pub unsafe fn firmwareVersion(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "softwareVersion", managed = "Other")]
    pub unsafe fn softwareVersion(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localIdentifier", managed = "Other")]
    pub unsafe fn localIdentifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "UDIDeviceIdentifier", managed = "Other")]
    pub unsafe fn UDIDeviceIdentifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(
        sel = "initWithName:manufacturer:model:hardwareVersion:firmwareVersion:softwareVersion:localIdentifier:UDIDeviceIdentifier:",
        managed = "Init"
    )]
    pub unsafe fn initWithName_manufacturer_model_hardwareVersion_firmwareVersion_softwareVersion_localIdentifier_UDIDeviceIdentifier(
        this: Option<Allocated<Self>>,
        name: Option<&NSString>,
        manufacturer: Option<&NSString>,
        model: Option<&NSString>,
        hardware_version: Option<&NSString>,
        firmware_version: Option<&NSString>,
        software_version: Option<&NSString>,
        local_identifier: Option<&NSString>,
        udi_device_identifier: Option<&NSString>,
    ) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "localDevice", managed = "Other")]
    pub unsafe fn localDevice() -> Id<HKDevice>;
}
