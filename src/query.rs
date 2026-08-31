//! Query methods for [`BitSet`] that make sense for sets over any universe.
//!
//! For methods specific to integer sets, see [`math`].

use crate::*;

impl<W: Word> BitSet<W> {
    /// Number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![4..=6].len(), 3);
    /// ```
    #[inline]
    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    /// Whether the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(Set::new().is_empty());
    /// ```
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == W::ZERO
    }

    /// Whether the set contains a given element.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![4, 5, 6].contains(5), true);
    /// assert_eq!(set![4, 5, 6].contains(8), false);
    /// ```
    #[inline]
    pub fn contains(self, e: Element) -> bool {
        Self::debug_bound_check(e);
        self.0 & (W::ONE << e) != W::ZERO
    }

    /// Whether `self` is a (non-strict) subset of `other`.
    ///
    /// This can also be written as `self <= other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2].is_subset(set![1, 2]), true);
    /// assert_eq!(set![1, 2].is_subset(set![1, 2, 3]), true);
    /// assert_eq!(set![1, 2, 3].is_subset(set![1, 2]), false);
    /// ```
    #[inline]
    pub fn is_subset(self, other: Self) -> bool {
        self.0 & !other.0 == W::ZERO
    }

    /// Whether `self` is a (non-strict) superset of `other`.
    ///
    /// This can also be written as `self >= other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2].is_superset(set![1, 2]), true);
    /// assert_eq!(set![1, 2].is_superset(set![1, 2, 3]), false);
    /// assert_eq!(set![1, 2, 3].is_superset(set![1, 2]), true);
    /// ```
    #[inline]
    pub fn is_superset(self, other: Self) -> bool {
        !self.0 & other.0 == W::ZERO
    }

    /// Whether `self` is a strict subset of `other`.
    ///
    /// This can also be written as `self < other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2].is_strict_subset(set![1, 2]), false);
    /// assert_eq!(set![1, 2].is_strict_subset(set![1, 2, 3]), true);
    /// assert_eq!(set![1, 2, 3].is_strict_subset(set![1, 2]), false);
    /// ```
    #[inline]
    pub fn is_strict_subset(self, other: Self) -> bool {
        self != other && self.is_subset(other)
    }

    /// Whether `self` is a strict superset of `other`.
    ///
    /// This can also be written as `self > other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2].is_strict_superset(set![1, 2]), false);
    /// assert_eq!(set![1, 2].is_strict_superset(set![1, 2, 3]), false);
    /// assert_eq!(set![1, 2, 3].is_strict_superset(set![1, 2]), true);
    /// ```
    #[inline]
    pub fn is_strict_superset(self, other: Self) -> bool {
        self != other && self.is_superset(other)
    }

    /// Whether `self` and `other` have elements in common.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2].intersects(set![2, 3]), true);
    /// assert_eq!(set![1, 2].intersects(set![3, 4]), false);
    /// ```
    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != W::ZERO
    }

    /// Whether `self` and `other` have no elements in common.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2].is_disjoint(set![2, 3]), false);
    /// assert_eq!(set![1, 2].is_disjoint(set![3, 4]), true);
    /// ```
    #[inline]
    pub fn is_disjoint(self, other: Self) -> bool {
        self.0 & other.0 == W::ZERO
    }
}
