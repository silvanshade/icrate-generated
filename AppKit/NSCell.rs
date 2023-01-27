//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCellType {
        NSNullCellType = 0,
        NSTextCellType = 1,
        NSImageCellType = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCellAttribute {
        NSCellDisabled = 0,
        NSCellState = 1,
        NSPushInCell = 2,
        NSCellEditable = 3,
        NSChangeGrayCell = 4,
        NSCellHighlighted = 5,
        NSCellLightsByContents = 6,
        NSCellLightsByGray = 7,
        NSChangeBackgroundCell = 8,
        NSCellLightsByBackground = 9,
        NSCellIsBordered = 10,
        NSCellHasOverlappingImage = 11,
        NSCellHasImageHorizontal = 12,
        NSCellHasImageOnLeftOrBottom = 13,
        NSCellChangesContents = 14,
        NSCellIsInsetButton = 15,
        NSCellAllowsMixedState = 16,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCellImagePosition {
        NSNoImage = 0,
        NSImageOnly = 1,
        NSImageLeft = 2,
        NSImageRight = 3,
        NSImageBelow = 4,
        NSImageAbove = 5,
        NSImageOverlaps = 6,
        NSImageLeading = 7,
        NSImageTrailing = 8,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageScaling {
        NSImageScaleProportionallyDown = 0,
        NSImageScaleAxesIndependently = 1,
        NSImageScaleNone = 2,
        NSImageScaleProportionallyUpOrDown = 3,
        #[deprecated = "Use NSImageScaleProportionallyDown instead"]
        NSScaleProportionally = 0,
        #[deprecated = "Use NSImageScaleAxesIndependently instead"]
        NSScaleToFit = 1,
        #[deprecated = "Use NSImageScaleNone instead"]
        NSScaleNone = 2,
    }
);

typed_extensible_enum!(
    pub type NSControlStateValue = NSInteger;
);

extern_static!(NSControlStateValueMixed: NSControlStateValue = -1);

extern_static!(NSControlStateValueOff: NSControlStateValue = 0);

extern_static!(NSControlStateValueOn: NSControlStateValue = 1);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSCellStyleMask {
        NSNoCellMask = 0,
        NSContentsCellMask = 1,
        NSPushInCellMask = 2,
        NSChangeGrayCellMask = 4,
        NSChangeBackgroundCellMask = 8,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSControlTint {
        NSDefaultControlTint = 0,
        NSBlueControlTint = 1,
        NSGraphiteControlTint = 6,
        NSClearControlTint = 7,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSControlSize {
        NSControlSizeRegular = 0,
        NSControlSizeSmall = 1,
        NSControlSizeMini = 2,
        NSControlSizeLarge = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCell")]
    pub struct NSCell;

    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl ClassType for NSCell {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSCell")]
unsafe impl NSAccessibility for NSCell {}

#[cfg(feature = "AppKit_NSCell")]
unsafe impl NSAccessibilityElementProtocol for NSCell {}

#[cfg(feature = "AppKit_NSCell")]
unsafe impl NSCoding for NSCell {}

#[cfg(feature = "AppKit_NSCell")]
unsafe impl NSObjectProtocol for NSCell {}

#[cfg(feature = "AppKit_NSCell")]
unsafe impl NSUserInterfaceItemIdentification for NSCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

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

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(prefersTrackingUntilMouseUp)]
        pub unsafe fn prefersTrackingUntilMouseUp() -> bool;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other controlView)]
        pub unsafe fn controlView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setControlView:)]
        pub unsafe fn setControlView(&self, control_view: Option<&NSView>);

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSCellType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: NSCellType);

        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;

        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);

        #[method(isScrollable)]
        pub unsafe fn isScrollable(&self) -> bool;

        #[method(setScrollable:)]
        pub unsafe fn setScrollable(&self, scrollable: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;

        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSFormatter")]
        #[method_id(@__retain_semantics Other formatter)]
        pub unsafe fn formatter(&self) -> Option<Id<NSFormatter, Shared>>;

        #[cfg(feature = "Foundation_NSFormatter")]
        #[method(setFormatter:)]
        pub unsafe fn setFormatter(&self, formatter: Option<&NSFormatter>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&Object>);

        #[method(hasValidObjectValue)]
        pub unsafe fn hasValidObjectValue(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, string_value: &NSString);

        #[method(compare:)]
        pub unsafe fn compare(&self, other_cell: &Object) -> NSComparisonResult;

        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;

        #[method(setIntValue:)]
        pub unsafe fn setIntValue(&self, int_value: c_int);

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, float_value: c_float);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;

        #[method(setIntegerValue:)]
        pub unsafe fn setIntegerValue(&self, integer_value: NSInteger);

        #[method(takeIntValueFrom:)]
        pub unsafe fn takeIntValueFrom(&self, sender: Option<&Object>);

        #[method(takeFloatValueFrom:)]
        pub unsafe fn takeFloatValueFrom(&self, sender: Option<&Object>);

        #[method(takeDoubleValueFrom:)]
        pub unsafe fn takeDoubleValueFrom(&self, sender: Option<&Object>);

        #[method(takeStringValueFrom:)]
        pub unsafe fn takeStringValueFrom(&self, sender: Option<&Object>);

        #[method(takeObjectValueFrom:)]
        pub unsafe fn takeObjectValueFrom(&self, sender: Option<&Object>);

        #[method(takeIntegerValueFrom:)]
        pub unsafe fn takeIntegerValueFrom(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&Object>);

        #[method(cellAttribute:)]
        pub unsafe fn cellAttribute(&self, parameter: NSCellAttribute) -> NSInteger;

        #[method(setCellAttribute:to:)]
        pub unsafe fn setCellAttribute_to(&self, parameter: NSCellAttribute, value: NSInteger);

        #[method(imageRectForBounds:)]
        pub unsafe fn imageRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(titleRectForBounds:)]
        pub unsafe fn titleRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(drawingRectForBounds:)]
        pub unsafe fn drawingRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(cellSize)]
        pub unsafe fn cellSize(&self) -> NSSize;

        #[method(cellSizeForBounds:)]
        pub unsafe fn cellSizeForBounds(&self, rect: NSRect) -> NSSize;

        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other highlightColorWithFrame:inView:)]
        pub unsafe fn highlightColorWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        ) -> Option<Id<NSColor, Shared>>;

        #[method(calcDrawInfo:)]
        pub unsafe fn calcDrawInfo(&self, rect: NSRect);

        #[cfg(feature = "AppKit_NSText")]
        #[method_id(@__retain_semantics Other setUpFieldEditorAttributes:)]
        pub unsafe fn setUpFieldEditorAttributes(&self, text_obj: &NSText) -> Id<NSText, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(drawInteriorWithFrame:inView:)]
        pub unsafe fn drawInteriorWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method(drawWithFrame:inView:)]
        pub unsafe fn drawWithFrame_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(feature = "AppKit_NSView")]
        #[method(highlight:withFrame:inView:)]
        pub unsafe fn highlight_withFrame_inView(
            &self,
            flag: bool,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[method(mouseDownFlags)]
        pub unsafe fn mouseDownFlags(&self) -> NSInteger;

        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method(startTrackingAt:inView:)]
        pub unsafe fn startTrackingAt_inView(
            &self,
            start_point: NSPoint,
            control_view: &NSView,
        ) -> bool;

        #[cfg(feature = "AppKit_NSView")]
        #[method(continueTracking:at:inView:)]
        pub unsafe fn continueTracking_at_inView(
            &self,
            last_point: NSPoint,
            current_point: NSPoint,
            control_view: &NSView,
        ) -> bool;

        #[cfg(feature = "AppKit_NSView")]
        #[method(stopTracking:at:inView:mouseIsUp:)]
        pub unsafe fn stopTracking_at_inView_mouseIsUp(
            &self,
            last_point: NSPoint,
            stop_point: NSPoint,
            control_view: &NSView,
            flag: bool,
        );

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
        #[method(trackMouse:inRect:ofView:untilMouseUp:)]
        pub unsafe fn trackMouse_inRect_ofView_untilMouseUp(
            &self,
            event: &NSEvent,
            cell_frame: NSRect,
            control_view: &NSView,
            flag: bool,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSText",
            feature = "AppKit_NSView"
        ))]
        #[method(editWithFrame:inView:editor:delegate:event:)]
        pub unsafe fn editWithFrame_inView_editor_delegate_event(
            &self,
            rect: NSRect,
            control_view: &NSView,
            text_obj: &NSText,
            delegate: Option<&Object>,
            event: Option<&NSEvent>,
        );

        #[cfg(all(feature = "AppKit_NSText", feature = "AppKit_NSView"))]
        #[method(selectWithFrame:inView:editor:delegate:start:length:)]
        pub unsafe fn selectWithFrame_inView_editor_delegate_start_length(
            &self,
            rect: NSRect,
            control_view: &NSView,
            text_obj: &NSText,
            delegate: Option<&Object>,
            sel_start: NSInteger,
            sel_length: NSInteger,
        );

        #[cfg(feature = "AppKit_NSText")]
        #[method(endEditing:)]
        pub unsafe fn endEditing(&self, text_obj: &NSText);

        #[cfg(feature = "AppKit_NSView")]
        #[method(resetCursorRect:inView:)]
        pub unsafe fn resetCursorRect_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSMenu",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Other menuForEvent:inRect:ofView:)]
        pub unsafe fn menuForEvent_inRect_ofView(
            &self,
            event: &NSEvent,
            cell_frame: NSRect,
            view: &NSView,
        ) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other defaultMenu)]
        pub unsafe fn defaultMenu() -> Option<Id<NSMenu, Shared>>;

        #[method(sendsActionOnEndEditing)]
        pub unsafe fn sendsActionOnEndEditing(&self) -> bool;

        #[method(setSendsActionOnEndEditing:)]
        pub unsafe fn setSendsActionOnEndEditing(&self, sends_action_on_end_editing: bool);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(allowsUndo)]
        pub unsafe fn allowsUndo(&self) -> bool;

        #[method(setAllowsUndo:)]
        pub unsafe fn setAllowsUndo(&self, allows_undo: bool);

        #[method(truncatesLastVisibleLine)]
        pub unsafe fn truncatesLastVisibleLine(&self) -> bool;

        #[method(setTruncatesLastVisibleLine:)]
        pub unsafe fn setTruncatesLastVisibleLine(&self, truncates_last_visible_line: bool);

        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            user_interface_layout_direction: NSUserInterfaceLayoutDirection,
        );

        #[cfg(all(feature = "AppKit_NSTextView", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other fieldEditorForView:)]
        pub unsafe fn fieldEditorForView(
            &self,
            control_view: &NSView,
        ) -> Option<Id<NSTextView, Shared>>;

        #[method(usesSingleLineMode)]
        pub unsafe fn usesSingleLineMode(&self) -> bool;

        #[method(setUsesSingleLineMode:)]
        pub unsafe fn setUsesSingleLineMode(&self, uses_single_line_mode: bool);

        #[cfg(all(
            feature = "AppKit_NSDraggingImageComponent",
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other draggingImageComponentsWithFrame:inView:)]
        pub unsafe fn draggingImageComponentsWithFrame_inView(
            &self,
            frame: NSRect,
            view: &NSView,
        ) -> Id<NSArray<NSDraggingImageComponent>, Shared>;
    }
);

