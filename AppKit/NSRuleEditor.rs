//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

typed_enum!(
    pub type NSRuleEditorPredicatePartKey = Foundation::NSString;
);

extern_static!(NSRuleEditorPredicateLeftExpression: &'static AppKit::NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateRightExpression: &'static AppKit::NSRuleEditorPredicatePartKey);

extern_static!(
    NSRuleEditorPredicateComparisonModifier: &'static AppKit::NSRuleEditorPredicatePartKey
);

extern_static!(NSRuleEditorPredicateOptions: &'static AppKit::NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateOperatorType: &'static AppKit::NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateCustomSelector: &'static AppKit::NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateCompoundType: &'static AppKit::NSRuleEditorPredicatePartKey);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRuleEditorNestingMode {
        NSRuleEditorNestingModeSingle = 0,
        NSRuleEditorNestingModeList = 1,
        NSRuleEditorNestingModeCompound = 2,
        NSRuleEditorNestingModeSimple = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRuleEditorRowType {
        NSRuleEditorRowTypeSimple = 0,
        NSRuleEditorRowTypeCompound = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRuleEditor;

    unsafe impl ClassType for NSRuleEditor {
        #[inherits(AppKit::NSView, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSControl;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSRuleEditor")]
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSRuleEditorDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSRuleEditorDelegate>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other formattingStringsFilename)]
        pub unsafe fn formattingStringsFilename(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFormattingStringsFilename:)]
        pub unsafe fn setFormattingStringsFilename(
            &self,
            formattingStringsFilename: Option<&Foundation::NSString>,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other formattingDictionary)]
        pub unsafe fn formattingDictionary(
            &self,
        ) -> Option<Id<Foundation::NSDictionary<Foundation::NSString, Foundation::NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setFormattingDictionary:)]
        pub unsafe fn setFormattingDictionary(
            &self,
            formattingDictionary: Option<
                &Foundation::NSDictionary<Foundation::NSString, Foundation::NSString>,
            >,
        );

        #[method(reloadCriteria)]
        pub unsafe fn reloadCriteria(&self);

        #[method(nestingMode)]
        pub unsafe fn nestingMode(&self) -> AppKit::NSRuleEditorNestingMode;

        #[method(setNestingMode:)]
        pub unsafe fn setNestingMode(&self, nestingMode: AppKit::NSRuleEditorNestingMode);

        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> Foundation::CGFloat;

        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, rowHeight: Foundation::CGFloat);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(canRemoveAllRows)]
        pub unsafe fn canRemoveAllRows(&self) -> bool;

        #[method(setCanRemoveAllRows:)]
        pub unsafe fn setCanRemoveAllRows(&self, canRemoveAllRows: bool);

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<Foundation::NSPredicate, Shared>>;

        #[method(reloadPredicate)]
        pub unsafe fn reloadPredicate(&self);

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicateForRow:)]
        pub unsafe fn predicateForRow(
            &self,
            row: NSInteger,
        ) -> Option<Id<Foundation::NSPredicate, Shared>>;

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other subrowIndexesForRow:)]
        pub unsafe fn subrowIndexesForRow(
            &self,
            rowIndex: NSInteger,
        ) -> Id<Foundation::NSIndexSet, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other criteriaForRow:)]
        pub unsafe fn criteriaForRow(&self, row: NSInteger) -> Id<Foundation::NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other displayValuesForRow:)]
        pub unsafe fn displayValuesForRow(&self, row: NSInteger)
            -> Id<Foundation::NSArray, Shared>;

        #[method(rowForDisplayValue:)]
        pub unsafe fn rowForDisplayValue(&self, displayValue: &Object) -> NSInteger;

        #[method(rowTypeForRow:)]
        pub unsafe fn rowTypeForRow(&self, rowIndex: NSInteger) -> AppKit::NSRuleEditorRowType;

        #[method(parentRowForRow:)]
        pub unsafe fn parentRowForRow(&self, rowIndex: NSInteger) -> NSInteger;

        #[method(addRow:)]
        pub unsafe fn addRow(&self, sender: Option<&Object>);

        #[method(insertRowAtIndex:withType:asSubrowOfRow:animate:)]
        pub unsafe fn insertRowAtIndex_withType_asSubrowOfRow_animate(
            &self,
            rowIndex: NSInteger,
            rowType: AppKit::NSRuleEditorRowType,
            parentRow: NSInteger,
            shouldAnimate: bool,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCriteria:andDisplayValues:forRowAtIndex:)]
        pub unsafe fn setCriteria_andDisplayValues_forRowAtIndex(
            &self,
            criteria: &Foundation::NSArray,
            values: &Foundation::NSArray,
            rowIndex: NSInteger,
        );

        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, rowIndex: NSInteger);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeRowsAtIndexes:includeSubrows:)]
        pub unsafe fn removeRowsAtIndexes_includeSubrows(
            &self,
            rowIndexes: &Foundation::NSIndexSet,
            includeSubrows: bool,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other selectedRowIndexes)]
        pub unsafe fn selectedRowIndexes(&self) -> Id<Foundation::NSIndexSet, Shared>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(selectRowIndexes:byExtendingSelection:)]
        pub unsafe fn selectRowIndexes_byExtendingSelection(
            &self,
            indexes: &Foundation::NSIndexSet,
            extend: bool,
        );

        #[method(rowClass)]
        pub unsafe fn rowClass(&self) -> &'static Class;

        #[method(setRowClass:)]
        pub unsafe fn setRowClass(&self, rowClass: &Class);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other rowTypeKeyPath)]
        pub unsafe fn rowTypeKeyPath(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRowTypeKeyPath:)]
        pub unsafe fn setRowTypeKeyPath(&self, rowTypeKeyPath: &Foundation::NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subrowsKeyPath)]
        pub unsafe fn subrowsKeyPath(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubrowsKeyPath:)]
        pub unsafe fn setSubrowsKeyPath(&self, subrowsKeyPath: &Foundation::NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other criteriaKeyPath)]
        pub unsafe fn criteriaKeyPath(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCriteriaKeyPath:)]
        pub unsafe fn setCriteriaKeyPath(&self, criteriaKeyPath: &Foundation::NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayValuesKeyPath)]
        pub unsafe fn displayValuesKeyPath(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDisplayValuesKeyPath:)]
        pub unsafe fn setDisplayValuesKeyPath(&self, displayValuesKeyPath: &Foundation::NSString);
    }
);

