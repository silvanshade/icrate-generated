//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSAppKitVersionNumberWithContinuousScrollingBrowser: NSAppKitVersion = 680.0);

extern_static!(NSAppKitVersionNumberWithColumnResizingBrowser: NSAppKitVersion = 685.0);

pub type NSBrowserColumnsAutosaveName = NSString;

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSBrowserColumnResizingType {
    NSBrowserNoColumnResizing = 0,
    NSBrowserAutoColumnResizing = 1,
    NSBrowserUserColumnResizing = 2,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSBrowserDropOperation {
    NSBrowserDropOn = 0,
    NSBrowserDropAbove = 1,
}

#[objc2::interface(
    unsafe super = NSControl,
    unsafe inherits = [
        NSView,
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSBrowser")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSBrowser;
}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAccessibility for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAccessibilityElementProtocol for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAnimatablePropertyContainer for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAppearanceCustomization for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSCoding for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSDraggingDestination for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSObjectProtocol for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSUserInterfaceItemIdentification for NSBrowser {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSBrowser")]
    pub type NSBrowser;

    #[objc2::method(sel = "cellClass")]
    pub unsafe fn cellClass() -> &'static Class;

    #[objc2::method(sel = "loadColumnZero")]
    pub unsafe fn loadColumnZero(&self);

    #[objc2::method(sel = "isLoaded")]
    pub unsafe fn isLoaded(&self) -> bool;

    #[objc2::method(sel = "doubleAction")]
    pub unsafe fn doubleAction(&self) -> Option<Sel>;

    #[objc2::method(sel = "setDoubleAction:")]
    pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

    #[objc2::method(sel = "setCellClass:")]
    pub unsafe fn setCellClass(&self, factory_id: &Class);

    #[objc2::method(sel = "cellPrototype", managed = "Other")]
    pub unsafe fn cellPrototype(&self) -> Option<Id<Object>>;

    #[objc2::method(sel = "setCellPrototype:")]
    pub unsafe fn setCellPrototype(&self, cell_prototype: Option<&Object>);

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSBrowserDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSBrowserDelegate>>);

    #[objc2::method(sel = "reusesColumns")]
    pub unsafe fn reusesColumns(&self) -> bool;

    #[objc2::method(sel = "setReusesColumns:")]
    pub unsafe fn setReusesColumns(&self, reuses_columns: bool);

    #[objc2::method(sel = "hasHorizontalScroller")]
    pub unsafe fn hasHorizontalScroller(&self) -> bool;

    #[objc2::method(sel = "setHasHorizontalScroller:")]
    pub unsafe fn setHasHorizontalScroller(&self, has_horizontal_scroller: bool);

    #[objc2::method(sel = "autohidesScroller")]
    pub unsafe fn autohidesScroller(&self) -> bool;

    #[objc2::method(sel = "setAutohidesScroller:")]
    pub unsafe fn setAutohidesScroller(&self, autohides_scroller: bool);

    #[objc2::method(sel = "separatesColumns")]
    pub unsafe fn separatesColumns(&self) -> bool;

    #[objc2::method(sel = "setSeparatesColumns:")]
    pub unsafe fn setSeparatesColumns(&self, separates_columns: bool);

    #[objc2::method(sel = "isTitled")]
    pub unsafe fn isTitled(&self) -> bool;

    #[objc2::method(sel = "setTitled:")]
    pub unsafe fn setTitled(&self, titled: bool);

    #[objc2::method(sel = "minColumnWidth")]
    pub unsafe fn minColumnWidth(&self) -> CGFloat;

    #[objc2::method(sel = "setMinColumnWidth:")]
    pub unsafe fn setMinColumnWidth(&self, min_column_width: CGFloat);

    #[objc2::method(sel = "maxVisibleColumns")]
    pub unsafe fn maxVisibleColumns(&self) -> NSInteger;

    #[objc2::method(sel = "setMaxVisibleColumns:")]
    pub unsafe fn setMaxVisibleColumns(&self, max_visible_columns: NSInteger);

    #[objc2::method(sel = "allowsMultipleSelection")]
    pub unsafe fn allowsMultipleSelection(&self) -> bool;

    #[objc2::method(sel = "setAllowsMultipleSelection:")]
    pub unsafe fn setAllowsMultipleSelection(&self, allows_multiple_selection: bool);

    #[objc2::method(sel = "allowsBranchSelection")]
    pub unsafe fn allowsBranchSelection(&self) -> bool;

    #[objc2::method(sel = "setAllowsBranchSelection:")]
    pub unsafe fn setAllowsBranchSelection(&self, allows_branch_selection: bool);

    #[objc2::method(sel = "allowsEmptySelection")]
    pub unsafe fn allowsEmptySelection(&self) -> bool;

    #[objc2::method(sel = "setAllowsEmptySelection:")]
    pub unsafe fn setAllowsEmptySelection(&self, allows_empty_selection: bool);

    #[objc2::method(sel = "takesTitleFromPreviousColumn")]
    pub unsafe fn takesTitleFromPreviousColumn(&self) -> bool;

    #[objc2::method(sel = "setTakesTitleFromPreviousColumn:")]
    pub unsafe fn setTakesTitleFromPreviousColumn(&self, takes_title_from_previous_column: bool);

    #[objc2::method(sel = "sendsActionOnArrowKeys")]
    pub unsafe fn sendsActionOnArrowKeys(&self) -> bool;

    #[objc2::method(sel = "setSendsActionOnArrowKeys:")]
    pub unsafe fn setSendsActionOnArrowKeys(&self, sends_action_on_arrow_keys: bool);

    #[cfg(feature = "Foundation_NSIndexPath")]
    #[objc2::method(sel = "itemAtIndexPath:", managed = "Other")]
    pub unsafe fn itemAtIndexPath(&self, index_path: &NSIndexPath) -> Option<Id<Object>>;

    #[objc2::method(sel = "itemAtRow:inColumn:", managed = "Other")]
    pub unsafe fn itemAtRow_inColumn(
        &self,
        row: NSInteger,
        column: NSInteger,
    ) -> Option<Id<Object>>;

    #[cfg(feature = "Foundation_NSIndexPath")]
    #[objc2::method(sel = "indexPathForColumn:", managed = "Other")]
    pub unsafe fn indexPathForColumn(&self, column: NSInteger) -> Id<NSIndexPath>;

    #[objc2::method(sel = "isLeafItem:")]
    pub unsafe fn isLeafItem(&self, item: Option<&Object>) -> bool;

    #[cfg(feature = "Foundation_NSIndexSet")]
    #[objc2::method(sel = "reloadDataForRowIndexes:inColumn:")]
    pub unsafe fn reloadDataForRowIndexes_inColumn(
        &self,
        row_indexes: &NSIndexSet,
        column: NSInteger,
    );

    #[objc2::method(sel = "parentForItemsInColumn:", managed = "Other")]
    pub unsafe fn parentForItemsInColumn(&self, column: NSInteger) -> Option<Id<Object>>;

    #[objc2::method(sel = "scrollRowToVisible:inColumn:")]
    pub unsafe fn scrollRowToVisible_inColumn(&self, row: NSInteger, column: NSInteger);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTitle:ofColumn:")]
    pub unsafe fn setTitle_ofColumn(&self, string: &NSString, column: NSInteger);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "titleOfColumn:", managed = "Other")]
    pub unsafe fn titleOfColumn(&self, column: NSInteger) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathSeparator", managed = "Other")]
    pub unsafe fn pathSeparator(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPathSeparator:")]
    pub unsafe fn setPathSeparator(&self, path_separator: &NSString);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPath:")]
    pub unsafe fn setPath(&self, path: &NSString) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "path", managed = "Other")]
    pub unsafe fn path(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathToColumn:", managed = "Other")]
    pub unsafe fn pathToColumn(&self, column: NSInteger) -> Id<NSString>;

    #[objc2::method(sel = "clickedColumn")]
    pub unsafe fn clickedColumn(&self) -> NSInteger;

    #[objc2::method(sel = "clickedRow")]
    pub unsafe fn clickedRow(&self) -> NSInteger;

    #[objc2::method(sel = "selectedColumn")]
    pub unsafe fn selectedColumn(&self) -> NSInteger;

    #[objc2::method(sel = "selectedCell", managed = "Other")]
    pub unsafe fn selectedCell(&self) -> Option<Id<Object>>;

    #[objc2::method(sel = "selectedCellInColumn:", managed = "Other")]
    pub unsafe fn selectedCellInColumn(&self, column: NSInteger) -> Option<Id<Object>>;

    #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "selectedCells", managed = "Other")]
    pub unsafe fn selectedCells(&self) -> Option<Id<NSArray<NSCell>>>;

    #[objc2::method(sel = "selectRow:inColumn:")]
    pub unsafe fn selectRow_inColumn(&self, row: NSInteger, column: NSInteger);

    #[objc2::method(sel = "selectedRowInColumn:")]
    pub unsafe fn selectedRowInColumn(&self, column: NSInteger) -> NSInteger;

    #[cfg(feature = "Foundation_NSIndexPath")]
    #[objc2::method(sel = "selectionIndexPath", managed = "Other")]
    pub unsafe fn selectionIndexPath(&self) -> Option<Id<NSIndexPath>>;

    #[cfg(feature = "Foundation_NSIndexPath")]
    #[objc2::method(sel = "setSelectionIndexPath:")]
    pub unsafe fn setSelectionIndexPath(&self, selection_index_path: Option<&NSIndexPath>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
    #[objc2::method(sel = "selectionIndexPaths", managed = "Other")]
    pub unsafe fn selectionIndexPaths(&self) -> Id<NSArray<NSIndexPath>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
    #[objc2::method(sel = "setSelectionIndexPaths:")]
    pub unsafe fn setSelectionIndexPaths(&self, selection_index_paths: &NSArray<NSIndexPath>);

    #[cfg(feature = "Foundation_NSIndexSet")]
    #[objc2::method(sel = "selectRowIndexes:inColumn:")]
    pub unsafe fn selectRowIndexes_inColumn(&self, indexes: &NSIndexSet, column: NSInteger);

    #[cfg(feature = "Foundation_NSIndexSet")]
    #[objc2::method(sel = "selectedRowIndexesInColumn:", managed = "Other")]
    pub unsafe fn selectedRowIndexesInColumn(&self, column: NSInteger) -> Option<Id<NSIndexSet>>;

    #[objc2::method(sel = "reloadColumn:")]
    pub unsafe fn reloadColumn(&self, column: NSInteger);

    #[objc2::method(sel = "validateVisibleColumns")]
    pub unsafe fn validateVisibleColumns(&self);

    #[objc2::method(sel = "scrollColumnsRightBy:")]
    pub unsafe fn scrollColumnsRightBy(&self, shift_amount: NSInteger);

    #[objc2::method(sel = "scrollColumnsLeftBy:")]
    pub unsafe fn scrollColumnsLeftBy(&self, shift_amount: NSInteger);

    #[objc2::method(sel = "scrollColumnToVisible:")]
    pub unsafe fn scrollColumnToVisible(&self, column: NSInteger);

    #[objc2::method(sel = "lastColumn")]
    pub unsafe fn lastColumn(&self) -> NSInteger;

    #[objc2::method(sel = "setLastColumn:")]
    pub unsafe fn setLastColumn(&self, last_column: NSInteger);

    #[objc2::method(sel = "addColumn")]
    pub unsafe fn addColumn(&self);

    #[objc2::method(sel = "numberOfVisibleColumns")]
    pub unsafe fn numberOfVisibleColumns(&self) -> NSInteger;

    #[objc2::method(sel = "firstVisibleColumn")]
    pub unsafe fn firstVisibleColumn(&self) -> NSInteger;

    #[objc2::method(sel = "lastVisibleColumn")]
    pub unsafe fn lastVisibleColumn(&self) -> NSInteger;

    #[objc2::method(sel = "loadedCellAtRow:column:", managed = "Other")]
    pub unsafe fn loadedCellAtRow_column(
        &self,
        row: NSInteger,
        col: NSInteger,
    ) -> Option<Id<Object>>;

    #[objc2::method(sel = "selectAll:")]
    pub unsafe fn selectAll(&self, sender: Option<&Object>);

    #[objc2::method(sel = "tile")]
    pub unsafe fn tile(&self);

    #[objc2::method(sel = "doClick:")]
    pub unsafe fn doClick(&self, sender: Option<&Object>);

    #[objc2::method(sel = "doDoubleClick:")]
    pub unsafe fn doDoubleClick(&self, sender: Option<&Object>);

    #[objc2::method(sel = "sendAction")]
    pub unsafe fn sendAction(&self) -> bool;

    #[objc2::method(sel = "titleFrameOfColumn:")]
    pub unsafe fn titleFrameOfColumn(&self, column: NSInteger) -> NSRect;

    #[objc2::method(sel = "drawTitleOfColumn:inRect:")]
    pub unsafe fn drawTitleOfColumn_inRect(&self, column: NSInteger, rect: NSRect);

    #[objc2::method(sel = "titleHeight")]
    pub unsafe fn titleHeight(&self) -> CGFloat;

    #[objc2::method(sel = "frameOfColumn:")]
    pub unsafe fn frameOfColumn(&self, column: NSInteger) -> NSRect;

    #[objc2::method(sel = "frameOfInsideOfColumn:")]
    pub unsafe fn frameOfInsideOfColumn(&self, column: NSInteger) -> NSRect;

    #[objc2::method(sel = "frameOfRow:inColumn:")]
    pub unsafe fn frameOfRow_inColumn(&self, row: NSInteger, column: NSInteger) -> NSRect;

    #[objc2::method(sel = "getRow:column:forPoint:")]
    pub unsafe fn getRow_column_forPoint(
        &self,
        row: *mut NSInteger,
        column: *mut NSInteger,
        point: NSPoint,
    ) -> bool;

    #[objc2::method(sel = "columnWidthForColumnContentWidth:")]
    pub unsafe fn columnWidthForColumnContentWidth(&self, column_content_width: CGFloat)
        -> CGFloat;

    #[objc2::method(sel = "columnContentWidthForColumnWidth:")]
    pub unsafe fn columnContentWidthForColumnWidth(&self, column_width: CGFloat) -> CGFloat;

    #[objc2::method(sel = "columnResizingType")]
    pub unsafe fn columnResizingType(&self) -> NSBrowserColumnResizingType;

    #[objc2::method(sel = "setColumnResizingType:")]
    pub unsafe fn setColumnResizingType(&self, column_resizing_type: NSBrowserColumnResizingType);

    #[objc2::method(sel = "prefersAllColumnUserResizing")]
    pub unsafe fn prefersAllColumnUserResizing(&self) -> bool;

    #[objc2::method(sel = "setPrefersAllColumnUserResizing:")]
    pub unsafe fn setPrefersAllColumnUserResizing(&self, prefers_all_column_user_resizing: bool);

    #[objc2::method(sel = "setWidth:ofColumn:")]
    pub unsafe fn setWidth_ofColumn(&self, column_width: CGFloat, column_index: NSInteger);

    #[objc2::method(sel = "widthOfColumn:")]
    pub unsafe fn widthOfColumn(&self, column: NSInteger) -> CGFloat;

    #[objc2::method(sel = "rowHeight")]
    pub unsafe fn rowHeight(&self) -> CGFloat;

    #[objc2::method(sel = "setRowHeight:")]
    pub unsafe fn setRowHeight(&self, row_height: CGFloat);

    #[cfg(feature = "Foundation_NSIndexSet")]
    #[objc2::method(sel = "noteHeightOfRowsWithIndexesChanged:inColumn:")]
    pub unsafe fn noteHeightOfRowsWithIndexesChanged_inColumn(
        &self,
        index_set: &NSIndexSet,
        column_index: NSInteger,
    );

    #[objc2::method(sel = "setDefaultColumnWidth:")]
    pub unsafe fn setDefaultColumnWidth(&self, column_width: CGFloat);

    #[objc2::method(sel = "defaultColumnWidth")]
    pub unsafe fn defaultColumnWidth(&self) -> CGFloat;

    #[objc2::method(sel = "columnsAutosaveName", managed = "Other")]
    pub unsafe fn columnsAutosaveName(&self) -> Id<NSBrowserColumnsAutosaveName>;

    #[objc2::method(sel = "setColumnsAutosaveName:")]
    pub unsafe fn setColumnsAutosaveName(
        &self,
        columns_autosave_name: &NSBrowserColumnsAutosaveName,
    );

    #[objc2::method(sel = "removeSavedColumnsWithAutosaveName:")]
    pub unsafe fn removeSavedColumnsWithAutosaveName(name: &NSBrowserColumnsAutosaveName);

    #[cfg(all(feature = "AppKit_NSEvent", feature = "Foundation_NSIndexSet"))]
    #[objc2::method(sel = "canDragRowsWithIndexes:inColumn:withEvent:")]
    pub unsafe fn canDragRowsWithIndexes_inColumn_withEvent(
        &self,
        row_indexes: &NSIndexSet,
        column: NSInteger,
        event: &NSEvent,
    ) -> bool;

    #[cfg(all(
        feature = "AppKit_NSEvent",
        feature = "AppKit_NSImage",
        feature = "Foundation_NSIndexSet"
    ))]
    #[objc2::method(
        sel = "draggingImageForRowsWithIndexes:inColumn:withEvent:offset:",
        managed = "Other"
    )]
    pub unsafe fn draggingImageForRowsWithIndexes_inColumn_withEvent_offset(
        &self,
        row_indexes: &NSIndexSet,
        column: NSInteger,
        event: &NSEvent,
        drag_image_offset: NSPointPointer,
    ) -> Option<Id<NSImage>>;

    #[objc2::method(sel = "setDraggingSourceOperationMask:forLocal:")]
    pub unsafe fn setDraggingSourceOperationMask_forLocal(
        &self,
        mask: NSDragOperation,
        is_local: bool,
    );

    #[objc2::method(sel = "allowsTypeSelect")]
    pub unsafe fn allowsTypeSelect(&self) -> bool;

    #[objc2::method(sel = "setAllowsTypeSelect:")]
    pub unsafe fn setAllowsTypeSelect(&self, allows_type_select: bool);

    #[cfg(feature = "AppKit_NSColor")]
    #[objc2::method(sel = "backgroundColor", managed = "Other")]
    pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

    #[cfg(feature = "AppKit_NSColor")]
    #[objc2::method(sel = "setBackgroundColor:")]
    pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

    #[cfg(all(feature = "AppKit_NSEvent", feature = "Foundation_NSIndexPath"))]
    #[objc2::method(sel = "editItemAtIndexPath:withEvent:select:")]
    pub unsafe fn editItemAtIndexPath_withEvent_select(
        &self,
        index_path: &NSIndexPath,
        event: Option<&NSEvent>,
        select: bool,
    );
}

