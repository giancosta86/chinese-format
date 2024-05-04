/// Defines a `struct` implementing [Measure](crate::Measure) having a [Count](crate::Count) value.
///
/// Also includes a `new` constructor:
///
/// ```
/// use chinese_format::*;
///
/// //Actually, this is already provided by crate::weight::HalfKilogram.
/// define_count_measure!(pub, Jin, "斤");
///
/// let four_jin = Jin::new(4);
///
/// assert_eq!(four_jin.value().to_chinese(Variant::Simplified), "四");
/// assert_eq!(four_jin.to_chinese(Variant::Simplified), "四斤");
///
/// let four_count: Count = four_jin.into();
/// assert_eq!(four_count, Count(4));
/// ```
#[macro_export]
macro_rules! define_count_measure {
    (
        //The name of the PRIVATE type to create.
        $type: ident,

        //The unit - implementing ToChinese.
        $unit: expr
    ) => {
        $crate::define_count_measure!(
            ,
            $type,
            $unit
        );
    };

    (
        //The visibility of the type to create.
        $type_visibility: vis,

        //The name of the type to create.
        $type: ident,

        //The unit - implementing ToChinese.
        $unit: expr
    ) => {
        $crate::define_measure!($type_visibility, $type, pub(self), $crate::Count, $unit);

        impl $type {
            pub fn new(value: $crate::CountBase) -> $type {
                $type($crate::Count(value))
            }
        }
    };
}
