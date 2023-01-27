//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDraggingSession")]
    pub struct NSDraggingSession;

    #[cfg(feature = "AppKit_NSDraggingSession")]
    unsafe impl ClassType for NSDraggingSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSDraggingSession")]
unsafe impl NSObjectProtocol for NSDraggingSession {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDraggingSession")]
    unsafe impl NSDraggingSession {
        #[method(draggingFormation)]
        pub unsafe fn draggingFormation(&self) -> NSDraggingFormation;

        #[method(setDraggingFormation:)]
        pub unsafe fn setDraggingFormation(&self, dragging_formation: NSDraggingFormation);

        #[method(animatesToStartingPositionsOnCancelOrFail)]
        pub unsafe fn animatesToStartingPositionsOnCancelOrFail(&self) -> bool;

        #[method(setAnimatesToStartingPositionsOnCancelOrFail:)]
        pub unsafe fn setAnimatesToStartingPositionsOnCancelOrFail(
            &self,
            animates_to_starting_positions_on_cancel_or_fail: bool,
        );

        #[method(draggingLeaderIndex)]
        pub unsafe fn draggingLeaderIndex(&self) -> NSInteger;

        #[method(setDraggingLeaderIndex:)]
        pub unsafe fn setDraggingLeaderIndex(&self, dragging_leader_index: NSInteger);

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other draggingPasteboard)]
        pub unsafe fn draggingPasteboard(&self) -> Id<NSPasteboard, Shared>;

        #[method(draggingSequenceNumber)]
        pub unsafe fn draggingSequenceNumber(&self) -> NSInteger;

        #[method(draggingLocation)]
        pub unsafe fn draggingLocation(&self) -> NSPoint;

        #[cfg(all(
            feature = "AppKit_NSDraggingItem",
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:)]
        pub unsafe fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock(
            &self,
            enum_opts: NSDraggingItemEnumerationOptions,
            view: Option<&NSView>,
            class_array: &NSArray<TodoClass>,
            search_options: &NSDictionary<NSPasteboardReadingOptionKey, Object>,
            block: &Block<(NonNull<NSDraggingItem>, NSInteger, NonNull<Bool>), ()>,
        );
    }
);
