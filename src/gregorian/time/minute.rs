use super::MinuteOutOfRange;
use crate::define_measure;

define_measure!(pub, Minute, pub(self), u8, "分");

impl Minute {
    /// Returns the difference in a 60-minute time period.
    ///
    /// It is NOT defined for 0分 - returning, in this case,
    /// [MinuteOutOfRange].
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// # fn main() -> GenericResult<()> {
    ///
    /// let six: Minute = 6.try_into()?;
    /// let fifty_four: Minute = 54.try_into()?;
    ///
    /// assert_eq!(six.complement()?, fifty_four);
    /// assert_eq!(fifty_four.complement()?, six);
    ///
    ///
    /// let one: Minute = 1.try_into()?;
    /// let fifty_nine: Minute = 59.try_into()?;
    ///
    /// assert_eq!(one.complement()?, fifty_nine);
    /// assert_eq!(fifty_nine.complement()?, one);
    ///
    ///
    /// let thirty: Minute = 30.try_into()?;
    /// assert_eq!(thirty.complement()?, thirty);
    ///
    /// let zero: Minute = 0.try_into()?;
    /// assert_eq!(zero.complement(), Err(MinuteOutOfRange(60)));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn complement(&self) -> Result<Self, MinuteOutOfRange> {
        (60 - self.0).try_into()
    }
}

/// [Minute] can be instantiated via conversion from integers in the 0..=59 range.
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let lowest: Minute = 0.try_into().unwrap();
/// assert_eq!(lowest.to_chinese(Variant::Simplified), "零分");
///
/// let two: Minute = 2.try_into().unwrap();
/// assert_eq!(two.to_chinese(Variant::Simplified), "二分");
/// assert_eq!(two.to_chinese(Variant::Traditional), "二分");
///
/// let highest: Minute = 59.try_into().unwrap();
/// assert_eq!(highest.to_chinese(Variant::Simplified), "五十九分");
///
/// let minute_result: Result<Minute, MinuteOutOfRange> = 60.try_into();
/// assert_eq!(minute_result, Err(MinuteOutOfRange(60)));
///
/// # Ok(())
/// # }
/// ```
impl TryFrom<u8> for Minute {
    type Error = MinuteOutOfRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 60 {
            return Err(MinuteOutOfRange(value));
        }

        Ok(Self(value))
    }
}
