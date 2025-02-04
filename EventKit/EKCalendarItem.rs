//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKCalendarItem")]
    pub struct EKCalendarItem;

    #[cfg(feature = "EventKit_EKCalendarItem")]
    unsafe impl ClassType for EKCalendarItem {
        #[inherits(NSObject)]
        type Super = EKObject;
    }
);

#[cfg(feature = "EventKit_EKCalendarItem")]
unsafe impl NSObjectProtocol for EKCalendarItem {}

extern_methods!(
    #[cfg(feature = "EventKit_EKCalendarItem")]
    unsafe impl EKCalendarItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSString>;

        #[cfg(feature = "EventKit_EKCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<EKCalendar>>;

        #[cfg(feature = "EventKit_EKCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&EKCalendar>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other calendarItemIdentifier)]
        pub unsafe fn calendarItemIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other calendarItemExternalIdentifier)]
        pub unsafe fn calendarItemExternalIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocation:)]
        pub unsafe fn setLocation(&self, location: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other notes)]
        pub unsafe fn notes(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNotes:)]
        pub unsafe fn setNotes(&self, notes: Option<&NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastModifiedDate)]
        pub unsafe fn lastModifiedDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(hasAlarms)]
        pub unsafe fn hasAlarms(&self) -> bool;

        #[method(hasRecurrenceRules)]
        pub unsafe fn hasRecurrenceRules(&self) -> bool;

        #[method(hasAttendees)]
        pub unsafe fn hasAttendees(&self) -> bool;

        #[method(hasNotes)]
        pub unsafe fn hasNotes(&self) -> bool;

        #[cfg(all(feature = "EventKit_EKParticipant", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other attendees)]
        pub unsafe fn attendees(&self) -> Option<Id<NSArray<EKParticipant>>>;

        #[cfg(all(feature = "EventKit_EKAlarm", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other alarms)]
        pub unsafe fn alarms(&self) -> Option<Id<NSArray<EKAlarm>>>;

        #[cfg(all(feature = "EventKit_EKAlarm", feature = "Foundation_NSArray"))]
        #[method(setAlarms:)]
        pub unsafe fn setAlarms(&self, alarms: Option<&NSArray<EKAlarm>>);

        #[cfg(feature = "EventKit_EKAlarm")]
        #[method(addAlarm:)]
        pub unsafe fn addAlarm(&self, alarm: &EKAlarm);

        #[cfg(feature = "EventKit_EKAlarm")]
        #[method(removeAlarm:)]
        pub unsafe fn removeAlarm(&self, alarm: &EKAlarm);

        #[cfg(all(feature = "EventKit_EKRecurrenceRule", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recurrenceRules)]
        pub unsafe fn recurrenceRules(&self) -> Option<Id<NSArray<EKRecurrenceRule>>>;

        #[cfg(all(feature = "EventKit_EKRecurrenceRule", feature = "Foundation_NSArray"))]
        #[method(setRecurrenceRules:)]
        pub unsafe fn setRecurrenceRules(
            &self,
            recurrence_rules: Option<&NSArray<EKRecurrenceRule>>,
        );

        #[cfg(feature = "EventKit_EKRecurrenceRule")]
        #[method(addRecurrenceRule:)]
        pub unsafe fn addRecurrenceRule(&self, rule: &EKRecurrenceRule);

        #[cfg(feature = "EventKit_EKRecurrenceRule")]
        #[method(removeRecurrenceRule:)]
        pub unsafe fn removeRecurrenceRule(&self, rule: &EKRecurrenceRule);
    }
);
