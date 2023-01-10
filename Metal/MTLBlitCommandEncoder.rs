//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;
use crate::Metal;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLBlitOption {
        MTLBlitOptionNone = 0,
        MTLBlitOptionDepthFromDepthStencil = 1 << 0,
        MTLBlitOptionStencilFromDepthStencil = 1 << 1,
        MTLBlitOptionRowLinearPVRTC = 1 << 2,
    }
);

extern_protocol!(
    pub struct MTLBlitCommandEncoder;

    unsafe impl ProtocolType for MTLBlitCommandEncoder {
        #[method(synchronizeResource:)]
        pub fn synchronizeResource(&self, resource: &Metal::MTLResource);

        #[method(synchronizeTexture:slice:level:)]
        pub unsafe fn synchronizeTexture_slice_level(
            &self,
            texture: &Metal::MTLTexture,
            slice: NSUInteger,
            level: NSUInteger,
        );

        #[method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
        pub unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
            &self,
            sourceTexture: &Metal::MTLTexture,
            sourceSlice: NSUInteger,
            sourceLevel: NSUInteger,
            sourceOrigin: Metal::MTLOrigin,
            sourceSize: Metal::MTLSize,
            destinationTexture: &Metal::MTLTexture,
            destinationSlice: NSUInteger,
            destinationLevel: NSUInteger,
            destinationOrigin: Metal::MTLOrigin,
        );

        #[method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
        pub unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
            &self,
            sourceBuffer: &Metal::MTLBuffer,
            sourceOffset: NSUInteger,
            sourceBytesPerRow: NSUInteger,
            sourceBytesPerImage: NSUInteger,
            sourceSize: Metal::MTLSize,
            destinationTexture: &Metal::MTLTexture,
            destinationSlice: NSUInteger,
            destinationLevel: NSUInteger,
            destinationOrigin: Metal::MTLOrigin,
        );

        #[method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:)]
        pub unsafe fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
            &self,
            sourceBuffer: &Metal::MTLBuffer,
            sourceOffset: NSUInteger,
            sourceBytesPerRow: NSUInteger,
            sourceBytesPerImage: NSUInteger,
            sourceSize: Metal::MTLSize,
            destinationTexture: &Metal::MTLTexture,
            destinationSlice: NSUInteger,
            destinationLevel: NSUInteger,
            destinationOrigin: Metal::MTLOrigin,
            options: Metal::MTLBlitOption,
        );

        #[method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:)]
        pub unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage(
            &self,
            sourceTexture: &Metal::MTLTexture,
            sourceSlice: NSUInteger,
            sourceLevel: NSUInteger,
            sourceOrigin: Metal::MTLOrigin,
            sourceSize: Metal::MTLSize,
            destinationBuffer: &Metal::MTLBuffer,
            destinationOffset: NSUInteger,
            destinationBytesPerRow: NSUInteger,
            destinationBytesPerImage: NSUInteger,
        );

        #[method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)]
        pub unsafe fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options(
            &self,
            sourceTexture: &Metal::MTLTexture,
            sourceSlice: NSUInteger,
            sourceLevel: NSUInteger,
            sourceOrigin: Metal::MTLOrigin,
            sourceSize: Metal::MTLSize,
            destinationBuffer: &Metal::MTLBuffer,
            destinationOffset: NSUInteger,
            destinationBytesPerRow: NSUInteger,
            destinationBytesPerImage: NSUInteger,
            options: Metal::MTLBlitOption,
        );

        #[method(generateMipmapsForTexture:)]
        pub fn generateMipmapsForTexture(&self, texture: &Metal::MTLTexture);

        #[method(fillBuffer:range:value:)]
        pub fn fillBuffer_range_value(
            &self,
            buffer: &Metal::MTLBuffer,
            range: Foundation::NSRange,
            value: u8,
        );

        #[method(copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:)]
        pub unsafe fn copyFromTexture_sourceSlice_sourceLevel_toTexture_destinationSlice_destinationLevel_sliceCount_levelCount(
            &self,
            sourceTexture: &Metal::MTLTexture,
            sourceSlice: NSUInteger,
            sourceLevel: NSUInteger,
            destinationTexture: &Metal::MTLTexture,
            destinationSlice: NSUInteger,
            destinationLevel: NSUInteger,
            sliceCount: NSUInteger,
            levelCount: NSUInteger,
        );

        #[method(copyFromTexture:toTexture:)]
        pub unsafe fn copyFromTexture_toTexture(
            &self,
            sourceTexture: &Metal::MTLTexture,
            destinationTexture: &Metal::MTLTexture,
        );

        #[method(copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:)]
        pub unsafe fn copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
            &self,
            sourceBuffer: &Metal::MTLBuffer,
            sourceOffset: NSUInteger,
            destinationBuffer: &Metal::MTLBuffer,
            destinationOffset: NSUInteger,
            size: NSUInteger,
        );

        #[method(updateFence:)]
        pub fn updateFence(&self, fence: &Metal::MTLFence);

        #[method(waitForFence:)]
        pub fn waitForFence(&self, fence: &Metal::MTLFence);

        #[optional]
        #[method(getTextureAccessCounters:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:)]
        pub unsafe fn getTextureAccessCounters_region_mipLevel_slice_resetCounters_countersBuffer_countersBufferOffset(
            &self,
            texture: &Metal::MTLTexture,
            region: Metal::MTLRegion,
            mipLevel: NSUInteger,
            slice: NSUInteger,
            resetCounters: bool,
            countersBuffer: &Metal::MTLBuffer,
            countersBufferOffset: NSUInteger,
        );

        #[optional]
        #[method(resetTextureAccessCounters:region:mipLevel:slice:)]
        pub unsafe fn resetTextureAccessCounters_region_mipLevel_slice(
            &self,
            texture: &Metal::MTLTexture,
            region: Metal::MTLRegion,
            mipLevel: NSUInteger,
            slice: NSUInteger,
        );

        #[method(optimizeContentsForGPUAccess:)]
        pub fn optimizeContentsForGPUAccess(&self, texture: &Metal::MTLTexture);

        #[method(optimizeContentsForGPUAccess:slice:level:)]
        pub unsafe fn optimizeContentsForGPUAccess_slice_level(
            &self,
            texture: &Metal::MTLTexture,
            slice: NSUInteger,
            level: NSUInteger,
        );

        #[method(optimizeContentsForCPUAccess:)]
        pub unsafe fn optimizeContentsForCPUAccess(&self, texture: &Metal::MTLTexture);

        #[method(optimizeContentsForCPUAccess:slice:level:)]
        pub unsafe fn optimizeContentsForCPUAccess_slice_level(
            &self,
            texture: &Metal::MTLTexture,
            slice: NSUInteger,
            level: NSUInteger,
        );

        #[method(resetCommandsInBuffer:withRange:)]
        pub unsafe fn resetCommandsInBuffer_withRange(
            &self,
            buffer: &Metal::MTLIndirectCommandBuffer,
            range: Foundation::NSRange,
        );

        #[method(copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:)]
        pub unsafe fn copyIndirectCommandBuffer_sourceRange_destination_destinationIndex(
            &self,
            source: &Metal::MTLIndirectCommandBuffer,
            sourceRange: Foundation::NSRange,
            destination: &Metal::MTLIndirectCommandBuffer,
            destinationIndex: NSUInteger,
        );

        #[method(optimizeIndirectCommandBuffer:withRange:)]
        pub unsafe fn optimizeIndirectCommandBuffer_withRange(
            &self,
            indirectCommandBuffer: &Metal::MTLIndirectCommandBuffer,
            range: Foundation::NSRange,
        );

        #[method(sampleCountersInBuffer:atSampleIndex:withBarrier:)]
        pub unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier(
            &self,
            sampleBuffer: &Metal::MTLCounterSampleBuffer,
            sampleIndex: NSUInteger,
            barrier: bool,
        );

        #[method(resolveCounters:inRange:destinationBuffer:destinationOffset:)]
        pub unsafe fn resolveCounters_inRange_destinationBuffer_destinationOffset(
            &self,
            sampleBuffer: &Metal::MTLCounterSampleBuffer,
            range: Foundation::NSRange,
            destinationBuffer: &Metal::MTLBuffer,
            destinationOffset: NSUInteger,
        );
    }
);
