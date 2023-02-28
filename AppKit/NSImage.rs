//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSImageName = NSString;

extern_static!(NSImageHintCTM: &'static NSImageHintKey);

extern_static!(NSImageHintInterpolation: &'static NSImageHintKey);

extern_static!(NSImageHintUserInterfaceLayoutDirection: &'static NSImageHintKey);

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSImageLoadStatus {
    NSImageLoadStatusCompleted = 0,
    NSImageLoadStatusCancelled = 1,
    NSImageLoadStatusInvalidData = 2,
    NSImageLoadStatusUnexpectedEOF = 3,
    NSImageLoadStatusReadError = 4,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSImageCacheMode {
    NSImageCacheDefault = 0,
    NSImageCacheAlways = 1,
    NSImageCacheBySize = 2,
    NSImageCacheNever = 3,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImage")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSImage;
}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSObjectProtocol for NSImage {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImage")]
    pub type NSImage;

    #[objc2::method(sel = "imageNamed:", managed = "Other")]
    pub unsafe fn imageNamed(name: &NSImageName) -> Option<Id<NSImage>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(
        sel = "imageWithSystemSymbolName:accessibilityDescription:",
        managed = "Other"
    )]
    pub unsafe fn imageWithSystemSymbolName_accessibilityDescription(
        name: &NSString,
        description: Option<&NSString>,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(
        sel = "imageWithSystemSymbolName:variableValue:accessibilityDescription:",
        managed = "Other"
    )]
    pub unsafe fn imageWithSystemSymbolName_variableValue_accessibilityDescription(
        name: &NSString,
        value: c_double,
        description: Option<&NSString>,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "imageWithSymbolName:variableValue:", managed = "Other")]
    pub unsafe fn imageWithSymbolName_variableValue(
        name: &NSString,
        value: c_double,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "imageWithSymbolName:bundle:variableValue:", managed = "Other")]
    pub unsafe fn imageWithSymbolName_bundle_variableValue(
        name: &NSString,
        bundle: Option<&NSBundle>,
        value: c_double,
    ) -> Option<Id<Self>>;

    #[objc2::method(sel = "initWithSize:", managed = "Init")]
    pub unsafe fn initWithSize(this: Option<Allocated<Self>>, size: NSSize) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "initWithData:", managed = "Init")]
    pub unsafe fn initWithData(this: Option<Allocated<Self>>, data: &NSData) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithContentsOfFile:", managed = "Init")]
    pub unsafe fn initWithContentsOfFile(
        this: Option<Allocated<Self>>,
        file_name: &NSString,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "initWithContentsOfURL:", managed = "Init")]
    pub unsafe fn initWithContentsOfURL(
        this: Option<Allocated<Self>>,
        url: &NSURL,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initByReferencingFile:", managed = "Init")]
    pub unsafe fn initByReferencingFile(
        this: Option<Allocated<Self>>,
        file_name: &NSString,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "initByReferencingURL:", managed = "Init")]
    pub unsafe fn initByReferencingURL(this: Option<Allocated<Self>>, url: &NSURL) -> Id<Self>;

    #[cfg(feature = "AppKit_NSPasteboard")]
    #[objc2::method(sel = "initWithPasteboard:", managed = "Init")]
    pub unsafe fn initWithPasteboard(
        this: Option<Allocated<Self>>,
        pasteboard: &NSPasteboard,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "initWithDataIgnoringOrientation:", managed = "Init")]
    pub unsafe fn initWithDataIgnoringOrientation(
        this: Option<Allocated<Self>>,
        data: &NSData,
    ) -> Option<Id<Self>>;

    #[objc2::method(sel = "imageWithSize:flipped:drawingHandler:", managed = "Other")]
    pub unsafe fn imageWithSize_flipped_drawingHandler(
        size: NSSize,
        drawing_handler_should_be_called_with_flipped_context: bool,
        drawing_handler: &Block<(NSRect,), Bool>,
    ) -> Id<Self>;

    #[objc2::method(sel = "size")]
    pub unsafe fn size(&self) -> NSSize;

    #[objc2::method(sel = "setSize:")]
    pub unsafe fn setSize(&self, size: NSSize);

    #[objc2::method(sel = "setName:")]
    pub unsafe fn setName(&self, string: Option<&NSImageName>) -> bool;

    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Option<Id<NSImageName>>;

    #[cfg(feature = "AppKit_NSColor")]
    #[objc2::method(sel = "backgroundColor", managed = "Other")]
    pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

    #[cfg(feature = "AppKit_NSColor")]
    #[objc2::method(sel = "setBackgroundColor:")]
    pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

    #[objc2::method(sel = "usesEPSOnResolutionMismatch")]
    pub unsafe fn usesEPSOnResolutionMismatch(&self) -> bool;

    #[objc2::method(sel = "setUsesEPSOnResolutionMismatch:")]
    pub unsafe fn setUsesEPSOnResolutionMismatch(&self, uses_eps_on_resolution_mismatch: bool);

    #[objc2::method(sel = "prefersColorMatch")]
    pub unsafe fn prefersColorMatch(&self) -> bool;

    #[objc2::method(sel = "setPrefersColorMatch:")]
    pub unsafe fn setPrefersColorMatch(&self, prefers_color_match: bool);

    #[objc2::method(sel = "matchesOnMultipleResolution")]
    pub unsafe fn matchesOnMultipleResolution(&self) -> bool;

    #[objc2::method(sel = "setMatchesOnMultipleResolution:")]
    pub unsafe fn setMatchesOnMultipleResolution(&self, matches_on_multiple_resolution: bool);

    #[objc2::method(sel = "matchesOnlyOnBestFittingAxis")]
    pub unsafe fn matchesOnlyOnBestFittingAxis(&self) -> bool;

    #[objc2::method(sel = "setMatchesOnlyOnBestFittingAxis:")]
    pub unsafe fn setMatchesOnlyOnBestFittingAxis(&self, matches_only_on_best_fitting_axis: bool);

    #[objc2::method(sel = "drawAtPoint:fromRect:operation:fraction:")]
    pub unsafe fn drawAtPoint_fromRect_operation_fraction(
        &self,
        point: NSPoint,
        from_rect: NSRect,
        op: NSCompositingOperation,
        delta: CGFloat,
    );

    #[objc2::method(sel = "drawInRect:fromRect:operation:fraction:")]
    pub unsafe fn drawInRect_fromRect_operation_fraction(
        &self,
        rect: NSRect,
        from_rect: NSRect,
        op: NSCompositingOperation,
        delta: CGFloat,
    );

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "drawInRect:fromRect:operation:fraction:respectFlipped:hints:")]
    pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
        &self,
        dst_space_portion_rect: NSRect,
        src_space_portion_rect: NSRect,
        op: NSCompositingOperation,
        requested_alpha: CGFloat,
        respect_context_is_flipped: bool,
        hints: Option<&NSDictionary<NSImageHintKey, Object>>,
    );

    #[cfg(feature = "AppKit_NSImageRep")]
    #[objc2::method(sel = "drawRepresentation:inRect:")]
    pub unsafe fn drawRepresentation_inRect(&self, image_rep: &NSImageRep, rect: NSRect) -> bool;

    #[objc2::method(sel = "drawInRect:")]
    pub unsafe fn drawInRect(&self, rect: NSRect);

    #[objc2::method(sel = "recache")]
    pub unsafe fn recache(&self);

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "TIFFRepresentation", managed = "Other")]
    pub unsafe fn TIFFRepresentation(&self) -> Option<Id<NSData>>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "TIFFRepresentationUsingCompression:factor:", managed = "Other")]
    pub unsafe fn TIFFRepresentationUsingCompression_factor(
        &self,
        comp: NSTIFFCompression,
        factor: c_float,
    ) -> Option<Id<NSData>>;

    #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "representations", managed = "Other")]
    pub unsafe fn representations(&self) -> Id<NSArray<NSImageRep>>;

    #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "addRepresentations:")]
    pub unsafe fn addRepresentations(&self, image_reps: &NSArray<NSImageRep>);

    #[cfg(feature = "AppKit_NSImageRep")]
    #[objc2::method(sel = "addRepresentation:")]
    pub unsafe fn addRepresentation(&self, image_rep: &NSImageRep);

    #[cfg(feature = "AppKit_NSImageRep")]
    #[objc2::method(sel = "removeRepresentation:")]
    pub unsafe fn removeRepresentation(&self, image_rep: &NSImageRep);

    #[objc2::method(sel = "isValid")]
    pub unsafe fn isValid(&self) -> bool;

    #[deprecated = "This method is incompatible with resolution-independent drawing and should not be used. Instead, use +[NSImage imageWithSize:flipped:drawingHandler:] to create a block-based image describing the desired image drawing, or use +[NSGraphicsContext graphicsContextWithBitmapImageRep:] to manipulate specific bitmap image representations."]
    #[objc2::method(sel = "lockFocus")]
    pub unsafe fn lockFocus(&self);

    #[deprecated = "This method is incompatible with resolution-independent drawing and should not be used. Instead, use +[NSImage imageWithSize:flipped:drawingHandler:] to create a block-based image describing the desired image drawing, or use +[NSGraphicsContext graphicsContextWithBitmapImageRep:] to manipulate specific bitmap image representations."]
    #[objc2::method(sel = "lockFocusFlipped:")]
    pub unsafe fn lockFocusFlipped(&self, flipped: bool);

    #[deprecated = "This method is incompatible with resolution-independent drawing and should not be used. Instead, use +[NSImage imageWithSize:flipped:drawingHandler:] to create a block-based image describing the desired image drawing, or use +[NSGraphicsContext graphicsContextWithBitmapImageRep:] to manipulate specific bitmap image representations."]
    #[objc2::method(sel = "unlockFocus")]
    pub unsafe fn unlockFocus(&self);

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSImageDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSImageDelegate>>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "imageTypes", managed = "Other")]
    pub unsafe fn imageTypes() -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "imageUnfilteredTypes", managed = "Other")]
    pub unsafe fn imageUnfilteredTypes() -> Id<NSArray<NSString>>;

    #[cfg(feature = "AppKit_NSPasteboard")]
    #[objc2::method(sel = "canInitWithPasteboard:")]
    pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

    #[objc2::method(sel = "cancelIncrementalLoad")]
    pub unsafe fn cancelIncrementalLoad(&self);

    #[objc2::method(sel = "cacheMode")]
    pub unsafe fn cacheMode(&self) -> NSImageCacheMode;

    #[objc2::method(sel = "setCacheMode:")]
    pub unsafe fn setCacheMode(&self, cache_mode: NSImageCacheMode);

    #[objc2::method(sel = "alignmentRect")]
    pub unsafe fn alignmentRect(&self) -> NSRect;

    #[objc2::method(sel = "setAlignmentRect:")]
    pub unsafe fn setAlignmentRect(&self, alignment_rect: NSRect);

    #[objc2::method(sel = "isTemplate")]
    pub unsafe fn isTemplate(&self) -> bool;

    #[objc2::method(sel = "setTemplate:")]
    pub unsafe fn setTemplate(&self, template: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "accessibilityDescription", managed = "Other")]
    pub unsafe fn accessibilityDescription(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setAccessibilityDescription:")]
    pub unsafe fn setAccessibilityDescription(&self, accessibility_description: Option<&NSString>);

    #[cfg(all(
        feature = "AppKit_NSGraphicsContext",
        feature = "AppKit_NSImageRep",
        feature = "Foundation_NSDictionary"
    ))]
    #[objc2::method(sel = "bestRepresentationForRect:context:hints:", managed = "Other")]
    pub unsafe fn bestRepresentationForRect_context_hints(
        &self,
        rect: NSRect,
        reference_context: Option<&NSGraphicsContext>,
        hints: Option<&NSDictionary<NSImageHintKey, Object>>,
    ) -> Option<Id<NSImageRep>>;

    #[cfg(all(
        feature = "AppKit_NSGraphicsContext",
        feature = "Foundation_NSDictionary"
    ))]
    #[objc2::method(sel = "hitTestRect:withImageDestinationRect:context:hints:flipped:")]
    pub unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped(
        &self,
        test_rect_dest_space: NSRect,
        image_rect_dest_space: NSRect,
        context: Option<&NSGraphicsContext>,
        hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        flipped: bool,
    ) -> bool;

    #[objc2::method(sel = "recommendedLayerContentsScale:")]
    pub unsafe fn recommendedLayerContentsScale(
        &self,
        preferred_contents_scale: CGFloat,
    ) -> CGFloat;

    #[objc2::method(sel = "layerContentsForContentsScale:", managed = "Other")]
    pub unsafe fn layerContentsForContentsScale(&self, layer_contents_scale: CGFloat)
        -> Id<Object>;

    #[objc2::method(sel = "capInsets")]
    pub unsafe fn capInsets(&self) -> NSEdgeInsets;

    #[objc2::method(sel = "setCapInsets:")]
    pub unsafe fn setCapInsets(&self, cap_insets: NSEdgeInsets);

    #[objc2::method(sel = "resizingMode")]
    pub unsafe fn resizingMode(&self) -> NSImageResizingMode;

    #[objc2::method(sel = "setResizingMode:")]
    pub unsafe fn setResizingMode(&self, resizing_mode: NSImageResizingMode);

    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    #[objc2::method(sel = "imageWithSymbolConfiguration:", managed = "Other")]
    pub unsafe fn imageWithSymbolConfiguration(
        &self,
        configuration: &NSImageSymbolConfiguration,
    ) -> Option<Id<NSImage>>;

    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    #[objc2::method(sel = "symbolConfiguration", managed = "Other")]
    pub unsafe fn symbolConfiguration(&self) -> Id<NSImageSymbolConfiguration>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImage")]
    pub type NSImage;
}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSPasteboardReading for NSImage {}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSPasteboardWriting for NSImage {}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSSecureCoding for NSImage {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImage")]
    pub type NSImage;
}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSItemProviderReading for NSImage {}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSItemProviderWriting for NSImage {}

