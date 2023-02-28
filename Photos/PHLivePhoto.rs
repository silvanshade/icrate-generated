//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

pub type PHLivePhotoRequestID = i32;

extern_static!(PHLivePhotoRequestIDInvalid: PHLivePhotoRequestID = 0);

extern_static!(PHLivePhotoInfoErrorKey: &'static NSString);

extern_static!(PHLivePhotoInfoIsDegradedKey: &'static NSString);

extern_static!(PHLivePhotoInfoCancelledKey: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "PhotoKit_PHLivePhoto")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type PHLivePhoto;
}

#[cfg(feature = "PhotoKit_PHLivePhoto")]
unsafe impl NSCoding for PHLivePhoto {}

#[cfg(feature = "PhotoKit_PHLivePhoto")]
unsafe impl NSObjectProtocol for PHLivePhoto {}

#[cfg(feature = "PhotoKit_PHLivePhoto")]
unsafe impl NSSecureCoding for PHLivePhoto {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "PhotoKit_PHLivePhoto")]
    pub type PHLivePhoto;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[objc2::method(sel = "size")]
    pub unsafe fn size(&self) -> CGSize;

    #[cfg(all(
        feature = "AppKit_NSImage",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSURL"
    ))]
    #[objc2::method(
        sel = "requestLivePhotoWithResourceFileURLs:placeholderImage:targetSize:contentMode:resultHandler:"
    )]
    pub unsafe fn requestLivePhotoWithResourceFileURLs_placeholderImage_targetSize_contentMode_resultHandler(
        file_ur_ls: &NSArray<NSURL>,
        image: Option<&NSImage>,
        target_size: CGSize,
        content_mode: PHImageContentMode,
        result_handler: &Block<(*mut PHLivePhoto, NonNull<NSDictionary>), ()>,
    ) -> PHLivePhotoRequestID;

    #[objc2::method(sel = "cancelLivePhotoRequestWithRequestID:")]
    pub unsafe fn cancelLivePhotoRequestWithRequestID(request_id: PHLivePhotoRequestID);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "PhotoKit_PHLivePhoto")]
    pub type PHLivePhoto;
}

#[cfg(feature = "PhotoKit_PHLivePhoto")]
unsafe impl NSItemProviderReading for PHLivePhoto {}
