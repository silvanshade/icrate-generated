//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_static!(NSAppKitVersionNumberWithDirectionalTabs: AppKit::NSAppKitVersion = 631.0);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabViewType {
        NSTopTabsBezelBorder = 0,
        NSLeftTabsBezelBorder = 1,
        NSBottomTabsBezelBorder = 2,
        NSRightTabsBezelBorder = 3,
        NSNoTabsBezelBorder = 4,
        NSNoTabsLineBorder = 5,
        NSNoTabsNoBorder = 6,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabPosition {
        NSTabPositionNone = 0,
        NSTabPositionTop = 1,
        NSTabPositionLeft = 2,
        NSTabPositionBottom = 3,
        NSTabPositionRight = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabViewBorderType {
        NSTabViewBorderTypeNone = 0,
        NSTabViewBorderTypeLine = 1,
        NSTabViewBorderTypeBezel = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTabView;

    unsafe impl ClassType for NSTabView {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTabView")]
    unsafe impl NSTabView {
        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(selectTabViewItem:)]
        pub unsafe fn selectTabViewItem(&self, tabViewItem: Option<&AppKit::NSTabViewItem>);

        #[method(selectTabViewItemAtIndex:)]
        pub unsafe fn selectTabViewItemAtIndex(&self, index: NSInteger);

        #[method(selectTabViewItemWithIdentifier:)]
        pub unsafe fn selectTabViewItemWithIdentifier(&self, identifier: &Object);

        #[method(takeSelectedTabViewItemFromSender:)]
        pub unsafe fn takeSelectedTabViewItemFromSender(&self, sender: Option<&Object>);

        #[method(selectFirstTabViewItem:)]
        pub unsafe fn selectFirstTabViewItem(&self, sender: Option<&Object>);

        #[method(selectLastTabViewItem:)]
        pub unsafe fn selectLastTabViewItem(&self, sender: Option<&Object>);

        #[method(selectNextTabViewItem:)]
        pub unsafe fn selectNextTabViewItem(&self, sender: Option<&Object>);

        #[method(selectPreviousTabViewItem:)]
        pub unsafe fn selectPreviousTabViewItem(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method_id(@__retain_semantics Other selectedTabViewItem)]
        pub unsafe fn selectedTabViewItem(&self) -> Option<Id<AppKit::NSTabViewItem, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Id<AppKit::NSFont, Shared>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &AppKit::NSFont);

        #[method(tabViewType)]
        pub unsafe fn tabViewType(&self) -> AppKit::NSTabViewType;

        #[method(setTabViewType:)]
        pub unsafe fn setTabViewType(&self, tabViewType: AppKit::NSTabViewType);

        #[method(tabPosition)]
        pub unsafe fn tabPosition(&self) -> AppKit::NSTabPosition;

        #[method(setTabPosition:)]
        pub unsafe fn setTabPosition(&self, tabPosition: AppKit::NSTabPosition);

        #[method(tabViewBorderType)]
        pub unsafe fn tabViewBorderType(&self) -> AppKit::NSTabViewBorderType;

        #[method(setTabViewBorderType:)]
        pub unsafe fn setTabViewBorderType(&self, tabViewBorderType: AppKit::NSTabViewBorderType);

        #[cfg(all(feature = "AppKit_NSTabViewItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other tabViewItems)]
        pub unsafe fn tabViewItems(&self)
            -> Id<Foundation::NSArray<AppKit::NSTabViewItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSTabViewItem", feature = "Foundation_NSArray"))]
        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(
            &self,
            tabViewItems: &Foundation::NSArray<AppKit::NSTabViewItem>,
        );

        #[method(allowsTruncatedLabels)]
        pub unsafe fn allowsTruncatedLabels(&self) -> bool;

        #[method(setAllowsTruncatedLabels:)]
        pub unsafe fn setAllowsTruncatedLabels(&self, allowsTruncatedLabels: bool);

        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> Foundation::NSSize;

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> AppKit::NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: AppKit::NSControlSize);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(addTabViewItem:)]
        pub unsafe fn addTabViewItem(&self, tabViewItem: &AppKit::NSTabViewItem);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(insertTabViewItem:atIndex:)]
        pub unsafe fn insertTabViewItem_atIndex(
            &self,
            tabViewItem: &AppKit::NSTabViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(removeTabViewItem:)]
        pub unsafe fn removeTabViewItem(&self, tabViewItem: &AppKit::NSTabViewItem);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSTabViewDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSTabViewDelegate>);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItemAtPoint:)]
        pub unsafe fn tabViewItemAtPoint(
            &self,
            point: Foundation::NSPoint,
        ) -> Option<Id<AppKit::NSTabViewItem, Shared>>;

        #[method(contentRect)]
        pub unsafe fn contentRect(&self) -> Foundation::NSRect;

        #[method(numberOfTabViewItems)]
        pub unsafe fn numberOfTabViewItems(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(indexOfTabViewItem:)]
        pub unsafe fn indexOfTabViewItem(&self, tabViewItem: &AppKit::NSTabViewItem) -> NSInteger;

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItemAtIndex:)]
        pub unsafe fn tabViewItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Id<AppKit::NSTabViewItem, Shared>;

        #[method(indexOfTabViewItemWithIdentifier:)]
        pub unsafe fn indexOfTabViewItemWithIdentifier(&self, identifier: &Object) -> NSInteger;

        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> AppKit::NSControlTint;

        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, controlTint: AppKit::NSControlTint);
    }
);

extern_protocol!(
    pub struct NSTabViewDelegate;

    unsafe impl ProtocolType for NSTabViewDelegate {
        #[optional]
        #[method(tabView:shouldSelectTabViewItem:)]
        pub unsafe fn tabView_shouldSelectTabViewItem(
            &self,
            tabView: &AppKit::NSTabView,
            tabViewItem: Option<&AppKit::NSTabViewItem>,
        ) -> bool;

        #[optional]
        #[method(tabView:willSelectTabViewItem:)]
        pub unsafe fn tabView_willSelectTabViewItem(
            &self,
            tabView: &AppKit::NSTabView,
            tabViewItem: Option<&AppKit::NSTabViewItem>,
        );

        #[optional]
        #[method(tabView:didSelectTabViewItem:)]
        pub unsafe fn tabView_didSelectTabViewItem(
            &self,
            tabView: &AppKit::NSTabView,
            tabViewItem: Option<&AppKit::NSTabViewItem>,
        );

        #[optional]
        #[method(tabViewDidChangeNumberOfTabViewItems:)]
        pub unsafe fn tabViewDidChangeNumberOfTabViewItems(&self, tabView: &AppKit::NSTabView);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTabView")]
    unsafe impl AppKit::NSTabView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: Foundation::NSRect,
        ) -> Id<Self, Shared>;
    }
);
