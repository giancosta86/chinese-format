use super::{DayPart, Hour, Hour12, Hour24, Minute, Second};
use crate::{chinese_vec, Chinese, ChineseFormat, EmptyPlaceholder, Variant};

/// Time expression showing time linearly - from day part down to second.
///
/// As a bare minimum, the hour and the minute must be declared:
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
///
/// assert_eq!(
///     LinearTime {
///         day_part: false,
///         hour: 19.try_into()?,
///         minute: 24.try_into()?,
///         second: None,
///     }.to_chinese(Variant::Simplified),
///     Chinese {
///         logograms: "十九点二十四分".to_string(),
///         omissible: false
///     }
/// );
///
/// assert_eq!(
///     LinearTime {
///         day_part: true,
///         hour: 19.try_into()?,
///         minute: 24.try_into()?,
///         second: None,
///     }.to_chinese(Variant::Simplified),
///     "傍晚七点二十四分"
/// );
///
/// assert_eq!(
///     LinearTime {
///         day_part: false,
///         hour: 22.try_into()?,
///         minute: 48.try_into()?,
///         second: Some(37.try_into()?),
///     }.to_chinese(Variant::Simplified),
///     "二十二点四十八分三十七秒"
/// );
///
/// assert_eq!(
///     LinearTime {
///         day_part: true,
///         hour: 8.try_into()?,
///         minute: 31.try_into()?,
///         second: Some(52.try_into()?),
///     }.to_chinese(Variant::Simplified),
///     "上午八点三十一分五十二秒"
/// );
///
/// assert_eq!(
///     LinearTime {
///         day_part: true,
///         hour: 20.try_into()?,
///         minute: 31.try_into()?,
///         second: Some(52.try_into()?),
///     }.to_chinese(Variant::Simplified),
///     "晚上八点三十一分五十二秒"
/// );
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinearTime {
    /// Describes whether the [DayPart] should be included: in this case,
    /// the `hour` component is automatically converted to [Hour12].
    pub day_part: bool,

    /// The hour, in the format of a digital clock.
    pub hour: Hour24,

    /// The minute.
    pub minute: Minute,

    /// Optionally, the second.
    pub second: Option<Second>,
}

impl ChineseFormat for LinearTime {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        let (day_part, hour): (Option<DayPart>, Box<dyn Hour>) = if self.day_part {
            let day_part: DayPart = self.hour.into();
            let hour12: Hour12 = self.hour.into();
            (Some(day_part), Box::new(hour12))
        } else {
            (None, Box::new(self.hour))
        };

        chinese_vec!(
            variant,
            [
                EmptyPlaceholder::new(&day_part),
                hour,
                EmptyPlaceholder::new(&self.minute),
                EmptyPlaceholder::new(&self.second)
            ]
        )
        .collect()
    }
}
