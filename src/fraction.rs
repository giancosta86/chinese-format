use crate::{Chinese, ChineseVec, CrateError, CrateResult, Sign, ToChinese, Variant};
use vec_box::vec_box;

/// A fraction, convertible to Chinese.
///
/// Must be created by calling [try_new](Self::try_new); later, its components can
/// be accessed via the [numerator](Self::numerator) and [denominator](Self::denominator) methods:
///
/// ```
/// use chinese_format::*;
///
/// # fn main() -> CrateResult<()> {
/// let fraction = Fraction::try_new(8, 3)?;
///
/// assert_eq!(fraction.numerator(), 3);
/// assert_eq!(fraction.denominator(), 8);
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fraction {
    denominator: u128,
    numerator: i128,
}

impl Fraction {
    /// Tries to create a new fraction - failing with [CrateError::ZeroDenominator] if the denominator is 0.
    ///
    /// **Please, note:** as expected in Chinese, the `denominator` must be passed first.
    ///
    /// The construction only fails when 0 is passed as the denominator:
    ///
    /// ```
    /// use chinese_format::*;
    ///
    /// # fn main() -> CrateResult<()> {
    /// let fraction = Fraction::try_new(9, 4)?;
    /// assert_eq!(fraction.numerator(), 4);
    /// assert_eq!(fraction.denominator(), 9);
    ///
    /// let error_result = Fraction::try_new(0, 3);
    /// assert_eq!(error_result, Err(CrateError::ZeroDenominator));
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn try_new(denominator: u128, numerator: i128) -> CrateResult<Fraction> {
        if denominator == 0 {
            Err(CrateError::ZeroDenominator)
        } else {
            Ok(Fraction {
                denominator,
                numerator,
            })
        }
    }

    pub fn denominator(&self) -> u128 {
        self.denominator
    }

    pub fn numerator(&self) -> i128 {
        self.numerator
    }
}

/// Fractions can be infallibly converted to Chinese.
///
/// They are [omissible](Chinese::omissible) - and converted to `零` - only when the numerator is 0.
///
/// ```
/// use chinese_format::*;
///
/// # fn main() -> CrateResult<()> {
/// //Positive fractions
/// let positive_fraction = Fraction::try_new(8, 3)?;
/// assert_eq!(positive_fraction.to_chinese(Variant::Simplified), Chinese {
///     logograms: "八分之三".to_string(),
///     omissible: false
/// });
/// assert_eq!(positive_fraction.to_chinese(Variant::Traditional), "八分之三");
///
/// //Zero fractions, no matter the denominator
/// let zero_fraction = Fraction::try_new(8, 0)?;
/// assert_eq!(zero_fraction.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
/// assert_eq!(zero_fraction.to_chinese(Variant::Traditional), "零");
///
/// //Negative fractions
/// let negative_fraction = Fraction::try_new(3, -11)?;
/// assert_eq!(negative_fraction.to_chinese(Variant::Simplified), Chinese {
///     logograms: "负三分之十一".to_string(),
///     omissible: false
/// });
/// assert_eq!(negative_fraction.to_chinese(Variant::Traditional), "負三分之十一");
///
/// # Ok(())
/// # }
/// ```
impl ToChinese for Fraction {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        if self.numerator != 0 {
            ChineseVec::from(
                variant,
                vec_box![
                    Sign(self.numerator),
                    self.denominator,
                    "分之",
                    self.numerator.abs()
                ],
            )
            .collect()
        } else {
            Chinese {
                logograms: "零".to_string(),
                omissible: true,
            }
        }
    }
}
