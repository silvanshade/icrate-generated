//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLookAroundSnapshotter")]
    pub struct MKLookAroundSnapshotter;

    #[cfg(feature = "MapKit_MKLookAroundSnapshotter")]
    unsafe impl ClassType for MKLookAroundSnapshotter {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKLookAroundSnapshotter")]
unsafe impl NSObjectProtocol for MKLookAroundSnapshotter {}

extern_methods!(
    #[cfg(feature = "MapKit_MKLookAroundSnapshotter")]
    unsafe impl MKLookAroundSnapshotter {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "MapKit_MKLookAroundScene",
            feature = "MapKit_MKLookAroundSnapshotOptions"
        ))]
        #[method_id(@__retain_semantics Init initWithScene:options:)]
        pub unsafe fn initWithScene_options(
            this: Option<Allocated<Self>>,
            scene: &MKLookAroundScene,
            options: &MKLookAroundSnapshotOptions,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "MapKit_MKLookAroundSnapshot"
        ))]
        #[method(getSnapshotWithCompletionHandler:)]
        pub unsafe fn getSnapshotWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut MKLookAroundSnapshot, *mut NSError), ()>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;
    }
);
