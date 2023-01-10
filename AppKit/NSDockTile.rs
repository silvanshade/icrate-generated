//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_static!(NSAppKitVersionNumberWithDockTilePlugInSupport: AppKit::NSAppKitVersion = 1001.0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDockTile;

    unsafe impl ClassType for NSDockTile {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDockTile")]
    unsafe impl NSDockTile {
        #[method(size)]
        pub unsafe fn size(&self) -> Foundation::NSSize;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&AppKit::NSView>);

        #[method(display)]
        pub unsafe fn display(&self);

        #[method(showsApplicationBadge)]
        pub unsafe fn showsApplicationBadge(&self) -> bool;

        #[method(setShowsApplicationBadge:)]
        pub unsafe fn setShowsApplicationBadge(&self, showsApplicationBadge: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other badgeLabel)]
        pub unsafe fn badgeLabel(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBadgeLabel:)]
        pub unsafe fn setBadgeLabel(&self, badgeLabel: Option<&Foundation::NSString>);

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;
    }
);

extern_protocol!(
    pub struct NSDockTilePlugIn;

    unsafe impl ProtocolType for NSDockTilePlugIn {
        #[method(setDockTile:)]
        pub unsafe fn setDockTile(&self, dockTile: Option<&AppKit::NSDockTile>);

        #[optional]
        #[method_id(@__retain_semantics Other dockMenu)]
        pub unsafe fn dockMenu(&self) -> Option<Id<AppKit::NSMenu, Shared>>;
    }
);
