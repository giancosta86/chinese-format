/// Defines a new **placeholder** type.
///
/// Every *placeholder* wraps a [ToChinese](crate::ToChinese) instance and implements [ToChinese](crate::ToChinese) as follows:
///
///   * first, convert the wrapped [ToChinese](crate::ToChinese) instance to [Chinese](crate::Chinese)
///
///   * if the result is not [omissible](crate::Chinese::omissible), return it unchanged
///
///     * otherwise, return a [Chinese](crate::Chinese) that is still [omissible](crate::Chinese::omissible),
///       but whose [logograms](crate::Chinese::logograms) are the obtained by calling `$replacement_logograms.to_string()`.
///
/// Anyway, the `omissible` property of any placeholder always reflects the [Chinese](crate::Chinese) produced by the wrapped [ToChinese](crate::ToChinese) instance.
///
/// Last but not least, every placeholder can be built via its `new()` constructor.
///
/// # Ready-made placeholders
///
/// ## [LingPlaceholder]
///
/// Uses `零` in lieu of [omissible](crate::Chinese::omissible) logograms:
///
/// ```
/// use chinese_format::*;
///
/// let placeholder_with_non_omissible = LingPlaceholder::new(&"二九零四");
///
/// assert_eq!(placeholder_with_non_omissible.to_chinese(Variant::Simplified), Chinese {
///     logograms: "二九零四".to_string(),
///     omissible: false
/// });
///
/// assert_eq!(placeholder_with_non_omissible.to_chinese(Variant::Traditional), "二九零四");
///
///
/// let placeholder_with_omissible = LingPlaceholder::new(&"");
///
/// assert_eq!(placeholder_with_omissible.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零".to_string(),
///     omissible: true
/// });
///
/// assert_eq!(placeholder_with_omissible.to_chinese(Variant::Traditional), "零");
/// ```
///
/// ## [EmptyPlaceholder]
///
/// Uses an *empty string* in lieu of [omissible](crate::Chinese::omissible) logograms:
///
/// ```
/// use chinese_format::*;
///
/// let placeholder_with_non_omissible = EmptyPlaceholder::new(&"二九零四");
///
/// assert_eq!(placeholder_with_non_omissible.to_chinese(Variant::Simplified), Chinese {
///     logograms: "二九零四".to_string(),
///     omissible: false
/// });
///
/// assert_eq!(placeholder_with_non_omissible.to_chinese(Variant::Traditional), "二九零四");
///
///
/// let placeholder_with_omissible = EmptyPlaceholder::new(&"");
///
/// assert_eq!(placeholder_with_omissible.to_chinese(Variant::Simplified), Chinese {
///     logograms: "".to_string(),
///     omissible: true
/// });
///
/// assert_eq!(placeholder_with_omissible.to_chinese(Variant::Traditional), "");
/// ```
#[macro_export]
macro_rules! define_string_placeholder {
    (
        //The visibility of the type to create.
        $type_visibility: vis,

        //The name of the type to create.
        $type: ident,

        //String of logograms in lieu of wrapped omissible ones.
        $replacement_logograms: expr,

        //The RustDoc string for the type to create.
        $doc_string: literal
    ) => {
        #[doc = $doc_string]
        $type_visibility struct $type<'a>(&'a dyn $crate::ToChinese);

        impl <'a> $type<'a> {
            pub fn new<T: 'static + $crate::ToChinese>(value: &'a T) -> Self {
                Self(value)
            }
        }

        impl <'a> $crate::ToChinese for $type<'a>{
            fn to_chinese(&self, variant: $crate::Variant) -> $crate::Chinese {
                let wrapped_chinese = self.0.to_chinese(variant);

                let result_logograms = if wrapped_chinese.omissible {
                    $replacement_logograms.to_string()
                } else {
                    wrapped_chinese.logograms
                };

                $crate::Chinese {
                    logograms:  result_logograms,
                    omissible: wrapped_chinese.omissible
                }
            }
        }
    };
}

define_string_placeholder!(
    pub,
    LingPlaceholder,
    "零",
    "[Placeholder](crate::define_string_placeholder) replacing an *omissible* value with `零`."
);

define_string_placeholder!(
    pub,
    EmptyPlaceholder,
    "",
    "[Placeholder](crate::define_string_placeholder) replacing an *omissible* value with an empty string."
);
