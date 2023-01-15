//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSavePanel")]
    pub struct NSSavePanel;

    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl ClassType for NSSavePanel {
        #[inherits(NSWindow, NSResponder, NSObject)]
        type Super = NSPanel;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[method_id(@__retain_semantics Other savePanel)]
        pub unsafe fn savePanel() -> Id<NSSavePanel, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other directoryURL)]
        pub unsafe fn directoryURL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setDirectoryURL:)]
        pub unsafe fn setDirectoryURL(&self, directoryURL: Option<&NSURL>);

        #[method(allowsOtherFileTypes)]
        pub unsafe fn allowsOtherFileTypes(&self) -> bool;

        #[method(setAllowsOtherFileTypes:)]
        pub unsafe fn setAllowsOtherFileTypes(&self, allowsOtherFileTypes: bool);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSOpenSavePanelDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSOpenSavePanelDelegate>);

        #[method(isExpanded)]
        pub unsafe fn isExpanded(&self) -> bool;

        #[method(canCreateDirectories)]
        pub unsafe fn canCreateDirectories(&self) -> bool;

        #[method(setCanCreateDirectories:)]
        pub unsafe fn setCanCreateDirectories(&self, canCreateDirectories: bool);

        #[method(canSelectHiddenExtension)]
        pub unsafe fn canSelectHiddenExtension(&self) -> bool;

        #[method(setCanSelectHiddenExtension:)]
        pub unsafe fn setCanSelectHiddenExtension(&self, canSelectHiddenExtension: bool);

        #[method(isExtensionHidden)]
        pub unsafe fn isExtensionHidden(&self) -> bool;

        #[method(setExtensionHidden:)]
        pub unsafe fn setExtensionHidden(&self, extensionHidden: bool);

        #[method(treatsFilePackagesAsDirectories)]
        pub unsafe fn treatsFilePackagesAsDirectories(&self) -> bool;

        #[method(setTreatsFilePackagesAsDirectories:)]
        pub unsafe fn setTreatsFilePackagesAsDirectories(
            &self,
            treatsFilePackagesAsDirectories: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other prompt)]
        pub unsafe fn prompt(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPrompt:)]
        pub unsafe fn setPrompt(&self, prompt: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameFieldLabel)]
        pub unsafe fn nameFieldLabel(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNameFieldLabel:)]
        pub unsafe fn setNameFieldLabel(&self, nameFieldLabel: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameFieldStringValue)]
        pub unsafe fn nameFieldStringValue(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNameFieldStringValue:)]
        pub unsafe fn setNameFieldStringValue(&self, nameFieldStringValue: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[method(validateVisibleColumns)]
        pub unsafe fn validateVisibleColumns(&self);

        #[method(showsHiddenFiles)]
        pub unsafe fn showsHiddenFiles(&self) -> bool;

        #[method(setShowsHiddenFiles:)]
        pub unsafe fn setShowsHiddenFiles(&self, showsHiddenFiles: bool);

        #[method(showsTagField)]
        pub unsafe fn showsTagField(&self) -> bool;

        #[method(setShowsTagField:)]
        pub unsafe fn setShowsTagField(&self, showsTagField: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other tagNames)]
        pub unsafe fn tagNames(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setTagNames:)]
        pub unsafe fn setTagNames(&self, tagNames: Option<&NSArray<NSString>>);

        #[method(ok:)]
        pub unsafe fn ok(&self, sender: Option<&Object>);

        #[method(cancel:)]
        pub unsafe fn cancel(&self, sender: Option<&Object>);

        #[method(beginSheetModalForWindow:completionHandler:)]
        pub unsafe fn beginSheetModalForWindow_completionHandler(
            &self,
            window: &NSWindow,
            handler: &Block<(NSModalResponse,), ()>,
        );

        #[method(beginWithCompletionHandler:)]
        pub unsafe fn beginWithCompletionHandler(&self, handler: &Block<(NSModalResponse,), ()>);

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSModalResponse;
    }
);

extern_protocol!(
    pub struct NSOpenSavePanelDelegate;

    unsafe impl ProtocolType for NSOpenSavePanelDelegate {
        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(panel:shouldEnableURL:)]
        pub unsafe fn panel_shouldEnableURL(&self, sender: &Object, url: &NSURL) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(panel:validateURL:error:_)]
        pub unsafe fn panel_validateURL_error(
            &self,
            sender: &Object,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(panel:didChangeToDirectoryURL:)]
        pub unsafe fn panel_didChangeToDirectoryURL(&self, sender: &Object, url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSString")]
        #[optional]
        #[method_id(@__retain_semantics Other panel:userEnteredFilename:confirmed:)]
        pub unsafe fn panel_userEnteredFilename_confirmed(
            &self,
            sender: &Object,
            filename: &NSString,
            okFlag: bool,
        ) -> Option<Id<NSString, Shared>>;

        #[optional]
        #[method(panel:willExpand:)]
        pub unsafe fn panel_willExpand(&self, sender: &Object, expanding: bool);

        #[optional]
        #[method(panelSelectionDidChange:)]
        pub unsafe fn panelSelectionDidChange(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -URL instead"]
        #[method_id(@__retain_semantics Other filename)]
        pub unsafe fn filename(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -directoryURL instead"]
        #[method_id(@__retain_semantics Other directory)]
        pub unsafe fn directory(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -setDirectoryURL: instead"]
        #[method(setDirectory:)]
        pub unsafe fn setDirectory(&self, path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -allowedFileTypes instead"]
        #[method_id(@__retain_semantics Other requiredFileType)]
        pub unsafe fn requiredFileType(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -setAllowedFileTypes: instead"]
        #[method(setRequiredFileType:)]
        pub unsafe fn setRequiredFileType(&self, r#type: Option<&NSString>);

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSString"))]
        #[deprecated = "Use beginSheetModalForWindow:completionHandler: instead. The following parameters are replaced by properties: 'path' is replaced by 'directoryURL' and 'name' by 'nameFieldStringValue'."]
        #[method(beginSheetForDirectory:file:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: &NSString,
            name: Option<&NSString>,
            docWindow: Option<&NSWindow>,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -runModal instead. The following parameters are replaced by properties: 'path' is replaced by 'directoryURL' and 'name' by 'nameFieldStringValue'."]
        #[method(runModalForDirectory:file:)]
        pub unsafe fn runModalForDirectory_file(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
        ) -> NSInteger;

        #[deprecated = "Default implementation does nothing."]
        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&Object>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -allowedContentTypes instead"]
        #[method_id(@__retain_semantics Other allowedFileTypes)]
        pub unsafe fn allowedFileTypes(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -allowedContentTypes instead"]
        #[method(setAllowedFileTypes:)]
        pub unsafe fn setAllowedFileTypes(&self, allowedFileTypes: Option<&NSArray<NSString>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSSavePanel")]
    unsafe impl NSSavePanel {
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Option<Allocated<Self>>,
            contentRect: NSRect,
            style: NSWindowStyleMask,
            backingStoreType: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSScreen")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Option<Allocated<Self>>,
            contentRect: NSRect,
            style: NSWindowStyleMask,
            backingStoreType: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            contentViewController: &NSViewController,
        ) -> Id<Self, Shared>;
    }
);
