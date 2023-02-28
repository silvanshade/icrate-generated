//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSFileVersionAddingOptions {
    NSFileVersionAddingByMoving = 1 << 0,
}

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSFileVersionReplacingOptions {
    NSFileVersionReplacingByMoving = 1 << 0,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSFileVersion")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSFileVersion;
}

#[cfg(feature = "Foundation_NSFileVersion")]
unsafe impl NSObjectProtocol for NSFileVersion {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSFileVersion")]
    pub type NSFileVersion;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "currentVersionOfItemAtURL:", managed = "Other")]
    pub unsafe fn currentVersionOfItemAtURL(url: &NSURL) -> Option<Id<NSFileVersion>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "otherVersionsOfItemAtURL:", managed = "Other")]
    pub unsafe fn otherVersionsOfItemAtURL(url: &NSURL) -> Option<Id<NSArray<NSFileVersion>>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "unresolvedConflictVersionsOfItemAtURL:", managed = "Other")]
    pub unsafe fn unresolvedConflictVersionsOfItemAtURL(
        url: &NSURL,
    ) -> Option<Id<NSArray<NSFileVersion>>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(sel = "getNonlocalVersionsOfItemAtURL:completionHandler:")]
    pub unsafe fn getNonlocalVersionsOfItemAtURL_completionHandler(
        url: &NSURL,
        completion_handler: &Block<(*mut NSArray<NSFileVersion>, *mut NSError), ()>,
    );

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "versionOfItemAtURL:forPersistentIdentifier:", managed = "Other")]
    pub unsafe fn versionOfItemAtURL_forPersistentIdentifier(
        url: &NSURL,
        persistent_identifier: &Object,
    ) -> Option<Id<NSFileVersion>>;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(
        sel = "addVersionOfItemAtURL:withContentsOfURL:options:error:",
        managed = "Other",
        throws
    )]
    pub unsafe fn addVersionOfItemAtURL_withContentsOfURL_options_error(
        url: &NSURL,
        contents_url: &NSURL,
        options: NSFileVersionAddingOptions,
    ) -> Result<Id<NSFileVersion>, Id<NSError>>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(
        sel = "temporaryDirectoryURLForNewVersionOfItemAtURL:",
        managed = "Other"
    )]
    pub unsafe fn temporaryDirectoryURLForNewVersionOfItemAtURL(url: &NSURL) -> Id<NSURL>;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "URL", managed = "Other")]
    pub unsafe fn URL(&self) -> Id<NSURL>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedName", managed = "Other")]
    pub unsafe fn localizedName(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedNameOfSavingComputer", managed = "Other")]
    pub unsafe fn localizedNameOfSavingComputer(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSPersonNameComponents")]
    #[objc2::method(sel = "originatorNameComponents", managed = "Other")]
    pub unsafe fn originatorNameComponents(&self) -> Option<Id<NSPersonNameComponents>>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "modificationDate", managed = "Other")]
    pub unsafe fn modificationDate(&self) -> Option<Id<NSDate>>;

    #[objc2::method(sel = "persistentIdentifier", managed = "Other")]
    pub unsafe fn persistentIdentifier(&self) -> Id<ProtocolObject<dyn NSCoding>>;

    #[objc2::method(sel = "isConflict")]
    pub unsafe fn isConflict(&self) -> bool;

    #[objc2::method(sel = "isResolved")]
    pub unsafe fn isResolved(&self) -> bool;

    #[objc2::method(sel = "setResolved:")]
    pub unsafe fn setResolved(&self, resolved: bool);

    #[objc2::method(sel = "isDiscardable")]
    pub unsafe fn isDiscardable(&self) -> bool;

    #[objc2::method(sel = "setDiscardable:")]
    pub unsafe fn setDiscardable(&self, discardable: bool);

    #[objc2::method(sel = "hasLocalContents")]
    pub unsafe fn hasLocalContents(&self) -> bool;

    #[objc2::method(sel = "hasThumbnail")]
    pub unsafe fn hasThumbnail(&self) -> bool;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "replaceItemAtURL:options:error:", managed = "Other", throws)]
    pub unsafe fn replaceItemAtURL_options_error(
        &self,
        url: &NSURL,
        options: NSFileVersionReplacingOptions,
    ) -> Result<Id<NSURL>, Id<NSError>>;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "removeAndReturnError:", throws)]
    pub unsafe fn removeAndReturnError(&self) -> Result<(), Id<NSError>>;

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
    #[objc2::method(sel = "removeOtherVersionsOfItemAtURL:error:", throws)]
    pub unsafe fn removeOtherVersionsOfItemAtURL_error(url: &NSURL) -> Result<(), Id<NSError>>;
}
