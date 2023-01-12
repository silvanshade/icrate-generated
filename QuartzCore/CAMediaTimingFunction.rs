//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAMediaTimingFunctionName = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
    pub struct CAMediaTimingFunction;

    #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
    unsafe impl ClassType for CAMediaTimingFunction {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
    unsafe impl CAMediaTimingFunction {
        #[method_id(@__retain_semantics Other functionWithName:)]
        pub unsafe fn functionWithName(name: &CAMediaTimingFunctionName) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other functionWithControlPoints::::)]
        pub unsafe fn functionWithControlPoints(
            c1x: c_float,
            c1y: c_float,
            c2x: c_float,
            c2y: c_float,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithControlPoints::::)]
        pub unsafe fn initWithControlPoints(
            this: Option<Allocated<Self>>,
            c1x: c_float,
            c1y: c_float,
            c2x: c_float,
            c2y: c_float,
        ) -> Id<Self, Shared>;
    }
);

extern_static!(kCAMediaTimingFunctionLinear: &'static CAMediaTimingFunctionName);

extern_static!(kCAMediaTimingFunctionEaseIn: &'static CAMediaTimingFunctionName);

extern_static!(kCAMediaTimingFunctionEaseOut: &'static CAMediaTimingFunctionName);

extern_static!(kCAMediaTimingFunctionEaseInEaseOut: &'static CAMediaTimingFunctionName);

extern_static!(kCAMediaTimingFunctionDefault: &'static CAMediaTimingFunctionName);
