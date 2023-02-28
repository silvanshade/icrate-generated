//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSSpeechRecognizer")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSSpeechRecognizer;
}

#[cfg(feature = "AppKit_NSSpeechRecognizer")]
unsafe impl NSObjectProtocol for NSSpeechRecognizer {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSSpeechRecognizer")]
    pub type NSSpeechRecognizer;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Option<Id<Self>>;

    #[objc2::method(sel = "startListening")]
    pub unsafe fn startListening(&self);

    #[objc2::method(sel = "stopListening")]
    pub unsafe fn stopListening(&self);

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSpeechRecognizerDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(
        &self,
        delegate: Option<&ProtocolObject<dyn NSSpeechRecognizerDelegate>>,
    );

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "commands", managed = "Other")]
    pub unsafe fn commands(&self) -> Option<Id<NSArray<NSString>>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setCommands:")]
    pub unsafe fn setCommands(&self, commands: Option<&NSArray<NSString>>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "displayedCommandsTitle", managed = "Other")]
    pub unsafe fn displayedCommandsTitle(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setDisplayedCommandsTitle:")]
    pub unsafe fn setDisplayedCommandsTitle(&self, displayed_commands_title: Option<&NSString>);

    #[objc2::method(sel = "listensInForegroundOnly")]
    pub unsafe fn listensInForegroundOnly(&self) -> bool;

    #[objc2::method(sel = "setListensInForegroundOnly:")]
    pub unsafe fn setListensInForegroundOnly(&self, listens_in_foreground_only: bool);

    #[objc2::method(sel = "blocksOtherRecognizers")]
    pub unsafe fn blocksOtherRecognizers(&self) -> bool;

    #[objc2::method(sel = "setBlocksOtherRecognizers:")]
    pub unsafe fn setBlocksOtherRecognizers(&self, blocks_other_recognizers: bool);
}

#[objc2::protocol]
pub unsafe trait NSSpeechRecognizerDelegate: NSObjectProtocol {
    #[cfg(all(feature = "AppKit_NSSpeechRecognizer", feature = "Foundation_NSString"))]
    #[objc2::method(optional, sel = "speechRecognizer:didRecognizeCommand:")]
    unsafe fn speechRecognizer_didRecognizeCommand(
        &self,
        sender: &NSSpeechRecognizer,
        command: &NSString,
    );
}
