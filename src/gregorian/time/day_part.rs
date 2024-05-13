use super::{Hour, Hour24};
use crate::{Chinese, ChineseFormat, Variant};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// Each of the 8 traditional parts of the day-night cycle.
///
/// This partitioning scheme is mainly used in 12-hour formats,
/// just like the a.m./p.m. in English, but more fine-grained.
///
/// Each day part conventionally lasts 3 hours,
/// starting from `早上` at 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DayPart {
    EarlyMorning,
    Morning,
    Midday,
    Afternoon,
    EarlyEvening,
    Evening,
    Midnight,
    LateNight,
}

/// Even though [DayPart] is essentially designed
/// for 12-hour time formats, it can be infallibly obtained
/// only by converting an [Hour24],
/// because the day/night information is required.
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let hour: Hour24 = 21.try_into()?;
/// let day_part: DayPart = hour.into();
///
/// assert_eq!(day_part.to_chinese(Variant::Simplified), "晚上");
/// assert_eq!(day_part.to_chinese(Variant::Traditional), "晚上");
///
/// # Ok(())
/// # }
/// ```
impl From<Hour24> for DayPart {
    fn from(hour24: Hour24) -> Self {
        PART_OF_DAY_BY_HOUR[&(hour24.clock_value().0 as u8)]
    }
}

/// Each day part can be converted to Chinese logograms,
/// which are independent of the [Variant].
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// assert_eq!(
///     DayPart::EarlyMorning.to_chinese(Variant::Simplified),
///     "早上"
/// );
/// assert_eq!(
///     DayPart::EarlyMorning.to_chinese(Variant::Traditional),
///     "早上"
/// );
///
/// assert_eq!(
///     DayPart::Morning.to_chinese(Variant::Simplified),
///     "上午"
/// );
/// assert_eq!(
///     DayPart::Morning.to_chinese(Variant::Traditional),
///     "上午"
/// );
///
/// assert_eq!(
///     DayPart::Midday.to_chinese(Variant::Simplified),
///     "中午"
/// );
/// assert_eq!(
///     DayPart::Midday.to_chinese(Variant::Traditional),
///     "中午"
/// );
///
/// assert_eq!(
///     DayPart::Afternoon.to_chinese(Variant::Simplified),
///     "下午"
/// );
/// assert_eq!(
///     DayPart::Afternoon.to_chinese(Variant::Traditional),
///     "下午"
/// );
///
/// assert_eq!(
///     DayPart::EarlyEvening.to_chinese(Variant::Simplified),
///     "傍晚"
/// );
/// assert_eq!(
///     DayPart::EarlyEvening.to_chinese(Variant::Traditional),
///     "傍晚"
/// );
///
/// assert_eq!(
///     DayPart::Evening.to_chinese(Variant::Simplified),
///     "晚上"
/// );
/// assert_eq!(
///     DayPart::Evening.to_chinese(Variant::Traditional),
///     "晚上"
/// );
///
/// assert_eq!(
///     DayPart::Midnight.to_chinese(Variant::Simplified),
///     "午夜"
/// );
/// assert_eq!(
///     DayPart::Midnight.to_chinese(Variant::Traditional),
///     "午夜"
/// );
///
/// assert_eq!(
///     DayPart::LateNight.to_chinese(Variant::Simplified),
///     "深夜"
/// );
/// assert_eq!(
///     DayPart::LateNight.to_chinese(Variant::Traditional),
///     "深夜"
/// );
/// ```
impl ChineseFormat for DayPart {
    fn to_chinese(&self, _variant: Variant) -> Chinese {
        Chinese {
            logograms: match self {
                DayPart::EarlyMorning => "早上",
                DayPart::Morning => "上午",
                DayPart::Midday => "中午",
                DayPart::Afternoon => "下午",
                DayPart::EarlyEvening => "傍晚",
                DayPart::Evening => "晚上",
                DayPart::Midnight => "午夜",
                DayPart::LateNight => "深夜",
            }
            .to_string(),
            omissible: false,
        }
    }
}

lazy_static! {
    static ref PART_OF_DAY_BY_HOUR: HashMap<u8, DayPart> = HashMap::from([
        (5, DayPart::EarlyMorning),
        (6, DayPart::EarlyMorning),
        (7, DayPart::EarlyMorning),
        (8, DayPart::Morning),
        (9, DayPart::Morning),
        (10, DayPart::Morning),
        (11, DayPart::Midday),
        (12, DayPart::Midday),
        (13, DayPart::Midday),
        (14, DayPart::Afternoon),
        (15, DayPart::Afternoon),
        (16, DayPart::Afternoon),
        (17, DayPart::EarlyEvening),
        (18, DayPart::EarlyEvening),
        (19, DayPart::EarlyEvening),
        (20, DayPart::Evening),
        (21, DayPart::Evening),
        (22, DayPart::Evening),
        (23, DayPart::Midnight),
        (0, DayPart::Midnight),
        (1, DayPart::Midnight),
        (2, DayPart::LateNight),
        (3, DayPart::LateNight),
        (4, DayPart::LateNight),
    ]);
}
