//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSScriptCoercionHandler")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSScriptCoercionHandler;
}

#[cfg(feature = "Foundation_NSScriptCoercionHandler")]
unsafe impl NSObjectProtocol for NSScriptCoercionHandler {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSScriptCoercionHandler")]
    pub type NSScriptCoercionHandler;

    #[objc2::method(sel = "sharedCoercionHandler", managed = "Other")]
    pub unsafe fn sharedCoercionHandler() -> Id<NSScriptCoercionHandler>;

    #[objc2::method(sel = "coerceValue:toClass:", managed = "Other")]
    pub unsafe fn coerceValue_toClass(
        &self,
        value: &Object,
        to_class: &Class,
    ) -> Option<Id<Object>>;

    #[objc2::method(sel = "registerCoercer:selector:toConvertFromClass:toClass:")]
    pub unsafe fn registerCoercer_selector_toConvertFromClass_toClass(
        &self,
        coercer: &Object,
        selector: Sel,
        from_class: &Class,
        to_class: &Class,
    );
}
