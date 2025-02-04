//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLObjectElement")]
    #[deprecated]
    pub struct DOMHTMLObjectElement;

    #[cfg(feature = "WebKit_DOMHTMLObjectElement")]
    unsafe impl ClassType for DOMHTMLObjectElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLObjectElement")]
unsafe impl DOMEventTarget for DOMHTMLObjectElement {}

#[cfg(feature = "WebKit_DOMHTMLObjectElement")]
unsafe impl NSObjectProtocol for DOMHTMLObjectElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLObjectElement")]
    unsafe impl DOMHTMLObjectElement {
        #[cfg(feature = "WebKit_DOMHTMLFormElement")]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other code)]
        pub unsafe fn code(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCode:)]
        pub unsafe fn setCode(&self, code: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other archive)]
        pub unsafe fn archive(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setArchive:)]
        pub unsafe fn setArchive(&self, archive: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other border)]
        pub unsafe fn border(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBorder:)]
        pub unsafe fn setBorder(&self, border: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other codeBase)]
        pub unsafe fn codeBase(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCodeBase:)]
        pub unsafe fn setCodeBase(&self, code_base: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other codeType)]
        pub unsafe fn codeType(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCodeType:)]
        pub unsafe fn setCodeType(&self, code_type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setData:)]
        pub unsafe fn setData(&self, data: Option<&NSString>);

        #[method(declare)]
        pub unsafe fn declare(&self) -> bool;

        #[method(setDeclare:)]
        pub unsafe fn setDeclare(&self, declare: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other height)]
        pub unsafe fn height(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: Option<&NSString>);

        #[method(hspace)]
        pub unsafe fn hspace(&self) -> c_int;

        #[method(setHspace:)]
        pub unsafe fn setHspace(&self, hspace: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other standby)]
        pub unsafe fn standby(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStandby:)]
        pub unsafe fn setStandby(&self, standby: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other useMap)]
        pub unsafe fn useMap(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUseMap:)]
        pub unsafe fn setUseMap(&self, use_map: Option<&NSString>);

        #[method(vspace)]
        pub unsafe fn vspace(&self) -> c_int;

        #[method(setVspace:)]
        pub unsafe fn setVspace(&self, vspace: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMDocument")]
        #[method_id(@__retain_semantics Other contentDocument)]
        pub unsafe fn contentDocument(&self) -> Option<Id<DOMDocument>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other absoluteImageURL)]
        pub unsafe fn absoluteImageURL(&self) -> Id<NSURL>;
    }
);
