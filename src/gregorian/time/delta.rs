use super::{Hour12, Minute};
use crate::{chinese_vec, Chinese, ChineseFormat, Variant};

/// Time expressed as minutes (a *delta*) past/to an hour.
///
/// Traditionally, it supports a minimalist 12-hour format.
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let o_clock = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 0.try_into()?
/// };
/// assert_eq!(o_clock.to_chinese(Variant::Simplified), "六点钟");
/// assert_eq!(o_clock.to_chinese(Variant::Traditional), "六點鐘");
///
/// let past_one = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 1.try_into()?
/// };
/// assert_eq!(past_one.to_chinese(Variant::Simplified), "六点过一分");
/// assert_eq!(past_one.to_chinese(Variant::Traditional), "六點過一分");
///
/// let past_five = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 5.try_into()?
/// };
/// assert_eq!(past_five.to_chinese(Variant::Simplified), "六点过五分");
/// assert_eq!(past_five.to_chinese(Variant::Traditional), "六點過五分");
///
/// let past_fourteen = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 14.try_into()?
/// };
/// assert_eq!(past_fourteen.to_chinese(Variant::Simplified), "六点过十四分");
/// assert_eq!(past_fourteen.to_chinese(Variant::Traditional), "六點過十四分");
///
/// let quarter = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 15.try_into()?
/// };
/// assert_eq!(quarter.to_chinese(Variant::Simplified), "六点刻");
/// assert_eq!(quarter.to_chinese(Variant::Traditional), "六點刻");
///
/// let past_sixteen = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 16.try_into()?
/// };
/// assert_eq!(past_sixteen.to_chinese(Variant::Simplified), "六点过十六分");
/// assert_eq!(past_sixteen.to_chinese(Variant::Traditional), "六點過十六分");
///
/// let past_twenty_nine = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 29.try_into()?
/// };
/// assert_eq!(past_twenty_nine.to_chinese(Variant::Simplified), "六点过二十九分");
/// assert_eq!(past_twenty_nine.to_chinese(Variant::Traditional), "六點過二十九分");
///
/// let half = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 30.try_into()?
/// };
/// assert_eq!(half.to_chinese(Variant::Simplified), "六点半");
/// assert_eq!(half.to_chinese(Variant::Traditional), "六點半");
///
/// let twenty_nine_to = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 31.try_into()?
/// };
/// assert_eq!(twenty_nine_to.to_chinese(Variant::Simplified), "七点差二十九分");
/// assert_eq!(twenty_nine_to.to_chinese(Variant::Traditional), "七點差二十九分");
///
/// let sixteen_to = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 44.try_into()?
/// };
/// assert_eq!(sixteen_to.to_chinese(Variant::Simplified), "七点差十六分");
/// assert_eq!(sixteen_to.to_chinese(Variant::Traditional), "七點差十六分");
///
/// let three_quarters = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 45.try_into()?
/// };
/// assert_eq!(three_quarters.to_chinese(Variant::Simplified), "六点三刻");
/// assert_eq!(three_quarters.to_chinese(Variant::Traditional), "六點三刻");
///
/// let fourteen_to = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 46.try_into()?
/// };
/// assert_eq!(fourteen_to.to_chinese(Variant::Simplified), "七点差十四分");
/// assert_eq!(fourteen_to.to_chinese(Variant::Traditional), "七點差十四分");
///
/// let one_to = DeltaTime {
///     hour: 6.try_into()?,
///     minute: 59.try_into()?
/// };
/// assert_eq!(one_to.to_chinese(Variant::Simplified), "七点差一分");
/// assert_eq!(one_to.to_chinese(Variant::Traditional), "七點差一分");
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeltaTime {
    /// The hour, as displayed by the *hour* hand of an analog clock.
    pub hour: Hour12,

    /// The minute, as displayed by the *minute* hand of an analog clock.
    pub minute: Minute,
}

const ZHONG: (&str, &str) = ("钟", "鐘");

const GUO: (&str, &str) = ("过", "過");

const KE: &str = "刻";

const BAN: &str = "半";

const CHA: &str = "差";

impl ChineseFormat for DeltaTime {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match self.minute.into() {
            0 => chinese_vec!(variant, [self.hour, ZHONG]),

            1..=14 | 16..=29 => chinese_vec!(variant, [self.hour, GUO, self.minute]),

            15 => chinese_vec!(variant, [self.hour, KE]),

            30 => chinese_vec!(variant, [self.hour, BAN]),

            45 => chinese_vec!(variant, [self.hour, 3, KE]),

            _ => chinese_vec!(
                variant,
                [
                    self.hour.next(),
                    CHA,
                    self.minute.complement().expect("0 is not in this range")
                ]
            ),
        }
        .collect()
    }
}
