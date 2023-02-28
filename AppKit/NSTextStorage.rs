//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSTextStorageEditActions {
    NSTextStorageEditedAttributes = 1 << 0,
    NSTextStorageEditedCharacters = 1 << 1,
}

#[objc2::interface(
    unsafe super = NSMutableAttributedString,
    unsafe inherits = [
        NSAttributedString,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTextStorage")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSTextStorage;
}

#[cfg(feature = "AppKit_NSTextStorage")]
unsafe impl NSCoding for NSTextStorage {}

#[cfg(feature = "AppKit_NSTextStorage")]
unsafe impl NSObjectProtocol for NSTextStorage {}

#[cfg(feature = "AppKit_NSTextStorage")]
unsafe impl NSSecureCoding for NSTextStorage {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTextStorage")]
    pub type NSTextStorage;

    #[cfg(all(feature = "AppKit_NSLayoutManager", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "layoutManagers", managed = "Other")]
    pub unsafe fn layoutManagers(&self) -> Id<NSArray<NSLayoutManager>>;

    #[cfg(feature = "AppKit_NSLayoutManager")]
    #[objc2::method(sel = "addLayoutManager:")]
    pub unsafe fn addLayoutManager(&self, a_layout_manager: &NSLayoutManager);

    #[cfg(feature = "AppKit_NSLayoutManager")]
    #[objc2::method(sel = "removeLayoutManager:")]
    pub unsafe fn removeLayoutManager(&self, a_layout_manager: &NSLayoutManager);

    #[objc2::method(sel = "editedMask")]
    pub unsafe fn editedMask(&self) -> NSTextStorageEditActions;

    #[objc2::method(sel = "editedRange")]
    pub unsafe fn editedRange(&self) -> NSRange;

    #[objc2::method(sel = "changeInLength")]
    pub unsafe fn changeInLength(&self) -> NSInteger;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSTextStorageDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSTextStorageDelegate>>);

    #[objc2::method(sel = "edited:range:changeInLength:")]
    pub unsafe fn edited_range_changeInLength(
        &self,
        edited_mask: NSTextStorageEditActions,
        edited_range: NSRange,
        delta: NSInteger,
    );

    #[objc2::method(sel = "processEditing")]
    pub unsafe fn processEditing(&self);

    #[objc2::method(sel = "fixesAttributesLazily")]
    pub unsafe fn fixesAttributesLazily(&self) -> bool;

    #[objc2::method(sel = "invalidateAttributesInRange:")]
    pub unsafe fn invalidateAttributesInRange(&self, range: NSRange);

    #[objc2::method(sel = "ensureAttributesAreFixedInRange:")]
    pub unsafe fn ensureAttributesAreFixedInRange(&self, range: NSRange);

    #[objc2::method(sel = "textStorageObserver", managed = "Other")]
    pub unsafe fn textStorageObserver(
        &self,
    ) -> Option<Id<ProtocolObject<dyn NSTextStorageObserving>>>;

    #[objc2::method(sel = "setTextStorageObserver:")]
    pub unsafe fn setTextStorageObserver(
        &self,
        text_storage_observer: Option<&ProtocolObject<dyn NSTextStorageObserving>>,
    );
}

#[objc2::protocol]
pub unsafe trait NSTextStorageDelegate: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSTextStorage")]
    #[objc2::method(optional, sel = "textStorage:willProcessEditing:range:changeInLength:")]
    unsafe fn textStorage_willProcessEditing_range_changeInLength(
        &self,
        text_storage: &NSTextStorage,
        edited_mask: NSTextStorageEditActions,
        edited_range: NSRange,
        delta: NSInteger,
    );

    #[cfg(feature = "AppKit_NSTextStorage")]
    #[objc2::method(optional, sel = "textStorage:didProcessEditing:range:changeInLength:")]
    unsafe fn textStorage_didProcessEditing_range_changeInLength(
        &self,
        text_storage: &NSTextStorage,
        edited_mask: NSTextStorageEditActions,
        edited_range: NSRange,
        delta: NSInteger,
    );
}

extern_static!(NSTextStorageWillProcessEditingNotification: &'static NSNotificationName);

extern_static!(NSTextStorageDidProcessEditingNotification: &'static NSNotificationName);

#[objc2::protocol]
pub unsafe trait NSTextStorageObserving: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSTextStorage")]
    #[objc2::method(sel = "textStorage", managed = "Other")]
    unsafe fn textStorage(&self) -> Option<Id<NSTextStorage>>;

    #[cfg(feature = "AppKit_NSTextStorage")]
    #[objc2::method(sel = "setTextStorage:")]
    unsafe fn setTextStorage(&self, text_storage: Option<&NSTextStorage>);

    #[cfg(feature = "AppKit_NSTextStorage")]
    #[objc2::method(
        sel = "processEditingForTextStorage:edited:range:changeInLength:invalidatedRange:"
    )]
    unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange(
        &self,
        text_storage: &NSTextStorage,
        edit_mask: NSTextStorageEditActions,
        new_char_range: NSRange,
        delta: NSInteger,
        invalidated_char_range: NSRange,
    );

    #[cfg(feature = "AppKit_NSTextStorage")]
    #[objc2::method(sel = "performEditingTransactionForTextStorage:usingBlock:")]
    unsafe fn performEditingTransactionForTextStorage_usingBlock(
        &self,
        text_storage: &NSTextStorage,
        transaction: &Block<(), ()>,
    );
}

