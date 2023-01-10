//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOpenPanel;

    unsafe impl ClassType for NSOpenPanel {
        #[inherits(AppKit::NSPanel, AppKit::NSWindow, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSSavePanel;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics Other openPanel)]
        pub unsafe fn openPanel() -> Id<AppKit::NSOpenPanel, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLs)]
        pub unsafe fn URLs(&self) -> Id<Foundation::NSArray<Foundation::NSURL>, Shared>;

        #[method(resolvesAliases)]
        pub unsafe fn resolvesAliases(&self) -> bool;

        #[method(setResolvesAliases:)]
        pub unsafe fn setResolvesAliases(&self, resolvesAliases: bool);

        #[method(canChooseDirectories)]
        pub unsafe fn canChooseDirectories(&self) -> bool;

        #[method(setCanChooseDirectories:)]
        pub unsafe fn setCanChooseDirectories(&self, canChooseDirectories: bool);

        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allowsMultipleSelection: bool);

        #[method(canChooseFiles)]
        pub unsafe fn canChooseFiles(&self) -> bool;

        #[method(setCanChooseFiles:)]
        pub unsafe fn setCanChooseFiles(&self, canChooseFiles: bool);

        #[method(canResolveUbiquitousConflicts)]
        pub unsafe fn canResolveUbiquitousConflicts(&self) -> bool;

        #[method(setCanResolveUbiquitousConflicts:)]
        pub unsafe fn setCanResolveUbiquitousConflicts(&self, canResolveUbiquitousConflicts: bool);

        #[method(canDownloadUbiquitousContents)]
        pub unsafe fn canDownloadUbiquitousContents(&self) -> bool;

        #[method(setCanDownloadUbiquitousContents:)]
        pub unsafe fn setCanDownloadUbiquitousContents(&self, canDownloadUbiquitousContents: bool);

        #[method(isAccessoryViewDisclosed)]
        pub unsafe fn isAccessoryViewDisclosed(&self) -> bool;

        #[method(setAccessoryViewDisclosed:)]
        pub unsafe fn setAccessoryViewDisclosed(&self, accessoryViewDisclosed: bool);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl NSOpenPanel {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other filenames)]
        pub unsafe fn filenames(&self) -> Id<Foundation::NSArray, Shared>;

        #[cfg(all(
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(beginSheetForDirectory:file:types:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_types_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&Foundation::NSString>,
            name: Option<&Foundation::NSString>,
            fileTypes: Option<&Foundation::NSArray>,
            docWindow: Option<&AppKit::NSWindow>,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(beginForDirectory:file:types:modelessDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginForDirectory_file_types_modelessDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&Foundation::NSString>,
            name: Option<&Foundation::NSString>,
            fileTypes: Option<&Foundation::NSArray>,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(runModalForDirectory:file:types:)]
        pub unsafe fn runModalForDirectory_file_types(
            &self,
            path: Option<&Foundation::NSString>,
            name: Option<&Foundation::NSString>,
            fileTypes: Option<&Foundation::NSArray>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(runModalForTypes:)]
        pub unsafe fn runModalForTypes(&self, fileTypes: Option<&Foundation::NSArray>)
            -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSOpenPanel")]
    unsafe impl AppKit::NSOpenPanel {
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Option<Allocated<Self>>,
            contentRect: Foundation::NSRect,
            style: AppKit::NSWindowStyleMask,
            backingStoreType: AppKit::NSBackingStoreType,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSScreen")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Option<Allocated<Self>>,
            contentRect: Foundation::NSRect,
            style: AppKit::NSWindowStyleMask,
            backingStoreType: AppKit::NSBackingStoreType,
            flag: bool,
            screen: Option<&AppKit::NSScreen>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            contentViewController: &AppKit::NSViewController,
        ) -> Id<Self, Shared>;
    }
);
