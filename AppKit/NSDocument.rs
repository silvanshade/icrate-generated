//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDocumentChangeType {
        NSChangeDone = 0,
        NSChangeUndone = 1,
        NSChangeRedone = 5,
        NSChangeCleared = 2,
        NSChangeReadOtherContents = 3,
        NSChangeAutosaved = 4,
        NSChangeDiscardable = 256,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSaveOperationType {
        NSSaveOperation = 0,
        NSSaveAsOperation = 1,
        NSSaveToOperation = 2,
        NSAutosaveInPlaceOperation = 4,
        NSAutosaveElsewhereOperation = 3,
        NSAutosaveAsOperation = 5,
        NSAutosaveOperation = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDocument;

    unsafe impl ClassType for NSDocument {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithType:error:_)]
        pub unsafe fn initWithType_error(
            this: Option<Allocated<Self>>,
            typeName: &Foundation::NSString,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(canConcurrentlyReadDocumentsOfType:)]
        pub unsafe fn canConcurrentlyReadDocumentsOfType(typeName: &Foundation::NSString) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:error:_)]
        pub unsafe fn initWithContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initForURL:withContentsOfURL:ofType:error:_)]
        pub unsafe fn initForURL_withContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            urlOrNil: Option<&Foundation::NSURL>,
            contentsURL: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, fileType: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<Foundation::NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setFileURL:)]
        pub unsafe fn setFileURL(&self, fileURL: Option<&Foundation::NSURL>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other fileModificationDate)]
        pub unsafe fn fileModificationDate(&self) -> Option<Id<Foundation::NSDate, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setFileModificationDate:)]
        pub unsafe fn setFileModificationDate(
            &self,
            fileModificationDate: Option<&Foundation::NSDate>,
        );

        #[method(isDraft)]
        pub unsafe fn isDraft(&self) -> bool;

        #[method(setDraft:)]
        pub unsafe fn setDraft(&self, draft: bool);

        #[method(performActivityWithSynchronousWaiting:usingBlock:)]
        pub unsafe fn performActivityWithSynchronousWaiting_usingBlock(
            &self,
            waitSynchronously: bool,
            block: &Block<(NonNull<Block<(), ()>>,), ()>,
        );

        #[method(continueActivityUsingBlock:)]
        pub unsafe fn continueActivityUsingBlock(&self, block: &Block<(), ()>);

        #[method(continueAsynchronousWorkOnMainThreadUsingBlock:)]
        pub unsafe fn continueAsynchronousWorkOnMainThreadUsingBlock(&self, block: &Block<(), ()>);

        #[method(performSynchronousFileAccessUsingBlock:)]
        pub unsafe fn performSynchronousFileAccessUsingBlock(&self, block: &Block<(), ()>);

        #[method(performAsynchronousFileAccessUsingBlock:)]
        pub unsafe fn performAsynchronousFileAccessUsingBlock(
            &self,
            block: &Block<(NonNull<Block<(), ()>>,), ()>,
        );

        #[method(revertDocumentToSaved:)]
        pub unsafe fn revertDocumentToSaved(&self, sender: Option<&Object>);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(revertToContentsOfURL:ofType:error:_)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(readFromURL:ofType:error:_)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileWrapper",
            feature = "Foundation_NSString"
        ))]
        #[method(readFromFileWrapper:ofType:error:_)]
        pub unsafe fn readFromFileWrapper_ofType_error(
            &self,
            fileWrapper: &Foundation::NSFileWrapper,
            typeName: &Foundation::NSString,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(readFromData:ofType:error:_)]
        pub unsafe fn readFromData_ofType_error(
            &self,
            data: &Foundation::NSData,
            typeName: &Foundation::NSString,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[method(isEntireFileLoaded)]
        pub unsafe fn isEntireFileLoaded(&self) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:error:_)]
        pub unsafe fn writeToURL_ofType_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileWrapper",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fileWrapperOfType:error:_)]
        pub unsafe fn fileWrapperOfType_error(
            &self,
            typeName: &Foundation::NSString,
        ) -> Result<Id<Foundation::NSFileWrapper, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other dataOfType:error:_)]
        pub unsafe fn dataOfType_error(
            &self,
            typeName: &Foundation::NSString,
        ) -> Result<Id<Foundation::NSData, Shared>, Id<Foundation::NSError, Shared>>;

        #[method(unblockUserInteraction)]
        pub unsafe fn unblockUserInteraction(&self);

        #[method(autosavingIsImplicitlyCancellable)]
        pub unsafe fn autosavingIsImplicitlyCancellable(&self) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeSafelyToURL:ofType:forSaveOperation:error:_)]
        pub unsafe fn writeSafelyToURL_ofType_forSaveOperation_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
            absoluteOriginalContentsURL: Option<&Foundation::NSURL>,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other fileAttributesToWriteToURL:ofType:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn fileAttributesToWriteToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
            absoluteOriginalContentsURL: Option<&Foundation::NSURL>,
        ) -> Result<
            Id<Foundation::NSDictionary<Foundation::NSString, Object>, Shared>,
            Id<Foundation::NSError, Shared>,
        >;

        #[method(keepBackupFile)]
        pub unsafe fn keepBackupFile(&self) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other backupFileURL)]
        pub unsafe fn backupFileURL(&self) -> Option<Id<Foundation::NSURL, Shared>>;

        #[method(saveDocument:)]
        pub unsafe fn saveDocument(&self, sender: Option<&Object>);

        #[method(saveDocumentAs:)]
        pub unsafe fn saveDocumentAs(&self, sender: Option<&Object>);

        #[method(saveDocumentTo:)]
        pub unsafe fn saveDocumentTo(&self, sender: Option<&Object>);

        #[method(saveDocumentWithDelegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveDocumentWithDelegate_didSaveSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(runModalSavePanelForSaveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn runModalSavePanelForSaveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            saveOperation: AppKit::NSSaveOperationType,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(shouldRunSavePanelWithAccessoryView)]
        pub unsafe fn shouldRunSavePanelWithAccessoryView(&self) -> bool;

        #[cfg(feature = "AppKit_NSSavePanel")]
        #[method(prepareSavePanel:)]
        pub unsafe fn prepareSavePanel(&self, savePanel: &AppKit::NSSavePanel) -> bool;

        #[method(fileNameExtensionWasHiddenInLastRunSavePanel)]
        pub unsafe fn fileNameExtensionWasHiddenInLastRunSavePanel(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileTypeFromLastRunSavePanel)]
        pub unsafe fn fileTypeFromLastRunSavePanel(
            &self,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(saveToURL:ofType:forSaveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(saveToURL:ofType:forSaveOperation:completionHandler:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_completionHandler(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
            completionHandler: &Block<(*mut Foundation::NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(canAsynchronouslyWriteToURL:ofType:forSaveOperation:)]
        pub unsafe fn canAsynchronouslyWriteToURL_ofType_forSaveOperation(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method(checkAutosavingSafetyAndReturnError:_)]
        pub unsafe fn checkAutosavingSafetyAndReturnError(
            &self,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[method(scheduleAutosaving)]
        pub unsafe fn scheduleAutosaving(&self);

        #[method(hasUnautosavedChanges)]
        pub unsafe fn hasUnautosavedChanges(&self) -> bool;

        #[method(autosaveDocumentWithDelegate:didAutosaveSelector:contextInfo:)]
        pub unsafe fn autosaveDocumentWithDelegate_didAutosaveSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didAutosaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(autosaveWithImplicitCancellability:completionHandler:)]
        pub unsafe fn autosaveWithImplicitCancellability_completionHandler(
            &self,
            autosavingIsImplicitlyCancellable: bool,
            completionHandler: &Block<(*mut Foundation::NSError,), ()>,
        );

        #[method(autosavesInPlace)]
        pub unsafe fn autosavesInPlace() -> bool;

        #[method(preservesVersions)]
        pub unsafe fn preservesVersions() -> bool;

        #[method(browseDocumentVersions:)]
        pub unsafe fn browseDocumentVersions(&self, sender: Option<&Object>);

        #[method(isBrowsingVersions)]
        pub unsafe fn isBrowsingVersions(&self) -> bool;

        #[method(stopBrowsingVersionsWithCompletionHandler:)]
        pub unsafe fn stopBrowsingVersionsWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(), ()>>,
        );

        #[method(autosavesDrafts)]
        pub unsafe fn autosavesDrafts() -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other autosavingFileType)]
        pub unsafe fn autosavingFileType(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other autosavedContentsFileURL)]
        pub unsafe fn autosavedContentsFileURL(&self) -> Option<Id<Foundation::NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setAutosavedContentsFileURL:)]
        pub unsafe fn setAutosavedContentsFileURL(
            &self,
            autosavedContentsFileURL: Option<&Foundation::NSURL>,
        );

        #[method(canCloseDocumentWithDelegate:shouldCloseSelector:contextInfo:)]
        pub unsafe fn canCloseDocumentWithDelegate_shouldCloseSelector_contextInfo(
            &self,
            delegate: &Object,
            shouldCloseSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(close)]
        pub unsafe fn close(&self);

        #[method(duplicateDocument:)]
        pub unsafe fn duplicateDocument(&self, sender: Option<&Object>);

        #[method(duplicateDocumentWithDelegate:didDuplicateSelector:contextInfo:)]
        pub unsafe fn duplicateDocumentWithDelegate_didDuplicateSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didDuplicateSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other duplicateAndReturnError:_)]
        pub unsafe fn duplicateAndReturnError(
            &self,
        ) -> Result<Id<AppKit::NSDocument, Shared>, Id<Foundation::NSError, Shared>>;

        #[method(renameDocument:)]
        pub unsafe fn renameDocument(&self, sender: Option<&Object>);

        #[method(moveDocumentToUbiquityContainer:)]
        pub unsafe fn moveDocumentToUbiquityContainer(&self, sender: Option<&Object>);

        #[method(moveDocument:)]
        pub unsafe fn moveDocument(&self, sender: Option<&Object>);

        #[method(moveDocumentWithCompletionHandler:)]
        pub unsafe fn moveDocumentWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(moveToURL:completionHandler:)]
        pub unsafe fn moveToURL_completionHandler(
            &self,
            url: &Foundation::NSURL,
            completionHandler: Option<&Block<(*mut Foundation::NSError,), ()>>,
        );

        #[method(lockDocument:)]
        pub unsafe fn lockDocument(&self, sender: Option<&Object>);

        #[method(unlockDocument:)]
        pub unsafe fn unlockDocument(&self, sender: Option<&Object>);

        #[method(lockDocumentWithCompletionHandler:)]
        pub unsafe fn lockDocumentWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(lockWithCompletionHandler:)]
        pub unsafe fn lockWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(*mut Foundation::NSError,), ()>>,
        );

        #[method(unlockDocumentWithCompletionHandler:)]
        pub unsafe fn unlockDocumentWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(unlockWithCompletionHandler:)]
        pub unsafe fn unlockWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(*mut Foundation::NSError,), ()>>,
        );

        #[method(isLocked)]
        pub unsafe fn isLocked(&self) -> bool;

        #[method(runPageLayout:)]
        pub unsafe fn runPageLayout(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(runModalPageLayoutWithPrintInfo:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runModalPageLayoutWithPrintInfo_delegate_didRunSelector_contextInfo(
            &self,
            printInfo: &AppKit::NSPrintInfo,
            delegate: Option<&Object>,
            didRunSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSPageLayout")]
        #[method(preparePageLayout:)]
        pub unsafe fn preparePageLayout(&self, pageLayout: &AppKit::NSPageLayout) -> bool;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(shouldChangePrintInfo:)]
        pub unsafe fn shouldChangePrintInfo(&self, newPrintInfo: &AppKit::NSPrintInfo) -> bool;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Id<AppKit::NSPrintInfo, Shared>;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, printInfo: &AppKit::NSPrintInfo);

        #[method(printDocument:)]
        pub unsafe fn printDocument(&self, sender: Option<&Object>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(printDocumentWithSettings:showPrintPanel:delegate:didPrintSelector:contextInfo:)]
        pub unsafe fn printDocumentWithSettings_showPrintPanel_delegate_didPrintSelector_contextInfo(
            &self,
            printSettings: &Foundation::NSDictionary<AppKit::NSPrintInfoAttributeKey, Object>,
            showPrintPanel: bool,
            delegate: Option<&Object>,
            didPrintSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(all(
            feature = "AppKit_NSPrintOperation",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other printOperationWithSettings:error:_)]
        pub unsafe fn printOperationWithSettings_error(
            &self,
            printSettings: &Foundation::NSDictionary<AppKit::NSPrintInfoAttributeKey, Object>,
        ) -> Result<Id<AppKit::NSPrintOperation, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "AppKit_NSPrintOperation")]
        #[method(runModalPrintOperation:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runModalPrintOperation_delegate_didRunSelector_contextInfo(
            &self,
            printOperation: &AppKit::NSPrintOperation,
            delegate: Option<&Object>,
            didRunSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(saveDocumentToPDF:)]
        pub unsafe fn saveDocumentToPDF(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSPrintOperation")]
        #[method_id(@__retain_semantics Other PDFPrintOperation)]
        pub unsafe fn PDFPrintOperation(&self) -> Id<AppKit::NSPrintOperation, Shared>;

        #[method(allowsDocumentSharing)]
        pub unsafe fn allowsDocumentSharing(&self) -> bool;

        #[cfg(feature = "AppKit_NSSharingService")]
        #[method(shareDocumentWithSharingService:completionHandler:)]
        pub unsafe fn shareDocumentWithSharingService_completionHandler(
            &self,
            sharingService: &AppKit::NSSharingService,
            completionHandler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "AppKit_NSSharingServicePicker")]
        #[method(prepareSharingServicePicker:)]
        pub unsafe fn prepareSharingServicePicker(
            &self,
            sharingServicePicker: &AppKit::NSSharingServicePicker,
        );

        #[method(isDocumentEdited)]
        pub unsafe fn isDocumentEdited(&self) -> bool;

        #[method(isInViewingMode)]
        pub unsafe fn isInViewingMode(&self) -> bool;

        #[method(updateChangeCount:)]
        pub unsafe fn updateChangeCount(&self, change: AppKit::NSDocumentChangeType);

        #[method_id(@__retain_semantics Other changeCountTokenForSaveOperation:)]
        pub unsafe fn changeCountTokenForSaveOperation(
            &self,
            saveOperation: AppKit::NSSaveOperationType,
        ) -> Id<Object, Shared>;

        #[method(updateChangeCountWithToken:forSaveOperation:)]
        pub unsafe fn updateChangeCountWithToken_forSaveOperation(
            &self,
            changeCountToken: &Object,
            saveOperation: AppKit::NSSaveOperationType,
        );

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<Foundation::NSUndoManager, Shared>>;

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method(setUndoManager:)]
        pub unsafe fn setUndoManager(&self, undoManager: Option<&Foundation::NSUndoManager>);

        #[method(hasUndoManager)]
        pub unsafe fn hasUndoManager(&self) -> bool;

        #[method(setHasUndoManager:)]
        pub unsafe fn setHasUndoManager(&self, hasUndoManager: bool);

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

        #[cfg(feature = "Foundation_NSError")]
        #[method(willNotPresentError:)]
        pub unsafe fn willNotPresentError(&self, error: &Foundation::NSError);

        #[method(makeWindowControllers)]
        pub unsafe fn makeWindowControllers(&self);

        #[method_id(@__retain_semantics Other windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Id<AppKit::NSNibName, Shared>>;

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(windowControllerWillLoadNib:)]
        pub unsafe fn windowControllerWillLoadNib(
            &self,
            windowController: &AppKit::NSWindowController,
        );

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(windowControllerDidLoadNib:)]
        pub unsafe fn windowControllerDidLoadNib(
            &self,
            windowController: &AppKit::NSWindowController,
        );

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setWindow:)]
        pub unsafe fn setWindow(&self, window: Option<&AppKit::NSWindow>);

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(addWindowController:)]
        pub unsafe fn addWindowController(&self, windowController: &AppKit::NSWindowController);

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(removeWindowController:)]
        pub unsafe fn removeWindowController(&self, windowController: &AppKit::NSWindowController);

        #[method(showWindows)]
        pub unsafe fn showWindows(&self);

        #[cfg(all(feature = "AppKit_NSWindowController", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other windowControllers)]
        pub unsafe fn windowControllers(
            &self,
        ) -> Id<Foundation::NSArray<AppKit::NSWindowController>, Shared>;

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(shouldCloseWindowController:delegate:shouldCloseSelector:contextInfo:)]
        pub unsafe fn shouldCloseWindowController_delegate_shouldCloseSelector_contextInfo(
            &self,
            windowController: &AppKit::NSWindowController,
            delegate: Option<&Object>,
            shouldCloseSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultDraftName)]
        pub unsafe fn defaultDraftName(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other windowForSheet)]
        pub unsafe fn windowForSheet(&self) -> Option<Id<AppKit::NSWindow, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other readableTypes)]
        pub unsafe fn readableTypes() -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other writableTypes)]
        pub unsafe fn writableTypes() -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isNativeType:)]
        pub unsafe fn isNativeType(type_: &Foundation::NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other writableTypesForSaveOperation:)]
        pub unsafe fn writableTypesForSaveOperation(
            &self,
            saveOperation: AppKit::NSSaveOperationType,
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileNameExtensionForType:saveOperation:)]
        pub unsafe fn fileNameExtensionForType_saveOperation(
            &self,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &AppKit::NSValidatedUserInterfaceItem,
        ) -> bool;

        #[method(usesUbiquitousStorage)]
        pub unsafe fn usesUbiquitousStorage() -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(saveToURL:ofType:forSaveOperation:error:_)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_error(
            &self,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataRepresentationOfType:)]
        pub unsafe fn dataRepresentationOfType(
            &self,
            type_: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileAttributesToWriteToFile:ofType:saveOperation:)]
        pub unsafe fn fileAttributesToWriteToFile_ofType_saveOperation(
            &self,
            fullDocumentPath: &Foundation::NSString,
            documentTypeName: &Foundation::NSString,
            saveOperationType: AppKit::NSSaveOperationType,
        ) -> Option<Id<Foundation::NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileName)]
        pub unsafe fn fileName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSFileWrapper", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileWrapperRepresentationOfType:)]
        pub unsafe fn fileWrapperRepresentationOfType(
            &self,
            type_: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSFileWrapper, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:ofType:)]
        pub unsafe fn initWithContentsOfFile_ofType(
            this: Option<Allocated<Self>>,
            absolutePath: &Foundation::NSString,
            typeName: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:)]
        pub unsafe fn initWithContentsOfURL_ofType(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            typeName: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method(loadDataRepresentation:ofType:)]
        pub unsafe fn loadDataRepresentation_ofType(
            &self,
            data: &Foundation::NSData,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSFileWrapper", feature = "Foundation_NSString"))]
        #[method(loadFileWrapperRepresentation:ofType:)]
        pub unsafe fn loadFileWrapperRepresentation_ofType(
            &self,
            wrapper: &Foundation::NSFileWrapper,
            type_: &Foundation::NSString,
        ) -> bool;

        #[method(printShowingPrintPanel:)]
        pub unsafe fn printShowingPrintPanel(&self, flag: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method(readFromFile:ofType:)]
        pub unsafe fn readFromFile_ofType(
            &self,
            fileName: &Foundation::NSString,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(readFromURL:ofType:)]
        pub unsafe fn readFromURL_ofType(
            &self,
            url: &Foundation::NSURL,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(revertToSavedFromFile:ofType:)]
        pub unsafe fn revertToSavedFromFile_ofType(
            &self,
            fileName: &Foundation::NSString,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(revertToSavedFromURL:ofType:)]
        pub unsafe fn revertToSavedFromURL_ofType(
            &self,
            url: &Foundation::NSURL,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(runModalPageLayoutWithPrintInfo:)]
        pub unsafe fn runModalPageLayoutWithPrintInfo(
            &self,
            printInfo: &AppKit::NSPrintInfo,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(saveToFile:saveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveToFile_saveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            fileName: &Foundation::NSString,
            saveOperation: AppKit::NSSaveOperationType,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileName:)]
        pub unsafe fn setFileName(&self, fileName: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:ofType:)]
        pub unsafe fn writeToFile_ofType(
            &self,
            fileName: &Foundation::NSString,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:ofType:originalFile:saveOperation:)]
        pub unsafe fn writeToFile_ofType_originalFile_saveOperation(
            &self,
            fullDocumentPath: &Foundation::NSString,
            documentTypeName: &Foundation::NSString,
            fullOriginalDocumentPath: Option<&Foundation::NSString>,
            saveOperationType: AppKit::NSSaveOperationType,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(writeToURL:ofType:)]
        pub unsafe fn writeToURL_ofType(
            &self,
            url: &Foundation::NSURL,
            type_: &Foundation::NSString,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeWithBackupToFile:ofType:saveOperation:)]
        pub unsafe fn writeWithBackupToFile_ofType_saveOperation(
            &self,
            fullDocumentPath: &Foundation::NSString,
            documentTypeName: &Foundation::NSString,
            saveOperationType: AppKit::NSSaveOperationType,
        ) -> bool;
    }
);
