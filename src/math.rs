//! [`BitSet`] methods that treat it as a set of integers specifically.

use crate::*;

impl<W: Word> BitSet<W> {
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
            Some(Self::MAX - self.0.leading_zeros() as Element)
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
    pub fn sum(self) -> Element {
        self.iter().sum()
    }

    /// Ordinal position of an element in the set, counted from zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![4..=6];
    /// assert_eq!(set.position(3), None);
    /// assert_eq!(set.position(4), Some(0));
    /// assert_eq!(set.position(6), Some(2));
    /// ```
    #[inline]
    pub fn position(self, e: Element) -> Option<usize> {
        if self.contains(e) {
            Some(self.position_unchecked(e))
        } else {
            None
        }
    }

    /// Ordinal position of an element assumed to be in the set, counted from zero.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e` is contained in the set. Violating this precondition panics
    /// in debug builds and results in unspecified behavior in release builds.
    #[inline]
    pub fn position_unchecked(self, e: Element) -> usize {
        debug_assert!(self.contains(e));
        (self.0 & ((W::one() << e) - W::one())).count_ones() as usize
    }

    /// Ordinal position of an element in the set, counted from one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![4..=6];
    /// assert_eq!(set.rank(3), None);
    /// assert_eq!(set.rank(4), Some(1));
    /// assert_eq!(set.rank(6), Some(3));
    /// ```
    #[inline]
    pub fn rank(self, e: Element) -> Option<usize> {
        self.position(e).map(|p| p + 1)
    }

    /// Ordinal position of an element assumed to be in the set, counted from one.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e` is contained in the set. Violating this precondition panics
    /// in debug builds and results in unspecified behavior in release builds.
    #[inline]
    pub fn rank_unchecked(self, e: Element) -> usize {
        debug_assert!(self.contains(e));
        self.position_unchecked(e) + 1
    }

    /// Try to add each element in `self` to each element in `other`.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), returns [`None`].
    ///
    /// See [`Self::truncating_sumset`] for a variant that drops irrepresentable elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![2, 8];
    /// let b = set![1, 5];
    /// assert_eq!(a.sumset(b), Some(set![3, 7, 9, 13]));
    /// assert_eq!(a.sumset(set![]), Some(set![]));
    /// assert_eq!(a.sumset(set![0]), Some(a));
    /// assert_eq!(a.sumset(Set::full()), None);
    /// ```
    #[inline]
    pub fn sumset(self, other: Self) -> Option<Self> {
        let mut result = W::zero();

        let (smaller, larger_word) = if self.len() < other.len() {
            (self, other.0)
        } else {
            (other, self.0)
        };
        let e_max = larger_word.leading_zeros() as Element;

        for e in smaller {
            if e > e_max {
                return None;
            }
            result |= larger_word << e;
        }

        Some(Self(result))
    }

    /// Add each element in `self` to each element in `other`.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), they are discarded.
    ///
    /// See [`Self::sumset`] for a checked variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![2, 8];
    /// let b = set![1, 5];
    /// assert_eq!(a.truncating_sumset(b), set![3, 7, 9, 13]);
    /// assert_eq!(a.truncating_sumset(set![]), set![]);
    /// assert_eq!(a.truncating_sumset(set![0]), a);
    /// assert_eq!(a.truncating_sumset(Set::full()), Set::full() - set![0, 1]);
    /// ```
    #[inline]
    pub fn truncating_sumset(self, other: Self) -> Self {
        let mut result = W::zero();

        let (smaller, larger_word) = if self.len() < other.len() {
            (self, other.0)
        } else {
            (other, self.0)
        };

        for e in smaller {
            result |= larger_word << e;
        }

        Self(result)
    }

    /// Sum the elements in each subset of the set and collect these sums in a new set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), returns [`None`].
    ///
    /// See [`Self::truncating_subset_sums`] for a variant that drops irrepresentable elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![].subset_sums(), Some(set![0]));
    /// assert_eq!(set![0].subset_sums(), Some(set![0]));
    /// assert_eq!(set![1, 2, 4, 8].subset_sums(), Some(set![0..16]));
    /// assert_eq!(set![1..=5].subset_sums(), Some(set![0..16]));
    ///
    /// // Overflows are checked.
    /// assert_eq!(set![1, 3, 7].subset_sums(), Some(set![0, 1, 3, 4, 7, 8, 10, 11]));
    /// assert_eq!(bitset![u8; 1, 3, 7].subset_sums(), None);
    /// ```
    #[inline]
    pub fn subset_sums(self) -> Option<Self> {
        let mut result = W::one();
        let mut e_max = result.leading_zeros() as Element;

        for e in self {
            if e > e_max {
                return None;
            }
            e_max -= e;
            result |= result << e;
        }

        Some(Self(result))
    }

