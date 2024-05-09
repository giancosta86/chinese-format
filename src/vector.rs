use crate::{Chinese, ToChinese, Variant};

/// Creates [ChineseVec] instances with elegant simplicity.
///
/// It works almost like [ChineseVec::from], but you just need to pass *instances* of [ToChinese] instead of *references* - the `&` is added automatically.
///
/// ```
/// use chinese_format::*;
///
/// let empty_vec = chinese_vec!(Variant::Simplified, []);
///
/// assert_eq!(empty_vec.collect(), Chinese {
///     logograms: "".to_string(),
///     omissible: true
/// });
///
///
/// let one_item_vec = chinese_vec!(Variant::Simplified, [90]);
///
/// assert_eq!(one_item_vec.collect(), Chinese {
///     logograms: "九十".to_string(),
///     omissible: false
/// });
///
///
/// let two_item_vec = chinese_vec!(Variant::Simplified, [
///     "你好",
///     38,
/// ]);
///
/// assert_eq!(two_item_vec.collect(), Chinese {
///     logograms: "你好三十八".to_string(),
///     omissible: false
/// });
/// ```
///
#[macro_export]
macro_rules! chinese_vec {
    ($variant: expr, [$($item: expr),* $(,)?]) => {{
        let chinese_vector: $crate::ChineseVec =
        vec![ $($item.to_chinese($variant)),* ].into();

        chinese_vector
    }};
}

/// A vector containing [Chinese] expressions.
///
/// It can be manipulated with functional methods
/// such as [trim_end](Self::trim_end) - and the overall sequence can be
/// reduced to a single [Chinese] instance via [collect](Self::collect).
///
/// It can be instantiated using a `.into()` conversion from a `Vec<Chinese>`,
/// but also - and especially :
///
/// * using the [chinese_vec] macro, passing instances implementing [ToChinese].
///  
/// * via its [from](Self::from) method for [ToChinese] instances,
///   for more fine-grained control.
///
/// ```
/// use chinese_format::*;
///
/// let first_vec: ChineseVec = vec![
///     Chinese {
///         logograms: "很".to_string(),
///         omissible: false
///     },
///     Chinese {
///         logograms: "好".to_string(),
///         omissible: false
///     }
/// ].into();
///
/// assert_eq!(first_vec.collect(), Chinese {
///     logograms: "很好".to_string(),
///     omissible: false
/// });
///
///
/// let second_vec = chinese_vec!(Variant::Traditional, [
///     Count(2),
///     ("厘米", "釐米")
/// ]);
///
/// assert_eq!(second_vec.collect(), Chinese {
///     logograms: "兩釐米".to_string(),
///     omissible: false
/// });
///
///
/// let third_vec = ChineseVec::from(Variant::Traditional, vec![
///     &Count(7),
///     &("厘米", "釐米")
/// ]);
///
/// assert_eq!(third_vec.collect(), Chinese {
///     logograms: "七釐米".to_string(),
///     omissible: false
/// });
/// ```
pub struct ChineseVec(Vec<Chinese>);

impl ChineseVec {
    /// Creates a new [ChineseVec] by converting a sequence of [ToChinese],
    /// according to the given [Variant].
    ///
    /// Except specific needs, you'll most often prefer [chinese_vec].
    ///
    /// ```
    /// use chinese_format::*;
    ///
    /// //Toy example based on the real implementation of Fraction.
    /// let chinese_vec = ChineseVec::from(
    ///     Variant::Simplified,
    ///     vec![
    ///         &Sign(-5),
    ///         &7,
    ///         &"分之",
    ///         &5
    ///     ]
    /// );
    ///
    /// assert_eq!(chinese_vec.collect(), Chinese {
    ///     logograms: "负七分之五".to_string(),
    ///     omissible: false
    /// });
    /// ```
    pub fn from(variant: Variant, source: Vec<&dyn ToChinese>) -> ChineseVec {
        Self(
            source
                .into_iter()
                .map(|item| item.to_chinese(variant))
                .collect(),
        )
    }

    /// Removes the left-most sequence of [Chinese] characters that are [omissible](Chinese::omissible).
    ///
    /// ```
    /// use chinese_format::*;
    ///
    /// let chinese_vec = chinese_vec!(Variant::Simplified, [
    ///     0,
    ///     "",
    ///     Count(0),
    ///     8,
    ///     "",
    ///     "好",
    ///     "",
    ///     0,
    ///     Count(0)
    /// ]).trim_start();
    ///
    /// assert_eq!(chinese_vec.collect(), Chinese{
    ///     logograms: "八好零零".to_string(),
    ///     omissible: false
    /// });
    /// ```
    pub fn trim_start(&self) -> Self {
        ChineseVec(
            self.0
                .iter()
                .skip_while(|item| item.omissible)
                .cloned()
                .collect(),
        )
    }

