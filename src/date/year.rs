use crate::define_no_copy_measure;
use digit_sequence::DigitSequence;

define_no_copy_measure!(pub, Year, pub(self), DigitSequence, "年");

impl Year {
    /// Determines whether the year is leap - according to the standard algorithm.
    pub fn is_leap(&self) -> bool {
        let value: u16 = self.into();

        (value % 4 == 0) && (value % 100 != 0 || value % 400 == 0)
    }
}

/// [Year] can be infallibly obtained from [u16].
impl From<u16> for Year {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

/// &[Year] can be infallibly converted to [u16].
impl From<&Year> for u16 {
    fn from(source: &Year) -> Self {
        (&source.0)
            .try_into()
            .expect("A year can only be built from u16")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Year struct" {
            describe "according to leap test" {
                describe "when not divisible by 4" {
                    it "should not be leap" {
                        let not_divisible_by_4: Year = 2023.into();
                        assert!(!not_divisible_by_4.is_leap());
                    }
                }

                describe "when divisible by 4 but not 100" {
                    it "should be leap" {
                        let divisible_by_4_not_by_100: Year = 2024.into();
                        assert!(divisible_by_4_not_by_100.is_leap());
                    }
                }

                describe "when divisible by 100 but not 400" {
                    it "should not be leap" {
                        let divisible_by_100_not_by_400: Year = 1300.into();
                        assert!(!divisible_by_100_not_by_400.is_leap());
                    }
                }

                describe "when divisible by 400" {
                    it "should be leap" {
                        let divisible_by_400: Year = 2000.into();
                        assert!(divisible_by_400.is_leap());
                    }
                }
            }

            describe "conversion from unsigned" {
                it "should work" {
                    let year: Year = 1992.into();
                    eq!(
                        year.to_chinese(Variant::Simplified),
                        "一九九二年"
                    );
                }
            }

            describe "conversion to unsigned" {
                it "should work" {
                    let year: Year = 1492.into();
                    let converted: u16 = (&year).into();
                    eq!(converted, 1492);
                }
            }
        }
    }
}
