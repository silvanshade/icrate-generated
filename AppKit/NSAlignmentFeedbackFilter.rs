//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::protocol]
pub unsafe trait NSAlignmentFeedbackToken: NSObjectProtocol {}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSAlignmentFeedbackFilter")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSAlignmentFeedbackFilter;
}

#[cfg(feature = "AppKit_NSAlignmentFeedbackFilter")]
unsafe impl NSObjectProtocol for NSAlignmentFeedbackFilter {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSAlignmentFeedbackFilter")]
    pub type NSAlignmentFeedbackFilter;

    #[objc2::method(sel = "inputEventMask")]
    pub unsafe fn inputEventMask() -> NSEventMask;

    #[cfg(feature = "AppKit_NSEvent")]
    #[objc2::method(sel = "updateWithEvent:")]
    pub unsafe fn updateWithEvent(&self, event: &NSEvent);

    #[cfg(feature = "AppKit_NSPanGestureRecognizer")]
    #[objc2::method(sel = "updateWithPanRecognizer:")]
    pub unsafe fn updateWithPanRecognizer(&self, pan_recognizer: &NSPanGestureRecognizer);

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(
        sel = "alignmentFeedbackTokenForMovementInView:previousPoint:alignedPoint:defaultPoint:",
        managed = "Other"
    )]
    pub unsafe fn alignmentFeedbackTokenForMovementInView_previousPoint_alignedPoint_defaultPoint(
        &self,
        view: Option<&NSView>,
        previous_point: NSPoint,
        aligned_point: NSPoint,
        default_point: NSPoint,
    ) -> Option<Id<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(
        sel = "alignmentFeedbackTokenForHorizontalMovementInView:previousX:alignedX:defaultX:",
        managed = "Other"
    )]
    pub unsafe fn alignmentFeedbackTokenForHorizontalMovementInView_previousX_alignedX_defaultX(
        &self,
        view: Option<&NSView>,
        previous_x: CGFloat,
        aligned_x: CGFloat,
        default_x: CGFloat,
    ) -> Option<Id<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

    #[cfg(feature = "AppKit_NSView")]
    #[objc2::method(
        sel = "alignmentFeedbackTokenForVerticalMovementInView:previousY:alignedY:defaultY:",
        managed = "Other"
    )]
    pub unsafe fn alignmentFeedbackTokenForVerticalMovementInView_previousY_alignedY_defaultY(
        &self,
        view: Option<&NSView>,
        previous_y: CGFloat,
        aligned_y: CGFloat,
        default_y: CGFloat,
    ) -> Option<Id<ProtocolObject<dyn NSAlignmentFeedbackToken>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "performFeedback:performanceTime:")]
    pub unsafe fn performFeedback_performanceTime(
        &self,
        alignment_feedback_tokens: &NSArray<ProtocolObject<dyn NSAlignmentFeedbackToken>>,
        performance_time: NSHapticFeedbackPerformanceTime,
    );
}
