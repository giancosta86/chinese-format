use dyn_error::{declare_dyn_result, impl_err_equality};
use std::{error::Error, fmt::Display};

declare_dyn_result!(pub, GenericResult);

/// Custom version of [Result], based on [CrateError].
pub type CrateResult<T> = Result<T, CrateError>;

/// Custom error scenarios related to this [crate].
///
/// ```
/// use chinese_format::*;
///
/// assert_eq!(CrateError::ZeroDenominator.to_string(), "Zero passed as denominator");
///
/// assert_eq!(CrateError::DimesOutOfRange(200).to_string(), "Dimes out of range: 200");
///
/// assert_eq!(CrateError::CentsOutOfRange(200).to_string(), "Cents out of range: 200");
///
/// assert_eq!(CrateError::MonthOutOfRange(90).to_string(), "Month out of range: 90");
///
/// assert_eq!(CrateError::DayOutOfRange(90).to_string(), "Day out of range: 90");
///
/// assert_eq!(CrateError::InvalidDatePattern("dw".to_string()).to_string(), "Invalid date pattern: dw");
///
/// assert_eq!(
///     CrateError::InvalidDate {
///         year: None,
///         month: 2,
///         day: 31
///     }.to_string(),
///     "Invalid date: 2-31"
/// );
///
/// assert_eq!(
///     CrateError::InvalidDate {
///         year: Some(1986),
///         month: 2,
///         day: 31
///     }.to_string(),
///     "Invalid date: 1986-2-31"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CrateError {
    ZeroDenominator,
    DimesOutOfRange(u8),
    CentsOutOfRange(u8),
    MonthOutOfRange(u8),
    DayOutOfRange(u8),
    InvalidDatePattern(String),
    InvalidDate {
        year: Option<u16>,
        month: u8,
        day: u8,
    },
}

impl_err_equality!(CrateError);

impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDenominator => write!(f, "Zero passed as denominator"),

            Self::DimesOutOfRange(dimes) => write!(f, "Dimes out of range: {}", dimes),

            Self::CentsOutOfRange(cents) => write!(f, "Cents out of range: {}", cents),

            Self::MonthOutOfRange(month) => write!(f, "Month out of range: {}", month),

            Self::DayOutOfRange(day) => write!(f, "Day out of range: {}", day),

            Self::InvalidDatePattern(pattern) => {
                write!(f, "Invalid date pattern: {}", pattern)
            }

            Self::InvalidDate { year, month, day } => match year {
                Some(year) => write!(f, "Invalid date: {}-{}-{}", year, month, day),

                None => write!(f, "Invalid date: {}-{}", month, day),
            },
        }
    }
}

impl Error for CrateError {}
