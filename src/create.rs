//! Methods to create a single [`BitSet`].

use crate::*;

impl<W: Word> BitSet<W> {
    /// Create an empty set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(Set::new().is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a set containing all representable elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(BitSet::<u8>::full(), bitset![u8; 0..8]);
    /// assert_eq!(Set128::full().len(), 128);
    /// ```
    #[inline]
    pub fn full() -> Self {
        Self(W::one().wrapping_neg())
    }

    /// Create a singleton set.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::singleton(5), set![5]);
    /// ```
    #[inline]
    pub fn singleton(e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(W::one() << e)
    }

    /// Create a contiguous interval.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::interval(1, 3), set![1..=3]);
    /// assert_eq!(Set::interval(2, 2), set![2]);
    /// assert!(Set::interval(3, 1).is_empty());
    /// ```
    #[inline]
    pub fn interval(first: Element, last: Element) -> Self {
        Self::debug_bound_check(last);
        if first > last {
            Self(W::zero())
        } else if last == Self::MAX {
            Self(!W::zero() << first)
        } else {
            Self(((W::one() << (last - first + 1)) - W::one()) << first)
        }
    }

    /// Create a bitset from the underlying primitive type `W`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::from_word(1 + 4 + 16), set![0, 2, 4]);
    /// assert_eq!(Set::from_word(123).word(), 123);
    /// ```
    #[inline]
    pub fn from_word(word: W) -> Self {
        Self(word)
    }
}
