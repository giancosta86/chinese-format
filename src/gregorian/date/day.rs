use super::DayOutOfRange;
use crate::define_multi_register_measure;

define_multi_register_measure!(pub, Day, pub(self), u8, ("号", "號"), "日");

impl Day {
    fn validate(ordinal: u8) -> Result<(), DayOutOfRange> {
        if !(1..=31).contains(&ordinal) {
            return Err(DayOutOfRange(ordinal));
        }

        Ok(())
    }

    /// Creates a day having *formal* (`号`/`號`) unit.
    ///
    /// The *value* must belong to the 1..=31 range.
    pub fn try_new_formal(ordinal: u8) -> Result<Self, DayOutOfRange> {
        Self::validate(ordinal)?;

        Ok(Self {
            value: ordinal,
            formal: true,
        })
    }

    /// Creates a day having *informal* (`日`) unit.
    ///
    /// The *value* must belong to the 1..=31 range.
    pub fn try_new_informal(ordinal: u8) -> Result<Self, DayOutOfRange> {
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
    use pretty_assertions::assert_eq;

    #[test]
    fn format_formal() {
        let formal_day = Day::try_new_formal(5).unwrap();

        assert_eq!(formal_day.to_chinese(Variant::Simplified), "五号");
        assert_eq!(formal_day.to_chinese(Variant::Traditional), "五號");
    }

    #[test]
    fn create_formal_from_invalid_values() {
        assert_eq!(Day::try_new_formal(0), Err(DayOutOfRange(0)));
        assert_eq!(Day::try_new_formal(32), Err(DayOutOfRange(32)));
    }

    #[test]
    fn format_informal() {
        let informal_day = Day::try_new_informal(7).unwrap();

        assert_eq!(informal_day.to_chinese(Variant::Simplified), "七日");
        assert_eq!(informal_day.to_chinese(Variant::Traditional), "七日");
    }

    #[test]
    fn create_informal_from_invalid_values() {
        assert_eq!(Day::try_new_informal(0), Err(DayOutOfRange(0)));
        assert_eq!(Day::try_new_informal(32), Err(DayOutOfRange(32)));
    }
}
