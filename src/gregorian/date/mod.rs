mod day;
mod errors;
mod month;
mod pattern;
mod styled_week_day;
mod week_day;
mod week_format;
mod year;

pub use self::pattern::*;
pub use self::week_day::*;
pub use self::week_format::*;
pub use errors::*;

use self::{day::Day, month::Month, styled_week_day::StyledWeekDay, year::Year};
use crate::GenericResult;
use crate::{chinese_vec, Chinese, ChineseFormat, EmptyPlaceholder, Variant};

/// Provides a configurable way to build [Date] instances.
///
/// The date can be incrementally set up by providing its individual
/// components via a chain of the `with_` methods.
///
/// Upon [build](Self::build), the builder ensures the validity and the
/// existence of the date - e.g.: 29th February only in leap years; however,
/// consistency is **not** checked for week days, which can therefore be arbitrary.
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let date = DateBuilder::new()
///     .with_year(1998)
///     .with_month(6)
///     .with_day(13)
///     .with_week_day(WeekDay::Saturday)
///     .with_week_format(WeekFormat::LiBai)
///     .with_formal(false)
///     .build()?;
///
/// assert_eq!(date.to_chinese(Variant::Simplified), Chinese {
///     logograms: "一九九八年六月十三日礼拜六".to_string(),
///     omissible: false
/// });
///
/// assert_eq!(date.to_chinese(Variant::Traditional), Chinese {
///     logograms: "一九九八年六月十三日禮拜六".to_string(),
///     omissible: false
/// });
///
/// # Ok(())
/// # }
/// ```
///
/// The builder can also build single date components,
/// as well as a range of reasonable patterns:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let single_year = DateBuilder::new()
///     .with_year(1998)
///     .build()?;
/// assert_eq!(single_year.to_chinese(Variant::Simplified), "一九九八年");
/// assert_eq!(single_year.to_chinese(Variant::Traditional), "一九九八年");
///
/// let single_month = DateBuilder::new()
///     .with_month(4)
///     .build()?;
/// assert_eq!(single_month.to_chinese(Variant::Simplified), "四月");
/// assert_eq!(single_month.to_chinese(Variant::Traditional), "四月");
///
/// let single_day = DateBuilder::new()
///     .with_day(29)
///     .with_formal(true)
///     .build()?;
/// assert_eq!(single_day.to_chinese(Variant::Simplified), "二十九号");
/// assert_eq!(single_day.to_chinese(Variant::Traditional), "二十九號");
///
/// let single_week_day = DateBuilder::new()
///     .with_week_day(WeekDay::Sunday)
///     .with_week_format(WeekFormat::Zhou)
///     .build()?;
/// assert_eq!(single_week_day.to_chinese(Variant::Simplified), "周日");
/// assert_eq!(single_week_day.to_chinese(Variant::Traditional), "周日");
///
/// let year_month = DateBuilder::new()
///     .with_year(1996)
///     .with_month(2)
///     .build()?;
/// assert_eq!(year_month.to_chinese(Variant::Simplified), "一九九六年二月");
/// assert_eq!(year_month.to_chinese(Variant::Traditional), "一九九六年二月");
///  
/// let year_month_day = DateBuilder::new()
///     .with_year(2014)
///     .with_month(12)
///     .with_day(25)
///     .with_formal(false)
///     .build()?;
/// assert_eq!(year_month_day.to_chinese(Variant::Simplified), "二零一四年十二月二十五日");
/// assert_eq!(year_month_day.to_chinese(Variant::Traditional), "二零一四年十二月二十五日");
///  
/// let month_day = DateBuilder::new()
///     .with_month(4)
///     .with_day(29)
///     .with_formal(true)
///     .build()?;
/// assert_eq!(month_day.to_chinese(Variant::Simplified), "四月二十九号");
/// assert_eq!(month_day.to_chinese(Variant::Traditional), "四月二十九號");
///
/// let month_day_week_day = DateBuilder::new()
///     .with_month(10)
///     .with_day(17)
///     .with_formal(false)
///     .with_week_format(WeekFormat::XingQi)
///     .with_week_day(WeekDay::Monday)
///     .build()?;
/// assert_eq!(month_day_week_day.to_chinese(Variant::Simplified), "十月十七日星期一");
/// assert_eq!(month_day_week_day.to_chinese(Variant::Traditional), "十月十七日星期一");
///
/// let day_week_day = DateBuilder::new()
///     .with_day(16)
///     .with_formal(false)
///     .with_week_format(WeekFormat::Zhou)
///     .with_week_day(WeekDay::Tuesday)
///     .build()?;
/// assert_eq!(day_week_day.to_chinese(Variant::Simplified), "十六日周二");
/// assert_eq!(day_week_day.to_chinese(Variant::Traditional), "十六日周二");
///
/// # Ok(())
/// # }
/// ```
///
/// Invalid patterns (such as <year; week day>) and inexisting dates
/// are not allowed: in these cases, building a [Date] returns the
/// most suitable error.
///
/// ```
/// use chinese_format::{*, gregorian::*};
/// use dyn_error::*;
///
/// # fn main() -> GenericResult<()> {
/// let absurd_builder = DateBuilder::new()
///     .with_month(4)
///     .with_day(31);
/// assert_err_box!(absurd_builder.build(), InvalidDate {
///     year: None,
///     month: 4,
///     day: 31,
/// });
///  
/// let inexisting_builder = DateBuilder::new()
///     .with_year(2023)
///     .with_month(2)
///     .with_day(29);
/// assert_err_box!(inexisting_builder.build(), InvalidDate {
///     year: Some(2023),
///     month: 2,
///     day: 29,
/// });
///
/// let still_allowed_builder = DateBuilder::new()
///     .with_month(2)
///     .with_day(29);
/// assert!(still_allowed_builder.build().is_ok());
///
/// let day_out_of_range_builder = DateBuilder::new()
///     .with_year(2023)
///     .with_month(2)
///     .with_day(90);
/// assert_err_box!(day_out_of_range_builder.build(), DayOutOfRange(90));
///
/// let month_out_of_range_builder = DateBuilder::new()
///     .with_year(2023)
///     .with_month(67)
///     .with_day(9);
/// assert_err_box!(month_out_of_range_builder.build(), MonthOutOfRange(67));
///
/// let invalid_pattern_builder = DateBuilder::new()
///     .with_year(2023)
///     .with_day(9);
/// assert_err_box!(invalid_pattern_builder.build(), InvalidDatePattern("yd".to_string()));
///
/// # Ok(())
/// # }
/// ```
///
/// As an exception, the consistency of week day is not ensured:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let one_day_as_monday = DateBuilder::new()
///     .with_month(5)
///     .with_day(13)
///     .with_formal(false)
///     .with_week_format(WeekFormat::Zhou)
///     .with_week_day(WeekDay::Tuesday)
///     .build();
/// assert!(one_day_as_monday.is_ok());
///
/// let same_day_week_as_saturday = DateBuilder::new()
///     .with_month(5)
///     .with_day(13)
///     .with_formal(false)
///     .with_week_format(WeekFormat::Zhou)
///     .with_week_day(WeekDay::Saturday)
///     .build();
/// assert!(same_day_week_as_saturday.is_ok());
///
/// # Ok(())
/// # }
/// ```
pub struct DateBuilder {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
    week_day: Option<WeekDay>,
    formal: bool,
    week_format: WeekFormat,
}