pub type NSTextStorageEditedOptions = NSUInteger;

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSAttributedString`
        ///
        /// NSAttributedStringDocumentFormats
    #[cfg(feature = "AppKit_NSTextStorage")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTextStorage")]
    pub type NSTextStorage;

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSError",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "initWithURL:options:documentAttributes:error:",
        managed = "Init",
        throws
    )]
    pub unsafe fn initWithURL_options_documentAttributes_error(
        this: Option<Allocated<Self>>,
        url: &NSURL,
        options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Result<Id<Self>, Id<NSError>>;

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(
        sel = "initWithData:options:documentAttributes:error:",
        managed = "Init",
        throws
    )]
    pub unsafe fn initWithData_options_documentAttributes_error(
        this: Option<Allocated<Self>>,
        data: &NSData,
        options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Result<Id<Self>, Id<NSError>>;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
    #[objc2::method(sel = "initWithRTF:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithRTF_documentAttributes(
        this: Option<Allocated<Self>>,
        data: &NSData,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
    #[objc2::method(sel = "initWithRTFD:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithRTFD_documentAttributes(
        this: Option<Allocated<Self>>,
        data: &NSData,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
    #[objc2::method(sel = "initWithHTML:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithHTML_documentAttributes(
        this: Option<Allocated<Self>>,
        data: &NSData,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(
        feature = "Foundation_NSData",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(sel = "initWithHTML:baseURL:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithHTML_baseURL_documentAttributes(
        this: Option<Allocated<Self>>,
        data: &NSData,
        base: &NSURL,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
    #[objc2::method(sel = "initWithDocFormat:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithDocFormat_documentAttributes(
        this: Option<Allocated<Self>>,
        data: &NSData,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
    #[objc2::method(sel = "initWithHTML:options:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithHTML_options_documentAttributes(
        this: Option<Allocated<Self>>,
        data: &NSData,
        options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSFileWrapper"
    ))]
    #[objc2::method(sel = "initWithRTFDFileWrapper:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithRTFDFileWrapper_documentAttributes(
        this: Option<Allocated<Self>>,
        wrapper: &NSFileWrapper,
        dict: Option<&mut Option<Id<NSDictionary<NSAttributedStringDocumentAttributeKey, Object>>>>,
    ) -> Option<Id<Self>>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSAttributedString`
        ///
        /// NSDeprecatedKitAdditions
    #[cfg(feature = "AppKit_NSTextStorage")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTextStorage")]
    pub type NSTextStorage;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSURL"))]
    #[deprecated = "Use -initWithURL:options:documentAttributes:error: instead"]
    #[objc2::method(sel = "initWithURL:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithURL_documentAttributes(
        this: Option<Allocated<Self>>,
        url: &NSURL,
        dict: Option<&mut Option<Id<NSDictionary>>>,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[deprecated = "Use -initWithURL:options:documentAttributes:error: instead"]
    #[objc2::method(sel = "initWithPath:documentAttributes:", managed = "Init")]
    pub unsafe fn initWithPath_documentAttributes(
        this: Option<Allocated<Self>>,
        path: &NSString,
        dict: Option<&mut Option<Id<NSDictionary>>>,
    ) -> Option<Id<Self>>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSAttributedString`
        ///
        /// NSExtendedAttributedString
    #[cfg(feature = "AppKit_NSTextStorage")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTextStorage")]
    pub type NSTextStorage;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithString:", managed = "Init")]
    pub unsafe fn initWithString(this: Option<Allocated<Self>>, str: &NSString) -> Id<Self>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithString:attributes:", managed = "Init")]
    pub unsafe fn initWithString_attributes(
        this: Option<Allocated<Self>>,
        str: &NSString,
        attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
    ) -> Id<Self>;

    #[objc2::method(sel = "initWithAttributedString:", managed = "Init")]
    pub unsafe fn initWithAttributedString(
        this: Option<Allocated<Self>>,
        attr_str: &NSAttributedString,
    ) -> Id<Self>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSAttributedString`
        ///
        /// NSAttributedStringCreateFromMarkdown
    #[cfg(feature = "AppKit_NSTextStorage")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSTextStorage")]
    pub type NSTextStorage;

    #[cfg(all(
        feature = "Foundation_NSAttributedStringMarkdownParsingOptions",
        feature = "Foundation_NSError",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "initWithContentsOfMarkdownFileAtURL:options:baseURL:error:",
        managed = "Init",
        throws
    )]
    pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
        this: Option<Allocated<Self>>,
        markdown_file: &NSURL,
        options: Option<&NSAttributedStringMarkdownParsingOptions>,
        base_url: Option<&NSURL>,
    ) -> Result<Id<Self>, Id<NSError>>;

    #[cfg(all(
        feature = "Foundation_NSAttributedStringMarkdownParsingOptions",
        feature = "Foundation_NSData",
        feature = "Foundation_NSError",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "initWithMarkdown:options:baseURL:error:",
        managed = "Init",
        throws
    )]
    pub unsafe fn initWithMarkdown_options_baseURL_error(
        this: Option<Allocated<Self>>,
        markdown: &NSData,
        options: Option<&NSAttributedStringMarkdownParsingOptions>,
        base_url: Option<&NSURL>,
    ) -> Result<Id<Self>, Id<NSError>>;

    #[cfg(all(
        feature = "Foundation_NSAttributedStringMarkdownParsingOptions",
        feature = "Foundation_NSError",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "initWithMarkdownString:options:baseURL:error:",
        managed = "Init",
        throws
    )]
    pub unsafe fn initWithMarkdownString_options_baseURL_error(
        this: Option<Allocated<Self>>,
        markdown_string: &NSString,
        options: Option<&NSAttributedStringMarkdownParsingOptions>,
        base_url: Option<&NSURL>,
    ) -> Result<Id<Self>, Id<NSError>>;
}
