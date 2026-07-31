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

    /// The smallest element in the set, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![].min(), None);
    /// assert_eq!(set![4..=6].min(), Some(4));
    /// ```
    #[inline]
    pub fn min(self) -> Option<Element> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.trailing_zeros() as Element)
        }
    }

    /// The largest element in the set, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![].max(), None);
    /// assert_eq!(set![4..=6].max(), Some(6));
    /// ```
    #[inline]
    pub fn max(self) -> Option<Element> {
        if self.is_empty() {
            None
        } else {
            Some((Self::MAX - self.0.leading_zeros() as usize) as Element)
        }
    }

    /// The sum of all elements in the set.
    ///
    /// Returns `0` for the the empty set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![].sum(), 0);
    /// assert_eq!(set![1, 2, 4, 8].sum(), 15);
    /// ```
    #[inline]
    pub fn sum(self) -> usize {
        self.iter().sum()
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
        self.0 == W::zero()
    }

    /// Whether the set contains a given element.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
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
        self.0 & (W::one() << e) != W::zero()
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
        self.0 & !other.0 == W::zero()
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
        !self.0 & other.0 == W::zero()
    }

    /// Whether `self` is a strict subset of `other`.
    ///
    /// his can also be written as `self < other`.
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
        self.0 & other.0 != W::zero()
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
        self.0 & other.0 == W::zero()
    }

    /// Whether the elements form a contiguous interval.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![4..=6];
    /// assert_eq!(set.is_interval(), true);
    /// set.remove(5);
    /// assert_eq!(set.is_interval(), false);
    ///
    /// // Empty sets and singletons are intervals.
    /// assert!(set![].is_interval());
    /// assert!(set![5].is_interval());
    /// ```
    #[inline]
    pub fn is_interval(self) -> bool {
        if self.is_empty() {
            true
        } else {
            1 + self.max().unwrap() - self.min().unwrap() == self.len()
        }
    }
}
