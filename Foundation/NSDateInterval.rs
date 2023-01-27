//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDateInterval")]
    pub struct NSDateInterval;

    #[cfg(feature = "Foundation_NSDateInterval")]
    unsafe impl ClassType for NSDateInterval {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSDateInterval")]
unsafe impl NSCoding for NSDateInterval {}

#[cfg(feature = "Foundation_NSDateInterval")]
unsafe impl NSObjectProtocol for NSDateInterval {}

#[cfg(feature = "Foundation_NSDateInterval")]
unsafe impl NSSecureCoding for NSDateInterval {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDateInterval")]
    unsafe impl NSDateInterval {
        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Id<NSDate, Shared>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithStartDate:duration:)]
        pub unsafe fn initWithStartDate_duration(
            this: Option<Allocated<Self>>,
            start_date: &NSDate,
            duration: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithStartDate:endDate:)]
        pub unsafe fn initWithStartDate_endDate(
            this: Option<Allocated<Self>>,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Id<Self, Shared>;

        #[method(compare:)]
        pub unsafe fn compare(&self, date_interval: &NSDateInterval) -> NSComparisonResult;

        #[method(isEqualToDateInterval:)]
        pub unsafe fn isEqualToDateInterval(&self, date_interval: &NSDateInterval) -> bool;

        #[method(intersectsDateInterval:)]
        pub unsafe fn intersectsDateInterval(&self, date_interval: &NSDateInterval) -> bool;

        #[method_id(@__retain_semantics Other intersectionWithDateInterval:)]
        pub unsafe fn intersectionWithDateInterval(
            &self,
            date_interval: &NSDateInterval,
        ) -> Option<Id<NSDateInterval, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(containsDate:)]
        pub unsafe fn containsDate(&self, date: &NSDate) -> bool;
    }
);
