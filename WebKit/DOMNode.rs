//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[extern_enum]
#[underlying(c_uint)]
#[deprecated]
pub enum __anonymous__ {
    #[deprecated]
    DOM_ELEMENT_NODE = 1,
    #[deprecated]
    DOM_ATTRIBUTE_NODE = 2,
    #[deprecated]
    DOM_TEXT_NODE = 3,
    #[deprecated]
    DOM_CDATA_SECTION_NODE = 4,
    #[deprecated]
    DOM_ENTITY_REFERENCE_NODE = 5,
    #[deprecated]
    DOM_ENTITY_NODE = 6,
    #[deprecated]
    DOM_PROCESSING_INSTRUCTION_NODE = 7,
    #[deprecated]
    DOM_COMMENT_NODE = 8,
    #[deprecated]
    DOM_DOCUMENT_NODE = 9,
    #[deprecated]
    DOM_DOCUMENT_TYPE_NODE = 10,
    #[deprecated]
    DOM_DOCUMENT_FRAGMENT_NODE = 11,
    #[deprecated]
    DOM_NOTATION_NODE = 12,
    #[deprecated]
    DOM_DOCUMENT_POSITION_DISCONNECTED = 0x01,
    #[deprecated]
    DOM_DOCUMENT_POSITION_PRECEDING = 0x02,
    #[deprecated]
    DOM_DOCUMENT_POSITION_FOLLOWING = 0x04,
    #[deprecated]
    DOM_DOCUMENT_POSITION_CONTAINS = 0x08,
    #[deprecated]
    DOM_DOCUMENT_POSITION_CONTAINED_BY = 0x10,
    #[deprecated]
    DOM_DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC = 0x20,
}

#[objc2::interface(
    unsafe super = DOMObject,
    unsafe inherits = [
        WebScriptObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_DOMNode")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMNode;
}

#[cfg(feature = "WebKit_DOMNode")]
unsafe impl DOMEventTarget for DOMNode {}

