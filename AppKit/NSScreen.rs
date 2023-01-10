//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScreen;

    unsafe impl ClassType for NSScreen {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other screens)]
        pub unsafe fn screens() -> Id<Foundation::NSArray<AppKit::NSScreen>, Shared>;

        #[method_id(@__retain_semantics Other mainScreen)]
        pub unsafe fn mainScreen() -> Option<Id<AppKit::NSScreen, Shared>>;

        #[method_id(@__retain_semantics Other deepestScreen)]
        pub unsafe fn deepestScreen() -> Option<Id<AppKit::NSScreen, Shared>>;

        #[method(screensHaveSeparateSpaces)]
        pub unsafe fn screensHaveSeparateSpaces() -> bool;

        #[method(depth)]
        pub unsafe fn depth(&self) -> AppKit::NSWindowDepth;

        #[method(frame)]
        pub unsafe fn frame(&self) -> Foundation::NSRect;

        #[method(visibleFrame)]
        pub unsafe fn visibleFrame(&self) -> Foundation::NSRect;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Id<Foundation::NSDictionary<AppKit::NSDeviceDescriptionKey, Object>, Shared>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Option<Id<AppKit::NSColorSpace, Shared>>;

        #[method(supportedWindowDepths)]
        pub unsafe fn supportedWindowDepths(&self) -> NonNull<AppKit::NSWindowDepth>;

        #[method(canRepresentDisplayGamut:)]
        pub unsafe fn canRepresentDisplayGamut(&self, displayGamut: AppKit::NSDisplayGamut)
            -> bool;

        #[method(convertRectToBacking:)]
        pub unsafe fn convertRectToBacking(&self, rect: Foundation::NSRect) -> Foundation::NSRect;

        #[method(convertRectFromBacking:)]
        pub unsafe fn convertRectFromBacking(&self, rect: Foundation::NSRect)
            -> Foundation::NSRect;

        #[method(backingAlignedRect:options:)]
        pub unsafe fn backingAlignedRect_options(
            &self,
            rect: Foundation::NSRect,
            options: Foundation::NSAlignmentOptions,
        ) -> Foundation::NSRect;

        #[method(backingScaleFactor)]
        pub unsafe fn backingScaleFactor(&self) -> Foundation::CGFloat;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Id<Foundation::NSString, Shared>;

        #[method(safeAreaInsets)]
        pub unsafe fn safeAreaInsets(&self) -> Foundation::NSEdgeInsets;

        #[method(auxiliaryTopLeftArea)]
        pub unsafe fn auxiliaryTopLeftArea(&self) -> Foundation::NSRect;

        #[method(auxiliaryTopRightArea)]
        pub unsafe fn auxiliaryTopRightArea(&self) -> Foundation::NSRect;
    }
);

extern_static!(NSScreenColorSpaceDidChangeNotification: &'static Foundation::NSNotificationName);

extern_methods!(
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method(maximumExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumExtendedDynamicRangeColorComponentValue(&self) -> Foundation::CGFloat;

        #[method(maximumPotentialExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumPotentialExtendedDynamicRangeColorComponentValue(
            &self,
        ) -> Foundation::CGFloat;

        #[method(maximumReferenceExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumReferenceExtendedDynamicRangeColorComponentValue(
            &self,
        ) -> Foundation::CGFloat;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method(maximumFramesPerSecond)]
        pub unsafe fn maximumFramesPerSecond(&self) -> NSInteger;

        #[method(minimumRefreshInterval)]
        pub unsafe fn minimumRefreshInterval(&self) -> Foundation::NSTimeInterval;

        #[method(maximumRefreshInterval)]
        pub unsafe fn maximumRefreshInterval(&self) -> Foundation::NSTimeInterval;

        #[method(displayUpdateGranularity)]
        pub unsafe fn displayUpdateGranularity(&self) -> Foundation::NSTimeInterval;

        #[method(lastDisplayUpdateTimestamp)]
        pub unsafe fn lastDisplayUpdateTimestamp(&self) -> Foundation::NSTimeInterval;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSScreen")]
    unsafe impl NSScreen {
        #[method(userSpaceScaleFactor)]
        pub unsafe fn userSpaceScaleFactor(&self) -> Foundation::CGFloat;
    }
);
