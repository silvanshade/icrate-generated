//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CATiledLayer;

    unsafe impl ClassType for CATiledLayer {
        #[inherits(NSObject)]
        type Super = CoreAnimation::CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    unsafe impl CATiledLayer {
        #[method(fadeDuration)]
        pub unsafe fn fadeDuration() -> CoreFoundation::CFTimeInterval;

        #[method(levelsOfDetail)]
        pub unsafe fn levelsOfDetail(&self) -> usize;

        #[method(setLevelsOfDetail:)]
        pub unsafe fn setLevelsOfDetail(&self, levelsOfDetail: usize);

        #[method(levelsOfDetailBias)]
        pub unsafe fn levelsOfDetailBias(&self) -> usize;

        #[method(setLevelsOfDetailBias:)]
        pub unsafe fn setLevelsOfDetailBias(&self, levelsOfDetailBias: usize);

        #[method(tileSize)]
        pub unsafe fn tileSize(&self) -> Foundation::CGSize;

        #[method(setTileSize:)]
        pub unsafe fn setTileSize(&self, tileSize: Foundation::CGSize);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CATiledLayer")]
    unsafe impl CoreAnimation::CATiledLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
