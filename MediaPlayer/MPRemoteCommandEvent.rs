//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPRemoteCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
unsafe impl NSObjectProtocol for MPRemoteCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
    pub type MPRemoteCommandEvent;

    #[cfg(feature = "MediaPlayer_MPRemoteCommand")]
    #[objc2::method(sel = "command", managed = "Other")]
    pub unsafe fn command(&self) -> Id<MPRemoteCommand>;

    #[objc2::method(sel = "timestamp")]
    pub unsafe fn timestamp(&self) -> NSTimeInterval;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPSkipIntervalCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
unsafe impl NSObjectProtocol for MPSkipIntervalCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPSkipIntervalCommandEvent")]
    pub type MPSkipIntervalCommandEvent;

    #[objc2::method(sel = "interval")]
    pub unsafe fn interval(&self) -> NSTimeInterval;
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum MPSeekCommandEventType {
    MPSeekCommandEventTypeBeginSeeking = 0,
    MPSeekCommandEventTypeEndSeeking = 1,
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPSeekCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
unsafe impl NSObjectProtocol for MPSeekCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPSeekCommandEvent")]
    pub type MPSeekCommandEvent;

    #[objc2::method(sel = "type")]
    pub unsafe fn r#type(&self) -> MPSeekCommandEventType;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPRatingCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
unsafe impl NSObjectProtocol for MPRatingCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPRatingCommandEvent")]
    pub type MPRatingCommandEvent;

    #[objc2::method(sel = "rating")]
    pub unsafe fn rating(&self) -> c_float;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPChangePlaybackRateCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
unsafe impl NSObjectProtocol for MPChangePlaybackRateCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangePlaybackRateCommandEvent")]
    pub type MPChangePlaybackRateCommandEvent;

    #[objc2::method(sel = "playbackRate")]
    pub unsafe fn playbackRate(&self) -> c_float;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPFeedbackCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
unsafe impl NSObjectProtocol for MPFeedbackCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPFeedbackCommandEvent")]
    pub type MPFeedbackCommandEvent;

    #[objc2::method(sel = "isNegative")]
    pub unsafe fn isNegative(&self) -> bool;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPChangeLanguageOptionCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
unsafe impl NSObjectProtocol for MPChangeLanguageOptionCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangeLanguageOptionCommandEvent")]
    pub type MPChangeLanguageOptionCommandEvent;

    #[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
    #[objc2::method(sel = "languageOption", managed = "Other")]
    pub unsafe fn languageOption(&self) -> Id<MPNowPlayingInfoLanguageOption>;

    #[objc2::method(sel = "setting")]
    pub unsafe fn setting(&self) -> MPChangeLanguageOptionSetting;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPChangePlaybackPositionCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
unsafe impl NSObjectProtocol for MPChangePlaybackPositionCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangePlaybackPositionCommandEvent")]
    pub type MPChangePlaybackPositionCommandEvent;

    #[objc2::method(sel = "positionTime")]
    pub unsafe fn positionTime(&self) -> NSTimeInterval;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPChangeShuffleModeCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
unsafe impl NSObjectProtocol for MPChangeShuffleModeCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangeShuffleModeCommandEvent")]
    pub type MPChangeShuffleModeCommandEvent;

    #[objc2::method(sel = "shuffleType")]
    pub unsafe fn shuffleType(&self) -> MPShuffleType;

    #[objc2::method(sel = "preservesShuffleMode")]
    pub unsafe fn preservesShuffleMode(&self) -> bool;
}

#[objc2::interface(
    unsafe super = MPRemoteCommandEvent,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPChangeRepeatModeCommandEvent;
}

#[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
unsafe impl NSObjectProtocol for MPChangeRepeatModeCommandEvent {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPChangeRepeatModeCommandEvent")]
    pub type MPChangeRepeatModeCommandEvent;

    #[objc2::method(sel = "repeatType")]
    pub unsafe fn repeatType(&self) -> MPRepeatType;

    #[objc2::method(sel = "preservesRepeatMode")]
    pub unsafe fn preservesRepeatMode(&self) -> bool;
}
