mod cent;
mod dime;
mod yuan;

use self::{cent::Cent, dime::Dime, yuan::Yuan};
use super::CurrencyStyle;
use crate::{
    chinese_vec, Chinese, ChineseVec, CrateResult, EmptyPlaceholder, FinancialBase,
    LingPlaceholder, ToChinese, Variant,
};

/// Builds instances of [RenminbiCurrency] in a simple and consistent way.
///
/// ```
/// use chinese_format::{*, currency::*};
///
/// # fn main() -> GenericResult<()> {
///
/// let formal: RenminbiCurrency =
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(9)
///         .with_dimes(3)
///         .with_cents(8)
///         .with_style(CurrencyStyle::Everyday{formal: true})
///         .build()?;
///
/// assert_eq!(formal.to_chinese(Variant::Simplified), Chinese {
///     logograms: "九元三角八分".to_string(),
///     omissible: false
/// });
///
/// let informal: RenminbiCurrency =
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .with_dimes(4)
///         .with_cents(5)
///         .with_style(CurrencyStyle::Everyday{formal: false})
///         .build()?;
///
/// assert_eq!(informal.to_chinese(Variant::Simplified), Chinese {
///     logograms: "七块四毛五分".to_string(),
///     omissible: false
/// });
///    
/// let financial: RenminbiCurrency =
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(2)
///         .with_dimes(6)
///         .with_cents(1)
///         .with_style(CurrencyStyle::Financial)
///         .build()?;
///
/// assert_eq!(financial.to_chinese(Variant::Simplified), Chinese {
///     logograms: "贰元陆角壹分整".to_string(),
///     omissible: false
/// });
///  
/// # Ok(())
/// # }
/// ```
pub struct RenminbiCurrencyBuilder {
    yuan: FinancialBase,
    dimes: u8,
    cents: u8,
    style: CurrencyStyle,
}

impl RenminbiCurrencyBuilder {
    /// Creates an instance of the builder - its default value.
    ///
    /// In particular, the style defaults
    /// to [CurrencyStyle::Everyday], *formal*.
    ///
    /// ```
    /// use chinese_format::{*, currency::*};
    ///
    /// # fn main() -> GenericResult<()> {
    /// let default =
    ///     RenminbiCurrencyBuilder::new().build()?;
    ///
    /// assert_eq!(default.to_chinese(Variant::Simplified), Chinese {
    ///     logograms: "零元".to_string(),
    ///     omissible: true
    /// });
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the 元 (also named 块) part of the currency.
    pub fn with_yuan(mut self, yuan: FinancialBase) -> Self {
        self.yuan = yuan;
        self
    }

    /// Sets the 角 (also named 毛) part of the currency.
    ///
    /// **Please, note**: the value must be in the 0..=9 range;
    /// otherwise, the [build](Self::build) method will fail.
    pub fn with_dimes(mut self, dimes: u8) -> Self {
        self.dimes = dimes;
        self
    }

    /// Sets the 分 part of the currency.
    ///
    /// **Please, note**: the value must be in the 0..=9 range;
    /// otherwise, the [build](Self::build) method will fail.
    pub fn with_cents(mut self, cents: u8) -> Self {
        self.cents = cents;
        self
    }

    /// Sets the [CurrencyStyle] shared by all the currency units.
    pub fn with_style(mut self, style: CurrencyStyle) -> Self {
        self.style = style;
        self
    }

    /// Build an instance of [RenminbiCurrency] based on the provided settings.
    ///
    /// It may fail - for example, if dimes (角) or cents (分)
    /// are out of range:
    ///
    /// ```
    /// use chinese_format::{*, currency::*};
    ///
    /// let builder: RenminbiCurrencyBuilder =
    ///     RenminbiCurrencyBuilder::new()
    ///         .with_dimes(230);
    ///
    /// assert_eq!(builder.build(), CrateError::DimesOutOfRange(230));
    /// ```
    pub fn build(&self) -> CrateResult<RenminbiCurrency> {
        Ok(RenminbiCurrency {
            yuan: Yuan {
                value: self.yuan,
                style: self.style,
            },

            dimes: Dime::try_new(self.dimes, self.style)?,

            cents: Cent::try_new(self.cents, self.style)?,

            style: self.style,
        })
    }
}

