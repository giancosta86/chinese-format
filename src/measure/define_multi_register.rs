/// Defines a [Measure](crate::Measure) having both a formal and an informal unit.
///
/// The resulting `struct` is like the one described in [define_measure](crate::define_measure),
/// but with the following differences:
///
/// * it contains 2 fields:
///
///   * `value` - of the underlying type.
///   
///   * `formal` - a [bool] specifying whether the *formal* unit should be used instead of the *informal* unit when converting this measure instance to [Chinese](crate::Chinese).
///
///   You can keep both fields public, or you can add a custom constructor later.
///
/// * the [Measure](crate::Measure) implementation, when asked its [unit](crate::Measure::unit), will return one of the given units according to its `formal` field.
///
/// ```
/// use chinese_format::*;
///
/// define_multi_register_measure!(pub, Jiao, pub, Count, "角", "毛");
///
/// //Formal
/// let two_formal = Jiao { value: Count(2), formal: true };
///
/// assert_eq!(two_formal.value().to_chinese(Variant::Simplified), "两");
/// assert_eq!(two_formal.value().to_chinese(Variant::Traditional), "兩");
///
/// assert_eq!(two_formal.unit().to_chinese(Variant::Simplified), "角");
/// assert_eq!(two_formal.unit().to_chinese(Variant::Traditional), "角");
///
/// assert_eq!(two_formal.to_chinese(Variant::Simplified), Chinese{
///     logograms: "两角".to_string(),
///     omissible: false
/// });
/// assert_eq!(two_formal.to_chinese(Variant::Traditional), "兩角");
///
/// assert_eq!(two_formal.value(), 2);
/// assert_eq!(*two_formal.value_as_ref(), 2);
/// assert!(two_formal.formal);
///
/// let two_formal_count: Count = two_formal.into();
/// assert_eq!(two_formal_count, Count(2));
///
/// let two_formal_copy = two_formal;
/// assert!(two_formal_copy == two_formal);
///
///
/// //Informal
/// let two_informal = Jiao { value: Count(2), formal: false };
///
/// assert_eq!(two_informal.value().to_chinese(Variant::Simplified), "两");
/// assert_eq!(two_informal.value().to_chinese(Variant::Traditional), "兩");
///
/// assert_eq!(two_informal.unit().to_chinese(Variant::Simplified), "毛");
/// assert_eq!(two_informal.unit().to_chinese(Variant::Traditional), "毛");
///
/// assert_eq!(two_informal.to_chinese(Variant::Simplified), Chinese{
///     logograms: "两毛".to_string(),
///     omissible: false
/// });
/// assert_eq!(two_informal.to_chinese(Variant::Traditional), "兩毛");
///     
/// assert_eq!(two_informal.value(), 2);
/// assert_eq!(*two_informal.value_as_ref(), 2);
/// assert!(!two_informal.formal);
///
/// let two_informal_count: Count = two_informal.into();
/// assert_eq!(two_informal_count, Count(2));
///
/// let two_informal_copy = two_informal;
/// assert!(two_informal_copy == two_informal);
///
///
/// let zero = Jiao { value: Count(0), formal: true };
/// assert_eq!(zero.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零角".to_string(),
///     omissible: true
/// });
/// ```
#[macro_export]
macro_rules! define_multi_register_measure {
    (
        //The visibility of the type to create.
        $type_visibility: vis,

        //The name of the type to create.
        $type: ident,

        //The visibility of the fields.
        $field_visibility: vis,

        //The underlying type - implementing the required traits.
        $field_type: ty,

        //Formal unit - implementing ToChinese.
        $formal_unit: expr,

        //Informal unit - implementing ToChinese.
        $informal_unit: expr
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $type_visibility struct $type {
            $field_visibility value: $field_type,
            $field_visibility formal: bool,
        }

        impl $type {
            pub fn value_as_ref(&self) -> &$field_type {
                &self.value
            }

            pub fn value(&self) -> $field_type {
                self.value
            }
        }

        impl $crate::Measure for $type {
            fn value<'a>(&'a self) -> Box<dyn 'a + $crate::ToChinese> {
                Box::new(self.value)
            }

            fn unit<'a>(&'a self) -> Box<dyn 'a + $crate::ToChinese> {
                Box::new(
                    if self.formal {
                        $formal_unit
                    } else {
                        $informal_unit
                    }
                )
            }
        }

        impl From<$type> for $field_type {
            fn from(source: $type) -> Self {
                source.value
            }
        }
    };
}
