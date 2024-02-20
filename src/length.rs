//! Length measures.
//!
//! ```
//! use chinese_format::{*, length::*};
//!
//! let two_km = Kilometer::new(2);
//!
//! assert_eq!(
//!     two_km.to_chinese(Variant::Simplified),
//!     Chinese {
//!         logograms: "两公里".to_string(),
//!         omissible: false
//!     }
//! );
//!
//! assert_eq!(
//!     two_km.to_chinese(Variant::Traditional),
//!     Chinese {
//!         logograms: "兩公里".to_string(),
//!         omissible: false
//!     }
//! );
//!
//!
//! let two_cm = Centimeter::new(2);
//!
//! assert_eq!(
//!     two_cm.to_chinese(Variant::Simplified),
//!     Chinese {
//!         logograms: "两厘米".to_string(),
//!         omissible: false
//!     }
//! );
//!
//! assert_eq!(
//!     two_cm.to_chinese(Variant::Traditional),
//!     Chinese {
//!         logograms: "兩釐米".to_string(),
//!         omissible: false
//!     }
//! );
//!
//!
//! let zero_m = Meter::new(0);
//!
//! assert_eq!(
//!     zero_m.to_chinese(Variant::Simplified),
//!     Chinese {
//!         logograms: "零米".to_string(),
//!         omissible: true
//!     }
//! );
//!
//! assert_eq!(
//!     zero_m.to_chinese(Variant::Traditional),
//!     Chinese {
//!         logograms: "零米".to_string(),
//!         omissible: true
//!     }
//! );
//! ```
use crate::define_count_measure;

define_count_measure!(pub, Kilometer, "公里");

define_count_measure!(pub, HalfKilometer, "里");

define_count_measure!(pub, Meter, "米");

define_count_measure!(pub, Decimeter, "分米");

define_count_measure!(pub, Centimeter, ("厘米", "釐米"));

define_count_measure!(pub, Millimeter, "毫米");
