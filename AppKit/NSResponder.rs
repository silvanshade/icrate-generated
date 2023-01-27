//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSResponder")]
    pub struct NSResponder;

    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl ClassType for NSResponder {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSResponder")]
unsafe impl NSCoding for NSResponder {}

#[cfg(feature = "AppKit_NSResponder")]
unsafe impl NSObjectProtocol for NSResponder {}

extern_methods!(
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other nextResponder)]
        pub unsafe fn nextResponder(&self) -> Option<Id<NSResponder, Shared>>;

        #[method(setNextResponder:)]
        pub unsafe fn setNextResponder(&self, next_responder: Option<&NSResponder>);

        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&Object>) -> bool;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[method_id(@__retain_semantics Other validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            send_type: Option<&NSPasteboardType>,
            return_type: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rightMouseDown:)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(otherMouseDown:)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseUp:)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rightMouseUp:)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(otherMouseUp:)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseMoved:)]
        pub unsafe fn mouseMoved(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDragged:)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rightMouseDragged:)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(otherMouseDragged:)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(keyDown:)]
        pub unsafe fn keyDown(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(keyUp:)]
        pub unsafe fn keyUp(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(flagsChanged:)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(tabletPoint:)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(tabletProximity:)]
        pub unsafe fn tabletProximity(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(cursorUpdate:)]
        pub unsafe fn cursorUpdate(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(magnifyWithEvent:)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rotateWithEvent:)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(swipeWithEvent:)]
        pub unsafe fn swipeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(beginGestureWithEvent:)]
        pub unsafe fn beginGestureWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(endGestureWithEvent:)]
        pub unsafe fn endGestureWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(smartMagnifyWithEvent:)]
        pub unsafe fn smartMagnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(changeModeWithEvent:)]
        pub unsafe fn changeModeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesBeganWithEvent:)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesMovedWithEvent:)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesEndedWithEvent:)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(touchesCancelledWithEvent:)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(quickLookWithEvent:)]
        pub unsafe fn quickLookWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(pressureChangeWithEvent:)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);

        #[method(noResponderFor:)]
        pub unsafe fn noResponderFor(&self, event_selector: Sel);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(becomeFirstResponder)]
        pub unsafe fn becomeFirstResponder(&self) -> bool;

        #[method(resignFirstResponder)]
        pub unsafe fn resignFirstResponder(&self) -> bool;

        #[cfg(all(feature = "AppKit_NSEvent", feature = "Foundation_NSArray"))]
        #[method(interpretKeyEvents:)]
        pub unsafe fn interpretKeyEvents(&self, event_array: &NSArray<NSEvent>);

        #[method(flushBufferedKeyEvents)]
        pub unsafe fn flushBufferedKeyEvents(&self);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(showContextHelp:)]
        pub unsafe fn showContextHelp(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, event_ptr: &NSEvent);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(shouldBeTreatedAsInkEvent:)]
        pub unsafe fn shouldBeTreatedAsInkEvent(&self, event: &NSEvent) -> bool;

        #[method(wantsScrollEventsForSwipeTrackingOnAxis:)]
        pub unsafe fn wantsScrollEventsForSwipeTrackingOnAxis(
            &self,
            axis: NSEventGestureAxis,
        ) -> bool;

        #[method(wantsForwardedScrollEventsForAxis:)]
        pub unsafe fn wantsForwardedScrollEventsForAxis(&self, axis: NSEventGestureAxis) -> bool;

        #[method_id(@__retain_semantics Other supplementalTargetForAction:sender:)]
        pub unsafe fn supplementalTargetForAction_sender(
            &self,
            action: Sel,
            sender: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_protocol!(
    pub unsafe trait NSStandardKeyBindingResponding: NSObjectProtocol {
        #[optional]
        #[method(insertText:)]
        unsafe fn insertText(&self, insert_string: &Object);

        #[optional]
        #[method(doCommandBySelector:)]
        unsafe fn doCommandBySelector(&self, selector: Sel);

        #[optional]
        #[method(moveForward:)]
        unsafe fn moveForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveRight:)]
        unsafe fn moveRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveBackward:)]
        unsafe fn moveBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveLeft:)]
        unsafe fn moveLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveUp:)]
        unsafe fn moveUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveDown:)]
        unsafe fn moveDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordForward:)]
        unsafe fn moveWordForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordBackward:)]
        unsafe fn moveWordBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfLine:)]
        unsafe fn moveToBeginningOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfLine:)]
        unsafe fn moveToEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfParagraph:)]
        unsafe fn moveToBeginningOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfParagraph:)]
        unsafe fn moveToEndOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfDocument:)]
        unsafe fn moveToEndOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfDocument:)]
        unsafe fn moveToBeginningOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageDown:)]
        unsafe fn pageDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageUp:)]
        unsafe fn pageUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(centerSelectionInVisibleArea:)]
        unsafe fn centerSelectionInVisibleArea(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveBackwardAndModifySelection:)]
        unsafe fn moveBackwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveForwardAndModifySelection:)]
        unsafe fn moveForwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordForwardAndModifySelection:)]
        unsafe fn moveWordForwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordBackwardAndModifySelection:)]
        unsafe fn moveWordBackwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveUpAndModifySelection:)]
        unsafe fn moveUpAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveDownAndModifySelection:)]
        unsafe fn moveDownAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfLineAndModifySelection:)]
        unsafe fn moveToBeginningOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfLineAndModifySelection:)]
        unsafe fn moveToEndOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfParagraphAndModifySelection:)]
        unsafe fn moveToBeginningOfParagraphAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfParagraphAndModifySelection:)]
        unsafe fn moveToEndOfParagraphAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfDocumentAndModifySelection:)]
        unsafe fn moveToEndOfDocumentAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfDocumentAndModifySelection:)]
        unsafe fn moveToBeginningOfDocumentAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageDownAndModifySelection:)]
        unsafe fn pageDownAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageUpAndModifySelection:)]
        unsafe fn pageUpAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveParagraphForwardAndModifySelection:)]
        unsafe fn moveParagraphForwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveParagraphBackwardAndModifySelection:)]
        unsafe fn moveParagraphBackwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordRight:)]
        unsafe fn moveWordRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordLeft:)]
        unsafe fn moveWordLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveRightAndModifySelection:)]
        unsafe fn moveRightAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveLeftAndModifySelection:)]
        unsafe fn moveLeftAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordRightAndModifySelection:)]
        unsafe fn moveWordRightAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordLeftAndModifySelection:)]
        unsafe fn moveWordLeftAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToLeftEndOfLine:)]
        unsafe fn moveToLeftEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToRightEndOfLine:)]
        unsafe fn moveToRightEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToLeftEndOfLineAndModifySelection:)]
        unsafe fn moveToLeftEndOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToRightEndOfLineAndModifySelection:)]
        unsafe fn moveToRightEndOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollPageUp:)]
        unsafe fn scrollPageUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollPageDown:)]
        unsafe fn scrollPageDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollLineUp:)]
        unsafe fn scrollLineUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollLineDown:)]
        unsafe fn scrollLineDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollToBeginningOfDocument:)]
        unsafe fn scrollToBeginningOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollToEndOfDocument:)]
        unsafe fn scrollToEndOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(transpose:)]
        unsafe fn transpose(&self, sender: Option<&Object>);

        #[optional]
        #[method(transposeWords:)]
        unsafe fn transposeWords(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectAll:)]
        unsafe fn selectAll(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectParagraph:)]
        unsafe fn selectParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectLine:)]
        unsafe fn selectLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectWord:)]
        unsafe fn selectWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(indent:)]
        unsafe fn indent(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertTab:)]
        unsafe fn insertTab(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertBacktab:)]
        unsafe fn insertBacktab(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertNewline:)]
        unsafe fn insertNewline(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertParagraphSeparator:)]
        unsafe fn insertParagraphSeparator(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertNewlineIgnoringFieldEditor:)]
        unsafe fn insertNewlineIgnoringFieldEditor(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertTabIgnoringFieldEditor:)]
        unsafe fn insertTabIgnoringFieldEditor(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertLineBreak:)]
        unsafe fn insertLineBreak(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertContainerBreak:)]
        unsafe fn insertContainerBreak(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertSingleQuoteIgnoringSubstitution:)]
        unsafe fn insertSingleQuoteIgnoringSubstitution(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertDoubleQuoteIgnoringSubstitution:)]
        unsafe fn insertDoubleQuoteIgnoringSubstitution(&self, sender: Option<&Object>);

        #[optional]
        #[method(changeCaseOfLetter:)]
        unsafe fn changeCaseOfLetter(&self, sender: Option<&Object>);

        #[optional]
        #[method(uppercaseWord:)]
        unsafe fn uppercaseWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(lowercaseWord:)]
        unsafe fn lowercaseWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(capitalizeWord:)]
        unsafe fn capitalizeWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteForward:)]
        unsafe fn deleteForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteBackward:)]
        unsafe fn deleteBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteBackwardByDecomposingPreviousCharacter:)]
        unsafe fn deleteBackwardByDecomposingPreviousCharacter(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteWordForward:)]
        unsafe fn deleteWordForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteWordBackward:)]
        unsafe fn deleteWordBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToBeginningOfLine:)]
        unsafe fn deleteToBeginningOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToEndOfLine:)]
        unsafe fn deleteToEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToBeginningOfParagraph:)]
        unsafe fn deleteToBeginningOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToEndOfParagraph:)]
        unsafe fn deleteToEndOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(yank:)]
        unsafe fn yank(&self, sender: Option<&Object>);

        #[optional]
        #[method(complete:)]
        unsafe fn complete(&self, sender: Option<&Object>);

        #[optional]
        #[method(setMark:)]
        unsafe fn setMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToMark:)]
        unsafe fn deleteToMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectToMark:)]
        unsafe fn selectToMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(swapWithMark:)]
        unsafe fn swapWithMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(cancelOperation:)]
        unsafe fn cancelOperation(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeBaseWritingDirectionNatural:)]
        unsafe fn makeBaseWritingDirectionNatural(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeBaseWritingDirectionLeftToRight:)]
        unsafe fn makeBaseWritingDirectionLeftToRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeBaseWritingDirectionRightToLeft:)]
        unsafe fn makeBaseWritingDirectionRightToLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeTextWritingDirectionNatural:)]
        unsafe fn makeTextWritingDirectionNatural(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeTextWritingDirectionLeftToRight:)]
        unsafe fn makeTextWritingDirectionLeftToRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeTextWritingDirectionRightToLeft:)]
        unsafe fn makeTextWritingDirectionRightToLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(quickLookPreviewItems:)]
        unsafe fn quickLookPreviewItems(&self, sender: Option<&Object>);
    }

    unsafe impl ProtocolType for dyn NSStandardKeyBindingResponding {}
);

extern_methods!(
    /// NSStandardKeyBindingMethods
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {}
);

#[cfg(feature = "AppKit_NSResponder")]
unsafe impl NSStandardKeyBindingResponding for NSResponder {}

extern_methods!(
    /// NSUndoSupport
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager, Shared>>;
    }
);

extern_methods!(
    /// NSControlEditingSupport
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(feature = "AppKit_NSEvent")]
        #[method(validateProposedFirstResponder:forEvent:)]
        pub unsafe fn validateProposedFirstResponder_forEvent(
            &self,
            responder: &NSResponder,
            event: Option<&NSEvent>,
        ) -> bool;
    }
);

extern_methods!(
    /// NSErrorPresentation
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSError"))]
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&Object>,
            did_present_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Id<NSError, Shared>;
    }
);

extern_methods!(
    /// NSTextFinderSupport
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[method(performTextFinderAction:)]
        pub unsafe fn performTextFinderAction(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSWindowTabbing
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[method(newWindowForTab:)]
        pub unsafe fn newWindowForTab(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl NSResponder {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "This has always returned NO and had no effect on macOS"]
        #[method(performMnemonic:)]
        pub unsafe fn performMnemonic(&self, string: &NSString) -> bool;
    }
);
