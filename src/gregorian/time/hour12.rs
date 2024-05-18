use super::{Hour, Hour24};
use crate::{Count, CountBase, CrateError, CrateResult};

/// The hour shown on a traditional analog clock.
///
/// It must belong to the 1..=12 range and can be obtained:
///
/// * *infallibly*, from an existing [Hour24]:
///
///   ```
///   use chinese_format::{*, gregorian::*};
///
///   pub fn main() -> GenericResult<()> {
///   let h7: Hour24 = 7.try_into()?;
///   let seven_am: Hour12 = h7.into();
///
///   let h19: Hour24 = 19.try_into()?;
///   let seven_pm: Hour12 = h19.into();
///
///   assert_eq!(seven_am, seven_pm);
///   assert_eq!(seven_am.to_chinese(Variant::Simplified), "七点");
///   assert_eq!(seven_am.to_chinese(Variant::Traditional), "七點");
///
///   # Ok(())
///   # }
///   ```
/// * *fallibly*, from any [u8] - succeeding only if it
///   belongs to the 1..=12 range:
///
///   ```
///   use chinese_format::{*, gregorian::*};
///   
///   # fn main() -> GenericResult<()> {
///   let two: Hour12 = 2.try_into()?;
///   assert_eq!(two.to_chinese(Variant::Simplified), "两点");
///   assert_eq!(two.to_chinese(Variant::Traditional), "兩點");
///
///   let too_low_result: CrateResult<Hour12> = 0.try_into();
///   assert_eq!(too_low_result, Err(CrateError::HourOutOfRange(0)));
///
///   let too_high_result: CrateResult<Hour12> = 13.try_into();
///   assert_eq!(too_high_result, Err(CrateError::HourOutOfRange(13)));
///
///   # Ok(())
///   # }
///   ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hour12(Count);

impl Hour12 {
    /// Returns the next value in the analog clock.
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// # fn main() -> GenericResult<()> {
    /// let three: Hour12 = 3.try_into()?;
    /// let four: Hour12 = three.next();
    ///
    /// assert_eq!(four.to_chinese(Variant::Simplified), "四点");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Of course, the value after `12` wraps to `1`:
    ///
    /// ```
    /// use chinese_format::{*, gregorian::*};
    ///
    /// # fn main() -> GenericResult<()> {
    /// let twelve: Hour12 = 12.try_into()?;
    /// let one: Hour12 = twelve.next();
    ///
    /// assert_eq!(one.to_chinese(Variant::Simplified), "一点");
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn next(&self) -> Self {
        let numeric_value = self.0 .0 as u8;

        Self(Count(match numeric_value {
            12 => 1,
            _ => numeric_value + 1,
        } as CountBase))
    }
}

impl Hour for Hour12 {
    fn clock_value(&self) -> &Count {
        &self.0
    }
}

/// Conversion from [Hour24] is always *infallible*.
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
///
/// let h0: Hour24 = 0.try_into()?;
/// let midnight: Hour12 = h0.into();
/// assert_eq!(midnight.to_chinese(Variant::Simplified), "十二点");
///
/// let h5: Hour24 = 5.try_into()?;
/// let five_am: Hour12 = h5.into();
/// assert_eq!(five_am.to_chinese(Variant::Simplified), "五点");
///
/// let h12: Hour24 = 12.try_into()?;
/// let midday: Hour12 = h12.into();
/// assert_eq!(midday, midnight);
///
/// let h18: Hour24 = 18.try_into()?;
/// let six_pm: Hour12 = h18.into();
/// assert_eq!(six_pm.to_chinese(Variant::Simplified), "六点");
///
/// let h23: Hour24 = 23.try_into()?;
/// let eleven_pm: Hour12 = h23.into();
/// assert_eq!(eleven_pm.to_chinese(Variant::Simplified), "十一点");
///
/// # Ok(())
/// # }
/// ```
impl From<Hour24> for Hour12 {
    fn from(hour24: Hour24) -> Self {
        let value_in_24 = hour24.clock_value().0;

        Self(Count(match value_in_24 {
            0 => 12,
            1..=12 => value_in_24,
            _ => value_in_24 - 12,
        }))
    }
}

/// Conversion from [u8] is only allowed for the 1..=12 range.
impl TryFrom<u8> for Hour12 {
    type Error = CrateError;

    fn try_from(value: u8) -> CrateResult<Self> {
        if !(1..=12).contains(&value) {
            return Err(CrateError::HourOutOfRange(value));
        }

        Ok(Hour12(Count(value as CountBase)))
    }
}
