use crate::{
    currency::CurrencyStyle, define_measure, define_multi_register_measure, Chinese, ChineseFormat,
    Count, Financial, FinancialBase, Variant,
};

define_multi_register_measure!(EverydayYuan, pub, Count, "元", "块");

define_measure!(FinancialYuan, pub, Financial, "元");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Yuan {
    pub value: FinancialBase,
    pub style: CurrencyStyle,
}

impl From<Yuan> for FinancialBase {
    fn from(source: Yuan) -> Self {
        source.value
    }
}

impl ChineseFormat for Yuan {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match self.style {
            CurrencyStyle::Everyday { formal } => EverydayYuan {
                value: Count(self.value as u128),
                formal,
            }
            .to_chinese(variant),

            CurrencyStyle::Financial => FinancialYuan(Financial(self.value)).to_chinese(variant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn convert_to_the_underlying_numeric_type() {
        let converted: FinancialBase = Yuan {
            value: 90,
            style: CurrencyStyle::Everyday { formal: false },
        }
        .into();

        assert_eq!(converted, 90);
    }

    #[test]
    fn format_everyday_formal() {
        let two_formal = Yuan {
            value: 2,
            style: CurrencyStyle::Everyday { formal: true },
        };

        assert_eq!(
            two_formal.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "两元".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_formal.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "兩元".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_everyday_informal() {
        let two_informal = Yuan {
            value: 2,
            style: CurrencyStyle::Everyday { formal: false },
        };

        assert_eq!(
            two_informal.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "两块".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_informal.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "兩块".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_financial() {
        let two_financial = Yuan {
            value: 2,
            style: CurrencyStyle::Financial,
        };

        assert_eq!(
            two_financial.to_chinese(Variant::Simplified),
            Chinese {
                logograms: "贰元".to_string(),
                omissible: false
            }
        );

        assert_eq!(
            two_financial.to_chinese(Variant::Traditional),
            Chinese {
                logograms: "貳元".to_string(),
                omissible: false
            }
        );
    }

    #[test]
    fn format_everyday_formal_zero() {
        assert_eq!(
            Yuan {
                value: 0,
                style: CurrencyStyle::Everyday { formal: true }
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零元".to_string(),
                omissible: true
            }
        );
    }

    #[test]
    fn format_everyday_informal_zero() {
        assert_eq!(
            Yuan {
                value: 0,
                style: CurrencyStyle::Everyday { formal: false }
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零块".to_string(),
                omissible: true
            }
        );
    }

    #[test]
    fn format_financial_zero() {
        assert_eq!(
            Yuan {
                value: 0,
                style: CurrencyStyle::Financial
            }
            .to_chinese(Variant::Simplified),
            Chinese {
                logograms: "零元".to_string(),
                omissible: true
            }
        );
    }
}
