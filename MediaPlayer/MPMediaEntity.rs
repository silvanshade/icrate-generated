//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

pub type MPMediaEntityPersistentID = u64;

extern_static!(MPMediaEntityPropertyPersistentID: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPMediaEntity")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MPMediaEntity;
}

#[cfg(feature = "MediaPlayer_MPMediaEntity")]
unsafe impl NSCoding for MPMediaEntity {}

#[cfg(feature = "MediaPlayer_MPMediaEntity")]
unsafe impl NSObjectProtocol for MPMediaEntity {}

#[cfg(feature = "MediaPlayer_MPMediaEntity")]
unsafe impl NSSecureCoding for MPMediaEntity {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MediaPlayer_MPMediaEntity")]
    pub type MPMediaEntity;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "canFilterByProperty:")]
    pub unsafe fn canFilterByProperty(property: &NSString) -> bool;

    #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "enumerateValuesForProperties:usingBlock:")]
    pub unsafe fn enumerateValuesForProperties_usingBlock(
        &self,
        properties: &NSSet<NSString>,
        block: &Block<(NonNull<NSString>, NonNull<Object>, NonNull<Bool>), ()>,
    );

    #[objc2::method(sel = "objectForKeyedSubscript:", managed = "Other")]
    pub unsafe fn objectForKeyedSubscript(&self, key: &Object) -> Option<Id<Object>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "valueForProperty:", managed = "Other")]
    pub unsafe fn valueForProperty(&self, property: &NSString) -> Option<Id<Object>>;

    #[objc2::method(sel = "persistentID")]
    pub unsafe fn persistentID(&self) -> MPMediaEntityPersistentID;
}
