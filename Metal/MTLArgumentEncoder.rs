//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

#[objc2::protocol]
pub unsafe trait MTLArgumentEncoder: NSObjectProtocol {
    #[objc2::method(sel = "device", managed = "Other")]
    unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "label", managed = "Other")]
    unsafe fn label(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setLabel:")]
    unsafe fn setLabel(&self, label: Option<&NSString>);

    #[objc2::method(sel = "encodedLength")]
    fn encodedLength(&self) -> NSUInteger;

    #[objc2::method(sel = "alignment")]
    fn alignment(&self) -> NSUInteger;

    #[objc2::method(sel = "setArgumentBuffer:offset:")]
    unsafe fn setArgumentBuffer_offset(
        &self,
        argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        offset: NSUInteger,
    );

    #[objc2::method(sel = "setArgumentBuffer:startOffset:arrayElement:")]
    unsafe fn setArgumentBuffer_startOffset_arrayElement(
        &self,
        argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        start_offset: NSUInteger,
        array_element: NSUInteger,
    );

    #[objc2::method(sel = "setBuffer:offset:atIndex:")]
    unsafe fn setBuffer_offset_atIndex(
        &self,
        buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        offset: NSUInteger,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setBuffers:offsets:withRange:")]
    unsafe fn setBuffers_offsets_withRange(
        &self,
        buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
        offsets: NonNull<NSUInteger>,
        range: NSRange,
    );

    #[objc2::method(sel = "setTexture:atIndex:")]
    unsafe fn setTexture_atIndex(
        &self,
        texture: Option<&ProtocolObject<dyn MTLTexture>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setTextures:withRange:")]
    unsafe fn setTextures_withRange(
        &self,
        textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
        range: NSRange,
    );

    #[objc2::method(sel = "setSamplerState:atIndex:")]
    unsafe fn setSamplerState_atIndex(
        &self,
        sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setSamplerStates:withRange:")]
    unsafe fn setSamplerStates_withRange(
        &self,
        samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
        range: NSRange,
    );

    #[objc2::method(sel = "constantDataAtIndex:")]
    unsafe fn constantDataAtIndex(&self, index: NSUInteger) -> NonNull<c_void>;

    #[objc2::method(sel = "setRenderPipelineState:atIndex:")]
    unsafe fn setRenderPipelineState_atIndex(
        &self,
        pipeline: Option<&ProtocolObject<dyn MTLRenderPipelineState>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setRenderPipelineStates:withRange:")]
    unsafe fn setRenderPipelineStates_withRange(
        &self,
        pipelines: NonNull<*const ProtocolObject<dyn MTLRenderPipelineState>>,
        range: NSRange,
    );

    #[objc2::method(sel = "setComputePipelineState:atIndex:")]
    unsafe fn setComputePipelineState_atIndex(
        &self,
        pipeline: Option<&ProtocolObject<dyn MTLComputePipelineState>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setComputePipelineStates:withRange:")]
    unsafe fn setComputePipelineStates_withRange(
        &self,
        pipelines: NonNull<*const ProtocolObject<dyn MTLComputePipelineState>>,
        range: NSRange,
    );

    #[objc2::method(sel = "setIndirectCommandBuffer:atIndex:")]
    unsafe fn setIndirectCommandBuffer_atIndex(
        &self,
        indirect_command_buffer: Option<&ProtocolObject<dyn MTLIndirectCommandBuffer>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setIndirectCommandBuffers:withRange:")]
    unsafe fn setIndirectCommandBuffers_withRange(
        &self,
        buffers: NonNull<*const ProtocolObject<dyn MTLIndirectCommandBuffer>>,
        range: NSRange,
    );

    #[objc2::method(sel = "setAccelerationStructure:atIndex:")]
    unsafe fn setAccelerationStructure_atIndex(
        &self,
        acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "newArgumentEncoderForBufferAtIndex:", managed = "New")]
    unsafe fn newArgumentEncoderForBufferAtIndex(
        &self,
        index: NSUInteger,
    ) -> Option<Id<ProtocolObject<dyn MTLArgumentEncoder>>>;

    #[objc2::method(sel = "setVisibleFunctionTable:atIndex:")]
    unsafe fn setVisibleFunctionTable_atIndex(
        &self,
        visible_function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setVisibleFunctionTables:withRange:")]
    unsafe fn setVisibleFunctionTables_withRange(
        &self,
        visible_function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
        range: NSRange,
    );

    #[objc2::method(sel = "setIntersectionFunctionTable:atIndex:")]
    unsafe fn setIntersectionFunctionTable_atIndex(
        &self,
        intersection_function_table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "setIntersectionFunctionTables:withRange:")]
    unsafe fn setIntersectionFunctionTables_withRange(
        &self,
        intersection_function_tables: NonNull<
            *const ProtocolObject<dyn MTLIntersectionFunctionTable>,
        >,
        range: NSRange,
    );
}
