//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSNumberFormatterBehavior {
    NSNumberFormatterBehaviorDefault = 0,
    NSNumberFormatterBehavior10_0 = 1000,
    NSNumberFormatterBehavior10_4 = 1040,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSNumberFormatterStyle {
    NSNumberFormatterNoStyle = 0,
    NSNumberFormatterDecimalStyle = 1,
    NSNumberFormatterCurrencyStyle = 2,
    NSNumberFormatterPercentStyle = 3,
    NSNumberFormatterScientificStyle = 4,
    NSNumberFormatterSpellOutStyle = 5,
    NSNumberFormatterOrdinalStyle = 6,
    NSNumberFormatterCurrencyISOCodeStyle = 8,
    NSNumberFormatterCurrencyPluralStyle = 9,
    NSNumberFormatterCurrencyAccountingStyle = 10,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSNumberFormatterPadPosition {
    NSNumberFormatterPadBeforePrefix = 0,
    NSNumberFormatterPadAfterPrefix = 1,
    NSNumberFormatterPadBeforeSuffix = 2,
    NSNumberFormatterPadAfterSuffix = 3,
}

#[ns_enum]
#[underlying(NSUInteger)]
pub enum NSNumberFormatterRoundingMode {
    NSNumberFormatterRoundCeiling = 0,
    NSNumberFormatterRoundFloor = 1,
    NSNumberFormatterRoundDown = 2,
    NSNumberFormatterRoundUp = 3,
    NSNumberFormatterRoundHalfEven = 4,
    NSNumberFormatterRoundHalfDown = 5,
    NSNumberFormatterRoundHalfUp = 6,
}

#[objc2::interface(
    unsafe super = NSFormatter,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSNumberFormatter;
}

#[cfg(feature = "Foundation_NSNumberFormatter")]
unsafe impl NSCoding for NSNumberFormatter {}

#[cfg(feature = "Foundation_NSNumberFormatter")]
unsafe impl NSObjectProtocol for NSNumberFormatter {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    pub type NSNumberFormatter;

    #[objc2::method(sel = "formattingContext")]
    pub unsafe fn formattingContext(&self) -> NSFormattingContext;

