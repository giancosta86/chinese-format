use crate::{Chinese, ChineseFormat, Variant};
use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant};

macro_rules! impl_number_to_chinese {
    ($type:ty) => {
        /// Any integer number can be infallibly converted to Chinese.
        ///
        /// Of the Chinese outcomes, only 零 is [omissible](crate::Chinese::omissible).
        impl ChineseFormat for $type {
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
                .expect("Converting an integer to Chinese should never fail!");

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
    use paste::paste;

    macro_rules! assert_chinese {
        (
            $num_description: literal,
            $num: expr,
            $variant_description: ident,
            $variant: expr,
            $expected_chinese: literal
        ) => {
            paste! {
                #[test]
                fn [<format_ $num_description _to_ $variant_description>]() {
                    let chinese = ($num).to_chinese($variant);
                    assert_eq!(chinese, $expected_chinese);
                }
            }
        };
    }

    macro_rules! assert_simplified {
        (
            $num_description: literal,
            $num: expr,
            $expected_chinese: literal
        ) => {
            assert_chinese!(
                $num_description,
                $num,
                simplified,
                Variant::Simplified,
                $expected_chinese
            );
        };

        (
            $num: expr,
            $expected_chinese: literal
        ) => {
            paste! {
                assert_simplified!($num, $num, $expected_chinese);
            }
        };
    }

    macro_rules! assert_traditional {
        (
            $num_description: literal,
            $num: expr,
            $expected_chinese: literal
        ) => {
            assert_chinese!(
                $num_description,
                $num,
                traditional,
                Variant::Traditional,
                $expected_chinese
            );
        };

        (
            $num: expr,
            $expected_chinese: literal
        ) => {
            paste! {
                assert_traditional!($num, $num, $expected_chinese);
            }
        };
    }

    assert_simplified!(0_i128, "零");
    assert_simplified!(1_i128, "一");
    assert_simplified!(10_i128, "十");
    assert_simplified!(17_i128, "十七");
    assert_simplified!(86_i128, "八十六");
    assert_simplified!(100_i128, "一百");
    assert_simplified!(305_i128, "三百零五");
    assert_simplified!(330_i128, "三百三十");
    assert_simplified!(800_i128, "八百");
    assert_simplified!(3_000_i128, "三千");
    assert_simplified!(3_005_i128, "三千零五");
    assert_simplified!(3_017_i128, "三千零一十七");
    assert_simplified!(7_341_i128, "七千三百四十一");
    assert_simplified!(10_000_i128, "一万");
    assert_simplified!(10_008_i128, "一万零八");
    assert_simplified!(
        321_987_653_112_i128,
        "三千二百一十九亿八千七百六十五万三千一百一十二"
    );
    assert_simplified!("minus_58", -58, "负五十八");
    assert_simplified!("i128_min", i128::MIN, "负一百七十涧一千四百一十一沟八千三百四十六穰零四百六十九秭二千三百一十七垓三千一百六十八京七千三百零三兆七千一百五十八亿八千四百一十万五千七百二十八");

    assert_simplified!("u128_max", u128::MAX, "三百四十涧二千八百二十三沟六千六百九十二穰零九百三十八秭四千六百三十四垓六千三百三十七京四千六百零七兆四千三百一十七亿六千八百二十一万一千四百五十五");

    assert_simplified!(98_u128, "九十八");
    assert_simplified!(98_u64, "九十八");
    assert_simplified!(98_u32, "九十八");
    assert_simplified!(98_u16, "九十八");
    assert_simplified!(98_u8, "九十八");

    assert_simplified!(98_i128, "九十八");
    assert_simplified!(98_i64, "九十八");
    assert_simplified!(98_i32, "九十八");
    assert_simplified!(98_i16, "九十八");
    assert_simplified!(98_i8, "九十八");

    assert_traditional!(305, "三百零五");
    assert_traditional!("minus_58", -58, "負五十八");

    #[test]
    fn check_omissible_for_zero() {
        assert!(0.to_chinese(Variant::Simplified).omissible);
        assert!(0.to_chinese(Variant::Traditional).omissible);
    }

    #[test]
    fn check_omissible_for_nonzero() {
        assert!(!7.to_chinese(Variant::Simplified).omissible);
        assert!(!7.to_chinese(Variant::Traditional).omissible);
    }
}
