//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFormCell")]
    pub struct NSFormCell;

    #[cfg(feature = "AppKit_NSFormCell")]
    unsafe impl ClassType for NSFormCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

#[cfg(feature = "AppKit_NSFormCell")]
unsafe impl NSAccessibility for NSFormCell {}

#[cfg(feature = "AppKit_NSFormCell")]
unsafe impl NSAccessibilityElementProtocol for NSFormCell {}

#[cfg(feature = "AppKit_NSFormCell")]
unsafe impl NSCoding for NSFormCell {}

#[cfg(feature = "AppKit_NSFormCell")]
unsafe impl NSObjectProtocol for NSFormCell {}

#[cfg(feature = "AppKit_NSFormCell")]
unsafe impl NSUserInterfaceItemIdentification for NSFormCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFormCell")]
    unsafe impl NSFormCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;

        #[method(titleWidth:)]
        pub unsafe fn titleWidth_(&self, size: NSSize) -> CGFloat;

        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;

        #[method(setTitleWidth:)]
        pub unsafe fn setTitleWidth(&self, title_width: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Id<NSFont, Shared>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, title_font: &NSFont);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[method(titleAlignment)]
        pub unsafe fn titleAlignment(&self) -> NSTextAlignment;

        #[method(setTitleAlignment:)]
        pub unsafe fn setTitleAlignment(&self, title_alignment: NSTextAlignment);

        #[method(titleBaseWritingDirection)]
        pub unsafe fn titleBaseWritingDirection(&self) -> NSWritingDirection;

        #[method(setTitleBaseWritingDirection:)]
        pub unsafe fn setTitleBaseWritingDirection(
            &self,
            title_base_writing_direction: NSWritingDirection,
        );

        #[method(preferredTextFieldWidth)]
        pub unsafe fn preferredTextFieldWidth(&self) -> CGFloat;

        #[method(setPreferredTextFieldWidth:)]
        pub unsafe fn setPreferredTextFieldWidth(&self, preferred_text_field_width: CGFloat);
    }
);

extern_methods!(
    /// NSKeyboardUI
    #[cfg(feature = "AppKit_NSFormCell")]
    unsafe impl NSFormCell {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);

extern_methods!(
    /// NSFormCellAttributedStringMethods
    #[cfg(feature = "AppKit_NSFormCell")]
    unsafe impl NSFormCell {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);
    }
);
