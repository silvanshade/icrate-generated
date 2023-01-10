//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDateInterval;

    unsafe impl ClassType for NSDateInterval {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDateInterval")]
    unsafe impl NSDateInterval {
        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<Foundation::NSDate, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Id<Foundation::NSDate, Shared>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> Foundation::NSTimeInterval;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithStartDate:duration:)]
        pub unsafe fn initWithStartDate_duration(
            this: Option<Allocated<Self>>,
            startDate: &Foundation::NSDate,
            duration: Foundation::NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithStartDate:endDate:)]
        pub unsafe fn initWithStartDate_endDate(
            this: Option<Allocated<Self>>,
            startDate: &Foundation::NSDate,
            endDate: &Foundation::NSDate,
        ) -> Id<Self, Shared>;

        #[method(compare:)]
        pub unsafe fn compare(
            &self,
            dateInterval: &Foundation::NSDateInterval,
        ) -> Foundation::NSComparisonResult;

        #[method(isEqualToDateInterval:)]
        pub unsafe fn isEqualToDateInterval(
            &self,
            dateInterval: &Foundation::NSDateInterval,
        ) -> bool;

        #[method(intersectsDateInterval:)]
        pub unsafe fn intersectsDateInterval(
            &self,
            dateInterval: &Foundation::NSDateInterval,
        ) -> bool;

        #[method_id(@__retain_semantics Other intersectionWithDateInterval:)]
        pub unsafe fn intersectionWithDateInterval(
            &self,
            dateInterval: &Foundation::NSDateInterval,
        ) -> Option<Id<Foundation::NSDateInterval, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(containsDate:)]
        pub unsafe fn containsDate(&self, date: &Foundation::NSDate) -> bool;
    }
);
