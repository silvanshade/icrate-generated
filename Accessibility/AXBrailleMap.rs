//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXBrailleMap")]
    pub struct AXBrailleMap;

    #[cfg(feature = "Accessibility_AXBrailleMap")]
    unsafe impl ClassType for AXBrailleMap {
        type Super = NSObject;
    }
);

#[cfg(feature = "Accessibility_AXBrailleMap")]
unsafe impl NSCoding for AXBrailleMap {}

#[cfg(feature = "Accessibility_AXBrailleMap")]
unsafe impl NSObjectProtocol for AXBrailleMap {}

#[cfg(feature = "Accessibility_AXBrailleMap")]
unsafe impl NSSecureCoding for AXBrailleMap {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXBrailleMap")]
    unsafe impl AXBrailleMap {
        #[method(dimensions)]
        pub unsafe fn dimensions(&self) -> CGSize;

        #[method(setHeight:atPoint:)]
        pub unsafe fn setHeight_atPoint(&self, status: c_float, point: CGPoint);

        #[method(heightAtPoint:)]
        pub unsafe fn heightAtPoint(&self, point: CGPoint) -> c_float;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);

extern_protocol!(
    pub unsafe trait AXBrailleMapRenderer: NSObjectProtocol {
        #[optional]
        #[method(accessibilityBrailleMapRenderRegion)]
        unsafe fn accessibilityBrailleMapRenderRegion(&self) -> CGRect;

        #[optional]
        #[method(setAccessibilityBrailleMapRenderRegion:)]
        unsafe fn setAccessibilityBrailleMapRenderRegion(
            &self,
            accessibility_braille_map_render_region: CGRect,
        );

        #[cfg(feature = "Accessibility_AXBrailleMap")]
        #[optional]
        #[method(accessibilityBrailleMapRenderer)]
        unsafe fn accessibilityBrailleMapRenderer(
            &self,
        ) -> NonNull<Block<(NonNull<AXBrailleMap>,), ()>>;

        #[cfg(feature = "Accessibility_AXBrailleMap")]
        #[optional]
        #[method(setAccessibilityBrailleMapRenderer:)]
        unsafe fn setAccessibilityBrailleMapRenderer(
            &self,
            accessibility_braille_map_renderer: &Block<(NonNull<AXBrailleMap>,), ()>,
        );
    }

    unsafe impl ProtocolType for dyn AXBrailleMapRenderer {}
);
