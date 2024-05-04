use crate::{
    currency::CurrencyStyle, define_measure, Chinese, Count, CrateError, CrateResult, Financial,
    FinancialBase, ToChinese, Variant,
};

define_measure!(EverydayCent, pub, Count, "分");

define_measure!(FinancialCent, pub, Financial, "分");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cent {
    value: u8,
    style: CurrencyStyle,
}

impl Cent {
    pub fn try_new(value: u8, style: CurrencyStyle) -> CrateResult<Cent> {
        if value >= 10 {
            return Err(CrateError::CentsOutOfRange(value));
        }

        Ok(Self { value, style })
    }
}

impl From<Cent> for u8 {
    fn from(source: Cent) -> Self {
        source.value
    }
}

impl ToChinese for Cent {
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
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Cent (分) unit" {
            describe "when calling the constructor" {
                describe "with a value less than or equal to 9" {
                    it "should succeed" {
                        eq!(
                            Cent::try_new(9, CurrencyStyle::Financial).unwrap(),
                            Cent {
                                value: 9,
                                style: CurrencyStyle::Financial
                            }
                        );
                    }
                }

                describe "with a value greater than 9" {
                    it "should fail" {
                        eq!(Cent::try_new(10, CurrencyStyle::Financial), Err(CrateError::CentsOutOfRange(10)))
                    }
                }
            }

            describe "converting to the underlying numeric type" {
                it "should work" {
                    let converted: u8 = Cent {
                        value: 5,
                        style: CurrencyStyle::Financial
                    }.into();

                    eq!(converted, 5);
                }
            }


            describe "when converting to everyday formal style" {
                it "should work" {
                    let two_formal = Cent {
                        value: 2,
                        style: CurrencyStyle::Everyday { formal: true }
                    };

                    eq!(two_formal.to_chinese(Variant::Simplified), Chinese {
                        logograms: "两分".to_string(),
                        omissible: false
                    });

                    eq!(two_formal.to_chinese(Variant::Traditional), Chinese {
                        logograms: "兩分".to_string(),
                        omissible: false
                    });
                }
            }

            describe "when converting to everyday informal style" {
                it "should work" {
                    let two_informal = Cent {
                        value: 2,
                        style: CurrencyStyle::Everyday { formal: false }
                    };

                    eq!(two_informal.to_chinese(Variant::Simplified), Chinese {
                        logograms: "两分".to_string(),
                        omissible: false
                    });

                    eq!(two_informal.to_chinese(Variant::Traditional), Chinese {
                        logograms: "兩分".to_string(),
                        omissible: false
                    });
                }
            }

            describe "when converting to financial format" {
                it "should work" {
                    let two_financial = Cent {
                        value: 2,
                        style: CurrencyStyle::Financial
                    };

                    eq!(two_financial.to_chinese(Variant::Simplified), Chinese {
                        logograms: "贰分".to_string(),
                        omissible: false
                    });

                    eq!(two_financial.to_chinese(Variant::Traditional), Chinese {
                        logograms: "貳分".to_string(),
                        omissible: false
                    });
                }
            }

            describe "when converting zero" {
                it "should work" {
                    eq!(
                        Cent {
                            value: 0,
                            style: CurrencyStyle::Everyday { formal: true }
                        }.to_chinese(Variant::Simplified),

                        Chinese {
                            logograms: "零分".to_string(),
                            omissible: true
                        }
                    );

                    eq!(
                        Cent {
                            value: 0,
                            style: CurrencyStyle::Everyday { formal: false }
                        }.to_chinese(Variant::Simplified),

                        Chinese {
                            logograms: "零分".to_string(),
                            omissible: true
                        }
                    );

                    eq!(
                        Cent {
                            value: 0,
                            style: CurrencyStyle::Financial
                        }.to_chinese(Variant::Simplified),

                        Chinese {
                            logograms: "零分".to_string(),
                            omissible: true
                        }
                    );
                }
            }
        }
    }
}
