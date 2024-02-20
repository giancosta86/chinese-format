//! Weight measures.
//!
//! [HalfKilogram] seems especially used in Chinese, although [Kilogram] is available as well.
//!
//! ```
//! use chinese_format::{*, weight::*};
//!
//! let two_kg = Kilogram::new(2);
//!
//! assert_eq!(
//!     two_kg.to_chinese(Variant::Simplified),
//!     Chinese {
//!         logograms: "两公斤".to_string(),
//!         omissible: false
//!     }
//! );
//!
//! assert_eq!(
//!     two_kg.to_chinese(Variant::Traditional),
//!     Chinese {
//!         logograms: "兩公斤".to_string(),
//!         omissible: false
//!     }
//! );
//!
//!
//! let zero_hkg = HalfKilogram::new(0);
//!
//! assert_eq!(
//!     zero_hkg.to_chinese(Variant::Simplified),
//!     Chinese {
//!         logograms: "零斤".to_string(),
//!         omissible: true
//!     }
//! );
//!
//! assert_eq!(
//!     zero_hkg.to_chinese(Variant::Traditional),
//!     Chinese {
//!         logograms: "零斤".to_string(),
//!         omissible: true
//!     }
//! );
//! ```
use crate::define_count_measure;

define_count_measure!(pub, HalfKilogram, "斤");

define_count_measure!(pub, Kilogram, "公斤");
