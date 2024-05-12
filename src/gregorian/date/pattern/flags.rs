use std::fmt::Display;

/// Flags describing the components in a [DatePattern](super::DatePattern).
///
/// This data structure is used, for example, by [DatePattern::validate](super::DatePattern::validate).
pub struct DatePatternFlags {
    pub year: bool,

    pub month: bool,

    pub day: bool,

    pub week_day: bool,
}

/// [DatePatternFlags] can be converted to a [String]
/// consisting of cumulated single characters - one for each
/// enabled date component:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// assert_eq!(
///     DatePatternFlags {
///         year: false,
///         month: false,
///         day: false,
///         week_day: false
///     }.to_string(),
///     ""
/// );
///
/// assert_eq!(
///     DatePatternFlags {
///         year: true,
///         month: true,
///         day: true,
///         week_day: true
///     }.to_string(),
///     "ymdw"
/// );
///
/// # Ok(())
/// # }
/// ```
impl Display for DatePatternFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.year {
            write!(f, "y")?;
        }

        if self.month {
            write!(f, "m")?;
        }

        if self.day {
            write!(f, "d")?;
        }

        if self.week_day {
            write!(f, "w")?;
        }

        Ok(())
    }
}
