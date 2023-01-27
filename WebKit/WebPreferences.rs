//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
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
);

extern_static!(WebPreferencesChangedNotification: Option<&'static NSString>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebPreferences")]
    #[deprecated]
    pub struct WebPreferences;

    #[cfg(feature = "WebKit_WebPreferences")]
    unsafe impl ClassType for WebPreferences {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WebPreferences")]
unsafe impl NSCoding for WebPreferences {}

#[cfg(feature = "WebKit_WebPreferences")]
unsafe impl NSObjectProtocol for WebPreferences {}

extern_methods!(
    #[cfg(feature = "WebKit_WebPreferences")]
    unsafe impl WebPreferences {
        #[method_id(@__retain_semantics Other standardPreferences)]
        pub unsafe fn standardPreferences() -> Option<Id<WebPreferences, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            an_identifier: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other standardFontFamily)]
        pub unsafe fn standardFontFamily(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStandardFontFamily:)]
        pub unsafe fn setStandardFontFamily(&self, standard_font_family: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fixedFontFamily)]
        pub unsafe fn fixedFontFamily(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFixedFontFamily:)]
        pub unsafe fn setFixedFontFamily(&self, fixed_font_family: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serifFontFamily)]
        pub unsafe fn serifFontFamily(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSerifFontFamily:)]
        pub unsafe fn setSerifFontFamily(&self, serif_font_family: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sansSerifFontFamily)]
        pub unsafe fn sansSerifFontFamily(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSansSerifFontFamily:)]
        pub unsafe fn setSansSerifFontFamily(&self, sans_serif_font_family: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cursiveFontFamily)]
        pub unsafe fn cursiveFontFamily(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCursiveFontFamily:)]
        pub unsafe fn setCursiveFontFamily(&self, cursive_font_family: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fantasyFontFamily)]
        pub unsafe fn fantasyFontFamily(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFantasyFontFamily:)]
        pub unsafe fn setFantasyFontFamily(&self, fantasy_font_family: Option<&NSString>);

        #[method(defaultFontSize)]
        pub unsafe fn defaultFontSize(&self) -> c_int;

        #[method(setDefaultFontSize:)]
        pub unsafe fn setDefaultFontSize(&self, default_font_size: c_int);

        #[method(defaultFixedFontSize)]
        pub unsafe fn defaultFixedFontSize(&self) -> c_int;

        #[method(setDefaultFixedFontSize:)]
        pub unsafe fn setDefaultFixedFontSize(&self, default_fixed_font_size: c_int);

        #[method(minimumFontSize)]
        pub unsafe fn minimumFontSize(&self) -> c_int;

        #[method(setMinimumFontSize:)]
        pub unsafe fn setMinimumFontSize(&self, minimum_font_size: c_int);

        #[method(minimumLogicalFontSize)]
        pub unsafe fn minimumLogicalFontSize(&self) -> c_int;

        #[method(setMinimumLogicalFontSize:)]
        pub unsafe fn setMinimumLogicalFontSize(&self, minimum_logical_font_size: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultTextEncodingName)]
        pub unsafe fn defaultTextEncodingName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDefaultTextEncodingName:)]
        pub unsafe fn setDefaultTextEncodingName(
            &self,
            default_text_encoding_name: Option<&NSString>,
        );

        #[method(userStyleSheetEnabled)]
        pub unsafe fn userStyleSheetEnabled(&self) -> bool;

        #[method(setUserStyleSheetEnabled:)]
        pub unsafe fn setUserStyleSheetEnabled(&self, user_style_sheet_enabled: bool);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other userStyleSheetLocation)]
        pub unsafe fn userStyleSheetLocation(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setUserStyleSheetLocation:)]
        pub unsafe fn setUserStyleSheetLocation(&self, user_style_sheet_location: Option<&NSURL>);

        #[deprecated]
        #[method(isJavaEnabled)]
        pub unsafe fn isJavaEnabled(&self) -> bool;

        #[deprecated]
        #[method(setJavaEnabled:)]
        pub unsafe fn setJavaEnabled(&self, java_enabled: bool);

        #[method(isJavaScriptEnabled)]
        pub unsafe fn isJavaScriptEnabled(&self) -> bool;

        #[method(setJavaScriptEnabled:)]
        pub unsafe fn setJavaScriptEnabled(&self, java_script_enabled: bool);

        #[method(javaScriptCanOpenWindowsAutomatically)]
        pub unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> bool;

        #[method(setJavaScriptCanOpenWindowsAutomatically:)]
        pub unsafe fn setJavaScriptCanOpenWindowsAutomatically(
            &self,
            java_script_can_open_windows_automatically: bool,
        );

        #[method(arePlugInsEnabled)]
        pub unsafe fn arePlugInsEnabled(&self) -> bool;

        #[method(setPlugInsEnabled:)]
        pub unsafe fn setPlugInsEnabled(&self, plug_ins_enabled: bool);

        #[method(allowsAnimatedImages)]
        pub unsafe fn allowsAnimatedImages(&self) -> bool;

        #[method(setAllowsAnimatedImages:)]
        pub unsafe fn setAllowsAnimatedImages(&self, allows_animated_images: bool);

        #[method(allowsAnimatedImageLooping)]
        pub unsafe fn allowsAnimatedImageLooping(&self) -> bool;

        #[method(setAllowsAnimatedImageLooping:)]
        pub unsafe fn setAllowsAnimatedImageLooping(&self, allows_animated_image_looping: bool);

        #[method(loadsImagesAutomatically)]
        pub unsafe fn loadsImagesAutomatically(&self) -> bool;

        #[method(setLoadsImagesAutomatically:)]
        pub unsafe fn setLoadsImagesAutomatically(&self, loads_images_automatically: bool);

        #[method(autosaves)]
        pub unsafe fn autosaves(&self) -> bool;

        #[method(setAutosaves:)]
        pub unsafe fn setAutosaves(&self, autosaves: bool);

        #[method(shouldPrintBackgrounds)]
        pub unsafe fn shouldPrintBackgrounds(&self) -> bool;

        #[method(setShouldPrintBackgrounds:)]
        pub unsafe fn setShouldPrintBackgrounds(&self, should_print_backgrounds: bool);

        #[method(privateBrowsingEnabled)]
        pub unsafe fn privateBrowsingEnabled(&self) -> bool;

        #[method(setPrivateBrowsingEnabled:)]
        pub unsafe fn setPrivateBrowsingEnabled(&self, private_browsing_enabled: bool);

        #[method(tabsToLinks)]
        pub unsafe fn tabsToLinks(&self) -> bool;

        #[method(setTabsToLinks:)]
        pub unsafe fn setTabsToLinks(&self, tabs_to_links: bool);

        #[method(usesPageCache)]
        pub unsafe fn usesPageCache(&self) -> bool;

        #[method(setUsesPageCache:)]
        pub unsafe fn setUsesPageCache(&self, uses_page_cache: bool);

        #[method(cacheModel)]
        pub unsafe fn cacheModel(&self) -> WebCacheModel;

        #[method(setCacheModel:)]
        pub unsafe fn setCacheModel(&self, cache_model: WebCacheModel);

        #[method(suppressesIncrementalRendering)]
        pub unsafe fn suppressesIncrementalRendering(&self) -> bool;

        #[method(setSuppressesIncrementalRendering:)]
        pub unsafe fn setSuppressesIncrementalRendering(
            &self,
            suppresses_incremental_rendering: bool,
        );

        #[method(allowsAirPlayForMediaPlayback)]
        pub unsafe fn allowsAirPlayForMediaPlayback(&self) -> bool;

        #[method(setAllowsAirPlayForMediaPlayback:)]
        pub unsafe fn setAllowsAirPlayForMediaPlayback(
            &self,
            allows_air_play_for_media_playback: bool,
        );
    }
);