#[objc2::protocol]
pub unsafe trait NSImageDelegate: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(optional, sel = "imageDidNotDraw:inRect:", managed = "Other")]
    unsafe fn imageDidNotDraw_inRect(&self, sender: &NSImage, rect: NSRect) -> Option<Id<NSImage>>;

    #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
    #[objc2::method(optional, sel = "image:willLoadRepresentation:")]
    unsafe fn image_willLoadRepresentation(&self, image: &NSImage, rep: &NSImageRep);

    #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
    #[objc2::method(optional, sel = "image:didLoadRepresentationHeader:")]
    unsafe fn image_didLoadRepresentationHeader(&self, image: &NSImage, rep: &NSImageRep);

    #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
    #[objc2::method(optional, sel = "image:didLoadPartOfRepresentation:withValidRows:")]
    unsafe fn image_didLoadPartOfRepresentation_withValidRows(
        &self,
        image: &NSImage,
        rep: &NSImageRep,
        rows: NSInteger,
    );

    #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
    #[objc2::method(optional, sel = "image:didLoadRepresentation:withStatus:")]
    unsafe fn image_didLoadRepresentation_withStatus(
        &self,
        image: &NSImage,
        rep: &NSImageRep,
        status: NSImageLoadStatus,
    );
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSBundle")]
    pub type NSBundle;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "imageForResource:", managed = "Other")]
    pub unsafe fn imageForResource(&self, name: &NSImageName) -> Option<Id<NSImage>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathForImageResource:", managed = "Other")]
    pub unsafe fn pathForImageResource(&self, name: &NSImageName) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "URLForImageResource:", managed = "Other")]
    pub unsafe fn URLForImageResource(&self, name: &NSImageName) -> Option<Id<NSURL>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImage")]
    pub type NSImage;

    #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSDictionary"))]
    #[deprecated = "Use -[NSImage bestRepresentationForRect:context:hints:] instead.  Any deviceDescription dictionary is also a valid hints dictionary."]
    #[objc2::method(sel = "bestRepresentationForDevice:", managed = "Other")]
    pub unsafe fn bestRepresentationForDevice(
        &self,
        device_description: Option<&NSDictionary>,
    ) -> Option<Id<NSImageRep>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[deprecated = "Use +imageUnfilteredTypes instead"]
    #[objc2::method(sel = "imageUnfilteredFileTypes", managed = "Other")]
    pub unsafe fn imageUnfilteredFileTypes() -> Id<NSArray<NSString>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[deprecated = "Use +imageUnfilteredTypes instead"]
    #[objc2::method(sel = "imageUnfilteredPasteboardTypes", managed = "Other")]
    pub unsafe fn imageUnfilteredPasteboardTypes() -> Id<NSArray<NSPasteboardType>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[deprecated = "Use +imageTypes instead"]
    #[objc2::method(sel = "imageFileTypes", managed = "Other")]
    pub unsafe fn imageFileTypes() -> Id<NSArray<NSString>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[deprecated = "Use +imageTypes instead"]
    #[objc2::method(sel = "imagePasteboardTypes", managed = "Other")]
    pub unsafe fn imagePasteboardTypes() -> Id<NSArray<NSPasteboardType>>;

    #[deprecated = "The concept of flippedness for NSImage is deprecated.  Please see the AppKit 10.6 release notes for a discussion of why and for how to replace existing usage."]
    #[objc2::method(sel = "setFlipped:")]
    pub unsafe fn setFlipped(&self, flag: bool);

    #[deprecated = "The concept of flippedness for NSImage is deprecated.  Please see the AppKit 10.6 release notes for a discussion of why and for how to replace existing usage."]
    #[objc2::method(sel = "isFlipped")]
    pub unsafe fn isFlipped(&self) -> bool;

    #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
    #[objc2::method(sel = "dissolveToPoint:fraction:")]
    pub unsafe fn dissolveToPoint_fraction(&self, point: NSPoint, fraction: CGFloat);

    #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
    #[objc2::method(sel = "dissolveToPoint:fromRect:fraction:")]
    pub unsafe fn dissolveToPoint_fromRect_fraction(
        &self,
        point: NSPoint,
        rect: NSRect,
        fraction: CGFloat,
    );

    #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
    #[objc2::method(sel = "compositeToPoint:operation:")]
    pub unsafe fn compositeToPoint_operation(&self, point: NSPoint, op: NSCompositingOperation);

    #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
    #[objc2::method(sel = "compositeToPoint:fromRect:operation:")]
    pub unsafe fn compositeToPoint_fromRect_operation(
        &self,
        point: NSPoint,
        rect: NSRect,
        op: NSCompositingOperation,
    );

    #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
    #[objc2::method(sel = "compositeToPoint:operation:fraction:")]
    pub unsafe fn compositeToPoint_operation_fraction(
        &self,
        point: NSPoint,
        op: NSCompositingOperation,
        delta: CGFloat,
    );

    #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
    #[objc2::method(sel = "compositeToPoint:fromRect:operation:fraction:")]
    pub unsafe fn compositeToPoint_fromRect_operation_fraction(
        &self,
        point: NSPoint,
        rect: NSRect,
        op: NSCompositingOperation,
        delta: CGFloat,
    );

    #[cfg(feature = "AppKit_NSImageRep")]
    #[deprecated = "Create an image using +[NSImage imageWithSize:flipped:drawingHandler:], and begin your custom drawing with -[NSImageRep drawInRect:] instead."]
    #[objc2::method(sel = "lockFocusOnRepresentation:")]
    pub unsafe fn lockFocusOnRepresentation(&self, image_representation: Option<&NSImageRep>);

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "setScalesWhenResized:")]
    pub unsafe fn setScalesWhenResized(&self, flag: bool);

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "scalesWhenResized")]
    pub unsafe fn scalesWhenResized(&self) -> bool;

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "setDataRetained:")]
    pub unsafe fn setDataRetained(&self, flag: bool);

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "isDataRetained")]
    pub unsafe fn isDataRetained(&self) -> bool;

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "setCachedSeparately:")]
    pub unsafe fn setCachedSeparately(&self, flag: bool);

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "isCachedSeparately")]
    pub unsafe fn isCachedSeparately(&self) -> bool;

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "setCacheDepthMatchesImageDepth:")]
    pub unsafe fn setCacheDepthMatchesImageDepth(&self, flag: bool);

    #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
    #[objc2::method(sel = "cacheDepthMatchesImageDepth")]
    pub unsafe fn cacheDepthMatchesImageDepth(&self) -> bool;
}

