use crate::{define_measure, CrateError, CrateResult};

define_measure!(pub, Second, pub(self), u8, "秒");

/// [Second] can be instantiated via conversion from integers in the 0..=59 range.
///
/// ```
/// use chinese_format::{*, gregorian::*};
///
/// # fn main() -> GenericResult<()> {
/// let lowest: Second = 0.try_into().unwrap();
/// assert_eq!(lowest.to_chinese(Variant::Simplified), "零秒");
///
/// let two: Second = 2.try_into().unwrap();
/// assert_eq!(two.to_chinese(Variant::Simplified), "二秒");
/// assert_eq!(two.to_chinese(Variant::Traditional), "二秒");
///
/// let highest: Second = 59.try_into().unwrap();
/// assert_eq!(highest.to_chinese(Variant::Simplified), "五十九秒");
///
/// let second_result: CrateResult<Second> = 60.try_into();
/// assert_eq!(second_result, Err(CrateError::SecondOutOfRange(60)));
///
/// # Ok(())
/// # }
/// ```
impl TryFrom<u8> for Second {
    type Error = CrateError;

    fn try_from(value: u8) -> CrateResult<Self> {
        if value >= 60 {
            return Err(CrateError::SecondOutOfRange(value));
        }

        Ok(Self(value))
    }
}
