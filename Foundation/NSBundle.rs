//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[extern_enum]
#[underlying(c_uint)]
pub enum __anonymous__ {
    NSBundleExecutableArchitectureI386 = 0x00000007,
    NSBundleExecutableArchitecturePPC = 0x00000012,
    NSBundleExecutableArchitectureX86_64 = 0x01000007,
    NSBundleExecutableArchitecturePPC64 = 0x01000012,
    NSBundleExecutableArchitectureARM64 = 0x0100000c,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSBundle")]
    #[derive(PartialEq, Eq, Hash)]
    pub type NSBundle;
}

#[cfg(feature = "Foundation_NSBundle")]
unsafe impl NSObjectProtocol for NSBundle {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSBundle")]
    pub type NSBundle;

    #[objc2::method(sel = "mainBundle", managed = "Other")]
    pub fn mainBundle() -> Id<NSBundle>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "bundleWithPath:", managed = "Other")]
    pub unsafe fn bundleWithPath(path: &NSString) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithPath:", managed = "Init")]
    pub unsafe fn initWithPath(this: Option<Allocated<Self>>, path: &NSString) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "bundleWithURL:", managed = "Other")]
    pub unsafe fn bundleWithURL(url: &NSURL) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "initWithURL:", managed = "Init")]
    pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Option<Id<Self>>;

    #[objc2::method(sel = "bundleForClass:", managed = "Other")]
    pub unsafe fn bundleForClass(a_class: &Class) -> Id<NSBundle>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "bundleWithIdentifier:", managed = "Other")]
    pub unsafe fn bundleWithIdentifier(identifier: &NSString) -> Option<Id<NSBundle>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "allBundles", managed = "Other")]
    pub unsafe fn allBundles() -> Id<NSArray<NSBundle>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "allFrameworks", managed = "Other")]
    pub unsafe fn allFrameworks() -> Id<NSArray<NSBundle>>;

    #[objc2::method(sel = "load")]
    pub unsafe fn load(&self) -> bool;

    #[objc2::method(sel = "isLoaded")]
    pub unsafe fn isLoaded(&self) -> bool;

    #[objc2::method(sel = "unload")]
    pub unsafe fn unload(&self) -> bool;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "preflightAndReturnError:", throws)]
    pub unsafe fn preflightAndReturnError(&self) -> Result<(), Id<NSError>>;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "loadAndReturnError:", throws)]
    pub unsafe fn loadAndReturnError(&self) -> Result<(), Id<NSError>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "bundleURL", managed = "Other")]
    pub unsafe fn bundleURL(&self) -> Id<NSURL>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "resourceURL", managed = "Other")]
    pub unsafe fn resourceURL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "executableURL", managed = "Other")]
    pub unsafe fn executableURL(&self) -> Option<Id<NSURL>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "URLForAuxiliaryExecutable:", managed = "Other")]
    pub unsafe fn URLForAuxiliaryExecutable(&self, executable_name: &NSString)
        -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "privateFrameworksURL", managed = "Other")]
    pub unsafe fn privateFrameworksURL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "sharedFrameworksURL", managed = "Other")]
    pub unsafe fn sharedFrameworksURL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "sharedSupportURL", managed = "Other")]
    pub unsafe fn sharedSupportURL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "builtInPlugInsURL", managed = "Other")]
    pub unsafe fn builtInPlugInsURL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "appStoreReceiptURL", managed = "Other")]
    pub unsafe fn appStoreReceiptURL(&self) -> Option<Id<NSURL>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "bundlePath", managed = "Other")]
    pub unsafe fn bundlePath(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "resourcePath", managed = "Other")]
    pub unsafe fn resourcePath(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "executablePath", managed = "Other")]
    pub unsafe fn executablePath(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathForAuxiliaryExecutable:", managed = "Other")]
    pub unsafe fn pathForAuxiliaryExecutable(
        &self,
        executable_name: &NSString,
    ) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "privateFrameworksPath", managed = "Other")]
    pub unsafe fn privateFrameworksPath(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "sharedFrameworksPath", managed = "Other")]
    pub unsafe fn sharedFrameworksPath(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "sharedSupportPath", managed = "Other")]
    pub unsafe fn sharedSupportPath(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "builtInPlugInsPath", managed = "Other")]
    pub unsafe fn builtInPlugInsPath(&self) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
    #[objc2::method(
        sel = "URLForResource:withExtension:subdirectory:inBundleWithURL:",
        managed = "Other"
    )]
    pub unsafe fn URLForResource_withExtension_subdirectory_inBundleWithURL(
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        bundle_url: &NSURL,
    ) -> Option<Id<NSURL>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "URLsForResourcesWithExtension:subdirectory:inBundleWithURL:",
        managed = "Other"
    )]
    pub unsafe fn URLsForResourcesWithExtension_subdirectory_inBundleWithURL(
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        bundle_url: &NSURL,
    ) -> Option<Id<NSArray<NSURL>>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "URLForResource:withExtension:", managed = "Other")]
    pub unsafe fn URLForResource_withExtension(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
    ) -> Option<Id<NSURL>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "URLForResource:withExtension:subdirectory:", managed = "Other")]
    pub unsafe fn URLForResource_withExtension_subdirectory(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> Option<Id<NSURL>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
    #[objc2::method(
        sel = "URLForResource:withExtension:subdirectory:localization:",
        managed = "Other"
    )]
    pub unsafe fn URLForResource_withExtension_subdirectory_localization(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localization_name: Option<&NSString>,
    ) -> Option<Id<NSURL>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(sel = "URLsForResourcesWithExtension:subdirectory:", managed = "Other")]
    pub unsafe fn URLsForResourcesWithExtension_subdirectory(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> Option<Id<NSArray<NSURL>>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "URLsForResourcesWithExtension:subdirectory:localization:",
        managed = "Other"
    )]
    pub unsafe fn URLsForResourcesWithExtension_subdirectory_localization(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localization_name: Option<&NSString>,
    ) -> Option<Id<NSArray<NSURL>>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathForResource:ofType:inDirectory:", managed = "Other")]
    pub unsafe fn pathForResource_ofType_inDirectory_class(
        name: Option<&NSString>,
        ext: Option<&NSString>,
        bundle_path: &NSString,
    ) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "pathsForResourcesOfType:inDirectory:", managed = "Other")]
    pub unsafe fn pathsForResourcesOfType_inDirectory_class(
        ext: Option<&NSString>,
        bundle_path: &NSString,
    ) -> Id<NSArray<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathForResource:ofType:", managed = "Other")]
    pub unsafe fn pathForResource_ofType(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
    ) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "pathForResource:ofType:inDirectory:", managed = "Other")]
    pub unsafe fn pathForResource_ofType_inDirectory(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(
        sel = "pathForResource:ofType:inDirectory:forLocalization:",
        managed = "Other"
    )]
    pub unsafe fn pathForResource_ofType_inDirectory_forLocalization(
        &self,
        name: Option<&NSString>,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localization_name: Option<&NSString>,
    ) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "pathsForResourcesOfType:inDirectory:", managed = "Other")]
    pub unsafe fn pathsForResourcesOfType_inDirectory(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
    ) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(
        sel = "pathsForResourcesOfType:inDirectory:forLocalization:",
        managed = "Other"
    )]
    pub unsafe fn pathsForResourcesOfType_inDirectory_forLocalization(
        &self,
        ext: Option<&NSString>,
        subpath: Option<&NSString>,
        localization_name: Option<&NSString>,
    ) -> Id<NSArray<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedStringForKey:value:table:", managed = "Other")]
    pub unsafe fn localizedStringForKey_value_table(
        &self,
        key: &NSString,
        value: Option<&NSString>,
        table_name: Option<&NSString>,
    ) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "bundleIdentifier", managed = "Other")]
    pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "infoDictionary", managed = "Other")]
    pub fn infoDictionary(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "localizedInfoDictionary", managed = "Other")]
    pub unsafe fn localizedInfoDictionary(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "objectForInfoDictionaryKey:", managed = "Other")]
    pub unsafe fn objectForInfoDictionaryKey(&self, key: &NSString) -> Option<Id<Object>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "classNamed:")]
    pub unsafe fn classNamed(&self, class_name: &NSString) -> Option<&'static Class>;

    #[objc2::method(sel = "principalClass")]
    pub unsafe fn principalClass(&self) -> Option<&'static Class>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "preferredLocalizations", managed = "Other")]
    pub unsafe fn preferredLocalizations(&self) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "localizations", managed = "Other")]
    pub unsafe fn localizations(&self) -> Id<NSArray<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "developmentLocalization", managed = "Other")]
    pub unsafe fn developmentLocalization(&self) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "preferredLocalizationsFromArray:", managed = "Other")]
    pub unsafe fn preferredLocalizationsFromArray(
        localizations_array: &NSArray<NSString>,
    ) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(
        sel = "preferredLocalizationsFromArray:forPreferences:",
        managed = "Other"
    )]
    pub unsafe fn preferredLocalizationsFromArray_forPreferences(
        localizations_array: &NSArray<NSString>,
        preferences_array: Option<&NSArray<NSString>>,
    ) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
    #[objc2::method(sel = "executableArchitectures", managed = "Other")]
    pub unsafe fn executableArchitectures(&self) -> Option<Id<NSArray<NSNumber>>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSString")]
    pub type NSString;

    #[objc2::method(sel = "variantFittingPresentationWidth:", managed = "Other")]
    pub unsafe fn variantFittingPresentationWidth(&self, width: NSInteger) -> Id<NSString>;
}

