//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundTasks::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGTaskRequest")]
    pub struct BGTaskRequest;

    #[cfg(feature = "BackgroundTasks_BGTaskRequest")]
    unsafe impl ClassType for BGTaskRequest {
        type Super = NSObject;
    }
);

#[cfg(feature = "BackgroundTasks_BGTaskRequest")]
unsafe impl NSObjectProtocol for BGTaskRequest {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGTaskRequest")]
    unsafe impl BGTaskRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other earliestBeginDate)]
        pub unsafe fn earliestBeginDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setEarliestBeginDate:)]
        pub unsafe fn setEarliestBeginDate(&self, earliest_begin_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTaskRequest")]
    pub struct BGAppRefreshTaskRequest;

    #[cfg(feature = "BackgroundTasks_BGAppRefreshTaskRequest")]
    unsafe impl ClassType for BGAppRefreshTaskRequest {
        #[inherits(NSObject)]
        type Super = BGTaskRequest;
    }
);

#[cfg(feature = "BackgroundTasks_BGAppRefreshTaskRequest")]
unsafe impl NSObjectProtocol for BGAppRefreshTaskRequest {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTaskRequest")]
    unsafe impl BGAppRefreshTaskRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGProcessingTaskRequest")]
    pub struct BGProcessingTaskRequest;

    #[cfg(feature = "BackgroundTasks_BGProcessingTaskRequest")]
    unsafe impl ClassType for BGProcessingTaskRequest {
        #[inherits(NSObject)]
        type Super = BGTaskRequest;
    }
);

#[cfg(feature = "BackgroundTasks_BGProcessingTaskRequest")]
unsafe impl NSObjectProtocol for BGProcessingTaskRequest {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGProcessingTaskRequest")]
    unsafe impl BGProcessingTaskRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self>;

        #[method(requiresNetworkConnectivity)]
        pub unsafe fn requiresNetworkConnectivity(&self) -> bool;

        #[method(setRequiresNetworkConnectivity:)]
        pub unsafe fn setRequiresNetworkConnectivity(&self, requires_network_connectivity: bool);

        #[method(requiresExternalPower)]
        pub unsafe fn requiresExternalPower(&self) -> bool;

        #[method(setRequiresExternalPower:)]
        pub unsafe fn setRequiresExternalPower(&self, requires_external_power: bool);
    }
);
