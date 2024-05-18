use std::hash::Hash;

use crate::{Chinese, ChineseFormat, Variant};

/// Sign of a number.
#[derive(Debug, Clone, Copy)]
pub struct Sign(pub i128);

/// [Sign] instances are equal if the sign (+, - or 0) of their
/// underlying numeric values are equal:
///
/// ```
/// use chinese_format::*;
///
/// assert_eq!(Sign(-9), Sign(-3));
/// assert_eq!(Sign(0), Sign(0));
/// assert_eq!(Sign(13), Sign(90));
///
/// assert_ne!(Sign(-9), Sign(0));
/// assert_ne!(Sign(-9), Sign(7));
/// assert_ne!(Sign(0), Sign(-9));
/// assert_ne!(Sign(0), Sign(13));
/// assert_ne!(Sign(13), Sign(-9));
/// assert_ne!(Sign(13), Sign(0));
/// ```
impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        self.0.signum() == other.0.signum()
    }
}

impl Eq for Sign {}

/// [Sign] instances are sorted in this order:
///
/// 1. negative
/// 1. zero
/// 1. positive
///
/// The magnitude of the underlying integer values
/// does not affect the comparison:
///
/// ```
/// use chinese_format::*;
///
/// assert!(Sign(-90) == Sign(-4));
/// assert!(Sign(0) > Sign(-4));
/// assert!(Sign(0) < Sign(17));
/// assert!(Sign(17) == Sign(92));
/// ```
impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.signum().cmp(&other.0.signum())
    }
}

/// The hash depends uniquely on the sign of the underlying integer value.
///
/// ```
/// use chinese_format::*;
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
///
/// fn get_hash<T: Hash>(source: &T) -> u64 {
///     let mut hasher = DefaultHasher::new();
///     source.hash(&mut hasher);
///     hasher.finish()
/// }
///
/// fn main() {
///     assert_eq!(get_hash(&Sign(-54)), get_hash(&Sign(-7)));
///     assert_eq!(get_hash(&Sign(0)), get_hash(&Sign(0)));
///     assert_eq!(get_hash(&Sign(3)), get_hash(&Sign(90)));
///
///     assert_ne!(get_hash(&Sign(-54)), get_hash(&Sign(0)));
///     assert_ne!(get_hash(&Sign(0)), get_hash(&Sign(90)));
/// }
/// ```
impl Hash for Sign {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.signum().hash(state);
    }
}

/// When converted to Chinese, a sign renders as follows:
///
/// * for *positive numbers* and *zero*: an **empty string** (thus [omissible](Chinese::omissible), too).
///
/// * for *negative numbers*:
///
///   * `负` in [Variant::Simplified] script.
///
///   * `負` in [Variant::Traditional] script.
///
/// ```
/// use chinese_format::*;
///
/// //Positive numbers
/// assert_eq!(Sign(90).to_chinese(Variant::Simplified), Chinese {
///     logograms: "".to_string(),
///     omissible: true
/// });
/// assert_eq!(Sign(90).to_chinese(Variant::Traditional), "");
///
/// //Zero
/// assert_eq!(Sign(0).to_chinese(Variant::Simplified), Chinese {
///     logograms: "".to_string(),
///     omissible: true
/// });
/// assert_eq!(Sign(0).to_chinese(Variant::Traditional), "");
///
/// //Negative numbers
/// assert_eq!(Sign(-7).to_chinese(Variant::Simplified), Chinese {
///     logograms: "负".to_string(),
///     omissible: false
/// });
/// assert_eq!(Sign(-7).to_chinese(Variant::Traditional), "負");
/// ```
impl ChineseFormat for Sign {
    fn to_chinese(&self, variant: Variant) -> Chinese {
        if self.0 >= 0 {
            "".to_chinese(variant)
        } else {
            ("负", "負").to_chinese(variant)
        }
    }
}
