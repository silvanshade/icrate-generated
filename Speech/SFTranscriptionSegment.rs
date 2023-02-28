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
    #[cfg(feature = "Speech_SFTranscriptionSegment")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type SFTranscriptionSegment;
}

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSCoding for SFTranscriptionSegment {}

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSObjectProtocol for SFTranscriptionSegment {}

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSSecureCoding for SFTranscriptionSegment {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Speech_SFTranscriptionSegment")]
    pub type SFTranscriptionSegment;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "substring", managed = "Other")]
    pub unsafe fn substring(&self) -> Id<NSString>;

    #[objc2::method(sel = "substringRange")]
    pub unsafe fn substringRange(&self) -> NSRange;

    #[objc2::method(sel = "timestamp")]
    pub unsafe fn timestamp(&self) -> NSTimeInterval;

    #[objc2::method(sel = "duration")]
    pub unsafe fn duration(&self) -> NSTimeInterval;

    #[objc2::method(sel = "confidence")]
    pub unsafe fn confidence(&self) -> c_float;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "alternativeSubstrings", managed = "Other")]
    pub unsafe fn alternativeSubstrings(&self) -> Id<NSArray<NSString>>;

    #[cfg(feature = "Speech_SFVoiceAnalytics")]
    #[deprecated = "voiceAnalytics is moved to SFSpeechRecognitionMetadata"]
    #[objc2::method(sel = "voiceAnalytics", managed = "Other")]
    pub unsafe fn voiceAnalytics(&self) -> Option<Id<SFVoiceAnalytics>>;
}
