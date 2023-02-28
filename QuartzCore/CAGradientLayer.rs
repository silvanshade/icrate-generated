//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAGradientLayerType = NSString;
);

#[objc2::interface(
    unsafe super = CALayer,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CAGradientLayer;
}

#[cfg(feature = "CoreAnimation_CAGradientLayer")]
unsafe impl CAMediaTiming for CAGradientLayer {}

#[cfg(feature = "CoreAnimation_CAGradientLayer")]
unsafe impl NSCoding for CAGradientLayer {}

#[cfg(feature = "CoreAnimation_CAGradientLayer")]
unsafe impl NSObjectProtocol for CAGradientLayer {}

#[cfg(feature = "CoreAnimation_CAGradientLayer")]
unsafe impl NSSecureCoding for CAGradientLayer {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    pub type CAGradientLayer;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "colors", managed = "Other")]
    pub unsafe fn colors(&self) -> Option<Id<NSArray>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setColors:")]
    pub unsafe fn setColors(&self, colors: Option<&NSArray>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
    #[objc2::method(sel = "locations", managed = "Other")]
    pub unsafe fn locations(&self) -> Option<Id<NSArray<NSNumber>>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
    #[objc2::method(sel = "setLocations:")]
    pub unsafe fn setLocations(&self, locations: Option<&NSArray<NSNumber>>);

    #[objc2::method(sel = "startPoint")]
    pub unsafe fn startPoint(&self) -> CGPoint;

    #[objc2::method(sel = "setStartPoint:")]
    pub unsafe fn setStartPoint(&self, start_point: CGPoint);

    #[objc2::method(sel = "endPoint")]
    pub unsafe fn endPoint(&self) -> CGPoint;

    #[objc2::method(sel = "setEndPoint:")]
    pub unsafe fn setEndPoint(&self, end_point: CGPoint);

    #[objc2::method(sel = "type", managed = "Other")]
    pub unsafe fn r#type(&self) -> Id<CAGradientLayerType>;

    #[objc2::method(sel = "setType:")]
    pub unsafe fn setType(&self, r#type: &CAGradientLayerType);
}

extern_static!(kCAGradientLayerAxial: &'static CAGradientLayerType);

extern_static!(kCAGradientLayerRadial: &'static CAGradientLayerType);

extern_static!(kCAGradientLayerConic: &'static CAGradientLayerType);

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    pub type CAGradientLayer;

    #[objc2::method(sel = "layer", managed = "Other")]
    pub unsafe fn layer() -> Id<Self>;

    #[objc2::method(sel = "initWithLayer:", managed = "Init")]
    pub unsafe fn initWithLayer(this: Option<Allocated<Self>>, layer: &Object) -> Id<Self>;
}
