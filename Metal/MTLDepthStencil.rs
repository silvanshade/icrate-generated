//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;
use crate::Metal;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCompareFunction {
        MTLCompareFunctionNever = 0,
        MTLCompareFunctionLess = 1,
        MTLCompareFunctionEqual = 2,
        MTLCompareFunctionLessEqual = 3,
        MTLCompareFunctionGreater = 4,
        MTLCompareFunctionNotEqual = 5,
        MTLCompareFunctionGreaterEqual = 6,
        MTLCompareFunctionAlways = 7,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLStencilOperation {
        MTLStencilOperationKeep = 0,
        MTLStencilOperationZero = 1,
        MTLStencilOperationReplace = 2,
        MTLStencilOperationIncrementClamp = 3,
        MTLStencilOperationDecrementClamp = 4,
        MTLStencilOperationInvert = 5,
        MTLStencilOperationIncrementWrap = 6,
        MTLStencilOperationDecrementWrap = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStencilDescriptor;

    unsafe impl ClassType for MTLStencilDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    unsafe impl MTLStencilDescriptor {
        #[method(stencilCompareFunction)]
        pub fn stencilCompareFunction(&self) -> Metal::MTLCompareFunction;

        #[method(setStencilCompareFunction:)]
        pub fn setStencilCompareFunction(&self, stencilCompareFunction: Metal::MTLCompareFunction);

        #[method(stencilFailureOperation)]
        pub fn stencilFailureOperation(&self) -> Metal::MTLStencilOperation;

        #[method(setStencilFailureOperation:)]
        pub fn setStencilFailureOperation(
            &self,
            stencilFailureOperation: Metal::MTLStencilOperation,
        );

        #[method(depthFailureOperation)]
        pub fn depthFailureOperation(&self) -> Metal::MTLStencilOperation;

        #[method(setDepthFailureOperation:)]
        pub fn setDepthFailureOperation(&self, depthFailureOperation: Metal::MTLStencilOperation);

        #[method(depthStencilPassOperation)]
        pub fn depthStencilPassOperation(&self) -> Metal::MTLStencilOperation;

        #[method(setDepthStencilPassOperation:)]
        pub fn setDepthStencilPassOperation(
            &self,
            depthStencilPassOperation: Metal::MTLStencilOperation,
        );

        #[method(readMask)]
        pub fn readMask(&self) -> u32;

        #[method(setReadMask:)]
        pub fn setReadMask(&self, readMask: u32);

        #[method(writeMask)]
        pub fn writeMask(&self) -> u32;

        #[method(setWriteMask:)]
        pub fn setWriteMask(&self, writeMask: u32);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLDepthStencilDescriptor;

    unsafe impl ClassType for MTLDepthStencilDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    unsafe impl MTLDepthStencilDescriptor {
        #[method(depthCompareFunction)]
        pub fn depthCompareFunction(&self) -> Metal::MTLCompareFunction;

        #[method(setDepthCompareFunction:)]
        pub fn setDepthCompareFunction(&self, depthCompareFunction: Metal::MTLCompareFunction);

        #[method(isDepthWriteEnabled)]
        pub fn isDepthWriteEnabled(&self) -> bool;

        #[method(setDepthWriteEnabled:)]
        pub fn setDepthWriteEnabled(&self, depthWriteEnabled: bool);

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method_id(@__retain_semantics Other frontFaceStencil)]
        pub fn frontFaceStencil(&self) -> Id<Metal::MTLStencilDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method(setFrontFaceStencil:)]
        pub fn setFrontFaceStencil(&self, frontFaceStencil: Option<&Metal::MTLStencilDescriptor>);

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method_id(@__retain_semantics Other backFaceStencil)]
        pub fn backFaceStencil(&self) -> Id<Metal::MTLStencilDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method(setBackFaceStencil:)]
        pub fn setBackFaceStencil(&self, backFaceStencil: Option<&Metal::MTLStencilDescriptor>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&Foundation::NSString>);
    }
);

extern_protocol!(
    pub struct MTLDepthStencilState;

    unsafe impl ProtocolType for MTLDepthStencilState {
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<Metal::MTLDevice, Shared>;
    }
);
