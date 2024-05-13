use crate::{Chinese, ChineseFormat, Variant};

/// Sign of a number.
pub struct Sign(pub i128);

/// When converted to Chinese, a sign renders as follows:
///
/// * for *positive numbers* and *zero*: an **empty string** (thus [omissible](Chinese::omissible), too).
///
/// * for *negative numbers*:
///
///   * `负` in [Variant::Simplified] script.
///
///   * `負` in [Variant::Traditional] script.
///
/// ```
/// use chinese_format::*;
///
/// //Positive numbers
/// assert_eq!(Sign(90).to_chinese(Variant::Simplified), Chinese {
///     logograms: "".to_string(),
///     omissible: true
/// });
/// assert_eq!(Sign(90).to_chinese(Variant::Traditional), "");
///
/// //Zero
/// assert_eq!(Sign(0).to_chinese(Variant::Simplified), Chinese {
///     logograms: "".to_string(),
///     omissible: true
/// });
/// assert_eq!(Sign(0).to_chinese(Variant::Traditional), "");
///
/// //Negative numbers
/// assert_eq!(Sign(-7).to_chinese(Variant::Simplified), Chinese {
///     logograms: "负".to_string(),
///     omissible: false
/// });
/// assert_eq!(Sign(-7).to_chinese(Variant::Traditional), "負");
/// ```
impl ChineseFormat for Sign {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        if self.0 >= 0 {
            "".to_chinese(variant)
        } else {
            ("负", "負").to_chinese(variant)
        }
    }
}
