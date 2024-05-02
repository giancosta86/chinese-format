use crate::{Chinese, ToChinese, Variant};
use std::cmp::Ordering;

/// The integer type on which [Count] is based.
pub type CountBase = u128;

/// Integer quantity expressing the result of a counting process.
///
/// When converting to Chinese, it behaves like **unsigned** integers,
/// except when its value is `2`, because its translation becomes `两`(`兩`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(pub CountBase);

/// [Count] supports equality checks with [CountBase] values:
///
/// ```
/// use chinese_format::Count;
///
/// let count = Count(90);
///
/// assert_eq!(count, 90);
///
/// assert_ne!(count, 91);
impl PartialEq<CountBase> for Count {
    fn eq(&self, other: &CountBase) -> bool {
        self.0 == *other
    }
}

/// [Count] supports ordering with [CountBase] values:
///
/// ```
/// use chinese_format::Count;
///
/// let count = Count(90);
///
/// assert_eq!(count < 100, true);
///
/// assert_eq!(count > 70, true);
///
/// assert_eq!(count < 5, false);
///
/// assert_eq!(count > 98, false);
///
/// assert_eq!(count >= 90, true);
///
/// ```
impl PartialOrd<CountBase> for Count {
    fn partial_cmp(&self, other: &CountBase) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

/// In Chinese, [Count] is written as a standard number, *except* when its value is `2``;
/// in that case, its logogram becomes:
///
/// * `两` in [Variant::Simplified]
///
/// * `兩` in [Variant::Traditional]
///
/// ```
/// use chinese_format::{Chinese, Count, ToChinese, Variant};
///
/// //Positive numbers
/// assert_eq!(Count(7).to_chinese(Variant::Simplified), Chinese {
///     logograms: "七".to_string(),
///     omissible: false
/// });
/// assert_eq!(Count(42).to_chinese(Variant::Traditional), "四十二");
///
/// //Zero
/// assert_eq!(Count(0).to_chinese(Variant::Simplified), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
/// assert_eq!(Count(0).to_chinese(Variant::Traditional), "零");
///
/// //Two
/// assert_eq!(Count(2).to_chinese(Variant::Simplified), "两");
/// assert_eq!(Count(2).to_chinese(Variant::Traditional), "兩");
/// ```
impl ToChinese for Count {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        if self.0 == 2 {
            ("两", "兩").to_chinese(variant)
        } else {
            self.0.to_chinese(variant)
        }
    }
}
