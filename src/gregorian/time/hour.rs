use crate::{chinese_vec, Chinese, ChineseFormat, Count, Measure, Variant};

const HOUR_UNIT: (&str, &str) = ("点", "點");

/// The generic hour - independent of the specific time format.
pub trait Hour {
    /// The number representing the hour on a clock.
    fn clock_value(&self) -> &Count;
}

/// Can be converted to a [Measure],
/// using its clock value plus the `点`/`點` unit.
impl<T: Hour> Measure for T {
    fn value(&self) -> &dyn ChineseFormat {
        self.clock_value()
    }

    fn unit(&self) -> &dyn ChineseFormat {
        &HOUR_UNIT
    }
}

/// Also, any generic [Box] containing an implementation
/// can be converted to [Chinese].
impl ChineseFormat for Box<dyn Hour> {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        chinese_vec!(variant, [self.clock_value(), HOUR_UNIT]).collect()
    }
}
