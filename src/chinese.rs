use std::fmt::Display;

/// The two major Chinese variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variant {
    Simplified,
    Traditional,
}

/// Chinese expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chinese {
    /// Contains the sinograms, owned as a String.
    pub logograms: String,

    /// Declares that a field can be omitted in a variety of contexts -
    /// for example, in [placeholders](crate::define_string_placeholder) or
    /// in [crate::ChineseVec::trim_end].
    pub omissible: bool,
}

impl Display for Chinese {
    /// Converting [Chinese] to string returns its logograms:
    ///
    /// ```
    /// use chinese_format::Chinese;
    ///
    /// let chinese = Chinese {
    ///     logograms: "苹果".to_string(),
    ///     omissible: false
    /// };
    ///
    /// assert_eq!(chinese, "苹果");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.logograms)
    }
}

impl PartialEq<&str> for Chinese {
    /// [Chinese] can be compared with &[str].
    ///
    /// ```
    /// use chinese_format::Chinese;
    ///
    /// let chinese = Chinese {
    ///     logograms: "小猫".to_string(),
    ///     omissible: false
    /// };
    ///
    /// assert_eq!(chinese, "小猫");
    /// ```
    fn eq(&self, other: &&str) -> bool {
        self.logograms == *other
    }
}

impl PartialEq<String> for Chinese {
    /// [Chinese] can be compared with [String].
    ///
    /// ```
    /// use chinese_format::Chinese;
    ///
    /// let chinese = Chinese {
    ///     logograms: "电脑".to_string(),
    ///     omissible: false
    /// };
    ///
    /// assert_eq!(chinese, "电脑".to_string());
    /// ```
    fn eq(&self, other: &String) -> bool {
        self.logograms == *other
    }
}

/// Trait expressing support for infallible conversion to [Chinese].
pub trait ToChinese {
    fn to_chinese(&self, variant: Variant) -> Chinese;
}
