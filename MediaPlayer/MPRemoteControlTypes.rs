//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum MPShuffleType {
    MPShuffleTypeOff = 0,
    MPShuffleTypeItems = 1,
    MPShuffleTypeCollections = 2,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum MPRepeatType {
    MPRepeatTypeOff = 0,
    MPRepeatTypeOne = 1,
    MPRepeatTypeAll = 2,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum MPChangeLanguageOptionSetting {
    MPChangeLanguageOptionSettingNone = 0,
    MPChangeLanguageOptionSettingNowPlayingItemOnly = 1,
    MPChangeLanguageOptionSettingPermanent = 2,
}
