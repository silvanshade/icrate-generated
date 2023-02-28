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
    #[cfg(feature = "AppKit_NSMenu")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSMenu;
}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSAccessibility for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSAccessibilityElementProtocol for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSAppearanceCustomization for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSCoding for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSObjectProtocol for NSMenu {}

#[cfg(feature = "AppKit_NSMenu")]
unsafe impl NSUserInterfaceItemIdentification for NSMenu {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSMenu")]
    pub type NSMenu;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "initWithTitle:", managed = "Init")]
    pub unsafe fn initWithTitle(this: Option<Allocated<Self>>, title: &NSString) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "title", managed = "Other")]
    pub unsafe fn title(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setTitle:")]
    pub unsafe fn setTitle(&self, title: &NSString);

    #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
    #[objc2::method(sel = "popUpContextMenu:withEvent:forView:")]
    pub unsafe fn popUpContextMenu_withEvent_forView(menu: &NSMenu, event: &NSEvent, view: &NSView);

    #[cfg(all(
        feature = "AppKit_NSEvent",
        feature = "AppKit_NSFont",
        feature = "AppKit_NSView"
    ))]
    #[objc2::method(sel = "popUpContextMenu:withEvent:forView:withFont:")]
    pub unsafe fn popUpContextMenu_withEvent_forView_withFont(
        menu: &NSMenu,
        event: &NSEvent,
        view: &NSView,
        font: Option<&NSFont>,
    );

    #[cfg(all(feature = "AppKit_NSMenuItem", feature = "AppKit_NSView"))]
    #[objc2::method(sel = "popUpMenuPositioningItem:atLocation:inView:")]
    pub unsafe fn popUpMenuPositioningItem_atLocation_inView(
        &self,
        item: Option<&NSMenuItem>,
        location: NSPoint,
        view: Option<&NSView>,
    ) -> bool;

    #[objc2::method(sel = "setMenuBarVisible:")]
    pub unsafe fn setMenuBarVisible(visible: bool);

    #[objc2::method(sel = "menuBarVisible")]
    pub unsafe fn menuBarVisible() -> bool;

    #[objc2::method(sel = "supermenu", managed = "Other")]
    pub unsafe fn supermenu(&self) -> Option<Id<NSMenu>>;

    #[objc2::method(sel = "setSupermenu:")]
    pub unsafe fn setSupermenu(&self, supermenu: Option<&NSMenu>);

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "insertItem:atIndex:")]
    pub unsafe fn insertItem_atIndex(&self, new_item: &NSMenuItem, index: NSInteger);

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "addItem:")]
    pub unsafe fn addItem(&self, new_item: &NSMenuItem);

    #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
    #[objc2::method(
        sel = "insertItemWithTitle:action:keyEquivalent:atIndex:",
        managed = "Other"
    )]
    pub unsafe fn insertItemWithTitle_action_keyEquivalent_atIndex(
        &self,
        string: &NSString,
        selector: Option<Sel>,
        char_code: &NSString,
        index: NSInteger,
    ) -> Id<NSMenuItem>;

    #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "addItemWithTitle:action:keyEquivalent:", managed = "Other")]
    pub unsafe fn addItemWithTitle_action_keyEquivalent(
        &self,
        string: &NSString,
        selector: Option<Sel>,
        char_code: &NSString,
    ) -> Id<NSMenuItem>;

    #[objc2::method(sel = "removeItemAtIndex:")]
    pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "removeItem:")]
    pub unsafe fn removeItem(&self, item: &NSMenuItem);

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "setSubmenu:forItem:")]
    pub unsafe fn setSubmenu_forItem(&self, menu: Option<&NSMenu>, item: &NSMenuItem);

    #[objc2::method(sel = "removeAllItems")]
    pub unsafe fn removeAllItems(&self);

    #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "itemArray", managed = "Other")]
    pub unsafe fn itemArray(&self) -> Id<NSArray<NSMenuItem>>;

    #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
    #[objc2::method(sel = "setItemArray:")]
    pub unsafe fn setItemArray(&self, item_array: &NSArray<NSMenuItem>);

    #[objc2::method(sel = "numberOfItems")]
    pub unsafe fn numberOfItems(&self) -> NSInteger;

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "itemAtIndex:", managed = "Other")]
    pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<NSMenuItem>>;

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "indexOfItem:")]
    pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "indexOfItemWithTitle:")]
    pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;

    #[objc2::method(sel = "indexOfItemWithTag:")]
    pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

    #[objc2::method(sel = "indexOfItemWithRepresentedObject:")]
    pub unsafe fn indexOfItemWithRepresentedObject(&self, object: Option<&Object>) -> NSInteger;

    #[objc2::method(sel = "indexOfItemWithSubmenu:")]
    pub unsafe fn indexOfItemWithSubmenu(&self, submenu: Option<&NSMenu>) -> NSInteger;

    #[objc2::method(sel = "indexOfItemWithTarget:andAction:")]
    pub unsafe fn indexOfItemWithTarget_andAction(
        &self,
        target: Option<&Object>,
        action_selector: Option<Sel>,
    ) -> NSInteger;

    #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "itemWithTitle:", managed = "Other")]
    pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Id<NSMenuItem>>;

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "itemWithTag:", managed = "Other")]
    pub unsafe fn itemWithTag(&self, tag: NSInteger) -> Option<Id<NSMenuItem>>;

    #[objc2::method(sel = "autoenablesItems")]
    pub unsafe fn autoenablesItems(&self) -> bool;

    #[objc2::method(sel = "setAutoenablesItems:")]
    pub unsafe fn setAutoenablesItems(&self, autoenables_items: bool);

    #[objc2::method(sel = "update")]
    pub unsafe fn update(&self);

    #[cfg(feature = "AppKit_NSEvent")]
    #[objc2::method(sel = "performKeyEquivalent:")]
    pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "itemChanged:")]
    pub unsafe fn itemChanged(&self, item: &NSMenuItem);

    #[objc2::method(sel = "performActionForItemAtIndex:")]
    pub unsafe fn performActionForItemAtIndex(&self, index: NSInteger);

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSMenuDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSMenuDelegate>>);

    #[objc2::method(sel = "menuBarHeight")]
    pub unsafe fn menuBarHeight(&self) -> CGFloat;

    #[objc2::method(sel = "cancelTracking")]
    pub unsafe fn cancelTracking(&self);

    #[objc2::method(sel = "cancelTrackingWithoutAnimation")]
    pub unsafe fn cancelTrackingWithoutAnimation(&self);

    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "highlightedItem", managed = "Other")]
    pub unsafe fn highlightedItem(&self) -> Option<Id<NSMenuItem>>;

    #[objc2::method(sel = "minimumWidth")]
    pub unsafe fn minimumWidth(&self) -> CGFloat;

    #[objc2::method(sel = "setMinimumWidth:")]
    pub unsafe fn setMinimumWidth(&self, minimum_width: CGFloat);

    #[objc2::method(sel = "size")]
    pub unsafe fn size(&self) -> NSSize;

    #[cfg(feature = "AppKit_NSFont")]
    #[objc2::method(sel = "font", managed = "Other")]
    pub unsafe fn font(&self) -> Option<Id<NSFont>>;

    #[cfg(feature = "AppKit_NSFont")]
    #[objc2::method(sel = "setFont:")]
    pub unsafe fn setFont(&self, font: Option<&NSFont>);

    #[objc2::method(sel = "allowsContextMenuPlugIns")]
    pub unsafe fn allowsContextMenuPlugIns(&self) -> bool;

    #[objc2::method(sel = "setAllowsContextMenuPlugIns:")]
    pub unsafe fn setAllowsContextMenuPlugIns(&self, allows_context_menu_plug_ins: bool);

    #[objc2::method(sel = "showsStateColumn")]
    pub unsafe fn showsStateColumn(&self) -> bool;

    #[objc2::method(sel = "setShowsStateColumn:")]
    pub unsafe fn setShowsStateColumn(&self, shows_state_column: bool);

    #[objc2::method(sel = "userInterfaceLayoutDirection")]
    pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

    #[objc2::method(sel = "setUserInterfaceLayoutDirection:")]
    pub unsafe fn setUserInterfaceLayoutDirection(
        &self,
        user_interface_layout_direction: NSUserInterfaceLayoutDirection,
    );
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSMenu")]
    pub type NSMenu;

    #[objc2::method(sel = "submenuAction:")]
    pub unsafe fn submenuAction(&self, sender: Option<&Object>);
}

