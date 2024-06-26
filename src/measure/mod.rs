mod define;
mod define_count;
mod define_multi_register;
mod define_no_copy;

use crate::{Chinese, ChineseFormat, Variant};

/// Trait describing a [value](Self::value) combined with a [unit](Self::unit) of measurement.
pub trait Measure {
    /// The value, convertible to [Chinese].
    fn value(&self) -> &dyn ChineseFormat;

    /// The unit of measurement, convertible to [Chinese].
    fn unit(&self) -> &dyn ChineseFormat;
}

/// [Measure] automatically implements [ChineseFormat],
/// because its [Chinese] translation is obtained by concatenating
/// the logograms of its [value](Measure::value) and the logograms of its [unit](Measure::unit).
///
/// Furthermore, a [Measure] is [omissible](Chinese::omissible)
/// if and only if its [value](Measure::value) is omissible.
///
/// ```
/// use chinese_format::{*, length::*};
///
/// let three_km = Kilometer::new(3);
///
/// assert_eq!(three_km.to_chinese(Variant::Simplified), Chinese {
///     logograms: "三公里".to_string(),
///     omissible: false
/// });
///
///
/// let zero_km = Kilometer::new(0);
///
/// assert_eq!(zero_km.to_chinese(Variant::Simplified), Chinese {
///     logograms: "零公里".to_string(),
///     omissible: true
/// });
/// ```
impl<T: Measure> ChineseFormat for T {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        let value_chinese = self.value().to_chinese(variant);

        let logograms = format!("{}{}", value_chinese, self.unit().to_chinese(variant));

        let omissible = value_chinese.omissible;

        Chinese {
            logograms,
            omissible,
        }
    }
}
