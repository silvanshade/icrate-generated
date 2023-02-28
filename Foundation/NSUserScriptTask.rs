//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSUserScriptTaskCompletionHandler = *mut Block<(*mut NSError,), ()>;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserScriptTask")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserScriptTask;
}

#[cfg(feature = "Foundation_NSUserScriptTask")]
unsafe impl NSObjectProtocol for NSUserScriptTask {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserScriptTask")]
    pub type NSUserScriptTask;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "initWithURL:error:", managed = "Init", throws)]
    pub unsafe fn initWithURL_error(
        this: Option<Allocated<Self>>,
        url: &NSURL,
    ) -> Result<Id<Self>, Id<NSError>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "scriptURL", managed = "Other")]
    pub unsafe fn scriptURL(&self) -> Id<NSURL>;

    #[objc2::method(sel = "executeWithCompletionHandler:")]
    pub unsafe fn executeWithCompletionHandler(&self, handler: NSUserScriptTaskCompletionHandler);
}

pub type NSUserUnixTaskCompletionHandler = *mut Block<(*mut NSError,), ()>;

#[objc2::interface(
    unsafe super = NSUserScriptTask,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserUnixTask;
}

#[cfg(feature = "Foundation_NSUserUnixTask")]
unsafe impl NSObjectProtocol for NSUserUnixTask {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    pub type NSUserUnixTask;

    #[cfg(feature = "Foundation_NSFileHandle")]
    #[objc2::method(sel = "standardInput", managed = "Other")]
    pub unsafe fn standardInput(&self) -> Option<Id<NSFileHandle>>;

    #[cfg(feature = "Foundation_NSFileHandle")]
    #[objc2::method(sel = "setStandardInput:")]
    pub unsafe fn setStandardInput(&self, standard_input: Option<&NSFileHandle>);

    #[cfg(feature = "Foundation_NSFileHandle")]
    #[objc2::method(sel = "standardOutput", managed = "Other")]
    pub unsafe fn standardOutput(&self) -> Option<Id<NSFileHandle>>;

    #[cfg(feature = "Foundation_NSFileHandle")]
    #[objc2::method(sel = "setStandardOutput:")]
    pub unsafe fn setStandardOutput(&self, standard_output: Option<&NSFileHandle>);

    #[cfg(feature = "Foundation_NSFileHandle")]
    #[objc2::method(sel = "standardError", managed = "Other")]
    pub unsafe fn standardError(&self) -> Option<Id<NSFileHandle>>;

    #[cfg(feature = "Foundation_NSFileHandle")]
    #[objc2::method(sel = "setStandardError:")]
    pub unsafe fn setStandardError(&self, standard_error: Option<&NSFileHandle>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "executeWithArguments:completionHandler:")]
    pub unsafe fn executeWithArguments_completionHandler(
        &self,
        arguments: Option<&NSArray<NSString>>,
        handler: NSUserUnixTaskCompletionHandler,
    );
}

pub type NSUserAppleScriptTaskCompletionHandler =
    *mut Block<(*mut NSAppleEventDescriptor, *mut NSError), ()>;

#[objc2::interface(
    unsafe super = NSUserScriptTask,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserAppleScriptTask;
}

#[cfg(feature = "Foundation_NSUserAppleScriptTask")]
unsafe impl NSObjectProtocol for NSUserAppleScriptTask {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    pub type NSUserAppleScriptTask;

    #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
    #[objc2::method(sel = "executeWithAppleEvent:completionHandler:")]
    pub unsafe fn executeWithAppleEvent_completionHandler(
        &self,
        event: Option<&NSAppleEventDescriptor>,
        handler: NSUserAppleScriptTaskCompletionHandler,
    );
}

pub type NSUserAutomatorTaskCompletionHandler = *mut Block<(*mut Object, *mut NSError), ()>;

#[objc2::interface(
    unsafe super = NSUserScriptTask,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSUserAutomatorTask;
}

#[cfg(feature = "Foundation_NSUserAutomatorTask")]
unsafe impl NSObjectProtocol for NSUserAutomatorTask {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    pub type NSUserAutomatorTask;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "variables", managed = "Other")]
    pub unsafe fn variables(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setVariables:")]
    pub unsafe fn setVariables(&self, variables: Option<&NSDictionary<NSString, Object>>);

    #[objc2::method(sel = "executeWithInput:completionHandler:")]
    pub unsafe fn executeWithInput_completionHandler(
        &self,
        input: Option<&ProtocolObject<dyn NSSecureCoding>>,
        handler: NSUserAutomatorTaskCompletionHandler,
    );
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSUserScriptTask`
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    pub type NSUserUnixTask;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "initWithURL:error:", managed = "Init", throws)]
    pub unsafe fn initWithURL_error(
        this: Option<Allocated<Self>>,
        url: &NSURL,
    ) -> Result<Id<Self>, Id<NSError>>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSUserScriptTask`
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    pub type NSUserAppleScriptTask;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "initWithURL:error:", managed = "Init", throws)]
    pub unsafe fn initWithURL_error(
        this: Option<Allocated<Self>>,
        url: &NSURL,
    ) -> Result<Id<Self>, Id<NSError>>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSUserScriptTask`
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    pub type NSUserAutomatorTask;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "initWithURL:error:", managed = "Init", throws)]
    pub unsafe fn initWithURL_error(
        this: Option<Allocated<Self>>,
        url: &NSURL,
    ) -> Result<Id<Self>, Id<NSError>>;
}
