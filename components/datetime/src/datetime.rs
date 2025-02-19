// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`TypedDateTimeFormatter`].

use crate::{
    options::{length, preferences, DateTimeFormatterOptions},
    provider::calendar::{TimeLengthsV1Marker, TimeSymbolsV1Marker},
    provider::date_time::PatternSelector,
    provider::week_data::WeekDataV1Marker,
    raw,
};
use alloc::string::String;
use core::marker::PhantomData;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{
    calendar, input::DateInput, input::DateTimeInput, input::IsoTimeInput, CldrCalendar,
    DateTimeFormatterError, FormattedDateTime,
};

#[cfg(feature = "experimental")]
use crate::options::components;

/// [`TimeFormatter`] is a structure of the [`icu_datetime`] component that provides time formatting only.
/// When constructed, it uses data from the [data provider], selected locale and provided preferences to
/// collect all data necessary to format any time into that locale.
///
/// For that reason, one should think of the process of formatting a time in two steps - first, a computational
/// heavy construction of [`TimeFormatter`], and then fast formatting of [`DateTimeInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`TypedDateTimeFormatter`]: crate::datetime::TimeFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::DateTime;
/// use icu::datetime::{options::length::Time, TimeFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let provider = icu_testdata::get_provider();
///
/// let tf = TimeFormatter::try_new_with_buffer_provider(
///     &provider,
///     &locale!("en").into(),
///     Time::Short,
/// )
/// .expect("Failed to create TimeFormatter instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_writeable_eq!(tf.format(&datetime), "12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct TimeFormatter(pub(super) raw::TimeFormatter);

impl TimeFormatter {
    /// Constructor that takes a selected locale, reference to a [data provider] and
    /// a list of preferences, then collects all data necessary to format date and time values into the given locale,
    /// using the short style.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// TimeFormatter::try_new_with_buffer_provider(
    ///     &provider,
    ///     &locale!("en").into(),
    ///     Time::Short,
    /// )
    /// .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<TimeLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let preferences = Some(preferences::Bag::from_data_locale(locale));

        Ok(Self(raw::TimeFormatter::try_new(
            data_provider,
            locale,
            length,
            preferences,
        )?))
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        length: length::Time,
        error: DateTimeFormatterError
    );

    /// Takes a [`IsoTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// use writeable::assert_writeable_eq;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormatter::try_new_with_buffer_provider(&provider, &locale.into(), Time::Short)
    ///         .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(tf.format(&datetime), "12:34 PM");
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: IsoTimeInput,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`IsoTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormatter::try_new_with_buffer_provider(&provider, &locale.into(), Time::Short)
    ///         .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// tf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// assert_eq!(buffer, "12:34 PM");
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl IsoTimeInput,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`IsoTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::length::Time, TimeFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let tf =
    ///     TimeFormatter::try_new_with_buffer_provider(&provider, &locale.into(), Time::Short)
    ///         .expect("Failed to create TimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(tf.format_to_string(&datetime), "12:34 PM");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl IsoTimeInput) -> String {
        self.0.format_to_string(value)
    }
}

/// [`TypedDateFormatter`] is a formatter capable of formatting
/// dates from a calendar selected at compile time. For the difference between this
/// and [`DateFormatter`](crate::DateFormatter), please read the [crate root docs][crate].
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`TypedDateFormatter`], and then fast formatting of [`DateInput`] data using the instance.
///
/// [`icu_datetime`]: crate
///
/// # Examples
///
/// ```
/// use icu::calendar::{Date, Gregorian};
/// use icu::datetime::{options::length, TypedDateFormatter};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let df = TypedDateFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale!("en").into(), length::Date::Full)
///     .expect("Failed to create TypedDateFormatter instance.");
///
/// let date = Date::new_gregorian_date(2020, 9, 1)
///     .expect("Failed to construct Date.");
///
/// assert_eq!(df.format_to_string(&date), "Tuesday, September 1, 2020");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct TypedDateFormatter<C>(pub(super) raw::DateFormatter, PhantomData<C>);

