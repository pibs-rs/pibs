use crate::*;

impl<W: Word> BitSet<W> {
    /// Removes all elements from the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![0..23];
    /// assert_eq!(set.len(), 23);
    /// set.clear();
    /// assert_eq!(set.len(), 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.0 = W::zero();
    }

    /// Insert an element into the set (or leave it in).
    ///
    /// This the same as `self += e`.
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
    /// let mut set = set![1, 3];
    /// set.insert(2);
    /// assert_eq!(set, set![1..=3]);
    /// set.insert(2); // Does nothing.
    /// ```
    #[inline]
    pub fn insert(&mut self, e: Element) {
        Self::debug_bound_check(e);
        self.0 |= W::one() << e;
    }

    /// Removes an element from the set (if it exists).
    ///
    /// This the same as `self -= e`.
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
    /// let mut set = set![1..=3];
    /// set.remove(2);
    /// assert_eq!(set, set![1, 3]);
    /// set.remove(2); // Does nothing.
    /// ```
    #[inline]
    pub fn remove(&mut self, e: Element) {
        Self::debug_bound_check(e);
        self.0 &= !(W::one() << e);
    }

    /// Toggles the presence of an element in the set.
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
    /// let mut set = set![1..=3];
    /// set.toggle(2);
    /// assert_eq!(set, set![1, 3]);
    /// set.toggle(2);
    /// assert_eq!(set, set![1..=3]);
    /// ```
    #[inline]
    pub fn toggle(&mut self, e: Element) {
        Self::debug_bound_check(e);
        self.0 ^= W::one() << e;
    }

    /// Insert every element from another set.
    ///
    /// This the same as `self |= other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![1..=5];
    /// set.union_update(set![3..=7]);
    /// assert_eq!(set, set![1..=7]);
    /// ```
    #[inline]
    pub fn union_update(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Remove all elements not present in another set.
    ///
    /// This the same as `self &= other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![1..=5];
    /// set.intersection_update(set![3..=7]);
    /// assert_eq!(set, set![3..=5]);
    /// ```
    #[inline]
    pub fn intersection_update(&mut self, other: Self) {
        self.0 &= other.0;
    }

    /// Remove all elements present in another set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![1..=5];
    /// set.difference_update(set![3..=7]);
    /// assert_eq!(set, set![1..=2]);
    /// ```
    #[inline]
    pub fn difference_update(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Toggle all elements present in another set.
    ///
    /// This the same as `self ^= other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![1..=5];
    /// set.symmetric_difference_update(set![3..=7]);
    /// assert_eq!(set, set![1..=2, 6..=7]);
    /// ```
    #[inline]
    pub fn symmetric_difference_update(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}
