mod flags;

use crate::{CrateError, CrateResult};
use lazy_static::lazy_static;
use std::{collections::HashSet, fmt::Display};

pub use flags::*;

lazy_static! {
    static ref ALL_DATE_PATTERN_STRINGS: HashSet<String> = HashSet::from(
        [
            DatePattern::Year,
            DatePattern::Month,
            DatePattern::Day,
            DatePattern::WeekDay,
            DatePattern::YearMonth,
            DatePattern::YearMonthDay,
            DatePattern::MonthDay,
            DatePattern::MonthDayWeekDay,
            DatePattern::DayWeekDay,
            DatePattern::YearMonthDayWeekDay,
        ]
        .map(|pattern| pattern.to_string())
    );
}

/// Any of the acceptable [Date](super::Date) patterns.
pub enum DatePattern {
    Year,
    Month,
    Day,
    WeekDay,
    YearMonth,
    YearMonthDay,
    MonthDay,
    MonthDayWeekDay,
    DayWeekDay,
    YearMonthDayWeekDay,
}

impl DatePattern {
    /// If the given component flags define a valid [DatePattern],
    /// just returns [Ok]; otherwise returns [CrateError::InvalidDatePattern].
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// assert_eq!(
    ///     DatePattern::validate(
    ///         DatePatternFlags {
    ///             year: false,
    ///             month: false,
    ///             day: false,
    ///             week_day: false
    ///         }
    ///     ),
    ///     CrateError::InvalidDatePattern("".to_string())
    /// );
    ///
    /// assert_eq!(
    ///     DatePattern::validate(
    ///         DatePatternFlags {
    ///             year: false,
    ///             month: true,
    ///             day: false,
    ///             week_day: true
    ///         }
    ///     ),
    ///     CrateError::InvalidDatePattern("mw".to_string())
    /// );
    ///
    /// assert_eq!(
    ///     DatePattern::validate(
    ///         DatePatternFlags {
    ///             year: false,
    ///             month: false,
    ///             day: true,
    ///             week_day: false
    ///         }
    ///     ),
    ///     Ok(())
    /// );
    ///
    /// assert_eq!(
    ///     DatePattern::validate(
    ///         DatePatternFlags {
    ///             year: true,
    ///             month: true,
    ///             day: true,
    ///             week_day: true
    ///         }
    ///     ),
    ///     Ok(())
    /// );
    /// ```
    pub fn validate(flags: DatePatternFlags) -> CrateResult<()> {
        let flags_pattern = flags.to_string();

        if ALL_DATE_PATTERN_STRINGS.contains(flags_pattern.as_str()) {
            Ok(())
        } else {
            Err(CrateError::InvalidDatePattern(flags_pattern))
        }
    }

    /// Tells whether the current [DatePattern] has a *year* component:
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// assert!(DatePattern::YearMonth.has_year());
    /// assert!(!DatePattern::Day.has_year());
    /// ```
    pub fn has_year(&self) -> bool {
        matches!(
            self,
            Self::Year | Self::YearMonth | Self::YearMonthDay | Self::YearMonthDayWeekDay
        )
    }

    /// Tells whether the current [DatePattern] has a *month* component:
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// assert!(DatePattern::YearMonth.has_month());
    /// assert!(!DatePattern::Day.has_month());
    /// ```
    pub fn has_month(&self) -> bool {
        matches!(
            self,
            Self::Month
                | Self::YearMonth
                | Self::YearMonthDay
                | Self::MonthDay
                | Self::MonthDayWeekDay
                | Self::YearMonthDayWeekDay
        )
    }

    /// Tells whether the current [DatePattern] has a *day* component:
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// assert!(DatePattern::MonthDay.has_day());
    /// assert!(!DatePattern::Year.has_day());
    /// ```
    pub fn has_day(&self) -> bool {
        matches!(
            self,
            Self::Day
                | Self::YearMonthDay
                | Self::MonthDay
                | Self::MonthDayWeekDay
                | Self::DayWeekDay
                | Self::YearMonthDayWeekDay
        )
    }

    /// Tells whether the current [DatePattern] has a *week day* component:
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// assert!(DatePattern::MonthDayWeekDay.has_week_day());
    /// assert!(!DatePattern::Year.has_week_day());
    /// ```
    pub fn has_week_day(&self) -> bool {
        matches!(
            self,
            Self::WeekDay | Self::MonthDayWeekDay | Self::DayWeekDay | Self::YearMonthDayWeekDay
        )
    }
}

/// Every date pattern has a [String] representation,
/// consisting of cumulated single characters - one for each
/// enabled date component:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
///
/// assert_eq!(DatePattern::Year.to_string(), "y");
/// assert_eq!(DatePattern::Month.to_string(), "m");
/// assert_eq!(DatePattern::Day.to_string(), "d");
/// assert_eq!(DatePattern::WeekDay.to_string(), "w");
///
/// assert_eq!(DatePattern::YearMonth.to_string(), "ym");
///
/// assert_eq!(DatePattern::YearMonthDay.to_string(), "ymd");
///
/// assert_eq!(DatePattern::YearMonthDayWeekDay.to_string(), "ymdw");
///
/// # Ok(())
/// # }
/// ```
impl Display for DatePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            DatePatternFlags {
                year: self.has_year(),
                month: self.has_month(),
                day: self.has_day(),
                week_day: self.has_week_day()
            }
        )
    }
}