impl DateBuilder {
    /// Creates the default instance of the builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the year - a positive value.
    pub fn with_year(mut self, year: u16) -> Self {
        self.year = Some(year);
        self
    }

    /// Sets the month - between 1 and 12.
    pub fn with_month(mut self, month: u8) -> Self {
        self.month = Some(month);
        self
    }

    /// Sets the day - between 1 and 31.
    pub fn with_day(mut self, day: u8) -> Self {
        self.day = Some(day);
        self
    }

    /// Sets the week day.
    pub fn with_week_day(mut self, week_day: WeekDay) -> Self {
        self.week_day = Some(week_day);
        self
    }

    /// Sets whether the register is formal.
    pub fn with_formal(mut self, formal: bool) -> Self {
        self.formal = formal;
        self
    }

    /// Sets the word used to express a week.
    pub fn with_week_format(mut self, week_format: WeekFormat) -> Self {
        self.week_format = week_format;
        self
    }

    fn validate_consistency(&self, year: Option<&Year>) -> Result<(), InvalidDate> {
        let is_leap_year = year.map(Year::is_leap).unwrap_or(true);

        if let Some(month_ordinal) = self.month {
            if let Some(day_ordinal) = self.day {
                let day_is_valid = match month_ordinal {
                    4 | 6 | 9 | 11 => day_ordinal <= 30,
                    2 => {
                        let max_day = if is_leap_year { 29 } else { 28 };
                        day_ordinal <= max_day
                    }
                    _ => true,
                };

                if !day_is_valid {
                    return Err(InvalidDate {
                        year: self.year,
                        month: month_ordinal,
                        day: day_ordinal,
                    });
                }
            }
        }

        Ok(())
    }

    /// Creates a [Date] instance based on the current parameters,
    /// after performing validation.
    pub fn build(&self) -> GenericResult<Date> {
        DatePattern::validate(DatePatternFlags {
            year: self.year.is_some(),
            month: self.month.is_some(),
            day: self.day.is_some(),
            week_day: self.week_day.is_some(),
        })?;

        let year: Option<Year> = self.year.map(|year| year.into());

        let month: Option<Month> = self
            .month
            .map(|month_ordinal| month_ordinal.try_into())
            .transpose()?;

        let day: Option<Day> = self
            .day
            .map(|day_ordinal| {
                if self.formal {
                    Day::try_new_formal(day_ordinal)
                } else {
                    Day::try_new_informal(day_ordinal)
                }
            })
            .transpose()?;

        self.validate_consistency(year.as_ref())?;

        let week_day = self.week_day.map(|week_day| StyledWeekDay {
            week_format: self.week_format,
            week_day,
        });

        Ok(Date {
            year,
            month,
            day,
            week_day,
        })
    }
}

/// The default instance for [DateBuilder].
impl Default for DateBuilder {
    fn default() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            week_day: None,
            formal: true,
            week_format: WeekFormat::default(),
        }
    }
}

/// Naïve date based on the Gregorian calendar.
///
/// It can be built using the related [DateBuilder], which also
/// ensures its consistency and existence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: Option<Year>,
    month: Option<Month>,
    day: Option<Day>,
    week_day: Option<StyledWeekDay>,
}

impl ChineseFormat for Date {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        chinese_vec!(
            variant,
            [
                EmptyPlaceholder::new(&self.year),
                EmptyPlaceholder::new(&self.month),
                EmptyPlaceholder::new(&self.day),
                EmptyPlaceholder::new(&self.week_day)
            ]
        )
        .trim_end()
        .collect()
    }
}
