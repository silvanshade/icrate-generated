//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPredicateEditor;

    unsafe impl ClassType for NSPredicateEditor {
        #[inherits(AppKit::NSControl, AppKit::NSView, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSRuleEditor;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPredicateEditor")]
    unsafe impl NSPredicateEditor {
        #[cfg(all(
            feature = "AppKit_NSPredicateEditorRowTemplate",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other rowTemplates)]
        pub unsafe fn rowTemplates(
            &self,
        ) -> Id<Foundation::NSArray<AppKit::NSPredicateEditorRowTemplate>, Shared>;

        #[cfg(all(
            feature = "AppKit_NSPredicateEditorRowTemplate",
            feature = "Foundation_NSArray"
        ))]
        #[method(setRowTemplates:)]
        pub unsafe fn setRowTemplates(
            &self,
            rowTemplates: &Foundation::NSArray<AppKit::NSPredicateEditorRowTemplate>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSPredicateEditor")]
    unsafe impl AppKit::NSPredicateEditor {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
