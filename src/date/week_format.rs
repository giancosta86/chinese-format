use crate::{Chinese, ToChinese, Variant};

/// The Chinese ways to describe a week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeekFormat {
    /// `星期`
    XingQi,

    /// `周`
    Zhou,

    /// `礼拜`/`禮拜`
    LiBai,
}

/// The default for [WeekFormat].
impl Default for WeekFormat {
    fn default() -> Self {
        Self::XingQi
    }
}

/// Each [WeekFormat] can be converted to [Chinese]:
impl ToChinese for WeekFormat {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        match self {
            Self::XingQi => "星期".to_chinese(variant),
            Self::Zhou => "周".to_chinese(variant),
            Self::LiBai => ("礼拜", "禮拜").to_chinese(variant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "WeekFormat enum" {
            it "should be convertible to Chinese" {
                eq!(WeekFormat::XingQi.to_chinese(Variant::Simplified), "星期");
                eq!(WeekFormat::XingQi.to_chinese(Variant::Traditional), "星期");

                eq!(WeekFormat::Zhou.to_chinese(Variant::Simplified), "周");
                eq!(WeekFormat::Zhou.to_chinese(Variant::Traditional), "周");

                eq!(WeekFormat::LiBai.to_chinese(Variant::Simplified), "礼拜");
                eq!(WeekFormat::LiBai.to_chinese(Variant::Traditional), "禮拜");
            }

            it "should have a default value" {
                eq!(WeekFormat::default().to_chinese(Variant::Simplified), "星期");
                eq!(WeekFormat::default().to_chinese(Variant::Traditional),"星期");
            }
        }
    }
}
