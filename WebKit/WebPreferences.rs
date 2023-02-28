//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[ns_enum]
#[underlying(NSUInteger)]
#[deprecated]
pub enum WebCacheModel {
    #[deprecated]
    WebCacheModelDocumentViewer = 0,
    #[deprecated]
    WebCacheModelDocumentBrowser = 1,
    #[deprecated]
    WebCacheModelPrimaryWebBrowser = 2,
}

extern_static!(WebPreferencesChangedNotification: Option<&'static NSString>);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[deprecated]
    #[cfg(feature = "WebKit_WebPreferences")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type WebPreferences;
}

#[cfg(feature = "WebKit_WebPreferences")]
unsafe impl NSCoding for WebPreferences {}

#[cfg(feature = "WebKit_WebPreferences")]
unsafe impl NSObjectProtocol for WebPreferences {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_WebPreferences")]
    #[deprecated]
    pub type WebPreferences;

    #[objc2::method(sel = "standardPreferences", managed = "Other")]
    pub unsafe fn standardPreferences() -> Option<Id<WebPreferences>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithIdentifier:", managed = "Init")]
    pub unsafe fn initWithIdentifier(
        this: Option<Allocated<Self>>,
        an_identifier: Option<&NSString>,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "standardFontFamily", managed = "Other")]
    pub unsafe fn standardFontFamily(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setStandardFontFamily:")]
    pub unsafe fn setStandardFontFamily(&self, standard_font_family: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "fixedFontFamily", managed = "Other")]
    pub unsafe fn fixedFontFamily(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setFixedFontFamily:")]
    pub unsafe fn setFixedFontFamily(&self, fixed_font_family: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "serifFontFamily", managed = "Other")]
    pub unsafe fn serifFontFamily(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setSerifFontFamily:")]
    pub unsafe fn setSerifFontFamily(&self, serif_font_family: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "sansSerifFontFamily", managed = "Other")]
    pub unsafe fn sansSerifFontFamily(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setSansSerifFontFamily:")]
    pub unsafe fn setSansSerifFontFamily(&self, sans_serif_font_family: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "cursiveFontFamily", managed = "Other")]
    pub unsafe fn cursiveFontFamily(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCursiveFontFamily:")]
    pub unsafe fn setCursiveFontFamily(&self, cursive_font_family: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "fantasyFontFamily", managed = "Other")]
    pub unsafe fn fantasyFontFamily(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setFantasyFontFamily:")]
    pub unsafe fn setFantasyFontFamily(&self, fantasy_font_family: Option<&NSString>);

    #[objc2::method(sel = "defaultFontSize")]
    pub unsafe fn defaultFontSize(&self) -> c_int;

    #[objc2::method(sel = "setDefaultFontSize:")]
    pub unsafe fn setDefaultFontSize(&self, default_font_size: c_int);

    #[objc2::method(sel = "defaultFixedFontSize")]
    pub unsafe fn defaultFixedFontSize(&self) -> c_int;

    #[objc2::method(sel = "setDefaultFixedFontSize:")]
    pub unsafe fn setDefaultFixedFontSize(&self, default_fixed_font_size: c_int);

    #[objc2::method(sel = "minimumFontSize")]
    pub unsafe fn minimumFontSize(&self) -> c_int;

    #[objc2::method(sel = "setMinimumFontSize:")]
    pub unsafe fn setMinimumFontSize(&self, minimum_font_size: c_int);

    #[objc2::method(sel = "minimumLogicalFontSize")]
    pub unsafe fn minimumLogicalFontSize(&self) -> c_int;

    #[objc2::method(sel = "setMinimumLogicalFontSize:")]
    pub unsafe fn setMinimumLogicalFontSize(&self, minimum_logical_font_size: c_int);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "defaultTextEncodingName", managed = "Other")]
    pub unsafe fn defaultTextEncodingName(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setDefaultTextEncodingName:")]
    pub unsafe fn setDefaultTextEncodingName(&self, default_text_encoding_name: Option<&NSString>);

    #[objc2::method(sel = "userStyleSheetEnabled")]
    pub unsafe fn userStyleSheetEnabled(&self) -> bool;

    #[objc2::method(sel = "setUserStyleSheetEnabled:")]
    pub unsafe fn setUserStyleSheetEnabled(&self, user_style_sheet_enabled: bool);

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "userStyleSheetLocation", managed = "Other")]
    pub unsafe fn userStyleSheetLocation(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "setUserStyleSheetLocation:")]
    pub unsafe fn setUserStyleSheetLocation(&self, user_style_sheet_location: Option<&NSURL>);

    #[deprecated]
    #[objc2::method(sel = "isJavaEnabled")]
    pub unsafe fn isJavaEnabled(&self) -> bool;

    #[deprecated]
    #[objc2::method(sel = "setJavaEnabled:")]
    pub unsafe fn setJavaEnabled(&self, java_enabled: bool);

    #[objc2::method(sel = "isJavaScriptEnabled")]
    pub unsafe fn isJavaScriptEnabled(&self) -> bool;

    #[objc2::method(sel = "setJavaScriptEnabled:")]
    pub unsafe fn setJavaScriptEnabled(&self, java_script_enabled: bool);

    #[objc2::method(sel = "javaScriptCanOpenWindowsAutomatically")]
    pub unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> bool;

    #[objc2::method(sel = "setJavaScriptCanOpenWindowsAutomatically:")]
    pub unsafe fn setJavaScriptCanOpenWindowsAutomatically(
        &self,
        java_script_can_open_windows_automatically: bool,
    );

    #[objc2::method(sel = "arePlugInsEnabled")]
    pub unsafe fn arePlugInsEnabled(&self) -> bool;

    #[objc2::method(sel = "setPlugInsEnabled:")]
    pub unsafe fn setPlugInsEnabled(&self, plug_ins_enabled: bool);

    #[objc2::method(sel = "allowsAnimatedImages")]
    pub unsafe fn allowsAnimatedImages(&self) -> bool;

    #[objc2::method(sel = "setAllowsAnimatedImages:")]
    pub unsafe fn setAllowsAnimatedImages(&self, allows_animated_images: bool);

    #[objc2::method(sel = "allowsAnimatedImageLooping")]
    pub unsafe fn allowsAnimatedImageLooping(&self) -> bool;

    #[objc2::method(sel = "setAllowsAnimatedImageLooping:")]
    pub unsafe fn setAllowsAnimatedImageLooping(&self, allows_animated_image_looping: bool);

    #[objc2::method(sel = "loadsImagesAutomatically")]
    pub unsafe fn loadsImagesAutomatically(&self) -> bool;

    #[objc2::method(sel = "setLoadsImagesAutomatically:")]
    pub unsafe fn setLoadsImagesAutomatically(&self, loads_images_automatically: bool);

    #[objc2::method(sel = "autosaves")]
    pub unsafe fn autosaves(&self) -> bool;

    #[objc2::method(sel = "setAutosaves:")]
    pub unsafe fn setAutosaves(&self, autosaves: bool);

    #[objc2::method(sel = "shouldPrintBackgrounds")]
    pub unsafe fn shouldPrintBackgrounds(&self) -> bool;

    #[objc2::method(sel = "setShouldPrintBackgrounds:")]
    pub unsafe fn setShouldPrintBackgrounds(&self, should_print_backgrounds: bool);

    #[objc2::method(sel = "privateBrowsingEnabled")]
    pub unsafe fn privateBrowsingEnabled(&self) -> bool;

    #[objc2::method(sel = "setPrivateBrowsingEnabled:")]
    pub unsafe fn setPrivateBrowsingEnabled(&self, private_browsing_enabled: bool);

    #[objc2::method(sel = "tabsToLinks")]
    pub unsafe fn tabsToLinks(&self) -> bool;

    #[objc2::method(sel = "setTabsToLinks:")]
    pub unsafe fn setTabsToLinks(&self, tabs_to_links: bool);

    #[objc2::method(sel = "usesPageCache")]
    pub unsafe fn usesPageCache(&self) -> bool;

    #[objc2::method(sel = "setUsesPageCache:")]
    pub unsafe fn setUsesPageCache(&self, uses_page_cache: bool);

    #[objc2::method(sel = "cacheModel")]
    pub unsafe fn cacheModel(&self) -> WebCacheModel;

    #[objc2::method(sel = "setCacheModel:")]
    pub unsafe fn setCacheModel(&self, cache_model: WebCacheModel);

    #[objc2::method(sel = "suppressesIncrementalRendering")]
    pub unsafe fn suppressesIncrementalRendering(&self) -> bool;

    #[objc2::method(sel = "setSuppressesIncrementalRendering:")]
    pub unsafe fn setSuppressesIncrementalRendering(&self, suppresses_incremental_rendering: bool);

    #[objc2::method(sel = "allowsAirPlayForMediaPlayback")]
    pub unsafe fn allowsAirPlayForMediaPlayback(&self) -> bool;

    #[objc2::method(sel = "setAllowsAirPlayForMediaPlayback:")]
    pub unsafe fn setAllowsAirPlayForMediaPlayback(&self, allows_air_play_for_media_playback: bool);
}
