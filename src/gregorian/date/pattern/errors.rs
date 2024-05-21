use std::{error::Error, fmt::Display};

/// Error for when a date pattern is not supported by the library.
///
/// ```
/// use chinese_format::gregorian::*;
///
/// assert_eq!(
///     InvalidDatePattern("dw".to_string()).to_string(),
///     "Invalid date pattern: dw"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidDatePattern(pub String);

impl Display for InvalidDatePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid date pattern: {}", self.0)
    }
}

impl Error for InvalidDatePattern {}
