//! This crate focuses on converting data types to [Chinese],
//! which can be achieved by implementing the [ToChinese] trait.
//!
//! As a consequence, it provides:
//!
//! * **ready-made conversions** for standard data types such as integers,
//!   strings, ... - but also for fairly specific types like `DigitSequence`.
//!
//! * **custom data types** - such as [Sign] and [Fraction].
//!
//! * the [Measure] trait and its related macros - especially [define_measure].
//!
//! * the [ChineseVec] sequence, to simplify the manipulation of *complex
//!   chains of logograms*.
mod chinese;
mod count;
#[cfg(feature = "digit-sequence")]
mod digit_sequences;
mod financial;
mod fraction;
mod integers;
mod measure;
mod option;
mod placeholders;
mod result;
mod sign;
mod strings;
mod tuple;
mod vector;

#[cfg(feature = "currency")]
pub mod currency;
pub mod length;
pub mod weight;

pub use chinese::*;
pub use count::*;
pub use financial::*;
pub use fraction::*;
pub use measure::*;
pub use placeholders::*;
pub use result::*;
pub use sign::*;
pub use vector::*;