#[objc2::protocol]
pub unsafe trait NSMenuItemValidation: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSMenuItem")]
    #[objc2::method(sel = "validateMenuItem:")]
    unsafe fn validateMenuItem(&self, menu_item: &NSMenuItem) -> bool;
}

#[objc2::protocol]
pub unsafe trait NSMenuDelegate: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSMenu")]
    #[objc2::method(optional, sel = "menuNeedsUpdate:")]
    unsafe fn menuNeedsUpdate(&self, menu: &NSMenu);

    #[cfg(feature = "AppKit_NSMenu")]
    #[objc2::method(optional, sel = "numberOfItemsInMenu:")]
    unsafe fn numberOfItemsInMenu(&self, menu: &NSMenu) -> NSInteger;

    #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSMenuItem"))]
    #[objc2::method(optional, sel = "menu:updateItem:atIndex:shouldCancel:")]
    unsafe fn menu_updateItem_atIndex_shouldCancel(
        &self,
        menu: &NSMenu,
        item: &NSMenuItem,
        index: NSInteger,
        should_cancel: bool,
    ) -> bool;

    #[cfg(feature = "AppKit_NSMenu")]
    #[objc2::method(optional, sel = "menuWillOpen:")]
    unsafe fn menuWillOpen(&self, menu: &NSMenu);

    #[cfg(feature = "AppKit_NSMenu")]
    #[objc2::method(optional, sel = "menuDidClose:")]
    unsafe fn menuDidClose(&self, menu: &NSMenu);

    #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSMenuItem"))]
    #[objc2::method(optional, sel = "menu:willHighlightItem:")]
    unsafe fn menu_willHighlightItem(&self, menu: &NSMenu, item: Option<&NSMenuItem>);

    #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSScreen"))]
    #[objc2::method(optional, sel = "confinementRectForMenu:onScreen:")]
    unsafe fn confinementRectForMenu_onScreen(
        &self,
        menu: &NSMenu,
        screen: Option<&NSScreen>,
    ) -> NSRect;
}

