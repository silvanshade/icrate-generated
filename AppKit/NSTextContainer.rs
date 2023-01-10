//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextContainer;

    unsafe impl ClassType for NSTextContainer {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextContainer")]
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(
            this: Option<Allocated<Self>>,
            size: Foundation::NSSize,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<AppKit::NSLayoutManager, Shared>>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(setLayoutManager:)]
        pub unsafe fn setLayoutManager(&self, layoutManager: Option<&AppKit::NSLayoutManager>);

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(replaceLayoutManager:)]
        pub unsafe fn replaceLayoutManager(&self, newLayoutManager: &AppKit::NSLayoutManager);

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<AppKit::NSTextLayoutManager, Shared>>;

        #[method(size)]
        pub unsafe fn size(&self) -> Foundation::NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Foundation::NSSize);

        #[cfg(all(feature = "AppKit_NSBezierPath", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other exclusionPaths)]
        pub unsafe fn exclusionPaths(
            &self,
        ) -> Id<Foundation::NSArray<AppKit::NSBezierPath>, Shared>;

        #[cfg(all(feature = "AppKit_NSBezierPath", feature = "Foundation_NSArray"))]
        #[method(setExclusionPaths:)]
        pub unsafe fn setExclusionPaths(
            &self,
            exclusionPaths: &Foundation::NSArray<AppKit::NSBezierPath>,
        );

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> AppKit::NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, lineBreakMode: AppKit::NSLineBreakMode);

        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> Foundation::CGFloat;

        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, lineFragmentPadding: Foundation::CGFloat);

        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSUInteger;

        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximumNumberOfLines: NSUInteger);

        #[method(lineFragmentRectForProposedRect:atIndex:writingDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_atIndex_writingDirection_remainingRect(
            &self,
            proposedRect: Foundation::NSRect,
            characterIndex: NSUInteger,
            baseWritingDirection: AppKit::NSWritingDirection,
            remainingRect: *mut Foundation::NSRect,
        ) -> Foundation::NSRect;

        #[method(isSimpleRectangularTextContainer)]
        pub unsafe fn isSimpleRectangularTextContainer(&self) -> bool;

        #[method(widthTracksTextView)]
        pub unsafe fn widthTracksTextView(&self) -> bool;

        #[method(setWidthTracksTextView:)]
        pub unsafe fn setWidthTracksTextView(&self, widthTracksTextView: bool);

        #[method(heightTracksTextView)]
        pub unsafe fn heightTracksTextView(&self) -> bool;

        #[method(setHeightTracksTextView:)]
        pub unsafe fn setHeightTracksTextView(&self, heightTracksTextView: bool);

        #[cfg(feature = "AppKit_NSTextView")]
        #[method_id(@__retain_semantics Other textView)]
        pub unsafe fn textView(&self) -> Option<Id<AppKit::NSTextView, Shared>>;

        #[cfg(feature = "AppKit_NSTextView")]
        #[method(setTextView:)]
        pub unsafe fn setTextView(&self, textView: Option<&AppKit::NSTextView>);
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineSweepDirection {
        NSLineSweepLeft = 0,
        NSLineSweepRight = 1,
        NSLineSweepDown = 2,
        NSLineSweepUp = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineMovementDirection {
        NSLineDoesntMove = 0,
        NSLineMovesLeft = 1,
        NSLineMovesRight = 2,
        NSLineMovesDown = 3,
        NSLineMovesUp = 4,
    }
);

extern_methods!(
    /// NSTextContainerDeprecated
    #[cfg(feature = "AppKit_NSTextContainer")]
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithContainerSize:)]
        pub unsafe fn initWithContainerSize(
            this: Option<Allocated<Self>>,
            aContainerSize: Foundation::NSSize,
        ) -> Id<Self, Shared>;

        #[method(containerSize)]
        pub unsafe fn containerSize(&self) -> Foundation::NSSize;

        #[method(setContainerSize:)]
        pub unsafe fn setContainerSize(&self, containerSize: Foundation::NSSize);

        #[method(lineFragmentRectForProposedRect:sweepDirection:movementDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_sweepDirection_movementDirection_remainingRect(
            &self,
            proposedRect: Foundation::NSRect,
            sweepDirection: AppKit::NSLineSweepDirection,
            movementDirection: AppKit::NSLineMovementDirection,
            remainingRect: Foundation::NSRectPointer,
        ) -> Foundation::NSRect;

        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: Foundation::NSPoint) -> bool;
    }
);