extern_static!(NSBundleDidLoadNotification: &'static NSNotificationName);

extern_static!(NSLoadedClasses: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSBundleResourceRequest")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSBundleResourceRequest;
}

#[cfg(feature = "Foundation_NSBundleResourceRequest")]
unsafe impl NSObjectProtocol for NSBundleResourceRequest {}

#[cfg(feature = "Foundation_NSBundleResourceRequest")]
unsafe impl NSProgressReporting for NSBundleResourceRequest {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSBundleResourceRequest")]
    pub type NSBundleResourceRequest;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "initWithTags:", managed = "Init")]
    pub unsafe fn initWithTags(this: Option<Allocated<Self>>, tags: &NSSet<NSString>) -> Id<Self>;

    #[cfg(all(
        feature = "Foundation_NSBundle",
        feature = "Foundation_NSSet",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "initWithTags:bundle:", managed = "Init")]
    pub unsafe fn initWithTags_bundle(
        this: Option<Allocated<Self>>,
        tags: &NSSet<NSString>,
        bundle: &NSBundle,
    ) -> Id<Self>;

    #[objc2::method(sel = "loadingPriority")]
    pub unsafe fn loadingPriority(&self) -> c_double;

    #[objc2::method(sel = "setLoadingPriority:")]
    pub unsafe fn setLoadingPriority(&self, loading_priority: c_double);

    #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "tags", managed = "Other")]
    pub unsafe fn tags(&self) -> Id<NSSet<NSString>>;

    #[cfg(feature = "Foundation_NSBundle")]
    #[objc2::method(sel = "bundle", managed = "Other")]
    pub unsafe fn bundle(&self) -> Id<NSBundle>;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "beginAccessingResourcesWithCompletionHandler:")]
    pub unsafe fn beginAccessingResourcesWithCompletionHandler(
        &self,
        completion_handler: &Block<(*mut NSError,), ()>,
    );

    #[objc2::method(sel = "conditionallyBeginAccessingResourcesWithCompletionHandler:")]
    pub unsafe fn conditionallyBeginAccessingResourcesWithCompletionHandler(
        &self,
        completion_handler: &Block<(Bool,), ()>,
    );

    #[objc2::method(sel = "endAccessingResources")]
    pub unsafe fn endAccessingResources(&self);

    #[cfg(feature = "Foundation_NSProgress")]
    #[objc2::method(sel = "progress", managed = "Other")]
    pub unsafe fn progress(&self) -> Id<NSProgress>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSBundle")]
    pub type NSBundle;

    #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setPreservationPriority:forTags:")]
    pub unsafe fn setPreservationPriority_forTags(
        &self,
        priority: c_double,
        tags: &NSSet<NSString>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "preservationPriorityForTag:")]
    pub unsafe fn preservationPriorityForTag(&self, tag: &NSString) -> c_double;
}

extern_static!(NSBundleResourceRequestLowDiskSpaceNotification: &'static NSNotificationName);

extern_static!(NSBundleResourceRequestLoadingPriorityUrgent: c_double);
