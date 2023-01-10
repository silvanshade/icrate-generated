//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTIFFCompression {
        NSTIFFCompressionNone = 1,
        NSTIFFCompressionCCITTFAX3 = 3,
        NSTIFFCompressionCCITTFAX4 = 4,
        NSTIFFCompressionLZW = 5,
        NSTIFFCompressionJPEG = 6,
        NSTIFFCompressionNEXT = 32766,
        NSTIFFCompressionPackBits = 32773,
        NSTIFFCompressionOldJPEG = 32865,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBitmapImageFileType {
        NSBitmapImageFileTypeTIFF = 0,
        NSBitmapImageFileTypeBMP = 1,
        NSBitmapImageFileTypeGIF = 2,
        NSBitmapImageFileTypeJPEG = 3,
        NSBitmapImageFileTypePNG = 4,
        NSBitmapImageFileTypeJPEG2000 = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSImageRepLoadStatus {
        NSImageRepLoadStatusUnknownType = -1,
        NSImageRepLoadStatusReadingHeader = -2,
        NSImageRepLoadStatusWillNeedAllData = -3,
        NSImageRepLoadStatusInvalidData = -4,
        NSImageRepLoadStatusUnexpectedEOF = -5,
        NSImageRepLoadStatusCompleted = -6,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSBitmapFormat {
        NSBitmapFormatAlphaFirst = 1 << 0,
        NSBitmapFormatAlphaNonpremultiplied = 1 << 1,
        NSBitmapFormatFloatingPointSamples = 1 << 2,
        NSBitmapFormatSixteenBitLittleEndian = 1 << 8,
        NSBitmapFormatThirtyTwoBitLittleEndian = 1 << 9,
        NSBitmapFormatSixteenBitBigEndian = 1 << 10,
        NSBitmapFormatThirtyTwoBitBigEndian = 1 << 11,
    }
);

typed_extensible_enum!(
    pub type NSBitmapImageRepPropertyKey = Foundation::NSString;
);

extern_static!(NSImageCompressionMethod: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageCompressionFactor: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageDitherTransparency: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageRGBColorTable: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageInterlaced: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageColorSyncProfileData: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageFrameCount: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageCurrentFrame: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageCurrentFrameDuration: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageLoopCount: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageGamma: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageProgressive: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageEXIFData: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageIPTCData: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_static!(NSImageFallbackBackgroundColor: &'static AppKit::NSBitmapImageRepPropertyKey);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBitmapImageRep;

    unsafe impl ClassType for NSBitmapImageRep {
        #[inherits(NSObject)]
        type Super = AppKit::NSImageRep;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl NSBitmapImageRep {
        #[method_id(@__retain_semantics Init initWithFocusedViewRect:)]
        pub unsafe fn initWithFocusedViewRect(
            this: Option<Allocated<Self>>,
            rect: Foundation::NSRect,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            this: Option<Allocated<Self>>,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            isPlanar: bool,
            colorSpaceName: &AppKit::NSColorSpaceName,
            rBytes: NSInteger,
            pBits: NSInteger,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bitmapFormat:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bitmapFormat_bytesPerRow_bitsPerPixel(
            this: Option<Allocated<Self>>,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            isPlanar: bool,
            colorSpaceName: &AppKit::NSColorSpaceName,
            bitmapFormat: AppKit::NSBitmapFormat,
            rBytes: NSInteger,
            pBits: NSInteger,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other imageRepsWithData:)]
        pub unsafe fn imageRepsWithData(
            data: &Foundation::NSData,
        ) -> Id<Foundation::NSArray<AppKit::NSImageRep>, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(data: &Foundation::NSData) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method(bitmapData)]
        pub unsafe fn bitmapData(&self) -> *mut c_uchar;

        #[method(getBitmapDataPlanes:)]
        pub unsafe fn getBitmapDataPlanes(&self, data: NonNull<*mut c_uchar>);

        #[method(isPlanar)]
        pub unsafe fn isPlanar(&self) -> bool;

        #[method(samplesPerPixel)]
        pub unsafe fn samplesPerPixel(&self) -> NSInteger;

        #[method(bitsPerPixel)]
        pub unsafe fn bitsPerPixel(&self) -> NSInteger;

        #[method(bytesPerRow)]
        pub unsafe fn bytesPerRow(&self) -> NSInteger;

        #[method(bytesPerPlane)]
        pub unsafe fn bytesPerPlane(&self) -> NSInteger;

        #[method(numberOfPlanes)]
        pub unsafe fn numberOfPlanes(&self) -> NSInteger;

        #[method(bitmapFormat)]
        pub unsafe fn bitmapFormat(&self) -> AppKit::NSBitmapFormat;

        #[method(getCompression:factor:)]
        pub unsafe fn getCompression_factor(
            &self,
            compression: *mut AppKit::NSTIFFCompression,
            factor: *mut c_float,
        );

        #[method(setCompression:factor:)]
        pub unsafe fn setCompression_factor(
            &self,
            compression: AppKit::NSTIFFCompression,
            factor: c_float,
        );

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: AppKit::NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other TIFFRepresentationOfImageRepsInArray:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray(
            array: &Foundation::NSArray<AppKit::NSImageRep>,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other TIFFRepresentationOfImageRepsInArray:usingCompression:factor:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray_usingCompression_factor(
            array: &Foundation::NSArray<AppKit::NSImageRep>,
            comp: AppKit::NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[method(getTIFFCompressionTypes:count:)]
        pub unsafe fn getTIFFCompressionTypes_count(
            list: NonNull<*mut AppKit::NSTIFFCompression>,
            numTypes: NonNull<NSInteger>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedNameForTIFFCompressionType:)]
        pub unsafe fn localizedNameForTIFFCompressionType(
            compression: AppKit::NSTIFFCompression,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[method(canBeCompressedUsing:)]
        pub unsafe fn canBeCompressedUsing(&self, compression: AppKit::NSTIFFCompression) -> bool;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(colorizeByMappingGray:toColor:blackMapping:whiteMapping:)]
        pub unsafe fn colorizeByMappingGray_toColor_blackMapping_whiteMapping(
            &self,
            midPoint: Foundation::CGFloat,
            midPointColor: Option<&AppKit::NSColor>,
            shadowColor: Option<&AppKit::NSColor>,
            lightColor: Option<&AppKit::NSColor>,
        );

        #[method_id(@__retain_semantics Init initForIncrementalLoad)]
        pub unsafe fn initForIncrementalLoad(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(incrementalLoadFromData:complete:)]
        pub unsafe fn incrementalLoadFromData_complete(
            &self,
            data: &Foundation::NSData,
            complete: bool,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:atX:y:)]
        pub unsafe fn setColor_atX_y(&self, color: &AppKit::NSColor, x: NSInteger, y: NSInteger);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other colorAtX:y:)]
        pub unsafe fn colorAtX_y(
            &self,
            x: NSInteger,
            y: NSInteger,
        ) -> Option<Id<AppKit::NSColor, Shared>>;

        #[method(getPixel:atX:y:)]
        pub unsafe fn getPixel_atX_y(&self, p: NonNull<NSUInteger>, x: NSInteger, y: NSInteger);

        #[method(setPixel:atX:y:)]
        pub unsafe fn setPixel_atX_y(&self, p: NonNull<NSUInteger>, x: NSInteger, y: NSInteger);

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<AppKit::NSColorSpace, Shared>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other bitmapImageRepByConvertingToColorSpace:renderingIntent:)]
        pub unsafe fn bitmapImageRepByConvertingToColorSpace_renderingIntent(
            &self,
            targetSpace: &AppKit::NSColorSpace,
            renderingIntent: AppKit::NSColorRenderingIntent,
        ) -> Option<Id<AppKit::NSBitmapImageRep, Shared>>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other bitmapImageRepByRetaggingWithColorSpace:)]
        pub unsafe fn bitmapImageRepByRetaggingWithColorSpace(
            &self,
            newSpace: &AppKit::NSColorSpace,
        ) -> Option<Id<AppKit::NSBitmapImageRep, Shared>>;
    }
);

