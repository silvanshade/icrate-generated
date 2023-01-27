//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_NONE = 0,
        #[deprecated]
        DOM_CAPTURING_PHASE = 1,
        #[deprecated]
        DOM_AT_TARGET = 2,
        #[deprecated]
        DOM_BUBBLING_PHASE = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMEvent")]
    #[deprecated]
    pub struct DOMEvent;

    #[cfg(feature = "WebKit_DOMEvent")]
    unsafe impl ClassType for DOMEvent {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(feature = "WebKit_DOMEvent")]
unsafe impl NSObjectProtocol for DOMEvent {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMEvent")]
    unsafe impl DOMEvent {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<ProtocolObject<dyn DOMEventTarget>, Shared>>;

        #[method_id(@__retain_semantics Other currentTarget)]
        pub unsafe fn currentTarget(
            &self,
        ) -> Option<Id<ProtocolObject<dyn DOMEventTarget>, Shared>>;

        #[method(eventPhase)]
        pub unsafe fn eventPhase(&self) -> c_ushort;

        #[method(bubbles)]
        pub unsafe fn bubbles(&self) -> bool;

        #[method(cancelable)]
        pub unsafe fn cancelable(&self) -> bool;

        #[method(timeStamp)]
        pub unsafe fn timeStamp(&self) -> DOMTimeStamp;

        #[method_id(@__retain_semantics Other srcElement)]
        pub unsafe fn srcElement(&self) -> Option<Id<ProtocolObject<dyn DOMEventTarget>, Shared>>;

        #[method(returnValue)]
        pub unsafe fn returnValue(&self) -> bool;

        #[method(setReturnValue:)]
        pub unsafe fn setReturnValue(&self, return_value: bool);

        #[method(cancelBubble)]
        pub unsafe fn cancelBubble(&self) -> bool;

        #[method(setCancelBubble:)]
        pub unsafe fn setCancelBubble(&self, cancel_bubble: bool);

        #[method(stopPropagation)]
        pub unsafe fn stopPropagation(&self);

        #[method(preventDefault)]
        pub unsafe fn preventDefault(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(initEvent:canBubbleArg:cancelableArg:)]
        pub unsafe fn initEvent_canBubbleArg_cancelableArg(
            &self,
            event_type_arg: Option<&NSString>,
            can_bubble_arg: bool,
            cancelable_arg: bool,
        );
    }
);

extern_methods!(
    /// DOMEventDeprecated
    #[cfg(feature = "WebKit_DOMEvent")]
    unsafe impl DOMEvent {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(initEvent:::)]
        pub unsafe fn initEvent(
            &self,
            event_type_arg: Option<&NSString>,
            can_bubble_arg: bool,
            cancelable_arg: bool,
        );
    }
);
