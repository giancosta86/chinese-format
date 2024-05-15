/// Defines a struct implementing [Measure](crate::Measure).
///
/// The struct uses the provided name, visibility and underlying type -
/// which, in turn, must implement both [Copy] and [ChineseFormat](crate::ChineseFormat), as well as [Debug], [Clone], [PartialEq], [Eq], [PartialOrd], [Ord] and [Hash].
///
/// The generated data type automatically comes with:
///
/// * a dedicated implementation of [Measure](crate::Measure).
///
/// * the derived implementations of [Debug], [Clone], [Copy],
/// [PartialEq], [Eq], [PartialOrd], [Ord], [Hash].
///
/// * a [From] conversion to the underlying data type
///
/// ```
/// use chinese_format::*;
///
/// //Actually, this is already provided by crate::length::Centimeter.
/// define_measure!(pub, LiMi, pub(self), Count, ("厘米", "釐米"));
///
/// let two = LiMi(Count(2));
///
/// assert_eq!(two.value().to_chinese(Variant::Simplified), "两");
/// assert_eq!(two.value().to_chinese(Variant::Traditional), "兩");
///     
/// assert_eq!(two.unit().to_chinese(Variant::Simplified), "厘米");
/// assert_eq!(two.unit().to_chinese(Variant::Traditional), "釐米");
///
/// assert_eq!(
///     two.to_chinese(Variant::Simplified),
///     Chinese {
///         logograms: "两厘米".to_string(),
///         omissible: false
///     }
/// );
/// assert_eq!(
///     two.to_chinese(Variant::Traditional),
///     Chinese {
///         logograms: "兩釐米".to_string(),
///         omissible: false
///     }
/// );
///
///     let two_copy = two;
///     assert!(two_copy == two);
///
///     let converted: Count = two.into();
///     assert_eq!(converted, Count(2));
///
///     
///     let zero = LiMi(Count(0));
///     assert_eq!(zero.to_chinese(Variant::Simplified), Chinese {
///         logograms: "零厘米".to_string(),
///         omissible: true
///     });
///     assert_eq!(zero.to_chinese(Variant::Traditional), Chinese {
///         logograms: "零釐米".to_string(),
///         omissible: true
///     });
/// ```
#[macro_export]
macro_rules! define_measure {
    (
        //The name of the PRIVATE type to create.
        $type: ident,

        //The field visibility.
        $field_visibility: vis,

        //The field type - implementing the required traits.
        $field_type: ty,

        //The unit - implementing ChineseFormat.
        $unit: expr
    ) => {
        $crate::define_measure!(
            ,
            $type,
            $field_visibility,
            $field_type,
            $unit
        );
    };

    (
        //The visibility of the type to create.
        $type_visibility: vis,

        //The name of the type to create.
        $type: ident,

        //The field visibility.
        $field_visibility: vis,

        //The field type - implementing the required traits.
        $field_type: ty,

        //The unit - implementing ChineseFormat.
        $unit: expr
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $type_visibility struct $type($field_visibility $field_type);

        impl $crate::Measure for $type {
            fn value(&self) -> &dyn $crate::ChineseFormat {
                &self.0
            }

            fn unit(&self) -> &dyn $crate::ChineseFormat {
                &$unit
            }
        }

        impl From<$type> for $field_type {
            fn from(value: $type) -> Self {
                value.0
            }
        }
    };
}
