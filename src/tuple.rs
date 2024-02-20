use crate::{Chinese, ToChinese, Variant};

impl<T1: ToChinese, T2: ToChinese> ToChinese for (T1, T2) {
    /// Any pair of types implementing [ToChinese] can be infallibly converted to Chinese.
    ///
    /// In particular:
    ///
    /// * the former element is used for [Variant::Simplified].
    ///
    /// * the latter element is used for [Variant::Traditional].
    ///
    /// The result is [omissible](Chinese::omissible) according to the selected element.
    /// ```
    /// use chinese_format::*;
    ///
    /// let str_tuple = ("天气", "天氣");
    /// assert_eq!(str_tuple.to_chinese(Variant::Simplified), "天气");
    /// assert_eq!(str_tuple.to_chinese(Variant::Traditional), "天氣");
    ///
    /// let integer_tuple = (92, 0);
    /// assert_eq!(integer_tuple.to_chinese(Variant::Simplified), "九十二");
    /// assert_eq!(integer_tuple.to_chinese(Variant::Traditional), Chinese {
    ///     logograms: "零".to_string(),
    ///     omissible: true
    /// });
    /// ```
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match variant {
            Variant::Simplified => self.0.to_chinese(variant),
            Variant::Traditional => self.1.to_chinese(variant),
        }
    }
}