extern_protocol!(
    pub struct NSRuleEditorDelegate;

    unsafe impl ProtocolType for NSRuleEditorDelegate {
        #[method(ruleEditor:numberOfChildrenForCriterion:withRowType:)]
        pub unsafe fn ruleEditor_numberOfChildrenForCriterion_withRowType(
            &self,
            editor: &AppKit::NSRuleEditor,
            criterion: Option<&Object>,
            rowType: AppKit::NSRuleEditorRowType,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other ruleEditor:child:forCriterion:withRowType:)]
        pub unsafe fn ruleEditor_child_forCriterion_withRowType(
            &self,
            editor: &AppKit::NSRuleEditor,
            index: NSInteger,
            criterion: Option<&Object>,
            rowType: AppKit::NSRuleEditorRowType,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other ruleEditor:displayValueForCriterion:inRow:)]
        pub unsafe fn ruleEditor_displayValueForCriterion_inRow(
            &self,
            editor: &AppKit::NSRuleEditor,
            criterion: &Object,
            row: NSInteger,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other ruleEditor:predicatePartsForCriterion:withDisplayValue:inRow:)]
        pub unsafe fn ruleEditor_predicatePartsForCriterion_withDisplayValue_inRow(
            &self,
            editor: &AppKit::NSRuleEditor,
            criterion: &Object,
            value: &Object,
            row: NSInteger,
        ) -> Option<
            Id<Foundation::NSDictionary<AppKit::NSRuleEditorPredicatePartKey, Object>, Shared>,
        >;

        #[optional]
        #[method(ruleEditorRowsDidChange:)]
        pub unsafe fn ruleEditorRowsDidChange(&self, notification: &Foundation::NSNotification);
    }
);

extern_static!(NSRuleEditorRowsDidChangeNotification: &'static Foundation::NSNotificationName);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSRuleEditor")]
    unsafe impl AppKit::NSRuleEditor {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
