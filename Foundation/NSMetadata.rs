//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataQuery")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSMetadataQuery;
}

#[cfg(feature = "Foundation_NSMetadataQuery")]
unsafe impl NSObjectProtocol for NSMetadataQuery {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataQuery")]
    pub type NSMetadataQuery;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSMetadataQueryDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(
        &self,
        delegate: Option<&ProtocolObject<dyn NSMetadataQueryDelegate>>,
    );

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "predicate", managed = "Other")]
    pub unsafe fn predicate(&self) -> Option<Id<NSPredicate>>;

    #[cfg(feature = "Foundation_NSPredicate")]
    #[objc2::method(sel = "setPredicate:")]
    pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSSortDescriptor"
    ))]
    #[objc2::method(sel = "sortDescriptors", managed = "Other")]
    pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSSortDescriptor"
    ))]
    #[objc2::method(sel = "setSortDescriptors:")]
    pub unsafe fn setSortDescriptors(&self, sort_descriptors: &NSArray<NSSortDescriptor>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "valueListAttributes", managed = "Other")]
    pub unsafe fn valueListAttributes(&self) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setValueListAttributes:")]
    pub unsafe fn setValueListAttributes(&self, value_list_attributes: &NSArray<NSString>);

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "groupingAttributes", managed = "Other")]
    pub unsafe fn groupingAttributes(&self) -> Option<Id<NSArray<NSString>>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setGroupingAttributes:")]
    pub unsafe fn setGroupingAttributes(&self, grouping_attributes: Option<&NSArray<NSString>>);

    #[objc2::method(sel = "notificationBatchingInterval")]
    pub unsafe fn notificationBatchingInterval(&self) -> NSTimeInterval;

    #[objc2::method(sel = "setNotificationBatchingInterval:")]
    pub unsafe fn setNotificationBatchingInterval(
        &self,
        notification_batching_interval: NSTimeInterval,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "searchScopes", managed = "Other")]
    pub unsafe fn searchScopes(&self) -> Id<NSArray>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setSearchScopes:")]
    pub unsafe fn setSearchScopes(&self, search_scopes: &NSArray);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "searchItems", managed = "Other")]
    pub unsafe fn searchItems(&self) -> Option<Id<NSArray>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setSearchItems:")]
    pub unsafe fn setSearchItems(&self, search_items: Option<&NSArray>);

    #[cfg(feature = "Foundation_NSOperationQueue")]
    #[objc2::method(sel = "operationQueue", managed = "Other")]
    pub unsafe fn operationQueue(&self) -> Option<Id<NSOperationQueue>>;

    #[cfg(feature = "Foundation_NSOperationQueue")]
    #[objc2::method(sel = "setOperationQueue:")]
    pub unsafe fn setOperationQueue(&self, operation_queue: Option<&NSOperationQueue>);

    #[objc2::method(sel = "startQuery")]
    pub unsafe fn startQuery(&self) -> bool;

    #[objc2::method(sel = "stopQuery")]
    pub unsafe fn stopQuery(&self);

    #[objc2::method(sel = "isStarted")]
    pub unsafe fn isStarted(&self) -> bool;

    #[objc2::method(sel = "isGathering")]
    pub unsafe fn isGathering(&self) -> bool;

    #[objc2::method(sel = "isStopped")]
    pub unsafe fn isStopped(&self) -> bool;

    #[objc2::method(sel = "disableUpdates")]
    pub unsafe fn disableUpdates(&self);

    #[objc2::method(sel = "enableUpdates")]
    pub unsafe fn enableUpdates(&self);

    #[objc2::method(sel = "resultCount")]
    pub unsafe fn resultCount(&self) -> NSUInteger;

    #[objc2::method(sel = "resultAtIndex:", managed = "Other")]
    pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object>;

    #[objc2::method(sel = "enumerateResultsUsingBlock:")]
    pub unsafe fn enumerateResultsUsingBlock(
        &self,
        block: &Block<(NonNull<Object>, NSUInteger, NonNull<Bool>), ()>,
    );

    #[objc2::method(sel = "enumerateResultsWithOptions:usingBlock:")]
    pub unsafe fn enumerateResultsWithOptions_usingBlock(
        &self,
        opts: NSEnumerationOptions,
        block: &Block<(NonNull<Object>, NSUInteger, NonNull<Bool>), ()>,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "results", managed = "Other")]
    pub unsafe fn results(&self) -> Id<NSArray>;

    #[objc2::method(sel = "indexOfResult:")]
    pub unsafe fn indexOfResult(&self, result: &Object) -> NSUInteger;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSMetadataQueryAttributeValueTuple",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "valueLists", managed = "Other")]
    pub unsafe fn valueLists(
        &self,
    ) -> Id<NSDictionary<NSString, NSArray<NSMetadataQueryAttributeValueTuple>>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSMetadataQueryResultGroup"
    ))]
    #[objc2::method(sel = "groupedResults", managed = "Other")]
    pub unsafe fn groupedResults(&self) -> Id<NSArray<NSMetadataQueryResultGroup>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "valueOfAttribute:forResultAtIndex:", managed = "Other")]
    pub unsafe fn valueOfAttribute_forResultAtIndex(
        &self,
        attr_name: &NSString,
        idx: NSUInteger,
    ) -> Option<Id<Object>>;
}