impl<C: CldrCalendar> TypedDateFormatter<C> {
    /// Constructor that takes a selected locale, reference to a [data provider] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// TypedDateFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale!("en").into(), length::Date::Full)
    ///     .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_new_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        calendar::check_locale::<C>(locale)?;
        Ok(Self(
            raw::DateFormatter::try_new(
                data_provider,
                calendar::load_lengths_for_cldr_calendar::<C, _>(data_provider, locale)?,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(data_provider, locale),
                locale,
                length,
            )?,
            PhantomData,
        ))
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        length: length::Date,
        error: DateTimeFormatterError
    );

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// use writeable::assert_writeable_eq;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale.into(), length::Date::Full)
    ///     .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let date = Date::new_gregorian_date(2020, 9, 1)
    ///     .expect("Failed to construct Date.");
    ///
    /// assert_writeable_eq!(df.format(&date), "Tuesday, September 1, 2020");
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale.into(), length::Date::Short)
    ///     .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let date = Date::new_gregorian_date(2020, 9, 1)
    ///     .expect("Failed to construct Date.");
    ///
    /// let mut buffer = String::new();
    /// df.format_to_write(&mut buffer, &date)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// assert_eq!(buffer, "9/1/20");
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateInput<Calendar = C>,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    /// use icu::datetime::{options::length, TypedDateFormatter};
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale.into(), length::Date::Short)
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let date = Date::new_gregorian_date(2020, 9, 1)
    ///     .expect("Failed to construct Date.");
    ///
    /// assert_eq!(df.format_to_string(&date), "9/1/20");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }
}

/// [`TypedDateTimeFormatter`] is a formatter capable of formatting
/// date/times from a calendar selected at compile time. For the difference between this
///  and [`DateTimeFormatter`](crate::DateTimeFormatter), please read the [crate root docs][crate].
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`TypedDateTimeFormatter`], and then fast formatting of [`DateInput`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`TypedDateTimeFormatter`]: crate::datetime::TypedDateTimeFormatter
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::{options::length, TypedDateTimeFormatter};
/// use icu::locid::locale;
///
/// let provider = icu_testdata::get_provider();
///
/// let mut options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
///
/// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(
///     &provider,
///     &locale!("en").into(),
///     options.into(),
/// )
/// .expect("Failed to create TypedDateTimeFormatter instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// assert_eq!(dtf.format_to_string(&datetime), "Sep 1, 2020, 12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct TypedDateTimeFormatter<C>(pub(super) raw::DateTimeFormatter, PhantomData<C>);

impl<C: CldrCalendar> TypedDateTimeFormatter<C> {
    /// Constructor that takes a [`TimeFormatter`] and [`TypedDateFormatter`] and combines them into a [`TypedDateTimeFormatter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::length, TypedDateFormatter, TypedDateTimeFormatter, TimeFormatter,
    /// };
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let tf = TimeFormatter::try_new_with_buffer_provider(
    ///     &provider,
    ///     &locale!("en").into(),
    ///     length::Time::Short,
    /// )
    /// .expect("Failed to create TimeFormatter instance.");
    /// let df = TypedDateFormatter::<Gregorian>::try_new_with_buffer_provider(
    ///     &provider,
    ///     &locale!("en").into(),
    ///     length::Date::Short,
    /// )
    /// .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// TypedDateTimeFormatter::<Gregorian>::try_from_date_and_time(df, tf).unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_from_date_and_time(
        date: TypedDateFormatter<C>,
        time: TimeFormatter,
    ) -> Result<Self, DateTimeFormatterError>
where {
        Ok(Self(
            raw::DateTimeFormatter::try_from_date_and_time(date.0, time.0)?,
            PhantomData,
        ))
    }

