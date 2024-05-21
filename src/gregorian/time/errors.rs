use std::{error::Error, fmt::Display};

/// Error for when the *hour* part of a time expression is out of range.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     HourOutOfRange(90).to_string(),
///     "Hour out of range: 90"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HourOutOfRange(pub u8);

impl Display for HourOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hour out of range: {}", self.0)
    }
}

impl Error for HourOutOfRange {}

/// Error for when the *minute* part of a time expression is out of range.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     MinuteOutOfRange(90).to_string(),
///     "Minute out of range: 90"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinuteOutOfRange(pub u8);

impl Display for MinuteOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Minute out of range: {}", self.0)
    }
}

impl Error for MinuteOutOfRange {}

/// Error for when the *second* part of a time expression is out of range.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     SecondOutOfRange(90).to_string(),
///     "Second out of range: 90"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecondOutOfRange(pub u8);

impl Display for SecondOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Second out of range: {}", self.0)
    }
}

impl Error for SecondOutOfRange {}
