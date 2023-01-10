//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableRowView;

    unsafe impl ClassType for NSTableRowView {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl NSTableRowView {
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> AppKit::NSTableViewSelectionHighlightStyle;

        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selectionHighlightStyle: AppKit::NSTableViewSelectionHighlightStyle,
        );

        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[method(isGroupRowStyle)]
        pub unsafe fn isGroupRowStyle(&self) -> bool;

        #[method(setGroupRowStyle:)]
        pub unsafe fn setGroupRowStyle(&self, groupRowStyle: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isPreviousRowSelected)]
        pub unsafe fn isPreviousRowSelected(&self) -> bool;

        #[method(setPreviousRowSelected:)]
        pub unsafe fn setPreviousRowSelected(&self, previousRowSelected: bool);

        #[method(isNextRowSelected)]
        pub unsafe fn isNextRowSelected(&self) -> bool;

        #[method(setNextRowSelected:)]
        pub unsafe fn setNextRowSelected(&self, nextRowSelected: bool);

        #[method(isFloating)]
        pub unsafe fn isFloating(&self) -> bool;

        #[method(setFloating:)]
        pub unsafe fn setFloating(&self, floating: bool);

        #[method(isTargetForDropOperation)]
        pub unsafe fn isTargetForDropOperation(&self) -> bool;

        #[method(setTargetForDropOperation:)]
        pub unsafe fn setTargetForDropOperation(&self, targetForDropOperation: bool);

        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> AppKit::NSTableViewDraggingDestinationFeedbackStyle;

        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            draggingDestinationFeedbackStyle: AppKit::NSTableViewDraggingDestinationFeedbackStyle,
        );

        #[method(indentationForDropOperation)]
        pub unsafe fn indentationForDropOperation(&self) -> Foundation::CGFloat;

        #[method(setIndentationForDropOperation:)]
        pub unsafe fn setIndentationForDropOperation(
            &self,
            indentationForDropOperation: Foundation::CGFloat,
        );

        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> AppKit::NSBackgroundStyle;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<AppKit::NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &AppKit::NSColor);

        #[method(drawBackgroundInRect:)]
        pub unsafe fn drawBackgroundInRect(&self, dirtyRect: Foundation::NSRect);

        #[method(drawSelectionInRect:)]
        pub unsafe fn drawSelectionInRect(&self, dirtyRect: Foundation::NSRect);

        #[method(drawSeparatorInRect:)]
        pub unsafe fn drawSeparatorInRect(&self, dirtyRect: Foundation::NSRect);

        #[method(drawDraggingDestinationFeedbackInRect:)]
        pub unsafe fn drawDraggingDestinationFeedbackInRect(&self, dirtyRect: Foundation::NSRect);

        #[method_id(@__retain_semantics Other viewAtColumn:)]
        pub unsafe fn viewAtColumn(&self, column: NSInteger) -> Option<Id<Object, Shared>>;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl AppKit::NSTableRowView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