/// The default value contains only 0s,
/// with a *formal* [CurrencyStyle::Everyday].
impl Default for RenminbiCurrencyBuilder {
    fn default() -> Self {
        Self {
            yuan: 0,
            dimes: 0,
            cents: 0,
            style: CurrencyStyle::Everyday { formal: true },
        }
    }
}

/// Renminbi (人民币) currency.
///
/// It must be built using a [RenminbiCurrencyBuilder], and its fields
/// can be accessed via getter functions.
///
/// ```
/// use chinese_format::{*, currency::*};
///
/// # fn main() -> GenericResult<()> {
/// let currency = RenminbiCurrencyBuilder::new()
///     .with_yuan(34)
///     .with_dimes(7)
///     .with_cents(9)
///     .with_style(CurrencyStyle::Everyday { formal: false})
///     .build()?;
///
/// assert_eq!(currency.yuan(), 34);
/// assert_eq!(currency.dimes(), 7);
/// assert_eq!(currency.cents(), 9);
/// assert_eq!(currency.style(), CurrencyStyle::Everyday { formal: false});
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenminbiCurrency {
    yuan: Yuan,
    dimes: Dime,
    cents: Cent,
    style: CurrencyStyle,
}

impl RenminbiCurrency {
    const FINANCIAL_TERMINATOR: &'static str = "整";

    /// Returns the numeric value of the yuan (元) unit.
    pub fn yuan(&self) -> FinancialBase {
        self.yuan.into()
    }

    /// Returns the numeric value of the dime (角) unit.
    pub fn dimes(&self) -> u8 {
        self.dimes.into()
    }

    /// Returns the numeric value of the cents (分) unit.
    pub fn cents(&self) -> u8 {
        self.cents.into()
    }

    /// Returns the currency style.
    pub fn style(&self) -> CurrencyStyle {
        self.style
    }
}

/// [RenminbiCurrency] supports conversion to [Chinese].
///
/// ```
/// use chinese_format::{*, currency::*};
///
/// # fn main() -> GenericResult<()> {
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .with_dimes(4)
///         .with_cents(8)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "七元四角八分"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .with_dimes(4)
///         .with_cents(8)
///         .with_style(CurrencyStyle::Financial)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "柒元肆角捌分整"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .with_dimes(4)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "七元四角"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_dimes(4)
///         .with_cents(8)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "四角八分"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .with_cents(8)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "七元八分"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .with_cents(8)
///         .with_style(CurrencyStyle::Everyday { formal: false })
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "七块零八分"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_yuan(7)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "七元"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_dimes(4)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "四角"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_dimes(4)
///         .with_style(CurrencyStyle::Financial)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "肆角整"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_cents(8)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "八分"
/// );
///
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "零元"
/// );
///  
/// assert_eq!(
///     RenminbiCurrencyBuilder::new()
///         .with_style(CurrencyStyle::Financial)
///         .build()?
///         .to_chinese(Variant::Simplified),
///     "零元整"
/// );
///
/// # Ok(())
/// # }
/// ```
impl ToChinese for RenminbiCurrency {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        let dimes_box: Box<dyn ToChinese> = match self.style {
            CurrencyStyle::Everyday { formal: false } => {
                Box::new(LingPlaceholder::new(&self.dimes))
            }

            _ => Box::new(EmptyPlaceholder::new(&self.dimes)),
        };

        let concatenated_components = ChineseVec::from(
            variant,
            vec![
                &EmptyPlaceholder::new(&self.yuan),
                dimes_box.as_ref(),
                &EmptyPlaceholder::new(&self.cents),
            ],
        )
        .trim_start()
        .collect();

        let coalesced_result = if concatenated_components.omissible {
            self.yuan.to_chinese(variant)
        } else {
            concatenated_components
        };

        match self.style {
            CurrencyStyle::Financial => chinese_vec!(
                variant,
                [coalesced_result.logograms, Self::FINANCIAL_TERMINATOR]
            )
            .collect(),

            _ => coalesced_result,
        }
    }
}
