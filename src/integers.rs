use crate::{Chinese, ToChinese, Variant};
use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant};

macro_rules! impl_number_to_chinese {
    ($type:ty) => {
        /// Any integer number can be infallibly converted to Chinese.
        ///
        /// Of the Chinese outcomes, only 零 is [omissible](crate::Chinese::omissible).
        impl ToChinese for $type {
            fn to_chinese(&self, variant: Variant) -> Chinese {
                let logograms: String = chinese_number::NumberToChinese::to_chinese(
                    *self,
                    match variant {
                        Variant::Simplified => ChineseVariant::Simple,
                        Variant::Traditional => ChineseVariant::Traditional,
                    },
                    ChineseCase::Lower,
                    ChineseCountMethod::TenThousand,
                )
                .expect("Converting a Chinese number should never fail!");

                Chinese {
                    logograms,
                    omissible: *self == 0,
                }
            }
        }
    };
}

impl_number_to_chinese!(u128);
impl_number_to_chinese!(u64);
impl_number_to_chinese!(u32);
impl_number_to_chinese!(u16);
impl_number_to_chinese!(u8);

impl_number_to_chinese!(i128);
impl_number_to_chinese!(i64);
impl_number_to_chinese!(i32);
impl_number_to_chinese!(i16);
impl_number_to_chinese!(i8);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
            describe "Converting an integer" {
                describe "to simplified Chinese" {
                    fn test_signed(source: i128, expected_chinese: &str) {
                        let chinese = source.to_chinese(Variant::Simplified);
                        eq!(chinese, expected_chinese);
                    }

                    fn test_unsigned(source: u128, expected_chinese: &str) {
                        let chinese = source.to_chinese(Variant::Simplified);
                        eq!(chinese, expected_chinese);
                    }

                    it "should convert 0" {
                        test_signed(0, "零");
                    }

                    it "should convert 1" {
                       test_signed(1, "一");
                    }

                    it "should convert 10" {
                        test_signed(10, "十");
                    }

                    it "should convert 17" {
                        test_signed(17, "十七");
                    }

                    it "should convert 86" {
                        test_signed(86, "八十六");
                    }

                    it "should convert 100" {
                        test_signed(100, "一百");
                    }

                    it "should convert 305" {
                        test_signed(305, "三百零五");
                    }

                    it "should convert 330" {
                        test_signed(330, "三百三十");
                    }

                    it "should convert 800" {
                        test_signed(800, "八百");
                    }

                    it "should convert 3000" {
                        test_signed(3000, "三千");
                    }

                    it "should convert 3005" {
                        test_signed(3005, "三千零五");
                    }

                    it "should convert 3017" {
                        test_signed(3017, "三千零一十七");
                    }

                    it "should convert 7341" {
                        test_signed(7341, "七千三百四十一");
                    }

                    it "should convert 10_000" {
                        test_signed(10_000, "一万");
                    }

                    it "should convert 10_008" {
                        test_signed(10_008, "一万零八");
                    }

                    it "should convert 321_987_653_112" {
                        test_signed(321_987_653_112, "三千二百一十九亿八千七百六十五万三千一百一十二");
                    }

                    it "should convert u128::MAX" {
                        test_unsigned(u128::MAX, "三百四十涧二千八百二十三沟六千六百九十二穰零九百三十八秭四千六百三十四垓六千三百三十七京四千六百零七兆四千三百一十七亿六千八百二十一万一千四百五十五");
                    }

                    it "should convert -58" {
                        test_signed(-58, "负五十八");
                    }

                    it "should convert i128::MIN" {
                        test_signed(i128::MIN, "负一百七十涧一千四百一十一沟八千三百四十六穰零四百六十九秭二千三百一十七垓三千一百六十八京七千三百零三兆七千一百五十八亿八千四百一十万五千七百二十八");
                    }

                    describe "when converting different types" {
                        macro_rules! test_case {
                            ($source: expr, $expected_chinese: expr) => {
                                let chinese = $source.to_chinese(Variant::Simplified);

                                eq!(
                                    chinese,
                                    $expected_chinese
                                );
                            }
                        }

                        it "should convert u128" {
                            test_case!(98u128, "九十八");
                        }

                        it "should convert u64" {
                            test_case!(98u64, "九十八");
                        }

                        it "should convert u32" {
                            test_case!(98u32, "九十八");
                        }

                        it "should convert u16" {
                            test_case!(98u16, "九十八");
                        }

                        it "should convert u8" {
                            test_case!(98u8, "九十八");
                        }

                        it "should convert i128" {
                            test_case!(98i128, "九十八");
                        }

                        it "should convert i64" {
                            test_case!(98i64, "九十八");
                        }

                        it "should convert i32" {
                            test_case!(98i32, "九十八");
                        }

                        it "should convert i16" {
                            test_case!(98i16, "九十八");
                        }

                        it "should convert i8" {
                            test_case!(98i8, "九十八");
                        }
                    }
                }

                describe "to traditional Chinese" {
                    fn test_case(source: i128, expected_chinese: &str) {
                        let chinese = source.to_chinese(Variant::Traditional);
                        eq!(chinese, expected_chinese);
                    }

                    it "should convert 305" {
                        test_case(305, "三百零五");
                    }

                    it "should convert -58" {
                        test_case(-58, "負五十八");
                    }
                }

                describe "in terms of omissible" {
                    describe "when converting 0" {
                        it "should be omissible" {
                            assert!(0.to_chinese(Variant::Simplified).omissible);
                            assert!(0.to_chinese(Variant::Traditional).omissible);
                        }
                    }

                    describe "when converting non-zero" {
                        it "should NOT be omissible" {
                            assert!(!7.to_chinese(Variant::Simplified).omissible);
                            assert!(!7.to_chinese(Variant::Traditional).omissible);
                        }
                    }
                }
            }
    }
}
