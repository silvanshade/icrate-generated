//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

#[ns_options]
#[underlying(NSUInteger)]
pub enum MTLIndirectCommandType {
    MTLIndirectCommandTypeDraw = 1 << 0,
    MTLIndirectCommandTypeDrawIndexed = 1 << 1,
    MTLIndirectCommandTypeDrawPatches = 1 << 2,
    MTLIndirectCommandTypeDrawIndexedPatches = 1 << 3,
    MTLIndirectCommandTypeConcurrentDispatch = 1 << 5,
    MTLIndirectCommandTypeConcurrentDispatchThreads = 1 << 6,
}

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLIndirectCommandBufferExecutionRange {
        pub location: u32,
        pub length: u32,
    }
);

inline_fn!(
    pub unsafe fn MTLIndirectCommandBufferExecutionRangeMake(
        location: u32,
        length: u32,
    ) -> MTLIndirectCommandBufferExecutionRange {
        todo!()
    }
);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLIndirectCommandBufferDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLIndirectCommandBufferDescriptor;
}

#[cfg(feature = "Metal_MTLIndirectCommandBufferDescriptor")]
unsafe impl NSObjectProtocol for MTLIndirectCommandBufferDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLIndirectCommandBufferDescriptor")]
    pub type MTLIndirectCommandBufferDescriptor;

    #[objc2::method(sel = "commandTypes")]
    pub fn commandTypes(&self) -> MTLIndirectCommandType;

    #[objc2::method(sel = "setCommandTypes:")]
    pub fn setCommandTypes(&self, command_types: MTLIndirectCommandType);

    #[objc2::method(sel = "inheritPipelineState")]
    pub fn inheritPipelineState(&self) -> bool;

    #[objc2::method(sel = "setInheritPipelineState:")]
    pub fn setInheritPipelineState(&self, inherit_pipeline_state: bool);

    #[objc2::method(sel = "inheritBuffers")]
    pub fn inheritBuffers(&self) -> bool;

    #[objc2::method(sel = "setInheritBuffers:")]
    pub fn setInheritBuffers(&self, inherit_buffers: bool);

    #[objc2::method(sel = "maxVertexBufferBindCount")]
    pub fn maxVertexBufferBindCount(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaxVertexBufferBindCount:")]
    pub fn setMaxVertexBufferBindCount(&self, max_vertex_buffer_bind_count: NSUInteger);

    #[objc2::method(sel = "maxFragmentBufferBindCount")]
    pub fn maxFragmentBufferBindCount(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaxFragmentBufferBindCount:")]
    pub fn setMaxFragmentBufferBindCount(&self, max_fragment_buffer_bind_count: NSUInteger);

    #[objc2::method(sel = "maxKernelBufferBindCount")]
    pub fn maxKernelBufferBindCount(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaxKernelBufferBindCount:")]
    pub fn setMaxKernelBufferBindCount(&self, max_kernel_buffer_bind_count: NSUInteger);

    #[objc2::method(sel = "supportRayTracing")]
    pub unsafe fn supportRayTracing(&self) -> bool;

    #[objc2::method(sel = "setSupportRayTracing:")]
    pub unsafe fn setSupportRayTracing(&self, support_ray_tracing: bool);
}

#[objc2::protocol]
pub unsafe trait MTLIndirectCommandBuffer: MTLResource {
    #[objc2::method(sel = "size")]
    fn size(&self) -> NSUInteger;

    #[objc2::method(sel = "gpuResourceID")]
    unsafe fn gpuResourceID(&self) -> MTLResourceID;

    #[objc2::method(sel = "resetWithRange:")]
    unsafe fn resetWithRange(&self, range: NSRange);

    #[objc2::method(sel = "indirectRenderCommandAtIndex:", managed = "Other")]
    unsafe fn indirectRenderCommandAtIndex(
        &self,
        command_index: NSUInteger,
    ) -> Id<ProtocolObject<dyn MTLIndirectRenderCommand>>;

    #[objc2::method(sel = "indirectComputeCommandAtIndex:", managed = "Other")]
    unsafe fn indirectComputeCommandAtIndex(
        &self,
        command_index: NSUInteger,
    ) -> Id<ProtocolObject<dyn MTLIndirectComputeCommand>>;
}
