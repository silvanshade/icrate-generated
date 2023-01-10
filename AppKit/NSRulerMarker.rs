//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRulerMarker;

    unsafe impl ClassType for NSRulerMarker {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSRulerMarker")]
    unsafe impl NSRulerMarker {
        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSRulerView"))]
        #[method_id(@__retain_semantics Init initWithRulerView:markerLocation:image:imageOrigin:)]
        pub unsafe fn initWithRulerView_markerLocation_image_imageOrigin(
            this: Option<Allocated<Self>>,
            ruler: &AppKit::NSRulerView,
            location: Foundation::CGFloat,
            image: &AppKit::NSImage,
            imageOrigin: Foundation::NSPoint,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method_id(@__retain_semantics Other ruler)]
        pub unsafe fn ruler(&self) -> Option<Id<AppKit::NSRulerView, Shared>>;

        #[method(markerLocation)]
        pub unsafe fn markerLocation(&self) -> Foundation::CGFloat;

        #[method(setMarkerLocation:)]
        pub unsafe fn setMarkerLocation(&self, markerLocation: Foundation::CGFloat);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<AppKit::NSImage, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &AppKit::NSImage);

        #[method(imageOrigin)]
        pub unsafe fn imageOrigin(&self) -> Foundation::NSPoint;

        #[method(setImageOrigin:)]
        pub unsafe fn setImageOrigin(&self, imageOrigin: Foundation::NSPoint);

        #[method(isMovable)]
        pub unsafe fn isMovable(&self) -> bool;

        #[method(setMovable:)]
        pub unsafe fn setMovable(&self, movable: bool);

        #[method(isRemovable)]
        pub unsafe fn isRemovable(&self) -> bool;

        #[method(setRemovable:)]
        pub unsafe fn setRemovable(&self, removable: bool);

        #[method(isDragging)]
        pub unsafe fn isDragging(&self) -> bool;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, representedObject: Option<&Object>);

        #[method(imageRectInRuler)]
        pub unsafe fn imageRectInRuler(&self) -> Foundation::NSRect;

        #[method(thicknessRequiredInRuler)]
        pub unsafe fn thicknessRequiredInRuler(&self) -> Foundation::CGFloat;

        #[method(drawRect:)]
        pub unsafe fn drawRect(&self, rect: Foundation::NSRect);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(trackMouse:adding:)]
        pub unsafe fn trackMouse_adding(
            &self,
            mouseDownEvent: &AppKit::NSEvent,
            isAdding: bool,
        ) -> bool;
    }
);