#[ns_options]
#[underlying(NSUInteger)]
pub enum NSMenuProperties {
    NSMenuPropertyItemTitle = 1 << 0,
    NSMenuPropertyItemAttributedTitle = 1 << 1,
    NSMenuPropertyItemKeyEquivalent = 1 << 2,
    NSMenuPropertyItemImage = 1 << 3,
    NSMenuPropertyItemEnabled = 1 << 4,
    NSMenuPropertyItemAccessibilityDescription = 1 << 5,
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSMenu")]
    pub type NSMenu;

    #[objc2::method(sel = "propertiesToUpdate")]
    pub unsafe fn propertiesToUpdate(&self) -> NSMenuProperties;
}

extern_static!(NSMenuWillSendActionNotification: &'static NSNotificationName);

extern_static!(NSMenuDidSendActionNotification: &'static NSNotificationName);

extern_static!(NSMenuDidAddItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidRemoveItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidChangeItemNotification: &'static NSNotificationName);

extern_static!(NSMenuDidBeginTrackingNotification: &'static NSNotificationName);

extern_static!(NSMenuDidEndTrackingNotification: &'static NSNotificationName);

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSMenu")]
    pub type NSMenu;

    #[deprecated]
    #[objc2::method(sel = "setMenuRepresentation:")]
    pub unsafe fn setMenuRepresentation(&self, menu_rep: Option<&Object>);

    #[deprecated]
    #[objc2::method(sel = "menuRepresentation", managed = "Other")]
    pub unsafe fn menuRepresentation(&self) -> Option<Id<Object>>;

    #[deprecated]
    #[objc2::method(sel = "setContextMenuRepresentation:")]
    pub unsafe fn setContextMenuRepresentation(&self, menu_rep: Option<&Object>);

    #[deprecated]
    #[objc2::method(sel = "contextMenuRepresentation", managed = "Other")]
    pub unsafe fn contextMenuRepresentation(&self) -> Option<Id<Object>>;

    #[deprecated]
    #[objc2::method(sel = "setTearOffMenuRepresentation:")]
    pub unsafe fn setTearOffMenuRepresentation(&self, menu_rep: Option<&Object>);

    #[deprecated]
    #[objc2::method(sel = "tearOffMenuRepresentation", managed = "Other")]
    pub unsafe fn tearOffMenuRepresentation(&self) -> Option<Id<Object>>;

    #[deprecated]
    #[objc2::method(sel = "menuZone")]
    pub unsafe fn menuZone() -> *mut NSZone;

    #[deprecated]
    #[objc2::method(sel = "setMenuZone:")]
    pub unsafe fn setMenuZone(zone: *mut NSZone);

    #[deprecated]
    #[objc2::method(sel = "attachedMenu", managed = "Other")]
    pub unsafe fn attachedMenu(&self) -> Option<Id<NSMenu>>;

    #[deprecated]
    #[objc2::method(sel = "isAttached")]
    pub unsafe fn isAttached(&self) -> bool;

    #[deprecated]
    #[objc2::method(sel = "sizeToFit")]
    pub unsafe fn sizeToFit(&self);

    #[deprecated]
    #[objc2::method(sel = "locationForSubmenu:")]
    pub unsafe fn locationForSubmenu(&self, submenu: Option<&NSMenu>) -> NSPoint;

    #[deprecated]
    #[objc2::method(sel = "menuChangedMessagesEnabled")]
    pub unsafe fn menuChangedMessagesEnabled(&self) -> bool;

    #[deprecated]
    #[objc2::method(sel = "setMenuChangedMessagesEnabled:")]
    pub unsafe fn setMenuChangedMessagesEnabled(&self, menu_changed_messages_enabled: bool);

    #[cfg(feature = "AppKit_NSEvent")]
    #[deprecated]
    #[objc2::method(sel = "helpRequested:")]
    pub unsafe fn helpRequested(&self, event_ptr: &NSEvent);

    #[deprecated]
    #[objc2::method(sel = "isTornOff")]
    pub unsafe fn isTornOff(&self) -> bool;
}
