//! This crate focuses on converting data types to [Chinese],
//! which can be achieved by implementing the [ChineseFormat] trait.
//!
//! As a consequence, the library provides:
//!
//! - **ready-made conversions** for standard data types (integers,
//!   strings, [Option], pairs, ...) - but also for fairly specific types like [DigitSequence](digit_sequence::DigitSequence).
//!
//! - **Gregorian date/time**, in the [gregorian] module, in different formats via [DateBuilder](gregorian::DateBuilder), [LinearTime](gregorian::LinearTime) and [DeltaTime](gregorian::DeltaTime).
//!
//! - **Monetary units**, in the [currency] module - at present, [RenminbiCurrency](currency::RenminbiCurrency) (人民币).
//!
//! - **Dedicated numeric types** - such as [Decimal], [Fraction] and [Sign].
//!
//! - the [ChineseVec] sequence, to simplify the manipulation of _arbitrary
//!   chains of logograms_, as well as **placeholders**.
//!
//! - the [Measure] trait and its related macros - especially [define_measure].
//!
//! # Features
//!
//! The crate supports the following _optional_ features:
//!
//! - `digit-sequence`:
//!
//!   - enables conversions to Chinese for [DigitSequence](https://crates.io/crates/digit-sequence).
//!
//!   - enables the [Decimal] and [IntegerPart] types.
//!
//! - `currency`: enables the whole [currency] module for monetary conversions.
//!
//! - `gregorian`: enables the [gregorian] module for date/time conversions.
//!
//!   _Also enables_: `digit-sequence`.
mod chinese;
mod count;
#[cfg(feature = "digit-sequence")]
mod decimal;
#[cfg(feature = "digit-sequence")]
mod digit_sequences;
mod financial;
mod fraction;
mod integers;
mod left_padder;
mod measure;
mod option;
mod placeholders;
mod sign;
mod strings;
mod tuple;
mod vector;

#[cfg(feature = "currency")]
pub mod currency;
#[cfg(feature = "gregorian")]
pub mod gregorian;
pub mod length;
pub mod weight;

pub use chinese::*;
pub use count::*;
#[cfg(feature = "digit-sequence")]
pub use decimal::*;
pub use financial::*;
pub use fraction::*;
pub use left_padder::*;
pub use measure::*;
pub use placeholders::*;
pub use sign::*;
pub use vector::*;

use std::error::Error;

/// The most generic [Error]-based [Result].
pub type GenericResult<T> = Result<T, Box<dyn Error>>;
