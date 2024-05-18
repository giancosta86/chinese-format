use super::Hour;
use crate::{Count, CountBase, CrateError, CrateResult};

/// The hour in the 24-hour digital clock.
///
/// It can be instantiated via a *fallible* conversion from [u8],
/// succeeding if the integer is in the 0..=23 range; otherwise,
/// [CrateError::HourOutOfRange] is returned.
///
/// ```
/// use chinese_format::{*, gregorian::*};
/// # fn main() -> GenericResult<()> {
/// let midnight: Hour24 = 0.try_into()?;
///
/// assert_eq!(midnight.to_chinese(Variant::Simplified), "零点");
/// assert_eq!(midnight.to_chinese(Variant::Traditional), "零點");
///
///
/// let two: Hour24 = 2.try_into()?;
///
/// assert_eq!(two.to_chinese(Variant::Simplified), "两点");
/// assert_eq!(two.to_chinese(Variant::Traditional), "兩點");
///
///
/// let before_midnight: Hour24 = 23.try_into()?;
///
/// assert_eq!(before_midnight.to_chinese(Variant::Simplified), "二十三点");
/// assert_eq!(before_midnight.to_chinese(Variant::Traditional), "二十三點");
///
///
/// let invalid_result: CrateResult<Hour24> = 24.try_into();
///
/// assert_eq!(invalid_result, Err(CrateError::HourOutOfRange(24)));
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hour24(Count);

impl Hour for Hour24 {
    fn clock_value(&self) -> &Count {
        &self.0
    }
}

impl TryFrom<u8> for Hour24 {
    type Error = CrateError;

    fn try_from(value: u8) -> CrateResult<Self> {
        if value >= 24 {
            return Err(CrateError::HourOutOfRange(value));
        }

        Ok(Hour24(Count(value as CountBase)))
    }
}