#[objc2::protocol]
pub unsafe trait NSMetadataQueryDelegate: NSObjectProtocol {
    #[cfg(all(
        feature = "Foundation_NSMetadataItem",
        feature = "Foundation_NSMetadataQuery"
    ))]
    #[objc2::method(
        optional,
        sel = "metadataQuery:replacementObjectForResultObject:",
        managed = "Other"
    )]
    unsafe fn metadataQuery_replacementObjectForResultObject(
        &self,
        query: &NSMetadataQuery,
        result: &NSMetadataItem,
    ) -> Id<Object>;

    #[cfg(all(
        feature = "Foundation_NSMetadataQuery",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(
        optional,
        sel = "metadataQuery:replacementValueForAttribute:value:",
        managed = "Other"
    )]
    unsafe fn metadataQuery_replacementValueForAttribute_value(
        &self,
        query: &NSMetadataQuery,
        attr_name: &NSString,
        attr_value: &Object,
    ) -> Id<Object>;
}

extern_static!(NSMetadataQueryDidStartGatheringNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryGatheringProgressNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryDidFinishGatheringNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryDidUpdateNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryUpdateAddedItemsKey: &'static NSString);

extern_static!(NSMetadataQueryUpdateChangedItemsKey: &'static NSString);

extern_static!(NSMetadataQueryUpdateRemovedItemsKey: &'static NSString);

extern_static!(NSMetadataQueryResultContentRelevanceAttribute: &'static NSString);

extern_static!(NSMetadataQueryUserHomeScope: &'static NSString);

extern_static!(NSMetadataQueryLocalComputerScope: &'static NSString);

extern_static!(NSMetadataQueryNetworkScope: &'static NSString);

extern_static!(NSMetadataQueryIndexedLocalComputerScope: &'static NSString);

extern_static!(NSMetadataQueryIndexedNetworkScope: &'static NSString);

extern_static!(NSMetadataQueryUbiquitousDocumentsScope: &'static NSString);

extern_static!(NSMetadataQueryUbiquitousDataScope: &'static NSString);

extern_static!(NSMetadataQueryAccessibleUbiquitousExternalDocumentsScope: &'static NSString);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataItem")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSMetadataItem;
}

#[cfg(feature = "Foundation_NSMetadataItem")]
unsafe impl NSObjectProtocol for NSMetadataItem {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataItem")]
    pub type NSMetadataItem;

    #[cfg(feature = "Foundation_NSURL")]
    #[objc2::method(sel = "initWithURL:", managed = "Init")]
    pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Option<Id<Self>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "valueForAttribute:", managed = "Other")]
    pub unsafe fn valueForAttribute(&self, key: &NSString) -> Option<Id<Object>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "valuesForAttributes:", managed = "Other")]
    pub unsafe fn valuesForAttributes(
        &self,
        keys: &NSArray<NSString>,
    ) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "attributes", managed = "Other")]
    pub unsafe fn attributes(&self) -> Id<NSArray<NSString>>;
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataQueryAttributeValueTuple")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSMetadataQueryAttributeValueTuple;
}

#[cfg(feature = "Foundation_NSMetadataQueryAttributeValueTuple")]
unsafe impl NSObjectProtocol for NSMetadataQueryAttributeValueTuple {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataQueryAttributeValueTuple")]
    pub type NSMetadataQueryAttributeValueTuple;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "attribute", managed = "Other")]
    pub unsafe fn attribute(&self) -> Id<NSString>;

    #[objc2::method(sel = "value", managed = "Other")]
    pub unsafe fn value(&self) -> Option<Id<Object>>;

    #[objc2::method(sel = "count")]
    pub unsafe fn count(&self) -> NSUInteger;
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataQueryResultGroup")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSMetadataQueryResultGroup;
}

#[cfg(feature = "Foundation_NSMetadataQueryResultGroup")]
unsafe impl NSObjectProtocol for NSMetadataQueryResultGroup {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSMetadataQueryResultGroup")]
    pub type NSMetadataQueryResultGroup;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "attribute", managed = "Other")]
    pub unsafe fn attribute(&self) -> Id<NSString>;

    #[objc2::method(sel = "value", managed = "Other")]
    pub unsafe fn value(&self) -> Id<Object>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "subgroups", managed = "Other")]
    pub unsafe fn subgroups(&self) -> Option<Id<NSArray<NSMetadataQueryResultGroup>>>;

    #[objc2::method(sel = "resultCount")]
    pub unsafe fn resultCount(&self) -> NSUInteger;

    #[objc2::method(sel = "resultAtIndex:", managed = "Other")]
    pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "results", managed = "Other")]
    pub unsafe fn results(&self) -> Id<NSArray>;
}
