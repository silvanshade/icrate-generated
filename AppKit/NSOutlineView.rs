//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_enum!(
    #[underlying(c_int)]
    pub enum {
        NSOutlineViewDropOnItemIndex = -1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOutlineView;

    unsafe impl ClassType for NSOutlineView {
        #[inherits(AppKit::NSControl, AppKit::NSView, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSTableView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSOutlineViewDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSOutlineViewDelegate>);

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<AppKit::NSOutlineViewDataSource, Shared>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, dataSource: Option<&AppKit::NSOutlineViewDataSource>);

        #[cfg(feature = "AppKit_NSTableColumn")]
        #[method_id(@__retain_semantics Other outlineTableColumn)]
        pub unsafe fn outlineTableColumn(&self) -> Option<Id<AppKit::NSTableColumn, Shared>>;

        #[cfg(feature = "AppKit_NSTableColumn")]
        #[method(setOutlineTableColumn:)]
        pub unsafe fn setOutlineTableColumn(
            &self,
            outlineTableColumn: Option<&AppKit::NSTableColumn>,
        );

        #[method(isExpandable:)]
        pub unsafe fn isExpandable(&self, item: Option<&Object>) -> bool;

        #[method(numberOfChildrenOfItem:)]
        pub unsafe fn numberOfChildrenOfItem(&self, item: Option<&Object>) -> NSInteger;

        #[method_id(@__retain_semantics Other child:ofItem:)]
        pub unsafe fn child_ofItem(
            &self,
            index: NSInteger,
            item: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[method(expandItem:expandChildren:)]
        pub unsafe fn expandItem_expandChildren(&self, item: Option<&Object>, expandChildren: bool);

        #[method(expandItem:)]
        pub unsafe fn expandItem(&self, item: Option<&Object>);

        #[method(collapseItem:collapseChildren:)]
        pub unsafe fn collapseItem_collapseChildren(
            &self,
            item: Option<&Object>,
            collapseChildren: bool,
        );

        #[method(collapseItem:)]
        pub unsafe fn collapseItem(&self, item: Option<&Object>);

        #[method(reloadItem:reloadChildren:)]
        pub unsafe fn reloadItem_reloadChildren(&self, item: Option<&Object>, reloadChildren: bool);

        #[method(reloadItem:)]
        pub unsafe fn reloadItem(&self, item: Option<&Object>);

        #[method_id(@__retain_semantics Other parentForItem:)]
        pub unsafe fn parentForItem(&self, item: Option<&Object>) -> Option<Id<Object, Shared>>;

        #[method(childIndexForItem:)]
        pub unsafe fn childIndexForItem(&self, item: &Object) -> NSInteger;

        #[method_id(@__retain_semantics Other itemAtRow:)]
        pub unsafe fn itemAtRow(&self, row: NSInteger) -> Option<Id<Object, Shared>>;

        #[method(rowForItem:)]
        pub unsafe fn rowForItem(&self, item: Option<&Object>) -> NSInteger;

        #[method(levelForItem:)]
        pub unsafe fn levelForItem(&self, item: Option<&Object>) -> NSInteger;

        #[method(levelForRow:)]
        pub unsafe fn levelForRow(&self, row: NSInteger) -> NSInteger;

        #[method(isItemExpanded:)]
        pub unsafe fn isItemExpanded(&self, item: Option<&Object>) -> bool;

        #[method(indentationPerLevel)]
        pub unsafe fn indentationPerLevel(&self) -> CoreGraphics::CGFloat;

        #[method(setIndentationPerLevel:)]
        pub unsafe fn setIndentationPerLevel(&self, indentationPerLevel: CoreGraphics::CGFloat);

        #[method(indentationMarkerFollowsCell)]
        pub unsafe fn indentationMarkerFollowsCell(&self) -> bool;

        #[method(setIndentationMarkerFollowsCell:)]
        pub unsafe fn setIndentationMarkerFollowsCell(&self, indentationMarkerFollowsCell: bool);

        #[method(autoresizesOutlineColumn)]
        pub unsafe fn autoresizesOutlineColumn(&self) -> bool;

        #[method(setAutoresizesOutlineColumn:)]
        pub unsafe fn setAutoresizesOutlineColumn(&self, autoresizesOutlineColumn: bool);

        #[method(frameOfOutlineCellAtRow:)]
        pub unsafe fn frameOfOutlineCellAtRow(&self, row: NSInteger) -> Foundation::NSRect;

        #[method(setDropItem:dropChildIndex:)]
        pub unsafe fn setDropItem_dropChildIndex(&self, item: Option<&Object>, index: NSInteger);

        #[method(shouldCollapseAutoExpandedItemsForDeposited:)]
        pub unsafe fn shouldCollapseAutoExpandedItemsForDeposited(&self, deposited: bool) -> bool;

        #[method(autosaveExpandedItems)]
        pub unsafe fn autosaveExpandedItems(&self) -> bool;

        #[method(setAutosaveExpandedItems:)]
        pub unsafe fn setAutosaveExpandedItems(&self, autosaveExpandedItems: bool);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn insertItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &Foundation::NSIndexSet,
            parent: Option<&Object>,
            animationOptions: AppKit::NSTableViewAnimationOptions,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn removeItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &Foundation::NSIndexSet,
            parent: Option<&Object>,
            animationOptions: AppKit::NSTableViewAnimationOptions,
        );

        #[method(moveItemAtIndex:inParent:toIndex:inParent:)]
        pub unsafe fn moveItemAtIndex_inParent_toIndex_inParent(
            &self,
            fromIndex: NSInteger,
            oldParent: Option<&Object>,
            toIndex: NSInteger,
            newParent: Option<&Object>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertRowsAtIndexes:withAnimation:)]
        pub unsafe fn insertRowsAtIndexes_withAnimation(
            &self,
            indexes: &Foundation::NSIndexSet,
            animationOptions: AppKit::NSTableViewAnimationOptions,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeRowsAtIndexes:withAnimation:)]
        pub unsafe fn removeRowsAtIndexes_withAnimation(
            &self,
            indexes: &Foundation::NSIndexSet,
            animationOptions: AppKit::NSTableViewAnimationOptions,
        );

        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, oldIndex: NSInteger, newIndex: NSInteger);

        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self)
            -> AppKit::NSUserInterfaceLayoutDirection;

        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            userInterfaceLayoutDirection: AppKit::NSUserInterfaceLayoutDirection,
        );

        #[method(stronglyReferencesItems)]
        pub unsafe fn stronglyReferencesItems(&self) -> bool;

        #[method(setStronglyReferencesItems:)]
        pub unsafe fn setStronglyReferencesItems(&self, stronglyReferencesItems: bool);
    }
);