extern_methods!(
    /// NSKeyboardUI
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[method(refusesFirstResponder)]
        pub unsafe fn refusesFirstResponder(&self) -> bool;

        #[method(setRefusesFirstResponder:)]
        pub unsafe fn setRefusesFirstResponder(&self, refuses_first_responder: bool);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(showsFirstResponder)]
        pub unsafe fn showsFirstResponder(&self) -> bool;

        #[method(setShowsFirstResponder:)]
        pub unsafe fn setShowsFirstResponder(&self, shows_first_responder: bool);

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&Object>);

        #[method(focusRingType)]
        pub unsafe fn focusRingType(&self) -> NSFocusRingType;

        #[method(setFocusRingType:)]
        pub unsafe fn setFocusRingType(&self, focus_ring_type: NSFocusRingType);

        #[method(defaultFocusRingType)]
        pub unsafe fn defaultFocusRingType() -> NSFocusRingType;

        #[cfg(feature = "AppKit_NSView")]
        #[method(drawFocusRingMaskWithFrame:inView:)]
        pub unsafe fn drawFocusRingMaskWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method(focusRingMaskBoundsForFrame:inView:)]
        pub unsafe fn focusRingMaskBoundsForFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        ) -> NSRect;

        #[method(wantsNotificationForMarkedText)]
        pub unsafe fn wantsNotificationForMarkedText(&self) -> bool;
    }
);

