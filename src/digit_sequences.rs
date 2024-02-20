use crate::{Chinese, ToChinese, Variant};
use digit_sequence::DigitSequence;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CHINESE_DIGITS: HashMap<u8, char> = HashMap::from([
        (0, '零'),
        (1, '一'),
        (2, '二'),
        (3, '三'),
        (4, '四'),
        (5, '五'),
        (6, '六'),
        (7, '七'),
        (8, '八'),
        (9, '九'),
    ]);
}

impl ToChinese for DigitSequence {
    /// Any [DigitSequence] is infallibly convertible to a sequence of Chinese digits from 零 to 九.
    ///
    /// The resulting Chinese logograms are [omissible](Chinese::omissible) only when the sequence is empty.
    ///
    /// ```
    /// use chinese_format::*;
    /// use digit_sequence::*;
    ///
    /// # fn main() -> GenericResult<()> {
    ///
    /// //Non-empty sequence
    /// let sequence: DigitSequence = "9876543210123456789".parse()?;
    /// assert_eq!(sequence.to_chinese(Variant::Simplified), Chinese {
    ///     logograms: "九八七六五四三二一零一二三四五六七八九".to_string(),
    ///     omissible: false
    /// });
    /// assert_eq!(sequence.to_chinese(Variant::Traditional), "九八七六五四三二一零一二三四五六七八九");
    ///
    /// //Empty sequence
    /// let empty_sequence = DigitSequence::new();
    /// assert_eq!(empty_sequence.to_chinese(Variant::Simplified), Chinese {
    ///     logograms: "".to_string(),
    ///     omissible: true
    /// });
    /// assert_eq!(empty_sequence.to_chinese(Variant::Traditional), "");
    ///
    /// //Sequence containing 0
    /// let zero_sequence = "0".parse::<DigitSequence>()?;
    /// assert_eq!(zero_sequence.to_chinese(Variant::Simplified), Chinese {
    ///     logograms: "零".to_string(),
    ///     omissible: false
    /// });
    /// assert_eq!(zero_sequence.to_chinese(Variant::Traditional), Chinese {
    ///     logograms: "零".to_string(),
    ///     omissible: false
    /// });
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn to_chinese(&self, _variant: Variant) -> Chinese {
        let logograms: String = self
            .iter()
            .map(|digit| CHINESE_DIGITS[digit])
            .collect::<String>();

        Chinese {
            omissible: logograms.is_empty(),
            logograms,
        }
    }
}
