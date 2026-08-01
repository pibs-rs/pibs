//! Set operations for [`BitSet`] that make sense over any universe.
//!
//! For methods specific to integer sets, see [`math`].

use crate::*;

impl<W: Word> BitSet<W> {
    /// An iterator over the elements in sorted order.
    ///
    /// This method is equivalent to [`Self::into_iter`]: both take `self` by value and yield the
    /// elements by value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![7, 3..=5, 1];
    /// assert!(set.iter().eq([1, 3, 4, 5, 7]));
    /// ```
    #[inline]
    pub fn iter(self) -> BitSetIter<W> {
        BitSetIter::<W>(self.0)
    }

    /// The set with an element added to it (or left in).
    ///
    /// This the same as `self + e`.
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
    /// let set = set![1..=3];
    /// assert_eq!(set.with(2), set![1..=3]); // Does nothing.
    /// assert_eq!(set.with(4), set![1..=4]);
    /// ```
    #[inline]
    pub fn with(self, e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(self.0 | W::one() << e)
    }

    /// The set with an element removed from it (if present).
    ///
    /// This the same as `self - e`.
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
    /// let set = set![1..=3];
    /// assert_eq!(set.without(2), set![1, 3]);
    /// assert_eq!(set.without(4), set![1..=3]); // Does nothing.
    /// ```
    #[inline]
    pub fn without(self, e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(self.0 & !(W::one() << e))
    }

    /// The union of two sets.
    ///
    /// This the same as `self | other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=5];
    /// let b = set![3..=7];
    /// assert_eq!(a.union(b), set![1..=7]);
    /// ```
    #[inline]
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// The intersection of two sets.
    ///
    /// This the same as `self & other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=5];
    /// let b = set![3..=7];
    /// assert_eq!(a.intersection(b), set![3..=5]);
    /// ```
    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// The set with every element also present in another set removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=5];
    /// let b = set![3..=7];
    /// assert_eq!(a.difference(b), set![1..=2]);
    /// ```
    #[inline]
    pub fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// The symmetric difference of two sets.
    ///
    /// This the same as `self ^ other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=5];
    /// let b = set![3..=7];
    /// assert_eq!(a.symmetric_difference(b), set![1..=2, 6..=7]);
    /// ```
    #[inline]
    pub fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
}
