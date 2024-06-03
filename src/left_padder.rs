use crate::{Chinese, ChineseFormat, Variant};
use std::iter;

/// Pads by adding the given logogram to the left.
///
/// The padding logogram is added *only* if the string
/// of sinograms produced by the given source
/// is shorter than the required minimum width; otherwise,
/// the original output is not affected.
///
/// Despite the padding, the generated Chinese is [omissible](Chinese::omissible)
/// as long as the source is.
///
/// ```
/// use chinese_format::*;
///
/// let left_padder = LeftPadder {
///     logogram: '零',
///     min_width: 3,
///     source: &"五分"
/// };
/// assert_eq!(left_padder.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零五分".to_string(),
///     omissible: false
/// });
///
/// let left_padder = LeftPadder {
///     logogram: '零',
///     min_width: 5,
///     source: &"五分"
/// };
/// assert_eq!(left_padder.to_chinese(Variant::Simplified), "零零零五分");
///
/// let left_padder = LeftPadder {
///     logogram: '零',
///     min_width: 0,
///     source: &"五分"
/// };
/// assert_eq!(left_padder.to_chinese(Variant::Simplified), "五分");
///
/// let left_padder = LeftPadder {
///     logogram: '#',
///     min_width: 4,
///     source: &""
/// };
/// assert_eq!(left_padder.to_chinese(Variant::Simplified), Chinese {
///     logograms: "####".to_string(),
///     omissible: true
/// });
/// ```
pub struct LeftPadder<'a> {
    pub logogram: char,
    pub min_width: usize,
    pub source: &'a dyn ChineseFormat,
}

impl<'a> ChineseFormat for LeftPadder<'a> {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        let source_chinese = self.source.to_chinese(variant);

        let padding_length = self
            .min_width
            .saturating_sub(source_chinese.logograms.chars().count());

        let padding: String = iter::repeat(self.logogram).take(padding_length).collect();

        Chinese {
            logograms: format!("{}{}", padding, source_chinese.logograms),
            omissible: source_chinese.omissible,
        }
    }
}
