use std::{error::Error, fmt::Display};

/// Error for when a denominator is zero.
///
/// ```
/// use chinese_format::*;
///
/// assert_eq!(
///     ZeroDenominator.to_string(),
///     "Zero passed as denominator"
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZeroDenominator;

impl Display for ZeroDenominator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zero passed as denominator")
    }
}

impl Error for ZeroDenominator {}
