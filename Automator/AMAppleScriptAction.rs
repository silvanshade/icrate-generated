//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

#[objc2::interface(
    unsafe super = AMBundleAction,
    unsafe inherits = [
        AMAction,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type AMAppleScriptAction;
}

#[cfg(feature = "Automator_AMAppleScriptAction")]
unsafe impl NSCoding for AMAppleScriptAction {}

#[cfg(feature = "Automator_AMAppleScriptAction")]
unsafe impl NSObjectProtocol for AMAppleScriptAction {}

#[cfg(feature = "Automator_AMAppleScriptAction")]
unsafe impl NSSecureCoding for AMAppleScriptAction {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    pub type AMAppleScriptAction;

    #[cfg(feature = "OSAKit_OSAScript")]
    #[objc2::method(sel = "script", managed = "Other")]
    pub unsafe fn script(&self) -> Option<Id<OSAScript>>;

    #[cfg(feature = "OSAKit_OSAScript")]
    #[objc2::method(sel = "setScript:")]
    pub unsafe fn setScript(&self, script: Option<&OSAScript>);
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `AMAction`
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    pub type AMAppleScriptAction;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithDefinition:fromArchive:", managed = "Init")]
    pub unsafe fn initWithDefinition_fromArchive(
        this: Option<Allocated<Self>>,
        dict: Option<&NSDictionary<NSString, Object>>,
        archived: bool,
    ) -> Option<Id<Self>>;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "initWithContentsOfURL:error:", managed = "Init", throws)]
    pub unsafe fn initWithContentsOfURL_error(
        this: Option<Allocated<Self>>,
        file_url: &NSURL,
    ) -> Result<Id<Self>, Id<NSError>>;
}
