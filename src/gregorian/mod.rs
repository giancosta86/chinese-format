//! Chinese translation of the Gregorian calendar.
//!
//! In particular:
//!
//! * [DateBuilder] is the entry point for creating *dates* of different formats.
//!
//! * [LinearTime] and [DeltaTime] are required to create *time expressions*; both must be built by hand, via the related components like [Hour12], [Hour24], [Minute], [Second].
//!
//! **REQUIRED FEATURE**: `gregorian`.  

mod date;
mod time;

pub use date::*;
pub use time::*;
