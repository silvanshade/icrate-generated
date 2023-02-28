//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = DOMHTMLElement,
    unsafe inherits = [
        DOMElement,
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLSelectElement")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMHTMLSelectElement;
}

#[cfg(feature = "WebKit_DOMHTMLSelectElement")]
unsafe impl DOMEventTarget for DOMHTMLSelectElement {}

#[cfg(feature = "WebKit_DOMHTMLSelectElement")]
unsafe impl NSObjectProtocol for DOMHTMLSelectElement {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLSelectElement")]
    #[deprecated]
    pub type DOMHTMLSelectElement;

    #[objc2::method(sel = "autofocus")]
    pub unsafe fn autofocus(&self) -> bool;

    #[objc2::method(sel = "setAutofocus:")]
    pub unsafe fn setAutofocus(&self, autofocus: bool);

    #[objc2::method(sel = "disabled")]
    pub unsafe fn disabled(&self) -> bool;

    #[objc2::method(sel = "setDisabled:")]
    pub unsafe fn setDisabled(&self, disabled: bool);

    #[cfg(feature = "WebKit_DOMHTMLFormElement")]
    #[objc2::method(sel = "form", managed = "Other")]
    pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement>>;

    #[objc2::method(sel = "multiple")]
    pub unsafe fn multiple(&self) -> bool;

    #[objc2::method(sel = "setMultiple:")]
    pub unsafe fn setMultiple(&self, multiple: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setName:")]
    pub unsafe fn setName(&self, name: Option<&NSString>);

    #[objc2::method(sel = "size")]
    pub unsafe fn size(&self) -> c_int;

    #[objc2::method(sel = "setSize:")]
    pub unsafe fn setSize(&self, size: c_int);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "type", managed = "Other")]
    pub unsafe fn r#type(&self) -> Id<NSString>;

    #[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
    #[objc2::method(sel = "options", managed = "Other")]
    pub unsafe fn options(&self) -> Option<Id<DOMHTMLOptionsCollection>>;

    #[objc2::method(sel = "length")]
    pub unsafe fn length(&self) -> c_int;

    #[objc2::method(sel = "selectedIndex")]
    pub unsafe fn selectedIndex(&self) -> c_int;

    #[objc2::method(sel = "setSelectedIndex:")]
    pub unsafe fn setSelectedIndex(&self, selected_index: c_int);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "value", managed = "Other")]
    pub unsafe fn value(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setValue:")]
    pub unsafe fn setValue(&self, value: Option<&NSString>);

    #[objc2::method(sel = "willValidate")]
    pub unsafe fn willValidate(&self) -> bool;

    #[objc2::method(sel = "item:", managed = "Other")]
    pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "namedItem:", managed = "Other")]
    pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "add:before:")]
    pub unsafe fn add_before(
        &self,
        element: Option<&DOMHTMLElement>,
        before: Option<&DOMHTMLElement>,
    );

    #[objc2::method(sel = "remove:")]
    pub unsafe fn remove(&self, index: c_int);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMHTMLSelectElement")]
    pub type DOMHTMLSelectElement;

    #[cfg(feature = "WebKit_DOMHTMLElement")]
    #[deprecated]
    #[objc2::method(sel = "add::")]
    pub unsafe fn add(&self, element: Option<&DOMHTMLElement>, before: Option<&DOMHTMLElement>);
}
