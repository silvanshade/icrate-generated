//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLComputePassSampleBufferAttachmentDescriptor;
}

#[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    pub type MTLComputePassSampleBufferAttachmentDescriptor;

    #[objc2::method(sel = "sampleBuffer", managed = "Other")]
    pub unsafe fn sampleBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

    #[objc2::method(sel = "setSampleBuffer:")]
    pub unsafe fn setSampleBuffer(
        &self,
        sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
    );

    #[objc2::method(sel = "startOfEncoderSampleIndex")]
    pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

    #[objc2::method(sel = "setStartOfEncoderSampleIndex:")]
    pub unsafe fn setStartOfEncoderSampleIndex(&self, start_of_encoder_sample_index: NSUInteger);

    #[objc2::method(sel = "endOfEncoderSampleIndex")]
    pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

    #[objc2::method(sel = "setEndOfEncoderSampleIndex:")]
    pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLComputePassSampleBufferAttachmentDescriptorArray;
}

#[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptorArray {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    pub type MTLComputePassSampleBufferAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    #[objc2::method(sel = "objectAtIndexedSubscript:", managed = "Other")]
    pub unsafe fn objectAtIndexedSubscript(
        &self,
        attachment_index: NSUInteger,
    ) -> Id<MTLComputePassSampleBufferAttachmentDescriptor>;

    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    #[objc2::method(sel = "setObject:atIndexedSubscript:")]
    pub unsafe fn setObject_atIndexedSubscript(
        &self,
        attachment: Option<&MTLComputePassSampleBufferAttachmentDescriptor>,
        attachment_index: NSUInteger,
    );
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLComputePassDescriptor;
}

#[cfg(feature = "Metal_MTLComputePassDescriptor")]
unsafe impl NSObjectProtocol for MTLComputePassDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    pub type MTLComputePassDescriptor;

    #[objc2::method(sel = "computePassDescriptor", managed = "Other")]
    pub unsafe fn computePassDescriptor() -> Id<MTLComputePassDescriptor>;

    #[objc2::method(sel = "dispatchType")]
    pub unsafe fn dispatchType(&self) -> MTLDispatchType;

    #[objc2::method(sel = "setDispatchType:")]
    pub unsafe fn setDispatchType(&self, dispatch_type: MTLDispatchType);

    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    #[objc2::method(sel = "sampleBufferAttachments", managed = "Other")]
    pub unsafe fn sampleBufferAttachments(
        &self,
    ) -> Id<MTLComputePassSampleBufferAttachmentDescriptorArray>;
}
