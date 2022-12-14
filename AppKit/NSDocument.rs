//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

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
    unsafe impl NSDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithType:error:)]
        pub unsafe fn initWithType_error(
            this: Option<Allocated<Self>>,
            typeName: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method(canConcurrentlyReadDocumentsOfType:)]
        pub unsafe fn canConcurrentlyReadDocumentsOfType(typeName: &NSString) -> bool;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:error:)]
        pub unsafe fn initWithContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            typeName: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initForURL:withContentsOfURL:ofType:error:)]
        pub unsafe fn initForURL_withContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            urlOrNil: Option<&NSURL>,
            contentsURL: &NSURL,
            typeName: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString, Shared>>;

        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, fileType: Option<&NSString>);

        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setFileURL:)]
        pub unsafe fn setFileURL(&self, fileURL: Option<&NSURL>);

        #[method_id(@__retain_semantics Other fileModificationDate)]
        pub unsafe fn fileModificationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(setFileModificationDate:)]
        pub unsafe fn setFileModificationDate(&self, fileModificationDate: Option<&NSDate>);

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

        #[method(revertToContentsOfURL:ofType:error:)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(readFromURL:ofType:error:)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(readFromFileWrapper:ofType:error:)]
        pub unsafe fn readFromFileWrapper_ofType_error(
            &self,
            fileWrapper: &NSFileWrapper,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(readFromData:ofType:error:)]
        pub unsafe fn readFromData_ofType_error(
            &self,
            data: &NSData,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(isEntireFileLoaded)]
        pub unsafe fn isEntireFileLoaded(&self) -> bool;

        #[method(writeToURL:ofType:error:)]
        pub unsafe fn writeToURL_ofType_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileWrapperOfType:error:)]
        pub unsafe fn fileWrapperOfType_error(
            &self,
            typeName: &NSString,
        ) -> Result<Id<NSFileWrapper, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other dataOfType:error:)]
        pub unsafe fn dataOfType_error(
            &self,
            typeName: &NSString,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;

        #[method(unblockUserInteraction)]
        pub unsafe fn unblockUserInteraction(&self);

        #[method(autosavingIsImplicitlyCancellable)]
        pub unsafe fn autosavingIsImplicitlyCancellable(&self) -> bool;

        #[method(writeSafelyToURL:ofType:forSaveOperation:error:)]
        pub unsafe fn writeSafelyToURL_ofType_forSaveOperation_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
            absoluteOriginalContentsURL: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileAttributesToWriteToURL:ofType:forSaveOperation:originalContentsURL:error:)]
        pub unsafe fn fileAttributesToWriteToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
            absoluteOriginalContentsURL: Option<&NSURL>,
        ) -> Result<Id<NSDictionary<NSString, Object>, Shared>, Id<NSError, Shared>>;

        #[method(keepBackupFile)]
        pub unsafe fn keepBackupFile(&self) -> bool;

        #[method_id(@__retain_semantics Other backupFileURL)]
        pub unsafe fn backupFileURL(&self) -> Option<Id<NSURL, Shared>>;

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
            saveOperation: NSSaveOperationType,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(shouldRunSavePanelWithAccessoryView)]
        pub unsafe fn shouldRunSavePanelWithAccessoryView(&self) -> bool;

        #[method(prepareSavePanel:)]
        pub unsafe fn prepareSavePanel(&self, savePanel: &NSSavePanel) -> bool;

        #[method(fileNameExtensionWasHiddenInLastRunSavePanel)]
        pub unsafe fn fileNameExtensionWasHiddenInLastRunSavePanel(&self) -> bool;

        #[method_id(@__retain_semantics Other fileTypeFromLastRunSavePanel)]
        pub unsafe fn fileTypeFromLastRunSavePanel(&self) -> Option<Id<NSString, Shared>>;

        #[method(saveToURL:ofType:forSaveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(saveToURL:ofType:forSaveOperation:completionHandler:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_completionHandler(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
            completionHandler: &Block<(*mut NSError,), ()>,
        );

        #[method(canAsynchronouslyWriteToURL:ofType:forSaveOperation:)]
        pub unsafe fn canAsynchronouslyWriteToURL_ofType_forSaveOperation(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
        ) -> bool;

        #[method(checkAutosavingSafetyAndReturnError:)]
        pub unsafe fn checkAutosavingSafetyAndReturnError(&self)
            -> Result<(), Id<NSError, Shared>>;

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

        #[method(autosaveWithImplicitCancellability:completionHandler:)]
        pub unsafe fn autosaveWithImplicitCancellability_completionHandler(
            &self,
            autosavingIsImplicitlyCancellable: bool,
            completionHandler: &Block<(*mut NSError,), ()>,
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

        #[method_id(@__retain_semantics Other autosavingFileType)]
        pub unsafe fn autosavingFileType(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other autosavedContentsFileURL)]
        pub unsafe fn autosavedContentsFileURL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setAutosavedContentsFileURL:)]
        pub unsafe fn setAutosavedContentsFileURL(&self, autosavedContentsFileURL: Option<&NSURL>);

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

        #[method_id(@__retain_semantics Other duplicateAndReturnError:)]
        pub unsafe fn duplicateAndReturnError(
            &self,
        ) -> Result<Id<NSDocument, Shared>, Id<NSError, Shared>>;

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

        #[method(moveToURL:completionHandler:)]
        pub unsafe fn moveToURL_completionHandler(
            &self,
            url: &NSURL,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
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

        #[method(lockWithCompletionHandler:)]
        pub unsafe fn lockWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(unlockDocumentWithCompletionHandler:)]
        pub unsafe fn unlockDocumentWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(Bool,), ()>>,
        );

        #[method(unlockWithCompletionHandler:)]
        pub unsafe fn unlockWithCompletionHandler(
            &self,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(isLocked)]
        pub unsafe fn isLocked(&self) -> bool;

        #[method(runPageLayout:)]
        pub unsafe fn runPageLayout(&self, sender: Option<&Object>);

        #[method(runModalPageLayoutWithPrintInfo:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runModalPageLayoutWithPrintInfo_delegate_didRunSelector_contextInfo(
            &self,
            printInfo: &NSPrintInfo,
            delegate: Option<&Object>,
            didRunSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(preparePageLayout:)]
        pub unsafe fn preparePageLayout(&self, pageLayout: &NSPageLayout) -> bool;

        #[method(shouldChangePrintInfo:)]
        pub unsafe fn shouldChangePrintInfo(&self, newPrintInfo: &NSPrintInfo) -> bool;

        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo, Shared>;

        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, printInfo: &NSPrintInfo);

        #[method(printDocument:)]
        pub unsafe fn printDocument(&self, sender: Option<&Object>);

        #[method(printDocumentWithSettings:showPrintPanel:delegate:didPrintSelector:contextInfo:)]
        pub unsafe fn printDocumentWithSettings_showPrintPanel_delegate_didPrintSelector_contextInfo(
            &self,
            printSettings: &NSDictionary<NSPrintInfoAttributeKey, Object>,
            showPrintPanel: bool,
            delegate: Option<&Object>,
            didPrintSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method_id(@__retain_semantics Other printOperationWithSettings:error:)]
        pub unsafe fn printOperationWithSettings_error(
            &self,
            printSettings: &NSDictionary<NSPrintInfoAttributeKey, Object>,
        ) -> Result<Id<NSPrintOperation, Shared>, Id<NSError, Shared>>;

        #[method(runModalPrintOperation:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runModalPrintOperation_delegate_didRunSelector_contextInfo(
            &self,
            printOperation: &NSPrintOperation,
            delegate: Option<&Object>,
            didRunSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(saveDocumentToPDF:)]
        pub unsafe fn saveDocumentToPDF(&self, sender: Option<&Object>);

        #[method_id(@__retain_semantics Other PDFPrintOperation)]
        pub unsafe fn PDFPrintOperation(&self) -> Id<NSPrintOperation, Shared>;

        #[method(allowsDocumentSharing)]
        pub unsafe fn allowsDocumentSharing(&self) -> bool;

        #[method(shareDocumentWithSharingService:completionHandler:)]
        pub unsafe fn shareDocumentWithSharingService_completionHandler(
            &self,
            sharingService: &NSSharingService,
            completionHandler: Option<&Block<(Bool,), ()>>,
        );

        #[method(prepareSharingServicePicker:)]
        pub unsafe fn prepareSharingServicePicker(
            &self,
            sharingServicePicker: &NSSharingServicePicker,
        );

        #[method(isDocumentEdited)]
        pub unsafe fn isDocumentEdited(&self) -> bool;

        #[method(isInViewingMode)]
        pub unsafe fn isInViewingMode(&self) -> bool;

        #[method(updateChangeCount:)]
        pub unsafe fn updateChangeCount(&self, change: NSDocumentChangeType);

        #[method_id(@__retain_semantics Other changeCountTokenForSaveOperation:)]
        pub unsafe fn changeCountTokenForSaveOperation(
            &self,
            saveOperation: NSSaveOperationType,
        ) -> Id<Object, Shared>;

        #[method(updateChangeCountWithToken:forSaveOperation:)]
        pub unsafe fn updateChangeCountWithToken_forSaveOperation(
            &self,
            changeCountToken: &Object,
            saveOperation: NSSaveOperationType,
        );

        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager, Shared>>;

        #[method(setUndoManager:)]
        pub unsafe fn setUndoManager(&self, undoManager: Option<&NSUndoManager>);

        #[method(hasUndoManager)]
        pub unsafe fn hasUndoManager(&self) -> bool;

        #[method(setHasUndoManager:)]
        pub unsafe fn setHasUndoManager(&self, hasUndoManager: bool);

        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&Object>,
            didPresentSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[method_id(@__retain_semantics Other willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Id<NSError, Shared>;

        #[method(willNotPresentError:)]
        pub unsafe fn willNotPresentError(&self, error: &NSError);

        #[method(makeWindowControllers)]
        pub unsafe fn makeWindowControllers(&self);

        #[method_id(@__retain_semantics Other windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Id<NSNibName, Shared>>;

        #[method(windowControllerWillLoadNib:)]
        pub unsafe fn windowControllerWillLoadNib(&self, windowController: &NSWindowController);

        #[method(windowControllerDidLoadNib:)]
        pub unsafe fn windowControllerDidLoadNib(&self, windowController: &NSWindowController);

        #[method(setWindow:)]
        pub unsafe fn setWindow(&self, window: Option<&NSWindow>);

        #[method(addWindowController:)]
        pub unsafe fn addWindowController(&self, windowController: &NSWindowController);

        #[method(removeWindowController:)]
        pub unsafe fn removeWindowController(&self, windowController: &NSWindowController);

        #[method(showWindows)]
        pub unsafe fn showWindows(&self);

        #[method_id(@__retain_semantics Other windowControllers)]
        pub unsafe fn windowControllers(&self) -> Id<NSArray<NSWindowController>, Shared>;

        #[method(shouldCloseWindowController:delegate:shouldCloseSelector:contextInfo:)]
        pub unsafe fn shouldCloseWindowController_delegate_shouldCloseSelector_contextInfo(
            &self,
            windowController: &NSWindowController,
            delegate: Option<&Object>,
            shouldCloseSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other defaultDraftName)]
        pub unsafe fn defaultDraftName(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other windowForSheet)]
        pub unsafe fn windowForSheet(&self) -> Option<Id<NSWindow, Shared>>;

        #[method_id(@__retain_semantics Other readableTypes)]
        pub unsafe fn readableTypes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other writableTypes)]
        pub unsafe fn writableTypes() -> Id<NSArray<NSString>, Shared>;

        #[method(isNativeType:)]
        pub unsafe fn isNativeType(type_: &NSString) -> bool;

        #[method_id(@__retain_semantics Other writableTypesForSaveOperation:)]
        pub unsafe fn writableTypesForSaveOperation(
            &self,
            saveOperation: NSSaveOperationType,
        ) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other fileNameExtensionForType:saveOperation:)]
        pub unsafe fn fileNameExtensionForType_saveOperation(
            &self,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
        ) -> Option<Id<NSString, Shared>>;

        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(&self, item: &NSValidatedUserInterfaceItem)
            -> bool;

        #[method(usesUbiquitousStorage)]
        pub unsafe fn usesUbiquitousStorage() -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSDocument {
        #[method(saveToURL:ofType:forSaveOperation:error:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
            saveOperation: NSSaveOperationType,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other dataRepresentationOfType:)]
        pub unsafe fn dataRepresentationOfType(
            &self,
            type_: &NSString,
        ) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other fileAttributesToWriteToFile:ofType:saveOperation:)]
        pub unsafe fn fileAttributesToWriteToFile_ofType_saveOperation(
            &self,
            fullDocumentPath: &NSString,
            documentTypeName: &NSString,
            saveOperationType: NSSaveOperationType,
        ) -> Option<Id<NSDictionary, Shared>>;

        #[method_id(@__retain_semantics Other fileName)]
        pub unsafe fn fileName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other fileWrapperRepresentationOfType:)]
        pub unsafe fn fileWrapperRepresentationOfType(
            &self,
            type_: &NSString,
        ) -> Option<Id<NSFileWrapper, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:ofType:)]
        pub unsafe fn initWithContentsOfFile_ofType(
            this: Option<Allocated<Self>>,
            absolutePath: &NSString,
            typeName: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:)]
        pub unsafe fn initWithContentsOfURL_ofType(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            typeName: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method(loadDataRepresentation:ofType:)]
        pub unsafe fn loadDataRepresentation_ofType(&self, data: &NSData, type_: &NSString)
            -> bool;

        #[method(loadFileWrapperRepresentation:ofType:)]
        pub unsafe fn loadFileWrapperRepresentation_ofType(
            &self,
            wrapper: &NSFileWrapper,
            type_: &NSString,
        ) -> bool;

        #[method(printShowingPrintPanel:)]
        pub unsafe fn printShowingPrintPanel(&self, flag: bool);

        #[method(readFromFile:ofType:)]
        pub unsafe fn readFromFile_ofType(&self, fileName: &NSString, type_: &NSString) -> bool;

        #[method(readFromURL:ofType:)]
        pub unsafe fn readFromURL_ofType(&self, url: &NSURL, type_: &NSString) -> bool;

        #[method(revertToSavedFromFile:ofType:)]
        pub unsafe fn revertToSavedFromFile_ofType(
            &self,
            fileName: &NSString,
            type_: &NSString,
        ) -> bool;

        #[method(revertToSavedFromURL:ofType:)]
        pub unsafe fn revertToSavedFromURL_ofType(&self, url: &NSURL, type_: &NSString) -> bool;

        #[method(runModalPageLayoutWithPrintInfo:)]
        pub unsafe fn runModalPageLayoutWithPrintInfo(&self, printInfo: &NSPrintInfo) -> NSInteger;

        #[method(saveToFile:saveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveToFile_saveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            fileName: &NSString,
            saveOperation: NSSaveOperationType,
            delegate: Option<&Object>,
            didSaveSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(setFileName:)]
        pub unsafe fn setFileName(&self, fileName: Option<&NSString>);

        #[method(writeToFile:ofType:)]
        pub unsafe fn writeToFile_ofType(&self, fileName: &NSString, type_: &NSString) -> bool;

        #[method(writeToFile:ofType:originalFile:saveOperation:)]
        pub unsafe fn writeToFile_ofType_originalFile_saveOperation(
            &self,
            fullDocumentPath: &NSString,
            documentTypeName: &NSString,
            fullOriginalDocumentPath: Option<&NSString>,
            saveOperationType: NSSaveOperationType,
        ) -> bool;

        #[method(writeToURL:ofType:)]
        pub unsafe fn writeToURL_ofType(&self, url: &NSURL, type_: &NSString) -> bool;

        #[method(writeWithBackupToFile:ofType:saveOperation:)]
        pub unsafe fn writeWithBackupToFile_ofType_saveOperation(
            &self,
            fullDocumentPath: &NSString,
            documentTypeName: &NSString,
            saveOperationType: NSSaveOperationType,
        ) -> bool;
    }
);
