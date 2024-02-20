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
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CrateError {
    ZeroDenominator,
}

impl_err_equality!(CrateError);

impl Display for CrateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDenominator => write!(f, "Zero passed as denominator"),
        }
    }
}

impl Error for CrateError {}