    #[objc2::method(sel = "setFormattingContext:")]
    pub unsafe fn setFormattingContext(&self, formatting_context: NSFormattingContext);

    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "getObjectValue:forString:range:error:", throws)]
    pub unsafe fn getObjectValue_forString_range_error(
        &self,
        obj: Option<&mut Option<Id<Object>>>,
        string: &NSString,
        rangep: *mut NSRange,
    ) -> Result<(), Id<NSError>>;

    #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "stringFromNumber:", managed = "Other")]
    pub unsafe fn stringFromNumber(&self, number: &NSNumber) -> Option<Id<NSString>>;

    #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "numberFromString:", managed = "Other")]
    pub unsafe fn numberFromString(&self, string: &NSString) -> Option<Id<NSNumber>>;

    #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "localizedStringFromNumber:numberStyle:", managed = "Other")]
    pub unsafe fn localizedStringFromNumber_numberStyle(
        num: &NSNumber,
        nstyle: NSNumberFormatterStyle,
    ) -> Id<NSString>;

    #[objc2::method(sel = "defaultFormatterBehavior")]
    pub unsafe fn defaultFormatterBehavior() -> NSNumberFormatterBehavior;

    #[objc2::method(sel = "setDefaultFormatterBehavior:")]
    pub unsafe fn setDefaultFormatterBehavior(behavior: NSNumberFormatterBehavior);

    #[objc2::method(sel = "numberStyle")]
    pub unsafe fn numberStyle(&self) -> NSNumberFormatterStyle;

    #[objc2::method(sel = "setNumberStyle:")]
    pub unsafe fn setNumberStyle(&self, number_style: NSNumberFormatterStyle);

    #[cfg(feature = "Foundation_NSLocale")]
    #[objc2::method(sel = "locale", managed = "Other")]
    pub unsafe fn locale(&self) -> Id<NSLocale>;

    #[cfg(feature = "Foundation_NSLocale")]
    #[objc2::method(sel = "setLocale:")]
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

    #[objc2::method(sel = "generatesDecimalNumbers")]
    pub unsafe fn generatesDecimalNumbers(&self) -> bool;

    #[objc2::method(sel = "setGeneratesDecimalNumbers:")]
    pub unsafe fn setGeneratesDecimalNumbers(&self, generates_decimal_numbers: bool);

    #[objc2::method(sel = "formatterBehavior")]
    pub unsafe fn formatterBehavior(&self) -> NSNumberFormatterBehavior;

    #[objc2::method(sel = "setFormatterBehavior:")]
    pub unsafe fn setFormatterBehavior(&self, formatter_behavior: NSNumberFormatterBehavior);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "negativeFormat", managed = "Other")]
    pub unsafe fn negativeFormat(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNegativeFormat:")]
    pub unsafe fn setNegativeFormat(&self, negative_format: Option<&NSString>);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForNegativeValues", managed = "Other")]
    pub unsafe fn textAttributesForNegativeValues(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForNegativeValues:")]
    pub unsafe fn setTextAttributesForNegativeValues(
        &self,
        text_attributes_for_negative_values: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "positiveFormat", managed = "Other")]
    pub unsafe fn positiveFormat(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPositiveFormat:")]
    pub unsafe fn setPositiveFormat(&self, positive_format: Option<&NSString>);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForPositiveValues", managed = "Other")]
    pub unsafe fn textAttributesForPositiveValues(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForPositiveValues:")]
    pub unsafe fn setTextAttributesForPositiveValues(
        &self,
        text_attributes_for_positive_values: Option<&NSDictionary<NSString, Object>>,
    );

    #[objc2::method(sel = "allowsFloats")]
    pub unsafe fn allowsFloats(&self) -> bool;

    #[objc2::method(sel = "setAllowsFloats:")]
    pub unsafe fn setAllowsFloats(&self, allows_floats: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "decimalSeparator", managed = "Other")]
    pub unsafe fn decimalSeparator(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setDecimalSeparator:")]
    pub unsafe fn setDecimalSeparator(&self, decimal_separator: Option<&NSString>);

    #[objc2::method(sel = "alwaysShowsDecimalSeparator")]
    pub unsafe fn alwaysShowsDecimalSeparator(&self) -> bool;

    #[objc2::method(sel = "setAlwaysShowsDecimalSeparator:")]
    pub unsafe fn setAlwaysShowsDecimalSeparator(&self, always_shows_decimal_separator: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "currencyDecimalSeparator", managed = "Other")]
    pub unsafe fn currencyDecimalSeparator(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCurrencyDecimalSeparator:")]
    pub unsafe fn setCurrencyDecimalSeparator(&self, currency_decimal_separator: Option<&NSString>);

    #[objc2::method(sel = "usesGroupingSeparator")]
    pub unsafe fn usesGroupingSeparator(&self) -> bool;

    #[objc2::method(sel = "setUsesGroupingSeparator:")]
    pub unsafe fn setUsesGroupingSeparator(&self, uses_grouping_separator: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "groupingSeparator", managed = "Other")]
    pub unsafe fn groupingSeparator(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setGroupingSeparator:")]
    pub unsafe fn setGroupingSeparator(&self, grouping_separator: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "zeroSymbol", managed = "Other")]
    pub unsafe fn zeroSymbol(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setZeroSymbol:")]
    pub unsafe fn setZeroSymbol(&self, zero_symbol: Option<&NSString>);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForZero", managed = "Other")]
    pub unsafe fn textAttributesForZero(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForZero:")]
    pub unsafe fn setTextAttributesForZero(
        &self,
        text_attributes_for_zero: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "nilSymbol", managed = "Other")]
    pub unsafe fn nilSymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNilSymbol:")]
    pub unsafe fn setNilSymbol(&self, nil_symbol: &NSString);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForNil", managed = "Other")]
    pub unsafe fn textAttributesForNil(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForNil:")]
    pub unsafe fn setTextAttributesForNil(
        &self,
        text_attributes_for_nil: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "notANumberSymbol", managed = "Other")]
    pub unsafe fn notANumberSymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNotANumberSymbol:")]
    pub unsafe fn setNotANumberSymbol(&self, not_a_number_symbol: Option<&NSString>);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForNotANumber", managed = "Other")]
    pub unsafe fn textAttributesForNotANumber(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForNotANumber:")]
    pub unsafe fn setTextAttributesForNotANumber(
        &self,
        text_attributes_for_not_a_number: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "positiveInfinitySymbol", managed = "Other")]
    pub unsafe fn positiveInfinitySymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPositiveInfinitySymbol:")]
    pub unsafe fn setPositiveInfinitySymbol(&self, positive_infinity_symbol: &NSString);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForPositiveInfinity", managed = "Other")]
    pub unsafe fn textAttributesForPositiveInfinity(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForPositiveInfinity:")]
    pub unsafe fn setTextAttributesForPositiveInfinity(
        &self,
        text_attributes_for_positive_infinity: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "negativeInfinitySymbol", managed = "Other")]
    pub unsafe fn negativeInfinitySymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNegativeInfinitySymbol:")]
    pub unsafe fn setNegativeInfinitySymbol(&self, negative_infinity_symbol: &NSString);

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "textAttributesForNegativeInfinity", managed = "Other")]
    pub unsafe fn textAttributesForNegativeInfinity(
        &self,
    ) -> Option<Id<NSDictionary<NSString, Object>>>;

    #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "setTextAttributesForNegativeInfinity:")]
    pub unsafe fn setTextAttributesForNegativeInfinity(
        &self,
        text_attributes_for_negative_infinity: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "positivePrefix", managed = "Other")]
    pub unsafe fn positivePrefix(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPositivePrefix:")]
    pub unsafe fn setPositivePrefix(&self, positive_prefix: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "positiveSuffix", managed = "Other")]
    pub unsafe fn positiveSuffix(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPositiveSuffix:")]
    pub unsafe fn setPositiveSuffix(&self, positive_suffix: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "negativePrefix", managed = "Other")]
    pub unsafe fn negativePrefix(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNegativePrefix:")]
    pub unsafe fn setNegativePrefix(&self, negative_prefix: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "negativeSuffix", managed = "Other")]
    pub unsafe fn negativeSuffix(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setNegativeSuffix:")]
    pub unsafe fn setNegativeSuffix(&self, negative_suffix: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "currencyCode", managed = "Other")]
    pub unsafe fn currencyCode(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCurrencyCode:")]
    pub unsafe fn setCurrencyCode(&self, currency_code: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "currencySymbol", managed = "Other")]
    pub unsafe fn currencySymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCurrencySymbol:")]
    pub unsafe fn setCurrencySymbol(&self, currency_symbol: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "internationalCurrencySymbol", managed = "Other")]
    pub unsafe fn internationalCurrencySymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setInternationalCurrencySymbol:")]
    pub unsafe fn setInternationalCurrencySymbol(
        &self,
        international_currency_symbol: Option<&NSString>,
    );

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "percentSymbol", managed = "Other")]
    pub unsafe fn percentSymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPercentSymbol:")]
    pub unsafe fn setPercentSymbol(&self, percent_symbol: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "perMillSymbol", managed = "Other")]
    pub unsafe fn perMillSymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPerMillSymbol:")]
    pub unsafe fn setPerMillSymbol(&self, per_mill_symbol: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "minusSign", managed = "Other")]
    pub unsafe fn minusSign(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setMinusSign:")]
    pub unsafe fn setMinusSign(&self, minus_sign: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "plusSign", managed = "Other")]
    pub unsafe fn plusSign(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPlusSign:")]
    pub unsafe fn setPlusSign(&self, plus_sign: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "exponentSymbol", managed = "Other")]
    pub unsafe fn exponentSymbol(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setExponentSymbol:")]
    pub unsafe fn setExponentSymbol(&self, exponent_symbol: Option<&NSString>);

    #[objc2::method(sel = "groupingSize")]
    pub unsafe fn groupingSize(&self) -> NSUInteger;

    #[objc2::method(sel = "setGroupingSize:")]
    pub unsafe fn setGroupingSize(&self, grouping_size: NSUInteger);

    #[objc2::method(sel = "secondaryGroupingSize")]
    pub unsafe fn secondaryGroupingSize(&self) -> NSUInteger;

    #[objc2::method(sel = "setSecondaryGroupingSize:")]
    pub unsafe fn setSecondaryGroupingSize(&self, secondary_grouping_size: NSUInteger);

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "multiplier", managed = "Other")]
    pub unsafe fn multiplier(&self) -> Option<Id<NSNumber>>;

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "setMultiplier:")]
    pub unsafe fn setMultiplier(&self, multiplier: Option<&NSNumber>);

    #[objc2::method(sel = "formatWidth")]
    pub unsafe fn formatWidth(&self) -> NSUInteger;

    #[objc2::method(sel = "setFormatWidth:")]
    pub unsafe fn setFormatWidth(&self, format_width: NSUInteger);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "paddingCharacter", managed = "Other")]
    pub unsafe fn paddingCharacter(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setPaddingCharacter:")]
    pub unsafe fn setPaddingCharacter(&self, padding_character: Option<&NSString>);

    #[objc2::method(sel = "paddingPosition")]
    pub unsafe fn paddingPosition(&self) -> NSNumberFormatterPadPosition;

    #[objc2::method(sel = "setPaddingPosition:")]
    pub unsafe fn setPaddingPosition(&self, padding_position: NSNumberFormatterPadPosition);

    #[objc2::method(sel = "roundingMode")]
    pub unsafe fn roundingMode(&self) -> NSNumberFormatterRoundingMode;

    #[objc2::method(sel = "setRoundingMode:")]
    pub unsafe fn setRoundingMode(&self, rounding_mode: NSNumberFormatterRoundingMode);

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "roundingIncrement", managed = "Other")]
    pub unsafe fn roundingIncrement(&self) -> Id<NSNumber>;

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "setRoundingIncrement:")]
    pub unsafe fn setRoundingIncrement(&self, rounding_increment: Option<&NSNumber>);

    #[objc2::method(sel = "minimumIntegerDigits")]
    pub unsafe fn minimumIntegerDigits(&self) -> NSUInteger;

    #[objc2::method(sel = "setMinimumIntegerDigits:")]
    pub unsafe fn setMinimumIntegerDigits(&self, minimum_integer_digits: NSUInteger);

    #[objc2::method(sel = "maximumIntegerDigits")]
    pub unsafe fn maximumIntegerDigits(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaximumIntegerDigits:")]
    pub unsafe fn setMaximumIntegerDigits(&self, maximum_integer_digits: NSUInteger);

    #[objc2::method(sel = "minimumFractionDigits")]
    pub unsafe fn minimumFractionDigits(&self) -> NSUInteger;

    #[objc2::method(sel = "setMinimumFractionDigits:")]
    pub unsafe fn setMinimumFractionDigits(&self, minimum_fraction_digits: NSUInteger);

    #[objc2::method(sel = "maximumFractionDigits")]
    pub unsafe fn maximumFractionDigits(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaximumFractionDigits:")]
    pub unsafe fn setMaximumFractionDigits(&self, maximum_fraction_digits: NSUInteger);

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "minimum", managed = "Other")]
    pub unsafe fn minimum(&self) -> Option<Id<NSNumber>>;

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "setMinimum:")]
    pub unsafe fn setMinimum(&self, minimum: Option<&NSNumber>);

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "maximum", managed = "Other")]
    pub unsafe fn maximum(&self) -> Option<Id<NSNumber>>;

    #[cfg(feature = "Foundation_NSNumber")]
    #[objc2::method(sel = "setMaximum:")]
    pub unsafe fn setMaximum(&self, maximum: Option<&NSNumber>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "currencyGroupingSeparator", managed = "Other")]
    pub unsafe fn currencyGroupingSeparator(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setCurrencyGroupingSeparator:")]
    pub unsafe fn setCurrencyGroupingSeparator(
        &self,
        currency_grouping_separator: Option<&NSString>,
    );

    #[objc2::method(sel = "isLenient")]
    pub unsafe fn isLenient(&self) -> bool;

    #[objc2::method(sel = "setLenient:")]
    pub unsafe fn setLenient(&self, lenient: bool);

    #[objc2::method(sel = "usesSignificantDigits")]
    pub unsafe fn usesSignificantDigits(&self) -> bool;

    #[objc2::method(sel = "setUsesSignificantDigits:")]
    pub unsafe fn setUsesSignificantDigits(&self, uses_significant_digits: bool);

    #[objc2::method(sel = "minimumSignificantDigits")]
    pub unsafe fn minimumSignificantDigits(&self) -> NSUInteger;

    #[objc2::method(sel = "setMinimumSignificantDigits:")]
    pub unsafe fn setMinimumSignificantDigits(&self, minimum_significant_digits: NSUInteger);

    #[objc2::method(sel = "maximumSignificantDigits")]
    pub unsafe fn maximumSignificantDigits(&self) -> NSUInteger;

    #[objc2::method(sel = "setMaximumSignificantDigits:")]
    pub unsafe fn setMaximumSignificantDigits(&self, maximum_significant_digits: NSUInteger);

    #[objc2::method(sel = "isPartialStringValidationEnabled")]
    pub unsafe fn isPartialStringValidationEnabled(&self) -> bool;

    #[objc2::method(sel = "setPartialStringValidationEnabled:")]
    pub unsafe fn setPartialStringValidationEnabled(&self, partial_string_validation_enabled: bool);
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNumberFormatter")]
    pub type NSNumberFormatter;

    #[objc2::method(sel = "hasThousandSeparators")]
    pub unsafe fn hasThousandSeparators(&self) -> bool;

    #[objc2::method(sel = "setHasThousandSeparators:")]
    pub unsafe fn setHasThousandSeparators(&self, has_thousand_separators: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "thousandSeparator", managed = "Other")]
    pub unsafe fn thousandSeparator(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setThousandSeparator:")]
    pub unsafe fn setThousandSeparator(&self, thousand_separator: Option<&NSString>);

    #[objc2::method(sel = "localizesFormat")]
    pub unsafe fn localizesFormat(&self) -> bool;

    #[objc2::method(sel = "setLocalizesFormat:")]
    pub unsafe fn setLocalizesFormat(&self, localizes_format: bool);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "format", managed = "Other")]
    pub unsafe fn format(&self) -> Id<NSString>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setFormat:")]
    pub unsafe fn setFormat(&self, format: &NSString);

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "attributedStringForZero", managed = "Other")]
    pub unsafe fn attributedStringForZero(&self) -> Id<NSAttributedString>;

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "setAttributedStringForZero:")]
    pub unsafe fn setAttributedStringForZero(
        &self,
        attributed_string_for_zero: &NSAttributedString,
    );

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "attributedStringForNil", managed = "Other")]
    pub unsafe fn attributedStringForNil(&self) -> Id<NSAttributedString>;

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "setAttributedStringForNil:")]
    pub unsafe fn setAttributedStringForNil(&self, attributed_string_for_nil: &NSAttributedString);

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "attributedStringForNotANumber", managed = "Other")]
    pub unsafe fn attributedStringForNotANumber(&self) -> Id<NSAttributedString>;

    #[cfg(feature = "Foundation_NSAttributedString")]
    #[objc2::method(sel = "setAttributedStringForNotANumber:")]
    pub unsafe fn setAttributedStringForNotANumber(
        &self,
        attributed_string_for_not_a_number: &NSAttributedString,
    );

    #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
    #[objc2::method(sel = "roundingBehavior", managed = "Other")]
    pub unsafe fn roundingBehavior(&self) -> Id<NSDecimalNumberHandler>;

    #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
    #[objc2::method(sel = "setRoundingBehavior:")]
    pub unsafe fn setRoundingBehavior(&self, rounding_behavior: &NSDecimalNumberHandler);
}
