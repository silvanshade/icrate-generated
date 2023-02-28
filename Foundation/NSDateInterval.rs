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
    #[cfg(feature = "Foundation_NSDateInterval")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSDateInterval;
}

#[cfg(feature = "Foundation_NSDateInterval")]
unsafe impl NSCoding for NSDateInterval {}

#[cfg(feature = "Foundation_NSDateInterval")]
unsafe impl NSObjectProtocol for NSDateInterval {}

#[cfg(feature = "Foundation_NSDateInterval")]
unsafe impl NSSecureCoding for NSDateInterval {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSDateInterval")]
    pub type NSDateInterval;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "startDate", managed = "Other")]
    pub unsafe fn startDate(&self) -> Id<NSDate>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "endDate", managed = "Other")]
    pub unsafe fn endDate(&self) -> Id<NSDate>;

    #[objc2::method(sel = "duration")]
    pub unsafe fn duration(&self) -> NSTimeInterval;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "initWithStartDate:duration:", managed = "Init")]
    pub unsafe fn initWithStartDate_duration(
        this: Option<Allocated<Self>>,
        start_date: &NSDate,
        duration: NSTimeInterval,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "initWithStartDate:endDate:", managed = "Init")]
    pub unsafe fn initWithStartDate_endDate(
        this: Option<Allocated<Self>>,
        start_date: &NSDate,
        end_date: &NSDate,
    ) -> Id<Self>;

    #[objc2::method(sel = "compare:")]
    pub unsafe fn compare(&self, date_interval: &NSDateInterval) -> NSComparisonResult;

    #[objc2::method(sel = "isEqualToDateInterval:")]
    pub unsafe fn isEqualToDateInterval(&self, date_interval: &NSDateInterval) -> bool;

    #[objc2::method(sel = "intersectsDateInterval:")]
    pub unsafe fn intersectsDateInterval(&self, date_interval: &NSDateInterval) -> bool;

    #[objc2::method(sel = "intersectionWithDateInterval:", managed = "Other")]
    pub unsafe fn intersectionWithDateInterval(
        &self,
        date_interval: &NSDateInterval,
    ) -> Option<Id<NSDateInterval>>;

    #[cfg(feature = "Foundation_NSDate")]
    #[objc2::method(sel = "containsDate:")]
    pub unsafe fn containsDate(&self, date: &NSDate) -> bool;
}
