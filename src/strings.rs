use crate::{Chinese, ToChinese, Variant};

/// Any &[str] can be infallibly converted to Chinese.
///
/// ```
/// use chinese_format::*;
///
/// assert_eq!("星期".to_chinese(Variant::Simplified), "星期");
/// assert_eq!("星期".to_chinese(Variant::Traditional), "星期");
///
/// let reference_to_string: &str = &String::from("走");
/// assert_eq!(reference_to_string.to_chinese(Variant::Simplified), "走");
/// assert_eq!(reference_to_string.to_chinese(Variant::Traditional), "走");
/// ```
///
/// Of course, there is no auto-magic translation from Simplified to Traditional characters, or vice-versa:
///
/// ```
/// use chinese_format::*;
///
/// //The traditional form of "天气" is "天氣"
/// assert_eq!("天气".to_chinese(Variant::Traditional), "天气");
/// assert_eq!("天氣".to_chinese(Variant::Simplified), "天氣");
/// ```
///
/// Only the empty string is [omissible](Chinese::omissible):
///
/// ```
/// use chinese_format::*;
///
/// assert!(!"Test".to_chinese(Variant::Simplified).omissible);
///
/// assert!("".to_chinese(Variant::Simplified).omissible);
/// ```
impl ToChinese for &str {
    fn to_chinese(&self, _variant: Variant) -> Chinese {
        Chinese {
            logograms: (*self).into(),
            omissible: self.is_empty(),
        }
    }
}

/// [String] can be infallibly converted to Chinese.
///
/// ```
/// use chinese_format::*;
///
/// assert_eq!("星期".to_string().to_chinese(Variant::Simplified), "星期");
/// assert_eq!("星期".to_string().to_chinese(Variant::Traditional), "星期");
/// ```
///
/// Of course, there is no auto-magic translation from Simplified to Traditional characters, or vice-versa:
///
/// ```
/// use chinese_format::*;
///
/// //The traditional form of "天气" is "天氣"
/// assert_eq!("天气".to_string().to_chinese(Variant::Traditional), "天气");
/// assert_eq!("天氣".to_string().to_chinese(Variant::Simplified), "天氣");
/// ```
///
/// Only the empty string is omissible:
///
/// ```
/// use chinese_format::*;
///
/// assert!(!"Test".to_string().to_chinese(Variant::Simplified).omissible);
///
/// assert!("".to_string().to_chinese(Variant::Simplified).omissible);
/// ```
impl ToChinese for String {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        self.as_str().to_chinese(variant)
    }
}
