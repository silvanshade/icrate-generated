//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

#[ns_enum]
#[underlying(NSUInteger)]
pub enum MTLSamplerMinMagFilter {
    MTLSamplerMinMagFilterNearest = 0,
    MTLSamplerMinMagFilterLinear = 1,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum MTLSamplerMipFilter {
    MTLSamplerMipFilterNotMipmapped = 0,
    MTLSamplerMipFilterNearest = 1,
    MTLSamplerMipFilterLinear = 2,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum MTLSamplerAddressMode {
    MTLSamplerAddressModeClampToEdge = 0,
    MTLSamplerAddressModeMirrorClampToEdge = 1,
    MTLSamplerAddressModeRepeat = 2,
    MTLSamplerAddressModeMirrorRepeat = 3,
    MTLSamplerAddressModeClampToZero = 4,
    MTLSamplerAddressModeClampToBorderColor = 5,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum MTLSamplerBorderColor {
    MTLSamplerBorderColorTransparentBlack = 0,
    MTLSamplerBorderColorOpaqueBlack = 1,
    MTLSamplerBorderColorOpaqueWhite = 2,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLSamplerDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLSamplerDescriptor;
}

#[cfg(feature = "Metal_MTLSamplerDescriptor")]
unsafe impl NSObjectProtocol for MTLSamplerDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLSamplerDescriptor")]
    pub type MTLSamplerDescriptor;

    #[objc2::method(sel = "minFilter")]
    pub fn minFilter(&self) -> MTLSamplerMinMagFilter;

    #[objc2::method(sel = "setMinFilter:")]
    pub fn setMinFilter(&self, min_filter: MTLSamplerMinMagFilter);

    #[objc2::method(sel = "magFilter")]
    pub fn magFilter(&self) -> MTLSamplerMinMagFilter;

    #[objc2::method(sel = "setMagFilter:")]
    pub fn setMagFilter(&self, mag_filter: MTLSamplerMinMagFilter);

    #[objc2::method(sel = "mipFilter")]
    pub fn mipFilter(&self) -> MTLSamplerMipFilter;

    #[objc2::method(sel = "setMipFilter:")]
    pub fn setMipFilter(&self, mip_filter: MTLSamplerMipFilter);

    #[objc2::method(sel = "maxAnisotropy")]
    pub fn maxAnisotropy(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaxAnisotropy:")]
    pub fn setMaxAnisotropy(&self, max_anisotropy: NSUInteger);

    #[objc2::method(sel = "sAddressMode")]
    pub fn sAddressMode(&self) -> MTLSamplerAddressMode;

    #[objc2::method(sel = "setSAddressMode:")]
    pub fn setSAddressMode(&self, s_address_mode: MTLSamplerAddressMode);

    #[objc2::method(sel = "tAddressMode")]
    pub fn tAddressMode(&self) -> MTLSamplerAddressMode;

    #[objc2::method(sel = "setTAddressMode:")]
    pub fn setTAddressMode(&self, t_address_mode: MTLSamplerAddressMode);

    #[objc2::method(sel = "rAddressMode")]
    pub fn rAddressMode(&self) -> MTLSamplerAddressMode;

    #[objc2::method(sel = "setRAddressMode:")]
    pub fn setRAddressMode(&self, r_address_mode: MTLSamplerAddressMode);

    #[objc2::method(sel = "borderColor")]
    pub fn borderColor(&self) -> MTLSamplerBorderColor;

    #[objc2::method(sel = "setBorderColor:")]
    pub fn setBorderColor(&self, border_color: MTLSamplerBorderColor);

    #[objc2::method(sel = "normalizedCoordinates")]
    pub fn normalizedCoordinates(&self) -> bool;

    #[objc2::method(sel = "setNormalizedCoordinates:")]
    pub fn setNormalizedCoordinates(&self, normalized_coordinates: bool);

    #[objc2::method(sel = "lodMinClamp")]
    pub fn lodMinClamp(&self) -> c_float;

    #[objc2::method(sel = "setLodMinClamp:")]
    pub fn setLodMinClamp(&self, lod_min_clamp: c_float);

    #[objc2::method(sel = "lodMaxClamp")]
    pub fn lodMaxClamp(&self) -> c_float;

    #[objc2::method(sel = "setLodMaxClamp:")]
    pub fn setLodMaxClamp(&self, lod_max_clamp: c_float);

    #[objc2::method(sel = "lodAverage")]
    pub fn lodAverage(&self) -> bool;

    #[objc2::method(sel = "setLodAverage:")]
    pub fn setLodAverage(&self, lod_average: bool);

    #[objc2::method(sel = "compareFunction")]
    pub fn compareFunction(&self) -> MTLCompareFunction;

    #[objc2::method(sel = "setCompareFunction:")]
    pub fn setCompareFunction(&self, compare_function: MTLCompareFunction);

    #[objc2::method(sel = "supportArgumentBuffers")]
    pub fn supportArgumentBuffers(&self) -> bool;

    #[objc2::method(sel = "setSupportArgumentBuffers:")]
    pub fn setSupportArgumentBuffers(&self, support_argument_buffers: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "label", managed = "Other")]
    pub fn label(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setLabel:")]
    pub fn setLabel(&self, label: Option<&NSString>);
}

#[objc2::protocol]
pub unsafe trait MTLSamplerState: NSObjectProtocol {
    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "label", managed = "Other")]
    fn label(&self) -> Option<Id<NSString>>;

    #[objc2::method(sel = "device", managed = "Other")]
    fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

    #[objc2::method(sel = "gpuResourceID")]
    unsafe fn gpuResourceID(&self) -> MTLResourceID;
}