    /// Constructor that takes a selected locale, reference to a [data provider] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{options::length, TypedDateTimeFormatter};
    /// use icu::locid::locale;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let options = length::Bag::from_time_style(length::Time::Medium);
    ///
    /// TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(
    ///     &provider,
    ///     &locale!("en").into(),
    ///     options.into(),
    /// )
    /// .unwrap();
    /// ```
    ///
    /// [data provider]: icu_provider
    #[cfg(feature = "experimental")]
    #[inline]
    pub fn try_new_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<crate::provider::calendar::DateSkeletonPatternsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        calendar::check_locale::<C>(locale)?;
        let patterns = PatternSelector::for_options(
            data_provider,
            calendar::load_lengths_for_cldr_calendar::<C, _>(data_provider, locale)?,
            locale,
            &C::DEFAULT_BCP_47_IDENTIFIER,
            &options,
        )?;
        Ok(Self(
            raw::DateTimeFormatter::try_new(
                data_provider,
                patterns,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(data_provider, locale),
                locale,
            )?,
            PhantomData,
        ))
    }

    #[allow(missing_docs)] // The docs use the "experimental" version
    #[cfg(not(feature = "experimental"))]
    #[inline]
    pub fn try_new_unstable<D>(
        data_provider: &D,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker>
            + DataProvider<<C as CldrCalendar>::DateLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        calendar::check_locale::<C>(locale)?;
        let patterns = PatternSelector::for_options(
            data_provider,
            calendar::load_lengths_for_cldr_calendar::<C, _>(data_provider, locale)?,
            locale,
            &options,
        )?;
        Ok(Self(
            raw::DateTimeFormatter::try_new(
                data_provider,
                patterns,
                || calendar::load_symbols_for_cldr_calendar::<C, _>(data_provider, locale),
                locale,
            )?,
            PhantomData,
        ))
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: include,
        options: DateTimeFormatterOptions,
        error: DateTimeFormatterError
    );

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::TypedDateTimeFormatter;
    /// use writeable::assert_writeable_eq;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale.into(), options.into())
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_writeable_eq!(dtf.format(&datetime), "12:34:28 PM");
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but [`FormattedDateTime`] will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput<Calendar = C>,
    {
        self.0.format(value)
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::TypedDateTimeFormatter;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale.into(), options.into())
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// dtf.format_to_write(&mut buffer, &datetime)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// assert_eq!(buffer, "12:34:28 PM");
    /// ```
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = C>,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{DateTime, Gregorian};
    /// use icu::datetime::TypedDateTimeFormatter;
    /// # let locale = icu::locid::locale!("en");
    /// # let provider = icu_testdata::get_provider();
    /// # let options = icu::datetime::options::length::Bag::from_time_style(icu::datetime::options::length::Time::Medium);
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(&provider, &locale.into(), options.into())
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// assert_eq!(dtf.format_to_string(&datetime), "12:34:28 PM");
    /// ```
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = C>) -> String {
        self.0.format_to_string(value)
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`TypedDateTimeFormatter`]. The developer may request
    /// a certain set of options for a [`TypedDateTimeFormatter`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     TypedDateTimeFormatter, DateTimeFormatterOptions,
    /// };
    /// use icu::locid::locale;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    ///
    /// let provider = icu_testdata::get_provider();
    /// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(
    ///     &provider,
    ///     &locale!("en").into(),
    ///     options,
    /// )
    /// .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let mut expected_components_bag = components::Bag::default();
    /// expected_components_bag.year = Some(components::Year::Numeric);
    /// expected_components_bag.month = Some(components::Month::Short);
    /// expected_components_bag.day = Some(components::Day::NumericDayOfMonth);
    ///
    /// assert_eq!(dtf.resolve_components(), expected_components_bag);
    /// ```
    #[cfg(feature = "experimental")]
    pub fn resolve_components(&self) -> components::Bag {
        self.0.resolve_components()
    }
}
