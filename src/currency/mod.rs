//! Currencies from all over the world.
//!
//! Each currency is defined by a `struct` named `{CurrencyName}Currency` - and may be built via a dedicated `{CurrencyName}CurrencyBuilder`.
//!
//! **REQUIRED FEATURE**: `currency`.
mod renminbi;

/// Styles adopted when converting currencies to [Chinese](crate::Chinese).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CurrencyStyle {
    /// The currency style suitable for everyday life.
    ///
    /// It can support either a **formal** or **informal** approach to expressing
    /// quantities and units.
    Everyday {
        /// The linguistic register.
        formal: bool,
    },

    /// The currency format required in financial transactions.
    ///
    /// It is always **formal** - often with *anti-falsification measures*, such as the [Financial](crate::Financial) numeric type.
    Financial,
}

pub use renminbi::*;
