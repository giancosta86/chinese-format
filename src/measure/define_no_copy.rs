/// Like [define_measure](crate::define_measure), but the generated struct is not [Copy].
///
/// As a consequence, the underlying type does not need to implement [Copy],
/// and the `value()` method returns a *clone*.
///
/// ```
/// use chinese_format::*;
///
/// //String is NOT Copy, so neither can be any measure depending on it.
/// define_no_copy_measure!(pub, Nian, pub(self), String, "年");
///
/// let year_string: String = "2009".to_string();
/// let year = Nian(year_string.clone());
///
/// assert_eq!(year.value().to_chinese(Variant::Simplified), "2009");
/// assert_eq!(year.value().to_chinese(Variant::Traditional), "2009");
///     
/// assert_eq!(year.unit().to_chinese(Variant::Simplified), "年");
/// assert_eq!(year.unit().to_chinese(Variant::Traditional), "年");
///
/// assert_eq!(
///     year.to_chinese(Variant::Simplified),
///     Chinese {
///         logograms: "2009年".to_string(),
///         omissible: false
///     }
/// );
/// assert_eq!(
///     year.to_chinese(Variant::Traditional),
///     Chinese {
///         logograms: "2009年".to_string(),
///         omissible: false
///     }
/// );
///
/// let year_clone = year.clone();
/// assert!(year_clone == year);
///
/// let converted: String = year.into();
/// assert_eq!(converted, year_string);
///
///     
/// let empty_year = Nian(String::new());
/// assert_eq!(empty_year.to_chinese(Variant::Simplified), Chinese {
///     logograms: "年".to_string(),
///     omissible: true
/// });
/// assert_eq!(empty_year.to_chinese(Variant::Traditional), Chinese {
///     logograms: "年".to_string(),
///     omissible: true
/// });
/// ```
///
/// It is worth remembering that the generated struct will not support
/// copy on assignment, just as expected:
///
/// ```compile_fail
/// use chinese_format::*;
///
/// define_no_copy_measure!(pub, Nian, pub(self), String, "年");
///
/// let year = Nian("1999".to_string());
/// let year_move = year;
///
/// //This fails because assignment does not copy
/// assert!(year_move == year);
/// ```
#[macro_export]
macro_rules! define_no_copy_measure {
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
        $crate::define_no_copy_measure(
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
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
