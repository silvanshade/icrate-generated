//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSSearchFieldRecentsAutosaveName = Foundation::NSString;

extern_protocol!(
    pub struct NSSearchFieldDelegate;

    unsafe impl ProtocolType for NSSearchFieldDelegate {
        #[optional]
        #[method(searchFieldDidStartSearching:)]
        pub unsafe fn searchFieldDidStartSearching(&self, sender: &AppKit::NSSearchField);

        #[optional]
        #[method(searchFieldDidEndSearching:)]
        pub unsafe fn searchFieldDidEndSearching(&self, sender: &AppKit::NSSearchField);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSearchField;

    unsafe impl ClassType for NSSearchField {
        #[inherits(AppKit::NSControl, AppKit::NSView, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSTextField;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl NSSearchField {
        #[method(searchTextBounds)]
        pub unsafe fn searchTextBounds(&self) -> Foundation::NSRect;

        #[method(searchButtonBounds)]
        pub unsafe fn searchButtonBounds(&self) -> Foundation::NSRect;

        #[method(cancelButtonBounds)]
        pub unsafe fn cancelButtonBounds(&self) -> Foundation::NSRect;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other recentSearches)]
        pub unsafe fn recentSearches(
            &self,
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(
            &self,
            recentSearches: &Foundation::NSArray<Foundation::NSString>,
        );

        #[method_id(@__retain_semantics Other recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(
            &self,
        ) -> Option<Id<AppKit::NSSearchFieldRecentsAutosaveName, Shared>>;

        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recentsAutosaveName: Option<&AppKit::NSSearchFieldRecentsAutosaveName>,
        );

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other searchMenuTemplate)]
        pub unsafe fn searchMenuTemplate(&self) -> Option<Id<AppKit::NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setSearchMenuTemplate:)]
        pub unsafe fn setSearchMenuTemplate(&self, searchMenuTemplate: Option<&AppKit::NSMenu>);

        #[method(sendsWholeSearchString)]
        pub unsafe fn sendsWholeSearchString(&self) -> bool;

        #[method(setSendsWholeSearchString:)]
        pub unsafe fn setSendsWholeSearchString(&self, sendsWholeSearchString: bool);

        #[method(maximumRecents)]
        pub unsafe fn maximumRecents(&self) -> NSInteger;

        #[method(setMaximumRecents:)]
        pub unsafe fn setMaximumRecents(&self, maximumRecents: NSInteger);

        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;

        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sendsSearchStringImmediately: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSSearchFieldDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSSearchFieldDelegate>);
    }
);

extern_methods!(
    /// NSSearchField_Deprecated
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl NSSearchField {
        #[method(rectForSearchTextWhenCentered:)]
        pub unsafe fn rectForSearchTextWhenCentered(&self, isCentered: bool) -> Foundation::NSRect;

        #[method(rectForSearchButtonWhenCentered:)]
        pub unsafe fn rectForSearchButtonWhenCentered(
            &self,
            isCentered: bool,
        ) -> Foundation::NSRect;

        #[method(rectForCancelButtonWhenCentered:)]
        pub unsafe fn rectForCancelButtonWhenCentered(
            &self,
            isCentered: bool,
        ) -> Foundation::NSRect;

        #[method(centersPlaceholder)]
        pub unsafe fn centersPlaceholder(&self) -> bool;

        #[method(setCentersPlaceholder:)]
        pub unsafe fn setCentersPlaceholder(&self, centersPlaceholder: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextField`
    ///
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl AppKit::NSSearchField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(stringValue: &Foundation::NSString) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(
            stringValue: &Foundation::NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributedStringValue: &Foundation::NSAttributedString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(stringValue: &Foundation::NSString) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl AppKit::NSSearchField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