#[cfg(feature = "WebKit_DOMNode")]
unsafe impl NSObjectProtocol for DOMNode {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMNode")]
    #[deprecated]
    pub type DOMNode;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "nodeName", managed = "Other")]
    pub unsafe fn nodeName(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "nodeValue", managed = "Other")]
    pub unsafe fn nodeValue(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNodeValue:")]
    pub unsafe fn setNodeValue(&self, node_value: Option<&NSString>);

    #[objc2::method(sel = "nodeType")]
    pub unsafe fn nodeType(&self) -> c_ushort;

    #[objc2::method(sel = "parentNode", managed = "Other")]
    pub unsafe fn parentNode(&self) -> Option<Id<DOMNode>>;

    #[cfg(feature = "WebKit_DOMNodeList")]
    #[objc2::method(sel = "childNodes", managed = "Other")]
    pub unsafe fn childNodes(&self) -> Option<Id<DOMNodeList>>;

    #[objc2::method(sel = "firstChild", managed = "Other")]
    pub unsafe fn firstChild(&self) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "lastChild", managed = "Other")]
    pub unsafe fn lastChild(&self) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "previousSibling", managed = "Other")]
    pub unsafe fn previousSibling(&self) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "nextSibling", managed = "Other")]
    pub unsafe fn nextSibling(&self) -> Option<Id<DOMNode>>;

    #[cfg(feature = "WebKit_DOMDocument")]
    #[objc2::method(sel = "ownerDocument", managed = "Other")]
    pub unsafe fn ownerDocument(&self) -> Option<Id<DOMDocument>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "namespaceURI", managed = "Other")]
    pub unsafe fn namespaceURI(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "prefix", managed = "Other")]
    pub unsafe fn prefix(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPrefix:")]
    pub unsafe fn setPrefix(&self, prefix: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localName", managed = "Other")]
    pub unsafe fn localName(&self) -> Id<NSString>;

    #[cfg(feature = "WebKit_DOMNamedNodeMap")]
    #[objc2::method(sel = "attributes", managed = "Other")]
    pub unsafe fn attributes(&self) -> Option<Id<DOMNamedNodeMap>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "baseURI", managed = "Other")]
    pub unsafe fn baseURI(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "textContent", managed = "Other")]
    pub unsafe fn textContent(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTextContent:")]
    pub unsafe fn setTextContent(&self, text_content: Option<&NSString>);

    #[cfg(feature = "WebKit_DOMElement")]
    #[objc2::method(sel = "parentElement", managed = "Other")]
    pub unsafe fn parentElement(&self) -> Option<Id<DOMElement>>;

    #[objc2::method(sel = "isContentEditable")]
    pub unsafe fn isContentEditable(&self) -> bool;

    #[objc2::method(sel = "insertBefore:refChild:", managed = "Other")]
    pub unsafe fn insertBefore_refChild(
        &self,
        new_child: Option<&DOMNode>,
        ref_child: Option<&DOMNode>,
    ) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "replaceChild:oldChild:", managed = "Other")]
    pub unsafe fn replaceChild_oldChild(
        &self,
        new_child: Option<&DOMNode>,
        old_child: Option<&DOMNode>,
    ) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "removeChild:", managed = "Other")]
    pub unsafe fn removeChild(&self, old_child: Option<&DOMNode>) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "appendChild:", managed = "Other")]
    pub unsafe fn appendChild(&self, new_child: Option<&DOMNode>) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "hasChildNodes")]
    pub unsafe fn hasChildNodes(&self) -> bool;

    #[objc2::method(sel = "cloneNode:", managed = "Other")]
    pub unsafe fn cloneNode(&self, deep: bool) -> Option<Id<DOMNode>>;

    #[objc2::method(sel = "normalize")]
    pub unsafe fn normalize(&self);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "isSupported:version:")]
    pub unsafe fn isSupported_version(
        &self,
        feature: Option<&NSString>,
        version: Option<&NSString>,
    ) -> bool;

    #[objc2::method(sel = "hasAttributes")]
    pub unsafe fn hasAttributes(&self) -> bool;

    #[objc2::method(sel = "isSameNode:")]
    pub unsafe fn isSameNode(&self, other: Option<&DOMNode>) -> bool;

    #[objc2::method(sel = "isEqualNode:")]
    pub unsafe fn isEqualNode(&self, other: Option<&DOMNode>) -> bool;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "lookupPrefix:", managed = "Other")]
    pub unsafe fn lookupPrefix(&self, namespace_uri: Option<&NSString>) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "lookupNamespaceURI:", managed = "Other")]
    pub unsafe fn lookupNamespaceURI(&self, prefix: Option<&NSString>) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "isDefaultNamespace:")]
    pub unsafe fn isDefaultNamespace(&self, namespace_uri: Option<&NSString>) -> bool;

    #[objc2::method(sel = "compareDocumentPosition:")]
    pub unsafe fn compareDocumentPosition(&self, other: Option<&DOMNode>) -> c_ushort;

    #[objc2::method(sel = "contains:")]
    pub unsafe fn contains(&self, other: Option<&DOMNode>) -> bool;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMNode")]
    pub type DOMNode;

    #[deprecated]
    #[objc2::method(sel = "insertBefore::", managed = "Other")]
    pub unsafe fn insertBefore(
        &self,
        new_child: Option<&DOMNode>,
        ref_child: Option<&DOMNode>,
    ) -> Option<Id<DOMNode>>;

    #[deprecated]
    #[objc2::method(sel = "replaceChild::", managed = "Other")]
    pub unsafe fn replaceChild(
        &self,
        new_child: Option<&DOMNode>,
        old_child: Option<&DOMNode>,
    ) -> Option<Id<DOMNode>>;

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated]
    #[objc2::method(sel = "isSupported::")]
    pub unsafe fn isSupported(
        &self,
        feature: Option<&NSString>,
        version: Option<&NSString>,
    ) -> bool;
}
