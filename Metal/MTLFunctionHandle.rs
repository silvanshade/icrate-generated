//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;
use crate::Metal;

extern_protocol!(
    pub struct MTLFunctionHandle;

    unsafe impl ProtocolType for MTLFunctionHandle {
        #[method(functionType)]
        pub fn functionType(&self) -> Metal::MTLFunctionType;

        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<Foundation::NSString, Shared>;

        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<Metal::MTLDevice, Shared>;
    }
);
