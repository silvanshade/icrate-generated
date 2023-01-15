//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTickMarkPosition {
        NSTickMarkPositionBelow = 0,
        NSTickMarkPositionAbove = 1,
        NSTickMarkPositionLeading = NSTickMarkPositionAbove,
        NSTickMarkPositionTrailing = NSTickMarkPositionBelow,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSliderType {
        NSSliderTypeLinear = 0,
        NSSliderTypeCircular = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSliderCell")]
    pub struct NSSliderCell;

    #[cfg(feature = "AppKit_NSSliderCell")]
    unsafe impl ClassType for NSSliderCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSliderCell")]
    unsafe impl NSSliderCell {
        #[method(prefersTrackingUntilMouseUp)]
        pub unsafe fn prefersTrackingUntilMouseUp() -> bool;

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(altIncrementValue)]
        pub unsafe fn altIncrementValue(&self) -> c_double;

        #[method(setAltIncrementValue:)]
        pub unsafe fn setAltIncrementValue(&self, alt_increment_value: c_double);

        #[method(sliderType)]
        pub unsafe fn sliderType(&self) -> NSSliderType;

        #[method(setSliderType:)]
        pub unsafe fn setSliderType(&self, slider_type: NSSliderType);

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[method(trackRect)]
        pub unsafe fn trackRect(&self) -> NSRect;

        #[method(knobThickness)]
        pub unsafe fn knobThickness(&self) -> CGFloat;

        #[method(knobRectFlipped:)]
        pub unsafe fn knobRectFlipped(&self, flipped: bool) -> NSRect;

        #[method(barRectFlipped:)]
        pub unsafe fn barRectFlipped(&self, flipped: bool) -> NSRect;

        #[method(drawKnob:)]
        pub unsafe fn drawKnob_(&self, knob_rect: NSRect);

        #[method(drawKnob)]
        pub unsafe fn drawKnob(&self);

        #[method(drawBarInside:flipped:)]
        pub unsafe fn drawBarInside_flipped(&self, rect: NSRect, flipped: bool);
    }
);

extern_methods!(
    /// NSSliderCellVerticalGetter
    #[cfg(feature = "AppKit_NSSliderCell")]
    unsafe impl NSSliderCell {}
);

extern_methods!(
    /// NSTickMarkSupport
    #[cfg(feature = "AppKit_NSSliderCell")]
    unsafe impl NSSliderCell {
        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, number_of_tick_marks: NSInteger);

        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tick_mark_position: NSTickMarkPosition);

        #[method(allowsTickMarkValuesOnly)]
        pub unsafe fn allowsTickMarkValuesOnly(&self) -> bool;

        #[method(setAllowsTickMarkValuesOnly:)]
        pub unsafe fn setAllowsTickMarkValuesOnly(&self, allows_tick_mark_values_only: bool);

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;

        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[method(indexOfTickMarkAtPoint:)]
        pub unsafe fn indexOfTickMarkAtPoint(&self, point: NSPoint) -> NSInteger;

        #[method(closestTickMarkValueToValue:)]
        pub unsafe fn closestTickMarkValueToValue(&self, value: c_double) -> c_double;

        #[method(drawTickMarks)]
        pub unsafe fn drawTickMarks(&self);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSliderCell")]
    unsafe impl NSSliderCell {
        #[cfg(feature = "AppKit_NSCell")]
        #[deprecated = "-setTitleCell: had no effect since 10.0"]
        #[method(setTitleCell:)]
        pub unsafe fn setTitleCell(&self, cell: Option<&NSCell>);

        #[deprecated = "-titleCell has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other titleCell)]
        pub unsafe fn titleCell(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[deprecated = "-setTitleColor: had no effect since 10.0"]
        #[method(setTitleColor:)]
        pub unsafe fn setTitleColor(&self, new_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[deprecated = "-titleColor has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other titleColor)]
        pub unsafe fn titleColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "-setTitleFont: had no effect since 10.0"]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, font_obj: Option<&NSFont>);

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "-titleFont has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "-title has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "-setTitle: had no effect since 10.0"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: Option<&NSString>);

        #[deprecated = "-knobThickness has returned 0 since 10.0"]
        #[method(setKnobThickness:)]
        pub unsafe fn setKnobThickness(&self, thickness: CGFloat);

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated = "-setImage: had no effect since 10.0"]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, background_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated = "-image has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
    }
);

extern_static!(NSTickMarkBelow: NSTickMarkPosition = NSTickMarkPositionBelow);

extern_static!(NSTickMarkAbove: NSTickMarkPosition = NSTickMarkPositionAbove);

extern_static!(NSTickMarkLeft: NSTickMarkPosition = NSTickMarkPositionLeading);

extern_static!(NSTickMarkRight: NSTickMarkPosition = NSTickMarkPositionTrailing);

extern_static!(NSLinearSlider: NSSliderType = NSSliderTypeLinear);

extern_static!(NSCircularSlider: NSSliderType = NSSliderTypeCircular);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSSliderCell")]
    unsafe impl NSSliderCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);
