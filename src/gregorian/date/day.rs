use crate::{define_multi_register_measure, CrateError, CrateResult};

define_multi_register_measure!(pub, Day, pub(self), u8, ("号", "號"), "日");

impl Day {
    fn validate(ordinal: u8) -> CrateResult<()> {
        if !(1..=31).contains(&ordinal) {
            return Err(CrateError::DayOutOfRange(ordinal));
        }

        Ok(())
    }

    /// Creates a day having *formal* (`号`/`號`) unit.
    ///
    /// The *value* must belong to the 1..=31 range.
    pub fn try_new_formal(ordinal: u8) -> CrateResult<Self> {
        Self::validate(ordinal)?;

        Ok(Self {
            value: ordinal,
            formal: true,
        })
    }

    /// Creates a day having *informal* (`日`) unit.
    ///
    /// The *value* must belong to the 1..=31 range.
    pub fn try_new_informal(ordinal: u8) -> CrateResult<Self> {
        Self::validate(ordinal)?;

        Ok(Self {
            value: ordinal,
            formal: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Day struct" {
            describe "formal" {
                describe "when valid" {
                    before {
                        let formal_day = Day::try_new_formal(5).unwrap();
                    }

                    it "should be convertible to Chinese" {
                        eq!(formal_day.to_chinese(Variant::Simplified), "五号");

                        eq!(formal_day.to_chinese(Variant::Traditional), "五號");
                    }
                }

                describe "when converting from invalid values" {
                    it "should fail" {
                        eq!(Day::try_new_formal(0), Err(CrateError::DayOutOfRange(0)));

                        eq!(Day::try_new_formal(32), Err(CrateError::DayOutOfRange(32)));
                    }
                }
            }

            describe "informal" {
                describe "when valid" {
                    before {
                        let informal_day = Day::try_new_informal(7).unwrap();
                    }

                    it "should be convertible to Chinese" {
                        eq!(informal_day.to_chinese(Variant::Simplified), "七日");

                        eq!(informal_day.to_chinese(Variant::Traditional), "七日");
                    }
                }

                describe "when converting from invalid values" {
                    it "should fail" {
                        eq!(Day::try_new_informal(0), Err(CrateError::DayOutOfRange(0)));

                        eq!(Day::try_new_informal(32), Err(CrateError::DayOutOfRange(32)));
                    }
                }
            }
        }
    }
}
