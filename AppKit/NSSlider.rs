//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSlider;

    unsafe impl ClassType for NSSlider {
        #[inherits(AppKit::NSView, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSControl;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method(sliderType)]
        pub unsafe fn sliderType(&self) -> AppKit::NSSliderType;

        #[method(setSliderType:)]
        pub unsafe fn setSliderType(&self, sliderType: AppKit::NSSliderType);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);

        #[method(altIncrementValue)]
        pub unsafe fn altIncrementValue(&self) -> c_double;

        #[method(setAltIncrementValue:)]
        pub unsafe fn setAltIncrementValue(&self, altIncrementValue: c_double);

        #[method(knobThickness)]
        pub unsafe fn knobThickness(&self) -> CoreGraphics::CGFloat;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(acceptsFirstMouse:)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&AppKit::NSEvent>) -> bool;

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other trackFillColor)]
        pub unsafe fn trackFillColor(&self) -> Option<Id<AppKit::NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTrackFillColor:)]
        pub unsafe fn setTrackFillColor(&self, trackFillColor: Option<&AppKit::NSColor>);
    }
);

extern_methods!(
    /// NSSliderVerticalGetter
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {}
);

extern_methods!(
    /// NSTickMarkSupport
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, numberOfTickMarks: NSInteger);

        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> AppKit::NSTickMarkPosition;

        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tickMarkPosition: AppKit::NSTickMarkPosition);

        #[method(allowsTickMarkValuesOnly)]
        pub unsafe fn allowsTickMarkValuesOnly(&self) -> bool;

        #[method(setAllowsTickMarkValuesOnly:)]
        pub unsafe fn setAllowsTickMarkValuesOnly(&self, allowsTickMarkValuesOnly: bool);

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;

        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> Foundation::NSRect;

        #[method(indexOfTickMarkAtPoint:)]
        pub unsafe fn indexOfTickMarkAtPoint(&self, point: Foundation::NSPoint) -> NSInteger;

        #[method(closestTickMarkValueToValue:)]
        pub unsafe fn closestTickMarkValueToValue(&self, value: c_double) -> c_double;
    }
);

extern_methods!(
    /// NSSliderConvenience
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method_id(@__retain_semantics Other sliderWithTarget:action:)]
        pub unsafe fn sliderWithTarget_action(
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other sliderWithValue:minValue:maxValue:target:action:)]
        pub unsafe fn sliderWithValue_minValue_maxValue_target_action(
            value: c_double,
            minValue: c_double,
            maxValue: c_double,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSSliderDeprecated
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[cfg(feature = "AppKit_NSCell")]
        #[method(setTitleCell:)]
        pub unsafe fn setTitleCell(&self, cell: Option<&AppKit::NSCell>);

        #[method_id(@__retain_semantics Other titleCell)]
        pub unsafe fn titleCell(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTitleColor:)]
        pub unsafe fn setTitleColor(&self, newColor: Option<&AppKit::NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other titleColor)]
        pub unsafe fn titleColor(&self) -> Option<Id<AppKit::NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, fontObj: Option<&AppKit::NSFont>);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Option<Id<AppKit::NSFont, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: Option<&Foundation::NSString>);

        #[method(setKnobThickness:)]
        pub unsafe fn setKnobThickness(&self, thickness: CoreGraphics::CGFloat);

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, backgroundImage: Option<&AppKit::NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<AppKit::NSImage, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl AppKit::NSSlider {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
