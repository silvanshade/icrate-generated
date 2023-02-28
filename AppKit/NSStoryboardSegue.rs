//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSStoryboardSegueIdentifier = NSString;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSStoryboardSegue")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSStoryboardSegue;
}

#[cfg(feature = "AppKit_NSStoryboardSegue")]
unsafe impl NSObjectProtocol for NSStoryboardSegue {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSStoryboardSegue")]
    pub type NSStoryboardSegue;

    #[objc2::method(
        sel = "segueWithIdentifier:source:destination:performHandler:",
        managed = "Other"
    )]
    pub unsafe fn segueWithIdentifier_source_destination_performHandler(
        identifier: &NSStoryboardSegueIdentifier,
        source_controller: &Object,
        destination_controller: &Object,
        perform_handler: &Block<(), ()>,
    ) -> Id<Self>;

    #[objc2::method(sel = "initWithIdentifier:source:destination:", managed = "Init")]
    pub unsafe fn initWithIdentifier_source_destination(
        this: Option<Allocated<Self>>,
        identifier: &NSStoryboardSegueIdentifier,
        source_controller: &Object,
        destination_controller: &Object,
    ) -> Id<Self>;

    #[objc2::method(sel = "identifier", managed = "Other")]
    pub unsafe fn identifier(&self) -> Option<Id<NSStoryboardSegueIdentifier>>;

    #[objc2::method(sel = "sourceController", managed = "Other")]
    pub unsafe fn sourceController(&self) -> Id<Object>;

    #[objc2::method(sel = "destinationController", managed = "Other")]
    pub unsafe fn destinationController(&self) -> Id<Object>;

    #[objc2::method(sel = "perform")]
    pub unsafe fn perform(&self);
}

#[objc2::protocol]
pub unsafe trait NSSeguePerforming: NSObjectProtocol {
    #[cfg(feature = "AppKit_NSStoryboardSegue")]
    #[objc2::method(optional, sel = "prepareForSegue:sender:")]
    unsafe fn prepareForSegue_sender(&self, segue: &NSStoryboardSegue, sender: Option<&Object>);

    #[objc2::method(optional, sel = "performSegueWithIdentifier:sender:")]
    unsafe fn performSegueWithIdentifier_sender(
        &self,
        identifier: &NSStoryboardSegueIdentifier,
        sender: Option<&Object>,
    );

    #[objc2::method(optional, sel = "shouldPerformSegueWithIdentifier:sender:")]
    unsafe fn shouldPerformSegueWithIdentifier_sender(
        &self,
        identifier: &NSStoryboardSegueIdentifier,
        sender: Option<&Object>,
    ) -> bool;
}