extern_protocol!(
    pub struct NSOutlineViewDataSource;

    unsafe impl ProtocolType for NSOutlineViewDataSource {
        #[optional]
        #[method(outlineView:numberOfChildrenOfItem:)]
        pub unsafe fn outlineView_numberOfChildrenOfItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: Option<&Object>,
        ) -> NSInteger;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:child:ofItem:)]
        pub unsafe fn outlineView_child_ofItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            index: NSInteger,
            item: Option<&Object>,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method(outlineView:isItemExpandable:)]
        pub unsafe fn outlineView_isItemExpandable(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:objectValueForTableColumn:byItem:)]
        pub unsafe fn outlineView_objectValueForTableColumn_byItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(outlineView:setObjectValue:forTableColumn:byItem:)]
        pub unsafe fn outlineView_setObjectValue_forTableColumn_byItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            object: Option<&Object>,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: Option<&Object>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:itemForPersistentObject:)]
        pub unsafe fn outlineView_itemForPersistentObject(
            &self,
            outlineView: &AppKit::NSOutlineView,
            object: &Object,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:persistentObjectForItem:)]
        pub unsafe fn outlineView_persistentObjectForItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(outlineView:sortDescriptorsDidChange:)]
        pub unsafe fn outlineView_sortDescriptorsDidChange(
            &self,
            outlineView: &AppKit::NSOutlineView,
            oldDescriptors: &Foundation::NSArray<Foundation::NSSortDescriptor>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:pasteboardWriterForItem:)]
        pub unsafe fn outlineView_pasteboardWriterForItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> Option<Id<AppKit::NSPasteboardWriting, Shared>>;

        #[optional]
        #[method(outlineView:draggingSession:willBeginAtPoint:forItems:)]
        pub unsafe fn outlineView_draggingSession_willBeginAtPoint_forItems(
            &self,
            outlineView: &AppKit::NSOutlineView,
            session: &AppKit::NSDraggingSession,
            screenPoint: Foundation::NSPoint,
            draggedItems: &Foundation::NSArray,
        );

        #[optional]
        #[method(outlineView:draggingSession:endedAtPoint:operation:)]
        pub unsafe fn outlineView_draggingSession_endedAtPoint_operation(
            &self,
            outlineView: &AppKit::NSOutlineView,
            session: &AppKit::NSDraggingSession,
            screenPoint: Foundation::NSPoint,
            operation: AppKit::NSDragOperation,
        );

        #[optional]
        #[method(outlineView:writeItems:toPasteboard:)]
        pub unsafe fn outlineView_writeItems_toPasteboard(
            &self,
            outlineView: &AppKit::NSOutlineView,
            items: &Foundation::NSArray,
            pasteboard: &AppKit::NSPasteboard,
        ) -> bool;

        #[optional]
        #[method(outlineView:updateDraggingItemsForDrag:)]
        pub unsafe fn outlineView_updateDraggingItemsForDrag(
            &self,
            outlineView: &AppKit::NSOutlineView,
            draggingInfo: &AppKit::NSDraggingInfo,
        );

        #[optional]
        #[method(outlineView:validateDrop:proposedItem:proposedChildIndex:)]
        pub unsafe fn outlineView_validateDrop_proposedItem_proposedChildIndex(
            &self,
            outlineView: &AppKit::NSOutlineView,
            info: &AppKit::NSDraggingInfo,
            item: Option<&Object>,
            index: NSInteger,
        ) -> AppKit::NSDragOperation;

        #[optional]
        #[method(outlineView:acceptDrop:item:childIndex:)]
        pub unsafe fn outlineView_acceptDrop_item_childIndex(
            &self,
            outlineView: &AppKit::NSOutlineView,
            info: &AppKit::NSDraggingInfo,
            item: Option<&Object>,
            index: NSInteger,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:namesOfPromisedFilesDroppedAtDestination:forDraggedItems:)]
        pub unsafe fn outlineView_namesOfPromisedFilesDroppedAtDestination_forDraggedItems(
            &self,
            outlineView: &AppKit::NSOutlineView,
            dropDestination: &Foundation::NSURL,
            items: &Foundation::NSArray,
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;
    }
);

