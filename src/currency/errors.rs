use std::{error::Error, fmt::Display};

/// Error for when the *dimes* of a currency value are out of range.
///
/// ```
/// use chinese_format::currency::*;
///
/// assert_eq!(
///     DimesOutOfRange(200).to_string(),
///     "Dimes out of range: 200"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DimesOutOfRange(pub u8);

impl Display for DimesOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dimes out of range: {}", self.0)
    }
}

impl Error for DimesOutOfRange {}

/// Error for when the *cents* of a currency value are out of range.
///
/// ```
/// use chinese_format::currency::*;
///
/// assert_eq!(
///     CentsOutOfRange(200).to_string(),
///     "Cents out of range: 200"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CentsOutOfRange(pub u8);

impl Display for CentsOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cents out of range: {}", self.0)
    }
}

impl Error for CentsOutOfRange {}
