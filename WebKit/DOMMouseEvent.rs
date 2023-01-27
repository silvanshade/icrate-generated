//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMMouseEvent")]
    #[deprecated]
    pub struct DOMMouseEvent;

    #[cfg(feature = "WebKit_DOMMouseEvent")]
    unsafe impl ClassType for DOMMouseEvent {
        #[inherits(DOMEvent, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMUIEvent;
    }
);

#[cfg(feature = "WebKit_DOMMouseEvent")]
unsafe impl NSObjectProtocol for DOMMouseEvent {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMMouseEvent")]
    unsafe impl DOMMouseEvent {
        #[method(screenX)]
        pub unsafe fn screenX(&self) -> c_int;

        #[method(screenY)]
        pub unsafe fn screenY(&self) -> c_int;

        #[method(clientX)]
        pub unsafe fn clientX(&self) -> c_int;

        #[method(clientY)]
        pub unsafe fn clientY(&self) -> c_int;

        #[method(ctrlKey)]
        pub unsafe fn ctrlKey(&self) -> bool;

        #[method(shiftKey)]
        pub unsafe fn shiftKey(&self) -> bool;

        #[method(altKey)]
        pub unsafe fn altKey(&self) -> bool;

        #[method(metaKey)]
        pub unsafe fn metaKey(&self) -> bool;

        #[method(button)]
        pub unsafe fn button(&self) -> c_short;

        #[method_id(@__retain_semantics Other relatedTarget)]
        pub unsafe fn relatedTarget(
            &self,
        ) -> Option<Id<ProtocolObject<dyn DOMEventTarget>, Shared>>;

        #[method(offsetX)]
        pub unsafe fn offsetX(&self) -> c_int;

        #[method(offsetY)]
        pub unsafe fn offsetY(&self) -> c_int;

        #[method(x)]
        pub unsafe fn x(&self) -> c_int;

        #[method(y)]
        pub unsafe fn y(&self) -> c_int;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other fromElement)]
        pub unsafe fn fromElement(&self) -> Option<Id<DOMNode, Shared>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other toElement)]
        pub unsafe fn toElement(&self) -> Option<Id<DOMNode, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMAbstractView"))]
        #[method(initMouseEvent:canBubble:cancelable:view:detail:screenX:screenY:clientX:clientY:ctrlKey:altKey:shiftKey:metaKey:button:relatedTarget:)]
        pub unsafe fn initMouseEvent_canBubble_cancelable_view_detail_screenX_screenY_clientX_clientY_ctrlKey_altKey_shiftKey_metaKey_button_relatedTarget(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            detail: c_int,
            screen_x: c_int,
            screen_y: c_int,
            client_x: c_int,
            client_y: c_int,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
            button: c_ushort,
            related_target: Option<&ProtocolObject<dyn DOMEventTarget>>,
        );
    }
);

extern_methods!(
    /// DOMMouseEventDeprecated
    #[cfg(feature = "WebKit_DOMMouseEvent")]
    unsafe impl DOMMouseEvent {
        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMAbstractView"))]
        #[deprecated]
        #[method(initMouseEvent:::::::::::::::)]
        pub unsafe fn initMouseEvent(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            detail: c_int,
            screen_x: c_int,
            screen_y: c_int,
            client_x: c_int,
            client_y: c_int,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
            button: c_ushort,
            related_target: Option<&ProtocolObject<dyn DOMEventTarget>>,
        );
    }
);
