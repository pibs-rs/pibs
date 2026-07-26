use crate::*;
use core::ops::{Add, Sub};

impl<W: Word> Add<Element> for BitSet<W> {
    type Output = Self;

    /// The set with an element added (or left in).
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let set = set![4..=6];
    /// assert_eq!(set + 6, set);
    /// assert_eq!(set + 7, set![4..=7]);
    /// assert_eq!(set + 8, set.union(set![8]));
    /// ```
    #[inline]
    fn add(self, rhs: Element) -> Self {
        Self(self.0 | (W::one() << rhs))
    }
}

impl<W: Word> Sub<Element> for BitSet<W> {
    type Output = Self;

    /// The set with an element removed (if it existed).
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let set = set![4..=6];
    /// assert_eq!(set - 5, set.difference(set![5]));
    /// assert_eq!(set - 6, set![4..=5]);
    /// assert_eq!(set - 7, set);
    /// ```
    #[inline]
    fn sub(self, rhs: Element) -> Self {
        Self(self.0 & !(W::one() << rhs))
    }
}
