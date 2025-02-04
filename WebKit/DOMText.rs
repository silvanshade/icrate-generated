//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMText")]
    #[deprecated]
    pub struct DOMText;

    #[cfg(feature = "WebKit_DOMText")]
    unsafe impl ClassType for DOMText {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCharacterData;
    }
);

#[cfg(feature = "WebKit_DOMText")]
unsafe impl DOMEventTarget for DOMText {}

#[cfg(feature = "WebKit_DOMText")]
unsafe impl NSObjectProtocol for DOMText {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMText")]
    unsafe impl DOMText {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wholeText)]
        pub unsafe fn wholeText(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other splitText:)]
        pub unsafe fn splitText(&self, offset: c_uint) -> Option<Id<DOMText>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other replaceWholeText:)]
        pub unsafe fn replaceWholeText(&self, content: Option<&NSString>) -> Option<Id<DOMText>>;
    }
);
