use std::{error::Error, fmt::Display};

/// Error for when the *month* part of a date is out of range.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     MonthOutOfRange(90).to_string(),
///     "Month out of range: 90"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonthOutOfRange(pub u8);

impl Display for MonthOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Month out of range: {}", self.0)
    }
}

impl Error for MonthOutOfRange {}

/// Error for when the *day* part of a date is out of range.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     DayOutOfRange(91).to_string(),
///     "Day out of range: 91"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DayOutOfRange(pub u8);

impl Display for DayOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day out of range: {}", self.0)
    }
}

impl Error for DayOutOfRange {}

/// Error for when the *week day* part of a date is out of range.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     WeekDayOutOfRange(92).to_string(),
///     "Week day out of range: 92"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeekDayOutOfRange(pub u8);

impl Display for WeekDayOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Week day out of range: {}", self.0)
    }
}

impl Error for WeekDayOutOfRange {}

/// Error for when a date cannot exist in reality - such as `2009-02-31`.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     InvalidDate {
///         year: None,
///         month: 2,
///         day: 31
///     }.to_string(),
///     "Invalid date: 2-31"
/// );
///
/// assert_eq!(
///     InvalidDate {
///         year: Some(1986),
///         month: 2,
///         day: 31
///     }.to_string(),
///     "Invalid date: 1986-2-31"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidDate {
    pub year: Option<u16>,
    pub month: u8,
    pub day: u8,
}

impl Display for InvalidDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.year {
            Some(year) => write!(f, "Invalid date: {}-{}-{}", year, self.month, self.day),

            None => write!(f, "Invalid date: {}-{}", self.month, self.day),
        }
    }
}

impl Error for InvalidDate {}