    /// Sum the elements in each subset of the set and collect these sums in a new set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), they are discarded.
    ///
    /// See [`Self::subset_sums`] for a checked variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![].truncating_subset_sums(), set![0]);
    /// assert_eq!(set![0].truncating_subset_sums(), set![0]);
    /// assert_eq!(set![1, 2, 4, 8].truncating_subset_sums(), set![0..16]);
    /// assert_eq!(set![1..=5].truncating_subset_sums(), set![0..16]);
    ///
    /// // Overflows are truncated.
    /// assert_eq!(set![1, 3, 7].truncating_subset_sums(), set![0, 1, 3, 4, 7, 8, 10, 11]);
    /// assert_eq!(bitset![u8; 1, 3, 7].truncating_subset_sums(), bitset![u8; 0, 1, 3, 4, 7]);
    /// ```
    #[inline]
    pub fn truncating_subset_sums(self) -> Self {
        let mut result = W::one();
        for e in self {
            result |= result << e;
        }
        Self(result)
    }

    /// Try to add a number to each element in the set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), returns [`None`].
    ///
    /// See [`Self::truncating_add_to_all`] for a variant that drops irrepresentable elements.
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
    /// let set = bitset![u8; 1..=3, 5];
    /// assert_eq!(set.add_to_all(2), Some(bitset![u8; 3..=5, 7]));
    /// assert_eq!(set.add_to_all(3), None);
    /// ```
    #[inline]
    #[must_use = "not a mutating method"]
    pub fn add_to_all(self, e: Element) -> Option<Self> {
        if e > self.0.leading_zeros() as Element {
            None
        } else {
            Some(self.truncating_add_to_all(e))
        }
    }

    /// Add a number to each element in the set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), they are discarded.
    ///
    /// This can also be written as `self << e`.
    ///
    /// See [`Self::add_to_all`] for a checked variant.
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
    /// let set = bitset![u8; 1..=3, 5];
    /// assert_eq!(set.truncating_add_to_all(2), bitset![u8; 3..=5, 7]);
    /// assert_eq!(set.truncating_add_to_all(3), bitset![u8; 4..=6]);
    /// ```
    #[inline]
    #[must_use = "not a mutating method"]
    pub fn truncating_add_to_all(self, e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(self.0 << e)
    }

    /// Try to subtract a number from each element in the set.
    ///
    /// If resulting elements are not representable (below zero), returns [`None`].
    ///
    /// See [`Self::truncating_sub_from_all`] for a variant that drops irrepresentable elements.
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
    /// let set = set![1..=3, 5];
    /// assert_eq!(set.sub_from_all(1), Some(set![0..=2, 4]));
    /// assert_eq!(set.sub_from_all(3), None);
    /// ```
    #[inline]
    #[must_use = "not a mutating method"]
    pub fn sub_from_all(self, e: Element) -> Option<Self> {
        if e > self.0.trailing_zeros() as Element {
            None
        } else {
            Some(self.truncating_sub_from_all(e))
        }
    }

    /// Subtract a number from each element in the set.
    ///
    /// If resulting elements are not representable (below zero), they are discarded.
    ///
    /// This can also be written as `self >> e`.
    ///
    /// See [`Self::sub_from_all`] for a checked variant.
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
    /// let set = set![1..=3, 5];
    /// assert_eq!(set.truncating_sub_from_all(1), set![0..=2, 4]);
    /// assert_eq!(set.truncating_sub_from_all(3), set![0, 2]);
    /// ```
    #[inline]
    #[must_use = "not a mutating method"]
    pub fn truncating_sub_from_all(self, e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(self.0 >> e)
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
            Self::BITS - (self.0.leading_zeros() + self.0.trailing_zeros()) as usize == self.len()
        }
    }
}
