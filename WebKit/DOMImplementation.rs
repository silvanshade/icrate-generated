//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = DOMObject,
    unsafe inherits = [
        WebScriptObject,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_DOMImplementation")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type DOMImplementation;
}

#[cfg(feature = "WebKit_DOMImplementation")]
unsafe impl NSObjectProtocol for DOMImplementation {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMImplementation")]
    #[deprecated]
    pub type DOMImplementation;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "hasFeature:version:")]
    pub unsafe fn hasFeature_version(
        &self,
        feature: Option<&NSString>,
        version: Option<&NSString>,
    ) -> bool;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMDocumentType"))]
    #[objc2::method(sel = "createDocumentType:publicId:systemId:", managed = "Other")]
    pub unsafe fn createDocumentType_publicId_systemId(
        &self,
        qualified_name: Option<&NSString>,
        public_id: Option<&NSString>,
        system_id: Option<&NSString>,
    ) -> Option<Id<DOMDocumentType>>;

    #[cfg(all(
        feature = "Foundation_NSString",
        feature = "WebKit_DOMDocument",
        feature = "WebKit_DOMDocumentType"
    ))]
    #[objc2::method(sel = "createDocument:qualifiedName:doctype:", managed = "Other")]
    pub unsafe fn createDocument_qualifiedName_doctype(
        &self,
        namespace_uri: Option<&NSString>,
        qualified_name: Option<&NSString>,
        doctype: Option<&DOMDocumentType>,
    ) -> Option<Id<DOMDocument>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMCSSStyleSheet"))]
    #[objc2::method(sel = "createCSSStyleSheet:media:", managed = "Other")]
    pub unsafe fn createCSSStyleSheet_media(
        &self,
        title: Option<&NSString>,
        media: Option<&NSString>,
    ) -> Option<Id<DOMCSSStyleSheet>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMHTMLDocument"))]
    #[objc2::method(sel = "createHTMLDocument:", managed = "Other")]
    pub unsafe fn createHTMLDocument(
        &self,
        title: Option<&NSString>,
    ) -> Option<Id<DOMHTMLDocument>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_DOMImplementation")]
    pub type DOMImplementation;

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated]
    #[objc2::method(sel = "hasFeature::")]
    pub unsafe fn hasFeature(&self, feature: Option<&NSString>, version: Option<&NSString>)
        -> bool;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMDocumentType"))]
    #[deprecated]
    #[objc2::method(sel = "createDocumentType:::", managed = "Other")]
    pub unsafe fn createDocumentType(
        &self,
        qualified_name: Option<&NSString>,
        public_id: Option<&NSString>,
        system_id: Option<&NSString>,
    ) -> Option<Id<DOMDocumentType>>;

    #[cfg(all(
        feature = "Foundation_NSString",
        feature = "WebKit_DOMDocument",
        feature = "WebKit_DOMDocumentType"
    ))]
    #[deprecated]
    #[objc2::method(sel = "createDocument:::", managed = "Other")]
    pub unsafe fn createDocument(
        &self,
        namespace_uri: Option<&NSString>,
        qualified_name: Option<&NSString>,
        doctype: Option<&DOMDocumentType>,
    ) -> Option<Id<DOMDocument>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMCSSStyleSheet"))]
    #[deprecated]
    #[objc2::method(sel = "createCSSStyleSheet::", managed = "Other")]
    pub unsafe fn createCSSStyleSheet(
        &self,
        title: Option<&NSString>,
        media: Option<&NSString>,
    ) -> Option<Id<DOMCSSStyleSheet>>;
}
