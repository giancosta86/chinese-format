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
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CrateError {
    ZeroDenominator,
    DimesOutOfRange(u8),
    CentsOutOfRange(u8),
}

impl_err_equality!(CrateError);

impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDenominator => write!(f, "Zero passed as denominator"),

            Self::DimesOutOfRange(dimes) => write!(f, "Dimes out of range: {}", dimes),

            Self::CentsOutOfRange(cents) => write!(f, "Cents out of range: {}", cents),
        }
    }
}

impl Error for CrateError {}
