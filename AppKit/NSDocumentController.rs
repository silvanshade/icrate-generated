//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDocumentController;

    unsafe impl ClassType for NSDocumentController {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDocumentController")]
    unsafe impl NSDocumentController {
        #[method_id(@__retain_semantics Other sharedDocumentController)]
        pub unsafe fn sharedDocumentController() -> Id<AppKit::NSDocumentController, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other documents)]
        pub unsafe fn documents(&self) -> Id<Foundation::NSArray<AppKit::NSDocument>, Shared>;

        #[cfg(feature = "AppKit_NSDocument")]
        #[method_id(@__retain_semantics Other currentDocument)]
        pub unsafe fn currentDocument(&self) -> Option<Id<AppKit::NSDocument, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currentDirectory)]
        pub unsafe fn currentDirectory(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other documentForURL:)]
        pub unsafe fn documentForURL(
            &self,
            url: &Foundation::NSURL,
        ) -> Option<Id<AppKit::NSDocument, Shared>>;

        #[cfg(all(feature = "AppKit_NSDocument", feature = "AppKit_NSWindow"))]
        #[method_id(@__retain_semantics Other documentForWindow:)]
        pub unsafe fn documentForWindow(
            &self,
            window: &AppKit::NSWindow,
        ) -> Option<Id<AppKit::NSDocument, Shared>>;

        #[cfg(feature = "AppKit_NSDocument")]
        #[method(addDocument:)]
        pub unsafe fn addDocument(&self, document: &AppKit::NSDocument);

        #[cfg(feature = "AppKit_NSDocument")]
        #[method(removeDocument:)]
        pub unsafe fn removeDocument(&self, document: &AppKit::NSDocument);

        #[method(newDocument:)]
        pub unsafe fn newDocument(&self, sender: Option<&Object>);

        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other openUntitledDocumentAndDisplay:error:_)]
        pub unsafe fn openUntitledDocumentAndDisplay_error(
            &self,
            displayDocument: bool,
        ) -> Result<Id<AppKit::NSDocument, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other makeUntitledDocumentOfType:error:_)]
        pub unsafe fn makeUntitledDocumentOfType_error(
            &self,
            typeName: &Foundation::NSString,
        ) -> Result<Id<AppKit::NSDocument, Shared>, Id<Foundation::NSError, Shared>>;

        #[method(openDocument:)]
        pub unsafe fn openDocument(&self, sender: Option<&Object>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLsFromRunningOpenPanel)]
        pub unsafe fn URLsFromRunningOpenPanel(
            &self,
        ) -> Option<Id<Foundation::NSArray<Foundation::NSURL>, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSOpenPanel",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(runModalOpenPanel:forTypes:)]
        pub unsafe fn runModalOpenPanel_forTypes(
            &self,
            openPanel: &AppKit::NSOpenPanel,
            types: Option<&Foundation::NSArray<Foundation::NSString>>,
        ) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method(beginOpenPanelWithCompletionHandler:)]
        pub unsafe fn beginOpenPanelWithCompletionHandler(
            &self,
            completionHandler: &Block<(*mut Foundation::NSArray<Foundation::NSURL>,), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSOpenPanel",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(beginOpenPanel:forTypes:completionHandler:)]
        pub unsafe fn beginOpenPanel_forTypes_completionHandler(
            &self,
            openPanel: &AppKit::NSOpenPanel,
            inTypes: Option<&Foundation::NSArray<Foundation::NSString>>,
            completionHandler: &Block<(NSInteger,), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(openDocumentWithContentsOfURL:display:completionHandler:)]
        pub unsafe fn openDocumentWithContentsOfURL_display_completionHandler(
            &self,
            url: &Foundation::NSURL,
            displayDocument: bool,
            completionHandler: &Block<
                (*mut AppKit::NSDocument, Bool, *mut Foundation::NSError),
                (),
            >,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other makeDocumentWithContentsOfURL:ofType:error:_)]
        pub unsafe fn makeDocumentWithContentsOfURL_ofType_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<Id<AppKit::NSDocument, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(reopenDocumentForURL:withContentsOfURL:display:completionHandler:)]
        pub unsafe fn reopenDocumentForURL_withContentsOfURL_display_completionHandler(
            &self,
            urlOrNil: Option<&Foundation::NSURL>,
            contentsURL: &Foundation::NSURL,
            displayDocument: bool,
            completionHandler: &Block<
                (*mut AppKit::NSDocument, Bool, *mut Foundation::NSError),
                (),
            >,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other makeDocumentForURL:withContentsOfURL:ofType:error:_)]
        pub unsafe fn makeDocumentForURL_withContentsOfURL_ofType_error(
            &self,
            urlOrNil: Option<&Foundation::NSURL>,
            contentsURL: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<Id<AppKit::NSDocument, Shared>, Id<Foundation::NSError, Shared>>;

        #[method(autosavingDelay)]
        pub unsafe fn autosavingDelay(&self) -> Foundation::NSTimeInterval;

        #[method(setAutosavingDelay:)]
        pub unsafe fn setAutosavingDelay(&self, autosavingDelay: Foundation::NSTimeInterval);

        #[method(saveAllDocuments:)]
        pub unsafe fn saveAllDocuments(&self, sender: Option<&Object>);

        #[method(hasEditedDocuments)]
        pub unsafe fn hasEditedDocuments(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(reviewUnsavedDocumentsWithAlertTitle:cancellable:delegate:didReviewAllSelector:contextInfo:)]
        pub unsafe fn reviewUnsavedDocumentsWithAlertTitle_cancellable_delegate_didReviewAllSelector_contextInfo(
            &self,
            title: Option<&Foundation::NSString>,
            cancellable: bool,
            delegate: Option<&Object>,
            didReviewAllSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(closeAllDocumentsWithDelegate:didCloseAllSelector:contextInfo:)]
        pub unsafe fn closeAllDocumentsWithDelegate_didCloseAllSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didCloseAllSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other duplicateDocumentWithContentsOfURL:copying:displayName:error:_)]
        pub unsafe fn duplicateDocumentWithContentsOfURL_copying_displayName_error(
            &self,
            url: &Foundation::NSURL,
            duplicateByCopying: bool,
            displayNameOrNil: Option<&Foundation::NSString>,
        ) -> Result<Id<AppKit::NSDocument, Shared>, Id<Foundation::NSError, Shared>>;

        #[method(allowsAutomaticShareMenu)]
        pub unsafe fn allowsAutomaticShareMenu(&self) -> bool;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other standardShareMenuItem)]
        pub unsafe fn standardShareMenuItem(&self) -> Id<AppKit::NSMenuItem, Shared>;

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSError"))]
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &Foundation::NSError,
            window: &AppKit::NSWindow,
            delegate: Option<&Object>,
            didPresentSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &Foundation::NSError) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other willPresentError:)]
        pub unsafe fn willPresentError(
            &self,
            error: &Foundation::NSError,
        ) -> Id<Foundation::NSError, Shared>;

        #[method(maximumRecentDocumentCount)]
        pub unsafe fn maximumRecentDocumentCount(&self) -> NSUInteger;

        #[method(clearRecentDocuments:)]
        pub unsafe fn clearRecentDocuments(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSDocument")]
        #[method(noteNewRecentDocument:)]
        pub unsafe fn noteNewRecentDocument(&self, document: &AppKit::NSDocument);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(noteNewRecentDocumentURL:)]
        pub unsafe fn noteNewRecentDocumentURL(&self, url: &Foundation::NSURL);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other recentDocumentURLs)]
        pub unsafe fn recentDocumentURLs(
            &self,
        ) -> Id<Foundation::NSArray<Foundation::NSURL>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultType)]
        pub unsafe fn defaultType(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other typeForContentsOfURL:error:_)]
        pub unsafe fn typeForContentsOfURL_error(
            &self,
            url: &Foundation::NSURL,
        ) -> Result<Id<Foundation::NSString, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other documentClassNames)]
        pub unsafe fn documentClassNames(
            &self,
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(documentClassForType:)]
        pub unsafe fn documentClassForType(
            &self,
            typeName: &Foundation::NSString,
        ) -> Option<&'static Class>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayNameForType:)]
        pub unsafe fn displayNameForType(
            &self,
            typeName: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &AppKit::NSValidatedUserInterfaceItem,
        ) -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSDocumentController")]
    unsafe impl NSDocumentController {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other openDocumentWithContentsOfURL:display:error:_)]
        pub unsafe fn openDocumentWithContentsOfURL_display_error(
            &self,
            url: &Foundation::NSURL,
            displayDocument: bool,
        ) -> Result<Id<Object, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(reopenDocumentForURL:withContentsOfURL:error:_)]
        pub unsafe fn reopenDocumentForURL_withContentsOfURL_error(
            &self,
            url: Option<&Foundation::NSURL>,
            contentsURL: &Foundation::NSURL,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileExtensionsFromType:)]
        pub unsafe fn fileExtensionsFromType(
            &self,
            typeName: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeFromFileExtension:)]
        pub unsafe fn typeFromFileExtension(
            &self,
            fileNameExtensionOrHFSFileType: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other documentForFileName:)]
        pub unsafe fn documentForFileName(
            &self,
            fileName: &Foundation::NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other fileNamesFromRunningOpenPanel)]
        pub unsafe fn fileNamesFromRunningOpenPanel(
            &self,
        ) -> Option<Id<Foundation::NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other makeDocumentWithContentsOfFile:ofType:)]
        pub unsafe fn makeDocumentWithContentsOfFile_ofType(
            &self,
            fileName: &Foundation::NSString,
            type_: &Foundation::NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other makeDocumentWithContentsOfURL:ofType:)]
        pub unsafe fn makeDocumentWithContentsOfURL_ofType(
            &self,
            url: &Foundation::NSURL,
            type_: Option<&Foundation::NSString>,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other makeUntitledDocumentOfType:)]
        pub unsafe fn makeUntitledDocumentOfType(
            &self,
            type_: &Foundation::NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other openDocumentWithContentsOfFile:display:)]
        pub unsafe fn openDocumentWithContentsOfFile_display(
            &self,
            fileName: &Foundation::NSString,
            display: bool,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other openDocumentWithContentsOfURL:display:)]
        pub unsafe fn openDocumentWithContentsOfURL_display(
            &self,
            url: &Foundation::NSURL,
            display: bool,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other openUntitledDocumentOfType:display:)]
        pub unsafe fn openUntitledDocumentOfType_display(
            &self,
            type_: &Foundation::NSString,
            display: bool,
        ) -> Option<Id<Object, Shared>>;

        #[method(setShouldCreateUI:)]
        pub unsafe fn setShouldCreateUI(&self, flag: bool);

        #[method(shouldCreateUI)]
        pub unsafe fn shouldCreateUI(&self) -> bool;
    }
);
