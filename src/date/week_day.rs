/// The day of the week.
///
/// Each day can also be cast to its corresponding ordinal:
///
/// ```
/// use chinese_format::{*, date::*};
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