extern_static!(NSImageNameAddTemplate: &'static NSImageName);

extern_static!(NSImageNameBluetoothTemplate: &'static NSImageName);

extern_static!(NSImageNameBonjour: &'static NSImageName);

extern_static!(NSImageNameBookmarksTemplate: &'static NSImageName);

extern_static!(NSImageNameCaution: &'static NSImageName);

extern_static!(NSImageNameComputer: &'static NSImageName);

extern_static!(NSImageNameEnterFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameExitFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameFolder: &'static NSImageName);

extern_static!(NSImageNameFolderBurnable: &'static NSImageName);

extern_static!(NSImageNameFolderSmart: &'static NSImageName);

extern_static!(NSImageNameFollowLinkFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameHomeTemplate: &'static NSImageName);

extern_static!(NSImageNameIChatTheaterTemplate: &'static NSImageName);

extern_static!(NSImageNameLockLockedTemplate: &'static NSImageName);

extern_static!(NSImageNameLockUnlockedTemplate: &'static NSImageName);

extern_static!(NSImageNameNetwork: &'static NSImageName);

extern_static!(NSImageNamePathTemplate: &'static NSImageName);

extern_static!(NSImageNameQuickLookTemplate: &'static NSImageName);

extern_static!(NSImageNameRefreshFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameRefreshTemplate: &'static NSImageName);

extern_static!(NSImageNameRemoveTemplate: &'static NSImageName);

extern_static!(NSImageNameRevealFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameShareTemplate: &'static NSImageName);

extern_static!(NSImageNameSlideshowTemplate: &'static NSImageName);

extern_static!(NSImageNameStatusAvailable: &'static NSImageName);

extern_static!(NSImageNameStatusNone: &'static NSImageName);

extern_static!(NSImageNameStatusPartiallyAvailable: &'static NSImageName);

extern_static!(NSImageNameStatusUnavailable: &'static NSImageName);

extern_static!(NSImageNameStopProgressFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameStopProgressTemplate: &'static NSImageName);

extern_static!(NSImageNameTrashEmpty: &'static NSImageName);

extern_static!(NSImageNameTrashFull: &'static NSImageName);

extern_static!(NSImageNameActionTemplate: &'static NSImageName);

extern_static!(NSImageNameSmartBadgeTemplate: &'static NSImageName);

extern_static!(NSImageNameIconViewTemplate: &'static NSImageName);

extern_static!(NSImageNameListViewTemplate: &'static NSImageName);

extern_static!(NSImageNameColumnViewTemplate: &'static NSImageName);

extern_static!(NSImageNameFlowViewTemplate: &'static NSImageName);

extern_static!(NSImageNameInvalidDataFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameGoForwardTemplate: &'static NSImageName);

extern_static!(NSImageNameGoBackTemplate: &'static NSImageName);

extern_static!(NSImageNameGoRightTemplate: &'static NSImageName);

extern_static!(NSImageNameGoLeftTemplate: &'static NSImageName);

extern_static!(NSImageNameRightFacingTriangleTemplate: &'static NSImageName);

extern_static!(NSImageNameLeftFacingTriangleTemplate: &'static NSImageName);

extern_static!(NSImageNameDotMac: &'static NSImageName);

extern_static!(NSImageNameMobileMe: &'static NSImageName);

extern_static!(NSImageNameMultipleDocuments: &'static NSImageName);

extern_static!(NSImageNameUserAccounts: &'static NSImageName);

extern_static!(NSImageNamePreferencesGeneral: &'static NSImageName);

extern_static!(NSImageNameAdvanced: &'static NSImageName);

extern_static!(NSImageNameInfo: &'static NSImageName);

extern_static!(NSImageNameFontPanel: &'static NSImageName);

extern_static!(NSImageNameColorPanel: &'static NSImageName);

extern_static!(NSImageNameUser: &'static NSImageName);

extern_static!(NSImageNameUserGroup: &'static NSImageName);

extern_static!(NSImageNameEveryone: &'static NSImageName);

extern_static!(NSImageNameUserGuest: &'static NSImageName);

extern_static!(NSImageNameMenuOnStateTemplate: &'static NSImageName);

extern_static!(NSImageNameMenuMixedStateTemplate: &'static NSImageName);

extern_static!(NSImageNameApplicationIcon: &'static NSImageName);

extern_static!(NSImageNameTouchBarAddDetailTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAddTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAlarmTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioInputMuteTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioInputTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputMuteTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeHighTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeLowTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeMediumTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeOffTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarBookmarksTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarColorPickerFill: &'static NSImageName);

extern_static!(NSImageNameTouchBarColorPickerFont: &'static NSImageName);

extern_static!(NSImageNameTouchBarColorPickerStroke: &'static NSImageName);

extern_static!(NSImageNameTouchBarCommunicationAudioTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarCommunicationVideoTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarComposeTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarDeleteTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarDownloadTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarEnterFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarExitFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFastForwardTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFolderCopyToTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFolderMoveToTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFolderTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGetInfoTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoBackTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoDownTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoForwardTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoUpTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarHistoryTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarIconViewTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarListViewTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarMailTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarNewFolderTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarNewMessageTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarOpenInBrowserTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPauseTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPlayPauseTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPlayTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarQuickLookTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRecordStartTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRecordStopTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRefreshTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRemoveTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRewindTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRotateLeftTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRotateRightTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSearchTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarShareTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSidebarTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipAhead15SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipAhead30SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipAheadTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipBack15SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipBack30SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipBackTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipToEndTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipToStartTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSlideshowTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTagIconTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextBoldTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextBoxTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextCenterAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextItalicTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextJustifiedAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextLeftAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextListTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextRightAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextStrikethroughTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextUnderlineTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarUserAddTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarUserGroupTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarUserTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarVolumeDownTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarVolumeUpTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPlayheadTemplate: &'static NSImageName);

#[ns_enum]
#[underlying(NSInteger)]
pub enum NSImageSymbolScale {
    NSImageSymbolScaleSmall = 1,
    NSImageSymbolScaleMedium = 2,
    NSImageSymbolScaleLarge = 3,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSImageSymbolConfiguration;
}

#[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
unsafe impl NSCoding for NSImageSymbolConfiguration {}

#[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
unsafe impl NSObjectProtocol for NSImageSymbolConfiguration {}

#[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
unsafe impl NSSecureCoding for NSImageSymbolConfiguration {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    pub type NSImageSymbolConfiguration;

    #[objc2::method(sel = "configurationWithPointSize:weight:scale:", managed = "Other")]
    pub unsafe fn configurationWithPointSize_weight_scale(
        point_size: CGFloat,
        weight: NSFontWeight,
        scale: NSImageSymbolScale,
    ) -> Id<Self>;

    #[objc2::method(sel = "configurationWithPointSize:weight:", managed = "Other")]
    pub unsafe fn configurationWithPointSize_weight(
        point_size: CGFloat,
        weight: NSFontWeight,
    ) -> Id<Self>;

    #[objc2::method(sel = "configurationWithTextStyle:scale:", managed = "Other")]
    pub unsafe fn configurationWithTextStyle_scale(
        style: &NSFontTextStyle,
        scale: NSImageSymbolScale,
    ) -> Id<Self>;

    #[objc2::method(sel = "configurationWithTextStyle:", managed = "Other")]
    pub unsafe fn configurationWithTextStyle(style: &NSFontTextStyle) -> Id<Self>;

    #[objc2::method(sel = "configurationWithScale:", managed = "Other")]
    pub unsafe fn configurationWithScale(scale: NSImageSymbolScale) -> Id<Self>;

    #[objc2::method(sel = "configurationPreferringMonochrome", managed = "Other")]
    pub unsafe fn configurationPreferringMonochrome() -> Id<Self>;

    #[objc2::method(sel = "configurationPreferringHierarchical", managed = "Other")]
    pub unsafe fn configurationPreferringHierarchical() -> Id<Self>;

    #[cfg(feature = "AppKit_NSColor")]
    #[objc2::method(sel = "configurationWithHierarchicalColor:", managed = "Other")]
    pub unsafe fn configurationWithHierarchicalColor(hierarchical_color: &NSColor) -> Id<Self>;

    #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "configurationWithPaletteColors:", managed = "Other")]
    pub unsafe fn configurationWithPaletteColors(palette_colors: &NSArray<NSColor>) -> Id<Self>;

    #[objc2::method(sel = "configurationPreferringMulticolor", managed = "Other")]
    pub unsafe fn configurationPreferringMulticolor() -> Id<Self>;

    #[objc2::method(sel = "configurationByApplyingConfiguration:", managed = "Other")]
    pub unsafe fn configurationByApplyingConfiguration(
        &self,
        configuration: &NSImageSymbolConfiguration,
    ) -> Id<Self>;
}
