//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AVFoundation_AVMediaSelectionOption")]
    pub type AVMediaSelectionOption;

    #[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
    #[objc2::method(sel = "makeNowPlayingInfoLanguageOption", managed = "Other")]
    pub unsafe fn makeNowPlayingInfoLanguageOption(
        &self,
    ) -> Option<Id<MPNowPlayingInfoLanguageOption>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AVFoundation_AVMediaSelectionGroup")]
    pub type AVMediaSelectionGroup;

    #[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOptionGroup")]
    #[objc2::method(sel = "makeNowPlayingInfoLanguageOptionGroup", managed = "Other")]
    pub unsafe fn makeNowPlayingInfoLanguageOptionGroup(
        &self,
    ) -> Id<MPNowPlayingInfoLanguageOptionGroup>;
}
