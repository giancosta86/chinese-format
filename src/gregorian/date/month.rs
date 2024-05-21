use super::MonthOutOfRange;
use crate::define_measure;

define_measure!(pub, Month, pub(self), u8, "月");

/// [Month] can be obtained from [u8], for values in the 1..=12 range.
impl TryFrom<u8> for Month {
    type Error = MonthOutOfRange;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=12).contains(&value) {
            return Err(MonthOutOfRange(value));
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Month struct" {
            describe "conversion from unsigned" {
                describe "when the value is in range" {
                    it "should work" {
                        let month: Month = 3.try_into().unwrap();

                        eq!(month.to_chinese(Variant::Simplified), Chinese {
                            logograms: "三月".to_string(),
                            omissible: false
                        });
                    }
                }

                describe "when the value is too low" {
                    it "should fail" {
                        let too_small_result: Result<Month, MonthOutOfRange> = 0.try_into();

                        eq!(too_small_result, Err(MonthOutOfRange(0)));
                    }
                }

                describe "when the value is too high" {
                    it "should fail" {
                        let too_high_result: Result<Month, MonthOutOfRange> = 13.try_into();

                        eq!(too_high_result, Err(MonthOutOfRange(13)));
                    }
                }
            }
        }
    }
}
