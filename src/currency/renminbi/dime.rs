use crate::{
    currency::{CurrencyStyle, DimesOutOfRange},
    define_measure, define_multi_register_measure, Chinese, ChineseFormat, Count, Financial,
    FinancialBase, Variant,
};

define_multi_register_measure!(EverydayDime, pub, Count, "角", "毛");

define_measure!(FinancialDime, pub, Financial, "角");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dime {
    value: u8,
    style: CurrencyStyle,
}

impl Dime {
    pub fn try_new(value: u8, style: CurrencyStyle) -> Result<Dime, DimesOutOfRange> {
        if value >= 10 {
            return Err(DimesOutOfRange(value));
        }

        Ok(Self { value, style })
    }
}

impl From<Dime> for u8 {
    fn from(source: Dime) -> Self {
        source.value
    }
}

impl ChineseFormat for Dime {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match self.style {
            CurrencyStyle::Everyday { formal } => EverydayDime {
                value: Count(self.value as u128),
                formal,
            }
            .to_chinese(variant),

            CurrencyStyle::Financial => {
                FinancialDime(Financial(self.value as FinancialBase)).to_chinese(variant)
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
            Dime::try_new(9, CurrencyStyle::Financial).unwrap(),
            Dime {
                value: 9,
                style: CurrencyStyle::Financial
            }
        );
    }

    #[test]
    fn create_from_value_greater_than_9() {
        assert_eq!(
            Dime::try_new(10, CurrencyStyle::Financial),
            Err(DimesOutOfRange(10))
        )
    }

    #[test]
    fn convert_to_the_underlying_numeric_type() {
        let converted: u8 = Dime {
            value: 2,
            style: CurrencyStyle::Everyday { formal: false },
        }
        .into();

        assert_eq!(converted, 2);
    }

    #[test]
    fn format_everyday_formal() {
        let two_formal = Dime {
            value: 2,
            style: CurrencyStyle::Everyday { formal: true },
        };

        assert_eq!(
            two_formal.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "两角".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_formal.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "兩角".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_everyday_informal() {
        let two_informal = Dime {
            value: 2,
            style: CurrencyStyle::Everyday { formal: false },
        };

        assert_eq!(
            two_informal.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "两毛".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_informal.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "兩毛".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_financial() {
        let two_financial = Dime {
            value: 2,
            style: CurrencyStyle::Financial,
        };

        assert_eq!(
            two_financial.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "贰角".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_financial.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "貳角".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_everyday_formal_zero() {
        assert_eq!(
            Dime {
                value: 0,
                style: CurrencyStyle::Everyday { formal: true }
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零角".to_string(),
                omissible: true
            }
        );
    }

    #[test]
    fn format_everyday_informal_zero() {
        assert_eq!(
            Dime {
                value: 0,
                style: CurrencyStyle::Everyday { formal: false }
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零毛".to_string(),
                omissible: true
            }
        );
    }

    #[test]
    fn format_financial_zero() {
        assert_eq!(
            Dime {
                value: 0,
                style: CurrencyStyle::Financial
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零角".to_string(),
                omissible: true
            }
        );
    }
}