extern_methods!(
    /// NSCellAttributedStringMethods
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedStringValue)]
        pub unsafe fn attributedStringValue(&self) -> Id<NSAttributedString, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedStringValue:)]
        pub unsafe fn setAttributedStringValue(&self, attributed_string_value: &NSAttributedString);

        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;

        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allows_editing_text_attributes: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, imports_graphics: bool);
    }
);

extern_methods!(
    /// NSCellMixedState
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[method(allowsMixedState)]
        pub unsafe fn allowsMixedState(&self) -> bool;

        #[method(setAllowsMixedState:)]
        pub unsafe fn setAllowsMixedState(&self, allows_mixed_state: bool);

        #[method(nextState)]
        pub unsafe fn nextState(&self) -> NSInteger;

        #[method(setNextState)]
        pub unsafe fn setNextState(&self);
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSCellHitResult {
        NSCellHitNone = 0,
        NSCellHitContentArea = 1 << 0,
        NSCellHitEditableTextArea = 1 << 1,
        NSCellHitTrackableArea = 1 << 2,
    }
);

extern_methods!(
    /// NSCellHitTest
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
        #[method(hitTestForEvent:inRect:ofView:)]
        pub unsafe fn hitTestForEvent_inRect_ofView(
            &self,
            event: &NSEvent,
            cell_frame: NSRect,
            control_view: &NSView,
        ) -> NSCellHitResult;
    }
);

