use crate::{Chinese, ToChinese, Variant};

/// The [Option] of a [ToChinese] is a [ToChinese] itself.
///
/// In particular:
///
/// * in case of [Some], both the logograms and the [omissible](Chinese::omissible) property depend on the actual content.
///
/// * in case of [None], the logograms are an empty string and [omissible](Chinese::omissible) is `true`.
///
/// ```
/// use chinese_format::*;
///
/// let non_zero = Some(90u8);
/// assert_eq!(non_zero.to_chinese(Variant::Simplified), Chinese {
///     logograms: "九十".to_string(),
///     omissible: false
/// });
///
/// let zero = Some(0u8);
/// assert_eq!(
///     zero.to_chinese(Variant::Simplified),
///     Chinese {
///         logograms: "零".to_string(),
///         omissible: true
///     }
/// );
///
/// let none: Option<i8> = None;
/// assert_eq!(
///     none.to_chinese(Variant::Simplified),
///     Chinese {
///         logograms: "".to_string(),
///         omissible: true
///     }
/// );
/// ```
impl<T: ToChinese> ToChinese for Option<T> {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match self {
            Some(value) => value.to_chinese(variant),
            None => Chinese {
                logograms: String::new(),
                omissible: true,
            },
        }
    }
}
