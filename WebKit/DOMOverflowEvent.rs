//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_HORIZONTAL = 0,
        #[deprecated]
        DOM_VERTICAL = 1,
        #[deprecated]
        DOM_BOTH = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMOverflowEvent")]
    #[deprecated]
    pub struct DOMOverflowEvent;

    #[cfg(feature = "WebKit_DOMOverflowEvent")]
    unsafe impl ClassType for DOMOverflowEvent {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMEvent;
    }
);

#[cfg(feature = "WebKit_DOMOverflowEvent")]
unsafe impl NSObjectProtocol for DOMOverflowEvent {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMOverflowEvent")]
    unsafe impl DOMOverflowEvent {
        #[method(orient)]
        pub unsafe fn orient(&self) -> c_ushort;

        #[method(horizontalOverflow)]
        pub unsafe fn horizontalOverflow(&self) -> bool;

        #[method(verticalOverflow)]
        pub unsafe fn verticalOverflow(&self) -> bool;

        #[method(initOverflowEvent:horizontalOverflow:verticalOverflow:)]
        pub unsafe fn initOverflowEvent_horizontalOverflow_verticalOverflow(
            &self,
            orient: c_ushort,
            horizontal_overflow: bool,
            vertical_overflow: bool,
        );
    }
);