extern_methods!(
    /// NSCellExpansion
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[cfg(feature = "AppKit_NSView")]
        #[method(expansionFrameWithFrame:inView:)]
        pub unsafe fn expansionFrameWithFrame_inView(
            &self,
            cell_frame: NSRect,
            view: &NSView,
        ) -> NSRect;

        #[cfg(feature = "AppKit_NSView")]
        #[method(drawWithExpansionFrame:inView:)]
        pub unsafe fn drawWithExpansionFrame_inView(&self, cell_frame: NSRect, view: &NSView);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSBackgroundStyle {
        NSBackgroundStyleNormal = 0,
        NSBackgroundStyleEmphasized = 1,
        NSBackgroundStyleRaised = 2,
        NSBackgroundStyleLowered = 3,
    }
);

extern_methods!(
    /// NSCellBackgroundStyle
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;

        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, background_style: NSBackgroundStyle);

        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;
    }
);

extern_fn!(
    #[cfg(feature = "AppKit_NSImage")]
    pub unsafe fn NSDrawThreePartImage(
        frame: NSRect,
        start_cap: Option<&NSImage>,
        center_fill: Option<&NSImage>,
        end_cap: Option<&NSImage>,
        vertical: Bool,
        op: NSCompositingOperation,
        alpha_fraction: CGFloat,
        flipped: Bool,
    );
);

extern_fn!(
    #[cfg(feature = "AppKit_NSImage")]
    pub unsafe fn NSDrawNinePartImage(
        frame: NSRect,
        top_left_corner: Option<&NSImage>,
        top_edge_fill: Option<&NSImage>,
        top_right_corner: Option<&NSImage>,
        left_edge_fill: Option<&NSImage>,
        center_fill: Option<&NSImage>,
        right_edge_fill: Option<&NSImage>,
        bottom_left_corner: Option<&NSImage>,
        bottom_edge_fill: Option<&NSImage>,
        bottom_right_corner: Option<&NSImage>,
        op: NSCompositingOperation,
        alpha_fraction: CGFloat,
        flipped: Bool,
    );
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSCell")]
    unsafe impl NSCell {
        #[deprecated = "The controlTint property is not respected on 10.14 and later. For custom cells, use +[NSColor controlAccentColor] to respect the user's preferred accent color when drawing."]
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[deprecated = "The controlTint property is not respected on 10.14 and later. For custom cells, use +[NSColor controlAccentColor] to respect the user's preferred accent color when drawing."]
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);

        #[deprecated]
        #[method(entryType)]
        pub unsafe fn entryType(&self) -> NSInteger;

        #[deprecated]
        #[method(setEntryType:)]
        pub unsafe fn setEntryType(&self, r#type: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(isEntryAcceptable:)]
        pub unsafe fn isEntryAcceptable(&self, string: &NSString) -> bool;

        #[deprecated]
        #[method(setFloatingPointFormat:left:right:)]
        pub unsafe fn setFloatingPointFormat_left_right(
            &self,
            auto_range: bool,
            left_digits: NSUInteger,
            right_digits: NSUInteger,
        );

        #[deprecated]
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);

        #[deprecated]
        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other mnemonic)]
        pub unsafe fn mnemonic(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: &NSString);
    }
);

extern_static!(NSBackgroundStyleLight: NSBackgroundStyle = NSBackgroundStyleNormal);

extern_static!(NSBackgroundStyleDark: NSBackgroundStyle = NSBackgroundStyleEmphasized);

pub type NSCellStateValue = NSControlStateValue;

extern_static!(NSMixedState: NSControlStateValue = NSControlStateValueMixed);

extern_static!(NSOffState: NSControlStateValue = NSControlStateValueOff);

extern_static!(NSOnState: NSControlStateValue = NSControlStateValueOn);

extern_static!(NSRegularControlSize: NSControlSize = NSControlSizeRegular);

extern_static!(NSSmallControlSize: NSControlSize = NSControlSizeSmall);

extern_static!(NSMiniControlSize: NSControlSize = NSControlSizeMini);

extern_static!(NSControlTintDidChangeNotification: &'static NSNotificationName);

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        #[deprecated = "Use formatters instead"]
        NSAnyType = 0,
        #[deprecated = "Use formatters instead"]
        NSIntType = 1,
        #[deprecated = "Use formatters instead"]
        NSPositiveIntType = 2,
        #[deprecated = "Use formatters instead"]
        NSFloatType = 3,
        #[deprecated = "Use formatters instead"]
        NSPositiveFloatType = 4,
        #[deprecated = "Use formatters instead"]
        NSDoubleType = 6,
        #[deprecated = "Use formatters instead"]
        NSPositiveDoubleType = 7,
    }
);
