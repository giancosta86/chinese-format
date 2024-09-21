use crate::{
    currency::{CentsOutOfRange, CurrencyStyle},
    define_measure, Chinese, ChineseFormat, Count, Financial, FinancialBase, Variant,
};

define_measure!(EverydayCent, pub, Count, "分");

define_measure!(FinancialCent, pub, Financial, "分");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cent {
    value: u8,
    style: CurrencyStyle,
}

impl Cent {
    pub fn try_new(value: u8, style: CurrencyStyle) -> Result<Cent, CentsOutOfRange> {
        if value >= 10 {
            return Err(CentsOutOfRange(value));
        }

        Ok(Self { value, style })
    }
}

impl From<Cent> for u8 {
    fn from(source: Cent) -> Self {
        source.value
    }
}

impl ChineseFormat for Cent {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match self.style {
            CurrencyStyle::Everyday { formal: _ } => {
                EverydayCent(Count(self.value as u128)).to_chinese(variant)
            }

            CurrencyStyle::Financial => {
                FinancialCent(Financial(self.value as FinancialBase)).to_chinese(variant)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_from_value_less_than_or_equal_to_9() {
        assert_eq!(
            Cent::try_new(9, CurrencyStyle::Financial).unwrap(),
            Cent {
                value: 9,
                style: CurrencyStyle::Financial
            }
        );
    }

    #[test]
    fn create_from_value_greater_than_9() {
        assert_eq!(
            Cent::try_new(10, CurrencyStyle::Financial),
            Err(CentsOutOfRange(10))
        )
    }

    #[test]
    fn convert_to_the_underlying_numeric_type() {
        let converted: u8 = Cent {
            value: 5,
            style: CurrencyStyle::Financial,
        }
        .into();

        assert_eq!(converted, 5);
    }

    #[test]
    fn format_everyday_formal() {
        let two_formal = Cent {
            value: 2,
            style: CurrencyStyle::Everyday { formal: true },
        };

        assert_eq!(
            two_formal.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "两分".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_formal.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "兩分".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_everyday_informal() {
        let two_informal = Cent {
            value: 2,
            style: CurrencyStyle::Everyday { formal: false },
        };

        assert_eq!(
            two_informal.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "两分".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_informal.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "兩分".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_financial() {
        let two_financial = Cent {
            value: 2,
            style: CurrencyStyle::Financial,
        };

        assert_eq!(
            two_financial.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "贰分".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_financial.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "貳分".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_everyday_formal_zero() {
        assert_eq!(
            Cent {
                value: 0,
                style: CurrencyStyle::Everyday { formal: true }
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零分".to_string(),
                omissible: true
            }
        );
    }

    #[test]
    fn format_everyday_informal_zero() {
        assert_eq!(
            Cent {
                value: 0,
                style: CurrencyStyle::Everyday { formal: false }
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零分".to_string(),
                omissible: true
            }
        );
    }

    #[test]
    fn format_financial_zero() {
        assert_eq!(
            Cent {
                value: 0,
                style: CurrencyStyle::Financial
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零分".to_string(),
                omissible: true
            }
        );
    }
}