extern_protocol!(
    pub struct NSOutlineViewDelegate;

    unsafe impl ProtocolType for NSOutlineViewDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:viewForTableColumn:item:)]
        pub unsafe fn outlineView_viewForTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        ) -> Option<Id<AppKit::NSView, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:rowViewForItem:)]
        pub unsafe fn outlineView_rowViewForItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> Option<Id<AppKit::NSTableRowView, Shared>>;

        #[optional]
        #[method(outlineView:didAddRowView:forRow:)]
        pub unsafe fn outlineView_didAddRowView_forRow(
            &self,
            outlineView: &AppKit::NSOutlineView,
            rowView: &AppKit::NSTableRowView,
            row: NSInteger,
        );

        #[optional]
        #[method(outlineView:didRemoveRowView:forRow:)]
        pub unsafe fn outlineView_didRemoveRowView_forRow(
            &self,
            outlineView: &AppKit::NSOutlineView,
            rowView: &AppKit::NSTableRowView,
            row: NSInteger,
        );

        #[optional]
        #[method(outlineView:willDisplayCell:forTableColumn:item:)]
        pub unsafe fn outlineView_willDisplayCell_forTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            cell: &Object,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        );

        #[optional]
        #[method(outlineView:shouldEditTableColumn:item:)]
        pub unsafe fn outlineView_shouldEditTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method(selectionShouldChangeInOutlineView:)]
        pub unsafe fn selectionShouldChangeInOutlineView(
            &self,
            outlineView: &AppKit::NSOutlineView,
        ) -> bool;

        #[optional]
        #[method(outlineView:shouldSelectItem:)]
        pub unsafe fn outlineView_shouldSelectItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:selectionIndexesForProposedSelection:)]
        pub unsafe fn outlineView_selectionIndexesForProposedSelection(
            &self,
            outlineView: &AppKit::NSOutlineView,
            proposedSelectionIndexes: &Foundation::NSIndexSet,
        ) -> Id<Foundation::NSIndexSet, Shared>;

        #[optional]
        #[method(outlineView:shouldSelectTableColumn:)]
        pub unsafe fn outlineView_shouldSelectTableColumn(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
        ) -> bool;

        #[optional]
        #[method(outlineView:mouseDownInHeaderOfTableColumn:)]
        pub unsafe fn outlineView_mouseDownInHeaderOfTableColumn(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: &AppKit::NSTableColumn,
        );

        #[optional]
        #[method(outlineView:didClickTableColumn:)]
        pub unsafe fn outlineView_didClickTableColumn(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: &AppKit::NSTableColumn,
        );

        #[optional]
        #[method(outlineView:didDragTableColumn:)]
        pub unsafe fn outlineView_didDragTableColumn(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: &AppKit::NSTableColumn,
        );

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:toolTipForCell:rect:tableColumn:item:mouseLocation:)]
        pub unsafe fn outlineView_toolTipForCell_rect_tableColumn_item_mouseLocation(
            &self,
            outlineView: &AppKit::NSOutlineView,
            cell: &AppKit::NSCell,
            rect: Foundation::NSRectPointer,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
            mouseLocation: Foundation::NSPoint,
        ) -> Id<Foundation::NSString, Shared>;

        #[optional]
        #[method(outlineView:heightOfRowByItem:)]
        pub unsafe fn outlineView_heightOfRowByItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> CoreGraphics::CGFloat;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:tintConfigurationForItem:)]
        pub unsafe fn outlineView_tintConfigurationForItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> Option<Id<AppKit::NSTintConfiguration, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:typeSelectStringForTableColumn:item:)]
        pub unsafe fn outlineView_typeSelectStringForTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:nextTypeSelectMatchFromItem:toItem:forString:)]
        pub unsafe fn outlineView_nextTypeSelectMatchFromItem_toItem_forString(
            &self,
            outlineView: &AppKit::NSOutlineView,
            startItem: &Object,
            endItem: &Object,
            searchString: &Foundation::NSString,
        ) -> Option<Id<Object, Shared>>;

        #[optional]
        #[method(outlineView:shouldTypeSelectForEvent:withCurrentSearchString:)]
        pub unsafe fn outlineView_shouldTypeSelectForEvent_withCurrentSearchString(
            &self,
            outlineView: &AppKit::NSOutlineView,
            event: &AppKit::NSEvent,
            searchString: Option<&Foundation::NSString>,
        ) -> bool;

        #[optional]
        #[method(outlineView:shouldShowCellExpansionForTableColumn:item:)]
        pub unsafe fn outlineView_shouldShowCellExpansionForTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method(outlineView:shouldTrackCell:forTableColumn:item:)]
        pub unsafe fn outlineView_shouldTrackCell_forTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            cell: &AppKit::NSCell,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other outlineView:dataCellForTableColumn:item:)]
        pub unsafe fn outlineView_dataCellForTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        ) -> Option<Id<AppKit::NSCell, Shared>>;

        #[optional]
        #[method(outlineView:isGroupItem:)]
        pub unsafe fn outlineView_isGroupItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method(outlineView:shouldExpandItem:)]
        pub unsafe fn outlineView_shouldExpandItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method(outlineView:shouldCollapseItem:)]
        pub unsafe fn outlineView_shouldCollapseItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method(outlineView:willDisplayOutlineCell:forTableColumn:item:)]
        pub unsafe fn outlineView_willDisplayOutlineCell_forTableColumn_item(
            &self,
            outlineView: &AppKit::NSOutlineView,
            cell: &Object,
            tableColumn: Option<&AppKit::NSTableColumn>,
            item: &Object,
        );

        #[optional]
        #[method(outlineView:sizeToFitWidthOfColumn:)]
        pub unsafe fn outlineView_sizeToFitWidthOfColumn(
            &self,
            outlineView: &AppKit::NSOutlineView,
            column: NSInteger,
        ) -> CoreGraphics::CGFloat;

        #[optional]
        #[method(outlineView:shouldReorderColumn:toColumn:)]
        pub unsafe fn outlineView_shouldReorderColumn_toColumn(
            &self,
            outlineView: &AppKit::NSOutlineView,
            columnIndex: NSInteger,
            newColumnIndex: NSInteger,
        ) -> bool;

        #[optional]
        #[method(outlineView:shouldShowOutlineCellForItem:)]
        pub unsafe fn outlineView_shouldShowOutlineCellForItem(
            &self,
            outlineView: &AppKit::NSOutlineView,
            item: &Object,
        ) -> bool;

        #[optional]
        #[method(outlineViewSelectionDidChange:)]
        pub unsafe fn outlineViewSelectionDidChange(
            &self,
            notification: &Foundation::NSNotification,
        );

        #[optional]
        #[method(outlineViewColumnDidMove:)]
        pub unsafe fn outlineViewColumnDidMove(&self, notification: &Foundation::NSNotification);

        #[optional]
        #[method(outlineViewColumnDidResize:)]
        pub unsafe fn outlineViewColumnDidResize(&self, notification: &Foundation::NSNotification);

        #[optional]
        #[method(outlineViewSelectionIsChanging:)]
        pub unsafe fn outlineViewSelectionIsChanging(
            &self,
            notification: &Foundation::NSNotification,
        );

        #[optional]
        #[method(outlineViewItemWillExpand:)]
        pub unsafe fn outlineViewItemWillExpand(&self, notification: &Foundation::NSNotification);

        #[optional]
        #[method(outlineViewItemDidExpand:)]
        pub unsafe fn outlineViewItemDidExpand(&self, notification: &Foundation::NSNotification);

        #[optional]
        #[method(outlineViewItemWillCollapse:)]
        pub unsafe fn outlineViewItemWillCollapse(&self, notification: &Foundation::NSNotification);

        #[optional]
        #[method(outlineViewItemDidCollapse:)]
        pub unsafe fn outlineViewItemDidCollapse(&self, notification: &Foundation::NSNotification);
    }
);

extern_static!(NSOutlineViewDisclosureButtonKey: &'static AppKit::NSUserInterfaceItemIdentifier);

extern_static!(NSOutlineViewShowHideButtonKey: &'static AppKit::NSUserInterfaceItemIdentifier);

extern_static!(
    NSOutlineViewSelectionDidChangeNotification: &'static Foundation::NSNotificationName
);

extern_static!(NSOutlineViewColumnDidMoveNotification: &'static Foundation::NSNotificationName);

extern_static!(NSOutlineViewColumnDidResizeNotification: &'static Foundation::NSNotificationName);

extern_static!(
    NSOutlineViewSelectionIsChangingNotification: &'static Foundation::NSNotificationName
);

extern_static!(NSOutlineViewItemWillExpandNotification: &'static Foundation::NSNotificationName);

extern_static!(NSOutlineViewItemDidExpandNotification: &'static Foundation::NSNotificationName);

extern_static!(NSOutlineViewItemWillCollapseNotification: &'static Foundation::NSNotificationName);

extern_static!(NSOutlineViewItemDidCollapseNotification: &'static Foundation::NSNotificationName);

extern_methods!(
    /// Methods declared on superclass `NSTableView`
    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl AppKit::NSOutlineView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
