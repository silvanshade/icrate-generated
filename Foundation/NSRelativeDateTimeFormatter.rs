//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum NSRelativeDateTimeFormatterStyle {
    NSRelativeDateTimeFormatterStyleNumeric = 0,
    NSRelativeDateTimeFormatterStyleNamed = 1,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum NSRelativeDateTimeFormatterUnitsStyle {
    NSRelativeDateTimeFormatterUnitsStyleFull = 0,
    NSRelativeDateTimeFormatterUnitsStyleSpellOut = 1,
    NSRelativeDateTimeFormatterUnitsStyleShort = 2,
    NSRelativeDateTimeFormatterUnitsStyleAbbreviated = 3,
}

#[objc2::interface(
    unsafe super = NSFormatter,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSRelativeDateTimeFormatter;
}

#[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
unsafe impl NSCoding for NSRelativeDateTimeFormatter {}

#[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
unsafe impl NSObjectProtocol for NSRelativeDateTimeFormatter {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    pub type NSRelativeDateTimeFormatter;

    #[objc2::method(sel = "dateTimeStyle")]
    pub unsafe fn dateTimeStyle(&self) -> NSRelativeDateTimeFormatterStyle;

    #[objc2::method(sel = "setDateTimeStyle:")]
    pub unsafe fn setDateTimeStyle(&self, date_time_style: NSRelativeDateTimeFormatterStyle);

    #[objc2::method(sel = "unitsStyle")]
    pub unsafe fn unitsStyle(&self) -> NSRelativeDateTimeFormatterUnitsStyle;

    #[objc2::method(sel = "setUnitsStyle:")]
    pub unsafe fn setUnitsStyle(&self, units_style: NSRelativeDateTimeFormatterUnitsStyle);

    #[objc2::method(sel = "formattingContext")]
    pub unsafe fn formattingContext(&self) -> NSFormattingContext;

    #[objc2::method(sel = "setFormattingContext:")]
    pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

    #[cfg(feature = "Foundation_NSCalendar")]
    #[objc2::method(sel = "calendar", managed = "Other")]
    pub unsafe fn calendar(&self) -> Id<NSCalendar>;

    #[cfg(feature = "Foundation_NSCalendar")]
    #[objc2::method(sel = "setCalendar:")]
    pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

    #[cfg(feature = "Foundation_NSLocale")]
    #[objc2::method(sel = "locale", managed = "Other")]
    pub unsafe fn locale(&self) -> Id<NSLocale>;

    #[cfg(feature = "Foundation_NSLocale")]
    #[objc2::method(sel = "setLocale:")]
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

    #[cfg(all(
        feature = "Foundation_NSDateComponents",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(sel = "localizedStringFromDateComponents:", managed = "Other")]
    pub unsafe fn localizedStringFromDateComponents(
        &self,
        date_components: &NSDateComponents,
    ) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "localizedStringFromTimeInterval:", managed = "Other")]
    pub unsafe fn localizedStringFromTimeInterval(
        &self,
        time_interval: NSTimeInterval,
    ) -> Id<NSString>;

    #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "localizedStringForDate:relativeToDate:", managed = "Other")]
    pub unsafe fn localizedStringForDate_relativeToDate(
        &self,
        date: &NSDate,
        reference_date: &NSDate,
    ) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "stringForObjectValue:", managed = "Other")]
    pub unsafe fn stringForObjectValue(&self, obj: Option<&Object>) -> Option<Id<NSString>>;
}
