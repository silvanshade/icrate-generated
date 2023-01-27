//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMShellScriptAction")]
    pub struct AMShellScriptAction;

    #[cfg(feature = "Automator_AMShellScriptAction")]
    unsafe impl ClassType for AMShellScriptAction {
        #[inherits(AMAction, NSObject)]
        type Super = AMBundleAction;
    }
);

#[cfg(feature = "Automator_AMShellScriptAction")]
unsafe impl NSCoding for AMShellScriptAction {}

#[cfg(feature = "Automator_AMShellScriptAction")]
unsafe impl NSObjectProtocol for AMShellScriptAction {}

#[cfg(feature = "Automator_AMShellScriptAction")]
unsafe impl NSSecureCoding for AMShellScriptAction {}

extern_methods!(
    #[cfg(feature = "Automator_AMShellScriptAction")]
    unsafe impl AMShellScriptAction {
        #[method(remapLineEndings)]
        pub unsafe fn remapLineEndings(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other inputFieldSeparator)]
        pub unsafe fn inputFieldSeparator(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other outputFieldSeparator)]
        pub unsafe fn outputFieldSeparator(&self) -> Id<NSString, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AMAction`
    #[cfg(feature = "Automator_AMShellScriptAction")]
    unsafe impl AMShellScriptAction {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithDefinition:fromArchive:)]
        pub unsafe fn initWithDefinition_fromArchive(
            this: Option<Allocated<Self>>,
            dict: Option<&NSDictionary<NSString, Object>>,
            archived: bool,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            file_url: &NSURL,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);
