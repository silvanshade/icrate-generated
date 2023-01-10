//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_protocol!(
    pub struct NSTextViewportLayoutControllerDelegate;

    unsafe impl ProtocolType for NSTextViewportLayoutControllerDelegate {
        #[method(viewportBoundsForTextViewportLayoutController:)]
        pub unsafe fn viewportBoundsForTextViewportLayoutController(
            &self,
            textViewportLayoutController: &AppKit::NSTextViewportLayoutController,
        ) -> CoreGraphics::CGRect;

        #[method(textViewportLayoutController:configureRenderingSurfaceForTextLayoutFragment:)]
        pub unsafe fn textViewportLayoutController_configureRenderingSurfaceForTextLayoutFragment(
            &self,
            textViewportLayoutController: &AppKit::NSTextViewportLayoutController,
            textLayoutFragment: &AppKit::NSTextLayoutFragment,
        );

        #[optional]
        #[method(textViewportLayoutControllerWillLayout:)]
        pub unsafe fn textViewportLayoutControllerWillLayout(
            &self,
            textViewportLayoutController: &AppKit::NSTextViewportLayoutController,
        );

        #[optional]
        #[method(textViewportLayoutControllerDidLayout:)]
        pub unsafe fn textViewportLayoutControllerDidLayout(
            &self,
            textViewportLayoutController: &AppKit::NSTextViewportLayoutController,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextViewportLayoutController;

    unsafe impl ClassType for NSTextViewportLayoutController {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
    unsafe impl NSTextViewportLayoutController {
        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Init initWithTextLayoutManager:)]
        pub unsafe fn initWithTextLayoutManager(
            this: Option<Allocated<Self>>,
            textLayoutManager: &AppKit::NSTextLayoutManager,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<AppKit::NSTextViewportLayoutControllerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&AppKit::NSTextViewportLayoutControllerDelegate>,
        );

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<AppKit::NSTextLayoutManager, Shared>>;

        #[method(viewportBounds)]
        pub unsafe fn viewportBounds(&self) -> CoreGraphics::CGRect;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other viewportRange)]
        pub unsafe fn viewportRange(&self) -> Option<Id<AppKit::NSTextRange, Shared>>;

        #[method(layoutViewport)]
        pub unsafe fn layoutViewport(&self);

        #[method(relocateViewportToTextLocation:)]
        pub unsafe fn relocateViewportToTextLocation(
            &self,
            textLocation: &AppKit::NSTextLocation,
        ) -> CoreGraphics::CGFloat;

        #[method(adjustViewportByVerticalOffset:)]
        pub unsafe fn adjustViewportByVerticalOffset(&self, verticalOffset: CoreGraphics::CGFloat);
    }
);
