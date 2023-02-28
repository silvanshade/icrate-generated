//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSColorPicker")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSColorPicker;
}

#[cfg(feature = "AppKit_NSColorPicker")]
unsafe impl NSColorPickingDefault for NSColorPicker {}

#[cfg(feature = "AppKit_NSColorPicker")]
unsafe impl NSObjectProtocol for NSColorPicker {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSColorPicker")]
    pub type NSColorPicker;

    #[cfg(feature = "AppKit_NSColorPanel")]
    #[objc2::method(sel = "initWithPickerMask:colorPanel:", managed = "Init")]
    pub unsafe fn initWithPickerMask_colorPanel(
        this: Option<Allocated<Self>>,
        mask: NSUInteger,
        owning_color_panel: &NSColorPanel,
    ) -> Option<Id<Self>>;

    #[cfg(feature = "AppKit_NSColorPanel")]
    #[objc2::method(sel = "colorPanel", managed = "Other")]
    pub unsafe fn colorPanel(&self) -> Id<NSColorPanel>;

    #[cfg(feature = "AppKit_NSImage")]
    #[objc2::method(sel = "provideNewButtonImage", managed = "Other")]
    pub unsafe fn provideNewButtonImage(&self) -> Id<NSImage>;

    #[cfg(all(feature = "AppKit_NSButtonCell", feature = "AppKit_NSImage"))]
    #[objc2::method(sel = "insertNewButtonImage:in:")]
    pub unsafe fn insertNewButtonImage_in(
        &self,
        new_button_image: &NSImage,
        button_cell: &NSButtonCell,
    );

    #[objc2::method(sel = "viewSizeChanged:")]
    pub unsafe fn viewSizeChanged(&self, sender: Option<&Object>);

    #[cfg(feature = "AppKit_NSColorList")]
    #[objc2::method(sel = "attachColorList:")]
    pub unsafe fn attachColorList(&self, color_list: &NSColorList);

    #[cfg(feature = "AppKit_NSColorList")]
    #[objc2::method(sel = "detachColorList:")]
    pub unsafe fn detachColorList(&self, color_list: &NSColorList);

    #[objc2::method(sel = "setMode:")]
    pub unsafe fn setMode(&self, mode: NSColorPanelMode);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "buttonToolTip", managed = "Other")]
    pub unsafe fn buttonToolTip(&self) -> Id<NSString>;

    #[objc2::method(sel = "minContentSize")]
    pub unsafe fn minContentSize(&self) -> NSSize;
}
