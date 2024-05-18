use crate::{chinese_vec, Chinese, ChineseFormat, Variant};
use digit_sequence::DigitSequence;

/// The integer part of a [Decimal].
///
/// **REQUIRED FEATURE**: `digit-sequence`.
pub type IntegerPart = i128;

/// Accurate real number.
///
/// It supports *unbounded* **fractional** precision and length,
/// but the **integer** part must be within the range of the
/// [IntegerPart] type alias; negative numbers are supported.
///
/// ```
/// use chinese_format::*;
/// use digit_sequence::*;
///
/// let decimal = Decimal {
///     integer: 96,
///     fractional: 753u16.into()
/// };
/// assert_eq!(decimal.to_chinese(Variant::Simplified), Chinese {
///     logograms: "九十六点七五三".to_string(),
///     omissible: false
/// });
/// assert_eq!(decimal.to_chinese(Variant::Traditional), Chinese {
///     logograms: "九十六點七五三".to_string(),
///     omissible: false
/// });
/// ```
///
/// It is worth noting that [Decimal] supports both equality and ordering:
///
/// ```
/// use chinese_format::*;
/// use digit_sequence::DigitSequence;
///
/// let ninety = Decimal {
///     integer: 90,
///     fractional: DigitSequence::new()
/// };
///
///
/// let ninety_dot_five = Decimal {
///     integer: 90,
///     fractional: 5u8.into()
/// };
///
///
/// let ninety_dot_five_seven = Decimal {
///     integer: 90,
///     fractional: 57u8.into()
/// };
///
/// let ninety_two = Decimal {
///     integer: 92,
///     fractional: DigitSequence::new()
/// };
///
/// assert_eq!(ninety, ninety);
/// assert_ne!(ninety, ninety_dot_five);
///
/// assert!(ninety < ninety_dot_five);
/// assert!(ninety_dot_five < ninety_dot_five_seven);
/// assert!(ninety_dot_five_seven < ninety_two);
/// ```
///
/// **REQUIRED FEATURE**: `digit-sequence`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal {
    /// The digits *before* the decimal separator.
    pub integer: IntegerPart,

    /// The digits *after* the decimal separator.
    pub fractional: DigitSequence,
}

const COMMA: (&str, &str) = ("点", "點");

/// [Decimal] can be translated to [Chinese].
///
/// ```
/// use chinese_format::*;
/// use digit_sequence::*;
///
/// let integer_only = Decimal {
///     integer: 90,
///     fractional: DigitSequence::new()
/// };
/// assert_eq!(integer_only.to_chinese(Variant::Simplified), Chinese {
///     logograms: "九十".to_string(),
///     omissible: false
/// });
/// assert_eq!(integer_only.to_chinese(Variant::Traditional), Chinese {
///     logograms: "九十".to_string(),
///     omissible: false
/// });
///
/// let zero = Decimal {
///     integer: 0,
///     fractional: DigitSequence::new()
/// };
/// assert_eq!(zero.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
/// assert_eq!(zero.to_chinese(Variant::Traditional), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
///
/// let integer_and_decimal = Decimal {
///     integer: 35,
///     fractional: 28039u16.into()
/// };
/// assert_eq!(integer_and_decimal.to_chinese(Variant::Simplified), Chinese {
///     logograms: "三十五点二八零三九".to_string(),
///     omissible: false
/// });
/// assert_eq!(integer_and_decimal.to_chinese(Variant::Traditional), Chinese {
///     logograms: "三十五點二八零三九".to_string(),
///     omissible: false
/// });
///
/// let zero_comma = Decimal {
///     integer: 0,
///     fractional: 9052u16.into()
/// };
/// assert_eq!(zero_comma.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零点九零五二".to_string(),
///     omissible: false
/// });
/// assert_eq!(zero_comma.to_chinese(Variant::Traditional), Chinese {
///     logograms: "零點九零五二".to_string(),
///     omissible: false
/// });
///
/// let negative = Decimal {
///     integer: -487,
///     fractional: 309u16.into()
/// };
/// assert_eq!(negative.to_chinese(Variant::Simplified), Chinese {
///     logograms: "负四百八十七点三零九".to_string(),
///     omissible: false
/// });
/// assert_eq!(negative.to_chinese(Variant::Traditional), Chinese {
///     logograms: "負四百八十七點三零九".to_string(),
///     omissible: false
/// });
/// ```
impl ChineseFormat for Decimal {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        if self.fractional.is_empty() {
            self.integer.to_chinese(variant)
        } else {
            chinese_vec!(variant, [self.integer, COMMA, self.fractional]).collect()
        }
    }
}
