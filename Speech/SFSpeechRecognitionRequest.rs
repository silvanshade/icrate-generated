//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type SFSpeechRecognitionRequest;
}

#[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
unsafe impl NSObjectProtocol for SFSpeechRecognitionRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
    pub type SFSpeechRecognitionRequest;

    #[objc2::method(sel = "taskHint")]
    pub unsafe fn taskHint(&self) -> SFSpeechRecognitionTaskHint;

    #[objc2::method(sel = "setTaskHint:")]
    pub unsafe fn setTaskHint(&self, task_hint: SFSpeechRecognitionTaskHint);

    #[objc2::method(sel = "shouldReportPartialResults")]
    pub unsafe fn shouldReportPartialResults(&self) -> bool;

    #[objc2::method(sel = "setShouldReportPartialResults:")]
    pub unsafe fn setShouldReportPartialResults(&self, should_report_partial_results: bool);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "contextualStrings", managed = "Other")]
    pub unsafe fn contextualStrings(&self) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setContextualStrings:")]
    pub unsafe fn setContextualStrings(&self, contextual_strings: &NSArray<NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated = "Not used anymore"]
    #[objc2::method(sel = "interactionIdentifier", managed = "Other")]
    pub unsafe fn interactionIdentifier(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[deprecated = "Not used anymore"]
    #[objc2::method(sel = "setInteractionIdentifier:")]
    pub unsafe fn setInteractionIdentifier(&self, interaction_identifier: Option<&NSString>);

    #[objc2::method(sel = "requiresOnDeviceRecognition")]
    pub unsafe fn requiresOnDeviceRecognition(&self) -> bool;

    #[objc2::method(sel = "setRequiresOnDeviceRecognition:")]
    pub unsafe fn setRequiresOnDeviceRecognition(&self, requires_on_device_recognition: bool);

    #[objc2::method(sel = "addsPunctuation")]
    pub unsafe fn addsPunctuation(&self) -> bool;

    #[objc2::method(sel = "setAddsPunctuation:")]
    pub unsafe fn setAddsPunctuation(&self, adds_punctuation: bool);
}

#[objc2::interface(
    unsafe super = SFSpeechRecognitionRequest,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type SFSpeechURLRecognitionRequest;
}

#[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
unsafe impl NSObjectProtocol for SFSpeechURLRecognitionRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
    pub type SFSpeechURLRecognitionRequest;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "initWithURL:", managed = "Init")]
    pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Id<Self>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "URL", managed = "Other")]
    pub unsafe fn URL(&self) -> Id<NSURL>;
}

#[objc2::interface(
    unsafe super = SFSpeechRecognitionRequest,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type SFSpeechAudioBufferRecognitionRequest;
}

#[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
unsafe impl NSObjectProtocol for SFSpeechAudioBufferRecognitionRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
    pub type SFSpeechAudioBufferRecognitionRequest;

    #[cfg(feature = "AVFAudio_AVAudioFormat")]
    #[objc2::method(sel = "nativeAudioFormat", managed = "Other")]
    pub unsafe fn nativeAudioFormat(&self) -> Id<AVAudioFormat>;

    #[cfg(feature = "AVFAudio_AVAudioPCMBuffer")]
    #[objc2::method(sel = "appendAudioPCMBuffer:")]
    pub unsafe fn appendAudioPCMBuffer(&self, audio_pcm_buffer: &AVAudioPCMBuffer);

    #[objc2::method(sel = "endAudio")]
    pub unsafe fn endAudio(&self);
}
