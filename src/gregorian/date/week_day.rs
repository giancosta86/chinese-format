use super::WeekDayOutOfRange;

/// The day of the week.
///
/// Each day can also be cast to its corresponding ordinal:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// assert_eq!(WeekDay::Wednesday as u8, 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// A fallible conversion from [u8] is available,
/// provided that 0=Sunday and 6=Saturday:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
///
/// let day: WeekDay = 0.try_into()?;
/// assert_eq!(day, WeekDay::Sunday);
///
/// let day: WeekDay = 1.try_into()?;
/// assert_eq!(day, WeekDay::Monday);
///
/// let day: WeekDay = 2.try_into()?;
/// assert_eq!(day, WeekDay::Tuesday);
///
/// let day: WeekDay = 3.try_into()?;
/// assert_eq!(day, WeekDay::Wednesday);
///
/// let day: WeekDay = 4.try_into()?;
/// assert_eq!(day, WeekDay::Thursday);
///
/// let day: WeekDay = 5.try_into()?;
/// assert_eq!(day, WeekDay::Friday);
///
/// let day: WeekDay = 6.try_into()?;
/// assert_eq!(day, WeekDay::Saturday);
///
/// let result: Result<WeekDay, WeekDayOutOfRange> = 7.try_into();
/// assert_eq!(result, Err(WeekDayOutOfRange(7)));
///
/// # Ok(())
/// # }
/// ```
impl TryFrom<u8> for WeekDay {
    type Error = WeekDayOutOfRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sunday),
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            _ => Err(WeekDayOutOfRange(value)),
        }
    }
}
