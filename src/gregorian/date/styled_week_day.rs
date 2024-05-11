use super::{WeekDay, WeekFormat};
use crate::{chinese_vec, Chinese, ToChinese, Variant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyledWeekDay {
    pub week_format: WeekFormat,
    pub week_day: WeekDay,
}

impl StyledWeekDay {
    fn ordinal_logogram(&self, variant: Variant) -> String {
        match self.week_day {
            WeekDay::Sunday => match self.week_format {
                WeekFormat::XingQi | WeekFormat::LiBai => "天",
                WeekFormat::Zhou => "日",
            }
            .to_string(),
            _ => (self.week_day as u8).to_chinese(variant).logograms,
        }
    }
}

/// [WeekDay] can be converted to [Chinese].
impl ToChinese for StyledWeekDay {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        chinese_vec!(variant, [self.week_format, self.ordinal_logogram(variant)]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq as eq;
    use speculate2::*;

    speculate! {
        describe "Styled week day" {
            it "should be convertible to Chinese" {
                let friday = StyledWeekDay {
                    week_format: WeekFormat::XingQi,
                    week_day: WeekDay::Friday
                };
                eq!(friday.to_chinese(Variant::Simplified), "星期五");

                let sunday_xing_qi = StyledWeekDay {
                    week_format: WeekFormat::XingQi,
                    week_day: WeekDay::Sunday,
                };
                eq!(sunday_xing_qi.to_chinese(Variant::Simplified), "星期天");

                let sunday_zhou = StyledWeekDay {
                    week_format: WeekFormat::Zhou,
                    week_day: WeekDay::Sunday,
                };
                eq!(sunday_zhou.to_chinese(Variant::Simplified), "周日");

                let wednesday_xing_qi = StyledWeekDay {
                    week_format: WeekFormat::XingQi,
                    week_day: WeekDay::Wednesday,
                };
                eq!(wednesday_xing_qi.to_chinese(Variant::Simplified), "星期三");

                let wednesday_zhou = StyledWeekDay {
                    week_format: WeekFormat::Zhou,
                    week_day: WeekDay::Wednesday,
                };
                eq!(wednesday_zhou.to_chinese(Variant::Simplified), "周三");
            }
        }
    }
}