extern_methods!(
    /// NSBitmapImageFileTypeExtensions
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl NSBitmapImageRep {
        #[cfg(all(
            feature = "AppKit_NSImageRep",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other representationOfImageRepsInArray:usingType:properties:)]
        pub unsafe fn representationOfImageRepsInArray_usingType_properties(
            imageReps: &Foundation::NSArray<AppKit::NSImageRep>,
            storageType: AppKit::NSBitmapImageFileType,
            properties: &Foundation::NSDictionary<AppKit::NSBitmapImageRepPropertyKey, Object>,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other representationUsingType:properties:)]
        pub unsafe fn representationUsingType_properties(
            &self,
            storageType: AppKit::NSBitmapImageFileType,
            properties: &Foundation::NSDictionary<AppKit::NSBitmapImageRepPropertyKey, Object>,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[method(setProperty:withValue:)]
        pub unsafe fn setProperty_withValue(
            &self,
            property: &AppKit::NSBitmapImageRepPropertyKey,
            value: Option<&Object>,
        );

        #[method_id(@__retain_semantics Other valueForProperty:)]
        pub unsafe fn valueForProperty(
            &self,
            property: &AppKit::NSBitmapImageRepPropertyKey,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_static!(NSTIFFFileType: AppKit::NSBitmapImageFileType = NSBitmapImageFileTypeTIFF);

extern_static!(NSBMPFileType: AppKit::NSBitmapImageFileType = NSBitmapImageFileTypeBMP);

extern_static!(NSGIFFileType: AppKit::NSBitmapImageFileType = NSBitmapImageFileTypeGIF);

extern_static!(NSJPEGFileType: AppKit::NSBitmapImageFileType = NSBitmapImageFileTypeJPEG);

extern_static!(NSPNGFileType: AppKit::NSBitmapImageFileType = NSBitmapImageFileTypePNG);

extern_static!(NSJPEG2000FileType: AppKit::NSBitmapImageFileType = NSBitmapImageFileTypeJPEG2000);

extern_static!(NSAlphaFirstBitmapFormat: AppKit::NSBitmapFormat = NSBitmapFormatAlphaFirst);

extern_static!(
    NSAlphaNonpremultipliedBitmapFormat: AppKit::NSBitmapFormat =
        NSBitmapFormatAlphaNonpremultiplied
);

extern_static!(
    NSFloatingPointSamplesBitmapFormat: AppKit::NSBitmapFormat = NSBitmapFormatFloatingPointSamples
);

extern_static!(
    NS16BitLittleEndianBitmapFormat: AppKit::NSBitmapFormat = NSBitmapFormatSixteenBitLittleEndian
);

extern_static!(
    NS32BitLittleEndianBitmapFormat: AppKit::NSBitmapFormat =
        NSBitmapFormatThirtyTwoBitLittleEndian
);

extern_static!(
    NS16BitBigEndianBitmapFormat: AppKit::NSBitmapFormat = NSBitmapFormatSixteenBitBigEndian
);

extern_static!(
    NS32BitBigEndianBitmapFormat: AppKit::NSBitmapFormat = NSBitmapFormatThirtyTwoBitBigEndian
);
