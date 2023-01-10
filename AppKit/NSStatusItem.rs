//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSStatusItemAutosaveName = Foundation::NSString;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSStatusItemBehavior {
        NSStatusItemBehaviorRemovalAllowed = 1 << 1,
        NSStatusItemBehaviorTerminationOnRemoval = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStatusItem;

    unsafe impl ClassType for NSStatusItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSStatusItem")]
    unsafe impl NSStatusItem {
        #[cfg(feature = "AppKit_NSStatusBar")]
        #[method_id(@__retain_semantics Other statusBar)]
        pub unsafe fn statusBar(&self) -> Option<Id<AppKit::NSStatusBar, Shared>>;

        #[method(length)]
        pub unsafe fn length(&self) -> Foundation::CGFloat;

        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: Foundation::CGFloat);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<AppKit::NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&AppKit::NSMenu>);

        #[cfg(feature = "AppKit_NSStatusBarButton")]
        #[method_id(@__retain_semantics Other button)]
        pub unsafe fn button(&self) -> Option<Id<AppKit::NSStatusBarButton, Shared>>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> AppKit::NSStatusItemBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: AppKit::NSStatusItemBehavior);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Id<AppKit::NSStatusItemAutosaveName, Shared>;

        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(
            &self,
            autosaveName: Option<&AppKit::NSStatusItemAutosaveName>,
        );
    }
);

extern_methods!(
    /// NSStatusItemDeprecated
    #[cfg(feature = "AppKit_NSStatusItem")]
    unsafe impl NSStatusItem {
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<Foundation::NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(
            &self,
            attributedTitle: Option<&Foundation::NSAttributedString>,
        );

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<AppKit::NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&AppKit::NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<AppKit::NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternateImage: Option<&AppKit::NSImage>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(highlightMode)]
        pub unsafe fn highlightMode(&self) -> bool;

        #[method(setHighlightMode:)]
        pub unsafe fn setHighlightMode(&self, highlightMode: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&Foundation::NSString>);

        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: AppKit::NSEventMask) -> NSInteger;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&AppKit::NSView>);

        #[method(drawStatusBarBackgroundInRect:withHighlight:)]
        pub unsafe fn drawStatusBarBackgroundInRect_withHighlight(
            &self,
            rect: Foundation::NSRect,
            highlight: bool,
        );

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(popUpStatusItemMenu:)]
        pub unsafe fn popUpStatusItemMenu(&self, menu: &AppKit::NSMenu);
    }
);