    /// Removes the right-most sequence of [Chinese] characters that are [omissible](Chinese::omissible).
    ///
    /// ```
    /// use chinese_format::*;
    ///
    /// let chinese_vec = chinese_vec!(Variant::Simplified, [
    ///     0,
    ///     "",
    ///     Count(0),
    ///     8,
    ///     "",
    ///     "好",
    ///     "",
    ///     0,
    ///     Count(0)
    /// ]).trim_end();
    ///
    /// assert_eq!(chinese_vec.collect(), Chinese{
    ///     logograms: "零零八好".to_string(),
    ///     omissible: false
    /// });
    /// ```
    pub fn trim_end(&self) -> Self {
        let mut rev_result: Vec<Chinese> = vec![];

        for item in self.0.iter().rev().skip_while(|item| item.omissible) {
            rev_result.push(item.clone());
        }

        let result = rev_result.into_iter().rev().collect();

        ChineseVec(result)
    }

    /// Concatenates all the [Chinese] expressions into a single one.
    ///
    /// The resulting [Chinese] is defined as follows:
    ///
    /// * the [logograms](Chinese::logograms) are obtained by concatenating all the logograms.
    ///
    /// * it is [omissible](Chinese::omissible) in one of two cases:
    ///   
    ///   * the vector is empty.
    ///
    ///   * all of its items are [omissible](Chinese::omissible).
    ///
    /// ```
    /// use chinese_format::{*, length::*};
    ///
    /// let basic: ChineseVec = chinese_vec!(Variant::Simplified, [
    ///     9,
    ///     "点",
    ///     4,
    ///     "分"
    /// ]);
    /// assert_eq!(basic.collect(), Chinese {
    ///     logograms: "九点四分".to_string(),
    ///     omissible: false
    /// });
    ///
    /// let empty: ChineseVec = vec![].into();
    /// assert_eq!(empty.collect(), Chinese {
    ///     logograms: "".to_string(),
    ///     omissible: true
    /// });
    ///
    /// let only_omissible = chinese_vec!(Variant::Simplified, [
    ///     0,
    ///     Count(0),
    ///     "",
    ///     chinese_vec!(Variant::Simplified, [
    ///         Sign(9),
    ///         EmptyPlaceholder::new(&Meter::new(0))
    ///     ]),
    ///     ("", "Test")
    /// ]);
    /// assert_eq!(only_omissible.collect(), Chinese {
    ///     logograms: "零零".to_string(),
    ///     omissible: true
    /// })
    /// ```
    pub fn collect(&self) -> Chinese {
        Chinese {
            logograms: self
                .0
                .iter()
                .map(|item| item.logograms.as_str())
                .collect::<Vec<_>>()
                .join(""),

            omissible: self.0.is_empty() || self.0.iter().all(|item| item.omissible),
        }
    }
}

/// A [ChineseVec] can be infallibly built from a [Vec] of [Chinese]`.
///
/// ```
/// use chinese_format::*;
///
/// let chinese_vec: ChineseVec = vec![
///     Chinese {
///         logograms: "没".to_string(),
///         omissible: false
///     },
///     Chinese {
///         logograms: "关".to_string(),
///         omissible: false
///     },
///     Chinese {
///         logograms: "系".to_string(),
///         omissible: false
///     }
/// ].into();
///
/// assert_eq!(chinese_vec.collect(), Chinese {
///     logograms: "没关系".to_string(),
///     omissible: false
/// });
/// ```
impl From<Vec<Chinese>> for ChineseVec {
    fn from(value: Vec<Chinese>) -> Self {
        Self(value)
    }
}

/// [ChineseVec] supports [ToChinese] via its [collect](Self::collect) method.
///
/// Of course, the [Variant] parameter is ignored - because the
/// [Chinese] instances are already available in the vector.
///
/// ```
/// use chinese_format::*;
///
/// let chinese_vec = chinese_vec!(Variant::Simplified, [
///     "飞",
///     "机"
/// ]);
///
/// //In traditional script, 飞 is written 飛! No conversion can be performed.
/// assert_eq!(chinese_vec.to_chinese(Variant::Traditional), "飞机");
/// ```
impl ToChinese for ChineseVec {
    fn to_chinese(&self, _variant: Variant) -> Chinese {
        self.collect()
    }
}

///Any &[ChineseVec] can be infallibly converted to a [Vec] of [Chinese].
///
/// ```
/// use chinese_format::*;
///
/// let chinese_vec = chinese_vec!(Variant::Simplified, [
///     "你好",
///     "生日快乐"
/// ]);
///
/// let vec_of_chinese: Vec<Chinese> = (&chinese_vec).into();
///
/// assert_eq!(vec_of_chinese, vec![
///     Chinese {
///         logograms: "你好".to_string(),
///         omissible: false
///     },
///
///     Chinese {
///         logograms: "生日快乐".to_string(),
///         omissible: false
///     }
/// ]);
/// ```
impl From<&ChineseVec> for Vec<Chinese> {
    fn from(value: &ChineseVec) -> Self {
        value.0.to_vec()
    }
}
