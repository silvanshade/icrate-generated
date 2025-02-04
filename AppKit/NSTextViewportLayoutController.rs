//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSTextViewportLayoutControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
        #[method(viewportBoundsForTextViewportLayoutController:)]
        unsafe fn viewportBoundsForTextViewportLayoutController(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
        ) -> CGRect;

        #[cfg(all(
            feature = "AppKit_NSTextLayoutFragment",
            feature = "AppKit_NSTextViewportLayoutController"
        ))]
        #[method(textViewportLayoutController:configureRenderingSurfaceForTextLayoutFragment:)]
        unsafe fn textViewportLayoutController_configureRenderingSurfaceForTextLayoutFragment(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
            text_layout_fragment: &NSTextLayoutFragment,
        );

        #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
        #[optional]
        #[method(textViewportLayoutControllerWillLayout:)]
        unsafe fn textViewportLayoutControllerWillLayout(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
        );

        #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
        #[optional]
        #[method(textViewportLayoutControllerDidLayout:)]
        unsafe fn textViewportLayoutControllerDidLayout(
            &self,
            text_viewport_layout_controller: &NSTextViewportLayoutController,
        );
    }

    unsafe impl ProtocolType for dyn NSTextViewportLayoutControllerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
    pub struct NSTextViewportLayoutController;

    #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
    unsafe impl ClassType for NSTextViewportLayoutController {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextViewportLayoutController")]
unsafe impl NSObjectProtocol for NSTextViewportLayoutController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
    unsafe impl NSTextViewportLayoutController {
        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Init initWithTextLayoutManager:)]
        pub unsafe fn initWithTextLayoutManager(
            this: Option<Allocated<Self>>,
            text_layout_manager: &NSTextLayoutManager,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextViewportLayoutControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextViewportLayoutControllerDelegate>>,
        );

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[method(viewportBounds)]
        pub unsafe fn viewportBounds(&self) -> CGRect;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other viewportRange)]
        pub unsafe fn viewportRange(&self) -> Option<Id<NSTextRange>>;

        #[method(layoutViewport)]
        pub unsafe fn layoutViewport(&self);

        #[method(relocateViewportToTextLocation:)]
        pub unsafe fn relocateViewportToTextLocation(
            &self,
            text_location: &ProtocolObject<dyn NSTextLocation>,
        ) -> CGFloat;

        #[method(adjustViewportByVerticalOffset:)]
        pub unsafe fn adjustViewportByVerticalOffset(&self, vertical_offset: CGFloat);
    }
);
