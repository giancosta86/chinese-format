use crate::{Chinese, ChineseFormat, Variant};
use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant};

/// The integer type on which [Financial] is based.
pub type FinancialBase = u64;

/// Financial numbers designed to prevent falsification.
///
/// ```
/// use chinese_format::*;
///
/// let two = Financial(2);
///
/// assert_eq!(two.to_chinese(Variant::Simplified), Chinese {
///     logograms: "贰".to_string(),
///     omissible: false
/// });
///
/// assert_eq!(two.to_chinese(Variant::Traditional), Chinese {
///     logograms: "貳".to_string(),
///     omissible: false
/// });
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Financial(pub FinancialBase);

/// [Financial] supports conversion to [Chinese], by introducing
/// a dedicated set of digits.
///
/// ```
/// use chinese_format::*;
///
/// let ten = Financial(10);
///
/// assert_eq!(ten.to_chinese(Variant::Simplified), Chinese {
///     logograms: "拾".to_string(),
///     omissible: false
/// });
///
/// assert_eq!(ten.to_chinese(Variant::Traditional), Chinese {
///     logograms: "拾".to_string(),
///     omissible: false
/// });
///
///
/// let one_thousand = Financial(1000);
/// assert_eq!(one_thousand.to_chinese(Variant::Simplified), "壹仟");
/// assert_eq!(one_thousand.to_chinese(Variant::Traditional), "壹仟");
///
///
/// let zero = Financial(0);
///
/// assert_eq!(zero.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
///
/// assert_eq!(zero.to_chinese(Variant::Traditional), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
///
///
/// let max = Financial(FinancialBase::MAX);
///
/// assert_eq!(max.to_chinese(Variant::Simplified), Chinese {
///     logograms: "壹仟捌佰肆拾肆京陆仟柒佰肆拾肆兆零柒佰叁拾柒亿零玖佰伍拾伍万壹仟陆佰壹拾伍".to_string(),
///     omissible: false
/// });
///
/// assert_eq!(max.to_chinese(Variant::Traditional), Chinese {
///     logograms: "壹仟捌佰肆拾肆京陸仟柒佰肆拾肆兆零柒佰參拾柒億零玖佰伍拾伍萬壹仟陸佰壹拾伍".to_string(),
///     omissible: false
/// });
/// ```
impl ChineseFormat for Financial {
    fn to_chinese(&self, variant: crate::Variant) -> crate::Chinese {
        let logograms: String = chinese_number::NumberToChinese::to_chinese(
            self.0,
            match variant {
                Variant::Simplified => ChineseVariant::Simple,
                Variant::Traditional => ChineseVariant::Traditional,
            },
            ChineseCase::Upper,
            ChineseCountMethod::TenThousand,
        )
        .expect("Converting an integer to financial Chinese should never fail!");

        Chinese {
            logograms,
            omissible: self.0 == 0,
        }
    }
}

/// [Financial] supports equality with the underlying integer.
///
/// ```
/// use chinese_format::*;
///
/// assert_eq!(Financial(90), 90);
/// assert_ne!(Financial(90), 83);
/// ```
impl PartialEq<FinancialBase> for Financial {
    fn eq(&self, other: &FinancialBase) -> bool {
        self.0 == *other
    }
}

/// [Financial] supports comparisons with the underlying integer.
///
/// ```
/// use chinese_format::*;
///
/// assert!(Financial(90) < 100);
/// assert!(Financial(90) > 5);
/// assert!(Financial(90) >= 90);
/// ```
impl PartialOrd<FinancialBase> for Financial {
    fn partial_cmp(&self, other: &FinancialBase) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}