extern_static!(NSBrowserColumnConfigurationDidChangeNotification: &'static NSNotificationName);

#[objc2::protocol]
pub unsafe trait NSBrowserDelegate: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:numberOfRowsInColumn:")]
    unsafe fn browser_numberOfRowsInColumn(
        &self,
        sender: &NSBrowser,
        column: NSInteger,
    ) -> NSInteger;

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "AppKit_NSMatrix"))]
    #[objc2::method(optional, sel = "browser:createRowsForColumn:inMatrix:")]
    unsafe fn browser_createRowsForColumn_inMatrix(
        &self,
        sender: &NSBrowser,
        column: NSInteger,
        matrix: &NSMatrix,
    );

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:numberOfChildrenOfItem:")]
    unsafe fn browser_numberOfChildrenOfItem(
        &self,
        browser: &NSBrowser,
        item: Option<&Object>,
    ) -> NSInteger;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:child:ofItem:", managed = "Other")]
    unsafe fn browser_child_ofItem(
        &self,
        browser: &NSBrowser,
        index: NSInteger,
        item: Option<&Object>,
    ) -> Id<Object>;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:isLeafItem:")]
    unsafe fn browser_isLeafItem(&self, browser: &NSBrowser, item: Option<&Object>) -> bool;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:objectValueForItem:", managed = "Other")]
    unsafe fn browser_objectValueForItem(
        &self,
        browser: &NSBrowser,
        item: Option<&Object>,
    ) -> Option<Id<Object>>;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:heightOfRow:inColumn:")]
    unsafe fn browser_heightOfRow_inColumn(
        &self,
        browser: &NSBrowser,
        row: NSInteger,
        column_index: NSInteger,
    ) -> CGFloat;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "rootItemForBrowser:", managed = "Other")]
    unsafe fn rootItemForBrowser(&self, browser: &NSBrowser) -> Option<Id<Object>>;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:setObjectValue:forItem:")]
    unsafe fn browser_setObjectValue_forItem(
        &self,
        browser: &NSBrowser,
        object: Option<&Object>,
        item: Option<&Object>,
    );

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:shouldEditItem:")]
    unsafe fn browser_shouldEditItem(&self, browser: &NSBrowser, item: Option<&Object>) -> bool;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:willDisplayCell:atRow:column:")]
    unsafe fn browser_willDisplayCell_atRow_column(
        &self,
        sender: &NSBrowser,
        cell: &Object,
        row: NSInteger,
        column: NSInteger,
    );

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
    #[objc2::method(optional, sel = "browser:titleOfColumn:", managed = "Other")]
    unsafe fn browser_titleOfColumn(
        &self,
        sender: &NSBrowser,
        column: NSInteger,
    ) -> Option<Id<NSString>>;

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
    #[objc2::method(optional, sel = "browser:selectCellWithString:inColumn:")]
    unsafe fn browser_selectCellWithString_inColumn(
        &self,
        sender: &NSBrowser,
        title: &NSString,
        column: NSInteger,
    ) -> bool;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:selectRow:inColumn:")]
    unsafe fn browser_selectRow_inColumn(
        &self,
        sender: &NSBrowser,
        row: NSInteger,
        column: NSInteger,
    ) -> bool;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:isColumnValid:")]
    unsafe fn browser_isColumnValid(&self, sender: &NSBrowser, column: NSInteger) -> bool;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browserWillScroll:")]
    unsafe fn browserWillScroll(&self, sender: &NSBrowser);

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browserDidScroll:")]
    unsafe fn browserDidScroll(&self, sender: &NSBrowser);

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:shouldSizeColumn:forUserResize:toWidth:")]
    unsafe fn browser_shouldSizeColumn_forUserResize_toWidth(
        &self,
        browser: &NSBrowser,
        column_index: NSInteger,
        for_user_resize: bool,
        suggested_width: CGFloat,
    ) -> CGFloat;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:sizeToFitWidthOfColumn:")]
    unsafe fn browser_sizeToFitWidthOfColumn(
        &self,
        browser: &NSBrowser,
        column_index: NSInteger,
    ) -> CGFloat;

    #[cfg(feature = "Foundation_NSNotification")]
    #[objc2::method(optional, sel = "browserColumnConfigurationDidChange:")]
    unsafe fn browserColumnConfigurationDidChange(&self, notification: &NSNotification);

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:shouldShowCellExpansionForRow:column:")]
    unsafe fn browser_shouldShowCellExpansionForRow_column(
        &self,
        browser: &NSBrowser,
        row: NSInteger,
        column: NSInteger,
    ) -> bool;

    #[cfg(all(
        feature = "AppKit_NSBrowser",
        feature = "AppKit_NSPasteboard",
        feature = "Foundation_NSIndexSet"
    ))]
    #[objc2::method(optional, sel = "browser:writeRowsWithIndexes:inColumn:toPasteboard:")]
    unsafe fn browser_writeRowsWithIndexes_inColumn_toPasteboard(
        &self,
        browser: &NSBrowser,
        row_indexes: &NSIndexSet,
        column: NSInteger,
        pasteboard: &NSPasteboard,
    ) -> bool;

    #[cfg(all(
        feature = "AppKit_NSBrowser",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSIndexSet",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURL"
    ))]
    #[deprecated = "Use NSFilePromiseReceiver objects instead"]
    #[objc2::method(
        optional,
        sel = "browser:namesOfPromisedFilesDroppedAtDestination:forDraggedRowsWithIndexes:inColumn:",
        managed = "Other"
    )]
    unsafe fn browser_namesOfPromisedFilesDroppedAtDestination_forDraggedRowsWithIndexes_inColumn(
        &self,
        browser: &NSBrowser,
        drop_destination: &NSURL,
        row_indexes: &NSIndexSet,
        column: NSInteger,
    ) -> Id<NSArray<NSString>>;

    #[cfg(all(
        feature = "AppKit_NSBrowser",
        feature = "AppKit_NSEvent",
        feature = "Foundation_NSIndexSet"
    ))]
    #[objc2::method(optional, sel = "browser:canDragRowsWithIndexes:inColumn:withEvent:")]
    unsafe fn browser_canDragRowsWithIndexes_inColumn_withEvent(
        &self,
        browser: &NSBrowser,
        row_indexes: &NSIndexSet,
        column: NSInteger,
        event: &NSEvent,
    ) -> bool;

    #[cfg(all(
        feature = "AppKit_NSBrowser",
        feature = "AppKit_NSEvent",
        feature = "AppKit_NSImage",
        feature = "Foundation_NSIndexSet"
    ))]
    #[objc2::method(
        optional,
        sel = "browser:draggingImageForRowsWithIndexes:inColumn:withEvent:offset:",
        managed = "Other"
    )]
    unsafe fn browser_draggingImageForRowsWithIndexes_inColumn_withEvent_offset(
        &self,
        browser: &NSBrowser,
        row_indexes: &NSIndexSet,
        column: NSInteger,
        event: &NSEvent,
        drag_image_offset: NSPointPointer,
    ) -> Option<Id<NSImage>>;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(
        optional,
        sel = "browser:validateDrop:proposedRow:column:dropOperation:"
    )]
    unsafe fn browser_validateDrop_proposedRow_column_dropOperation(
        &self,
        browser: &NSBrowser,
        info: &ProtocolObject<dyn NSDraggingInfo>,
        row: NonNull<NSInteger>,
        column: NonNull<NSInteger>,
        drop_operation: NonNull<NSBrowserDropOperation>,
    ) -> NSDragOperation;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:acceptDrop:atRow:column:dropOperation:")]
    unsafe fn browser_acceptDrop_atRow_column_dropOperation(
        &self,
        browser: &NSBrowser,
        info: &ProtocolObject<dyn NSDraggingInfo>,
        row: NSInteger,
        column: NSInteger,
        drop_operation: NSBrowserDropOperation,
    ) -> bool;

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
    #[objc2::method(
        optional,
        sel = "browser:typeSelectStringForRow:inColumn:",
        managed = "Other"
    )]
    unsafe fn browser_typeSelectStringForRow_inColumn(
        &self,
        browser: &NSBrowser,
        row: NSInteger,
        column: NSInteger,
    ) -> Option<Id<NSString>>;

    #[cfg(all(
        feature = "AppKit_NSBrowser",
        feature = "AppKit_NSEvent",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(
        optional,
        sel = "browser:shouldTypeSelectForEvent:withCurrentSearchString:"
    )]
    unsafe fn browser_shouldTypeSelectForEvent_withCurrentSearchString(
        &self,
        browser: &NSBrowser,
        event: &NSEvent,
        search_string: Option<&NSString>,
    ) -> bool;

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
    #[objc2::method(
        optional,
        sel = "browser:nextTypeSelectMatchFromRow:toRow:inColumn:forString:"
    )]
    unsafe fn browser_nextTypeSelectMatchFromRow_toRow_inColumn_forString(
        &self,
        browser: &NSBrowser,
        start_row: NSInteger,
        end_row: NSInteger,
        column: NSInteger,
        search_string: Option<&NSString>,
    ) -> NSInteger;

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "AppKit_NSViewController"))]
    #[objc2::method(
        optional,
        sel = "browser:previewViewControllerForLeafItem:",
        managed = "Other"
    )]
    unsafe fn browser_previewViewControllerForLeafItem(
        &self,
        browser: &NSBrowser,
        item: &Object,
    ) -> Option<Id<NSViewController>>;

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "AppKit_NSViewController"))]
    #[objc2::method(
        optional,
        sel = "browser:headerViewControllerForItem:",
        managed = "Other"
    )]
    unsafe fn browser_headerViewControllerForItem(
        &self,
        browser: &NSBrowser,
        item: Option<&Object>,
    ) -> Option<Id<NSViewController>>;

    #[cfg(feature = "AppKit_NSBrowser")]
    #[objc2::method(optional, sel = "browser:didChangeLastColumn:toColumn:")]
    unsafe fn browser_didChangeLastColumn_toColumn(
        &self,
        browser: &NSBrowser,
        old_last_column: NSInteger,
        column: NSInteger,
    );

    #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSIndexSet"))]
    #[objc2::method(
        optional,
        sel = "browser:selectionIndexesForProposedSelection:inColumn:",
        managed = "Other"
    )]
    unsafe fn browser_selectionIndexesForProposedSelection_inColumn(
        &self,
        browser: &NSBrowser,
        proposed_selection_indexes: &NSIndexSet,
        column: NSInteger,
    ) -> Id<NSIndexSet>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSBrowser")]
    pub type NSBrowser;

    #[deprecated]
    #[objc2::method(sel = "setAcceptsArrowKeys:")]
    pub unsafe fn setAcceptsArrowKeys(&self, flag: bool);

    #[deprecated]
    #[objc2::method(sel = "acceptsArrowKeys")]
    pub unsafe fn acceptsArrowKeys(&self) -> bool;

    #[deprecated]
    #[objc2::method(sel = "displayColumn:")]
    pub unsafe fn displayColumn(&self, column: NSInteger);

    #[deprecated]
    #[objc2::method(sel = "displayAllColumns")]
    pub unsafe fn displayAllColumns(&self);

    #[cfg(feature = "AppKit_NSScroller")]
    #[deprecated]
    #[objc2::method(sel = "scrollViaScroller:")]
    pub unsafe fn scrollViaScroller(&self, sender: Option<&NSScroller>);

    #[deprecated]
    #[objc2::method(sel = "updateScroller")]
    pub unsafe fn updateScroller(&self);

    #[deprecated = "Use the item based NSBrowser instead"]
    #[objc2::method(sel = "setMatrixClass:")]
    pub unsafe fn setMatrixClass(&self, factory_id: &Class);

    #[deprecated = "Use the item based NSBrowser instead"]
    #[objc2::method(sel = "matrixClass")]
    pub unsafe fn matrixClass(&self) -> &'static Class;

    #[cfg(feature = "AppKit_NSMatrix")]
    #[deprecated = "Use the item based NSBrowser instead"]
    #[objc2::method(sel = "columnOfMatrix:")]
    pub unsafe fn columnOfMatrix(&self, matrix: &NSMatrix) -> NSInteger;

    #[cfg(feature = "AppKit_NSMatrix")]
    #[deprecated = "Use the item based NSBrowser instead"]
    #[objc2::method(sel = "matrixInColumn:", managed = "Other")]
    pub unsafe fn matrixInColumn(&self, column: NSInteger) -> Option<Id<NSMatrix>>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSBrowser")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSBrowser")]
    pub type NSBrowser;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}
