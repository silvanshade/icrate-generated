//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[objc2::interface(
    unsafe super = HKSeriesBuilder,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKHeartbeatSeriesBuilder")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type HKHeartbeatSeriesBuilder;
}

#[cfg(feature = "HealthKit_HKHeartbeatSeriesBuilder")]
unsafe impl NSObjectProtocol for HKHeartbeatSeriesBuilder {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "HealthKit_HKHeartbeatSeriesBuilder")]
    pub type HKHeartbeatSeriesBuilder;

    #[objc2::method(sel = "maximumCount")]
    pub unsafe fn maximumCount() -> NSUInteger;

    #[cfg(all(
        feature = "Foundation_NSDate",
        feature = "HealthKit_HKDevice",
        feature = "HealthKit_HKHealthStore"
    ))]
    #[objc2::method(sel = "initWithHealthStore:device:startDate:", managed = "Init")]
    pub unsafe fn initWithHealthStore_device_startDate(
        this: Option<Allocated<Self>>,
        health_store: &HKHealthStore,
        device: Option<&HKDevice>,
        start_date: &NSDate,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(
        sel = "addHeartbeatWithTimeIntervalSinceSeriesStartDate:precededByGap:completion:"
    )]
    pub unsafe fn addHeartbeatWithTimeIntervalSinceSeriesStartDate_precededByGap_completion(
        &self,
        time_interval_since_start: NSTimeInterval,
        preceded_by_gap: bool,
        completion: &Block<(Bool, *mut NSError), ()>,
    );

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSError",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "addMetadata:completion:")]
    pub unsafe fn addMetadata_completion(
        &self,
        metadata: &NSDictionary<NSString, Object>,
        completion: &Block<(Bool, *mut NSError), ()>,
    );

    #[cfg(all(
        feature = "Foundation_NSError",
        feature = "HealthKit_HKHeartbeatSeriesSample"
    ))]
    #[objc2::method(sel = "finishSeriesWithCompletion:")]
    pub unsafe fn finishSeriesWithCompletion(
        &self,
        completion: &Block<(*mut HKHeartbeatSeriesSample, *mut NSError), ()>,
    );
}
