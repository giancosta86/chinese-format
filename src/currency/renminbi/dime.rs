use crate::{
    currency::CurrencyStyle, define_measure, define_multi_register_measure, Chinese, ChineseFormat,
    Count, CrateError, CrateResult, Financial, FinancialBase, Variant,
};

define_multi_register_measure!(EverydayDime, pub, Count, "角", "毛");

define_measure!(FinancialDime, pub, Financial, "角");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dime {
    value: u8,
    style: CurrencyStyle,
}

impl Dime {
    pub fn try_new(value: u8, style: CurrencyStyle) -> CrateResult<Dime> {
        if value >= 10 {
            return Err(CrateError::DimesOutOfRange(value));
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
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Dime (角) unit" {
            describe "when calling the constructor" {
                describe "with a value less than or equal to 9" {
                    it "should succeed" {
                        eq!(
                            Dime::try_new(9, CurrencyStyle::Financial).unwrap(),
                            Dime {
                                value: 9,
                                style: CurrencyStyle::Financial
                            }
                        );
                    }
                }

                describe "with a value greater than 9" {
                    it "should fail" {
                        eq!(Dime::try_new(10, CurrencyStyle::Financial), Err(CrateError::DimesOutOfRange(10)))
                    }
                }
            }

            describe "converting to the underlying numeric type" {
                it "should work" {
                    let converted: u8 = Dime {
                        value: 2,
                        style: CurrencyStyle::Financial
                    }.into();

                    eq!(converted, 2);
                }
            }

            describe "when converting to everyday formal style" {
                it "should work" {
                    let two_formal = Dime {
                        value: 2,
                        style: CurrencyStyle::Everyday { formal: true }
                    };

                    eq!(two_formal.to_chinese(Variant::Simplified), Chinese {
                        logograms: "两角".to_string(),
                        omissible: false
                    });

                    eq!(two_formal.to_chinese(Variant::Traditional), Chinese {
                        logograms: "兩角".to_string(),
                        omissible: false
                    });
                }
            }

            describe "when converting to everyday informal style" {
                it "should work" {
                    let two_informal = Dime {
                        value: 2,
                        style: CurrencyStyle::Everyday { formal: false }
                    };

                    eq!(two_informal.to_chinese(Variant::Simplified), Chinese {
                        logograms: "两毛".to_string(),
                        omissible: false
                    });

                    eq!(two_informal.to_chinese(Variant::Traditional), Chinese {
                        logograms: "兩毛".to_string(),
                        omissible: false
                    });
                }
            }

            describe "when converting to financial format" {
                it "should work" {
                    let two_financial = Dime {
                        value: 2,
                        style: CurrencyStyle::Financial
                    };

                    eq!(two_financial.to_chinese(Variant::Simplified), Chinese {
                        logograms: "贰角".to_string(),
                        omissible: false
                    });

                    eq!(two_financial.to_chinese(Variant::Traditional), Chinese {
                        logograms: "貳角".to_string(),
                        omissible: false
                    });
                }
            }

            describe "when converting zero" {
                it "should work" {
                    eq!(
                        Dime {
                            value: 0,
                            style: CurrencyStyle::Everyday { formal: true }
                        }.to_chinese(Variant::Simplified),

                        Chinese {
                            logograms: "零角".to_string(),
                            omissible: true
                        }
                    );

                    eq!(
                        Dime {
                            value: 0,
                            style: CurrencyStyle::Everyday { formal: false }
                        }.to_chinese(Variant::Simplified),

                        Chinese {
                            logograms: "零毛".to_string(),
                            omissible: true
                        }
                    );

                    eq!(
                        Dime {
                            value: 0,
                            style: CurrencyStyle::Financial
                        }.to_chinese(Variant::Simplified),

                        Chinese {
                            logograms: "零角".to_string(),
                            omissible: true
                        }
                    );
                }
            }
        }
    }
}
