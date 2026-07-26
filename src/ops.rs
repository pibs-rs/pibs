use crate::*;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign,
};

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

impl<W: Word> AddAssign<Element> for BitSet<W> {
    /// Add an element to the set (or leave it in).
    #[inline]
    fn add_assign(&mut self, rhs: Element) {
        self.0 |= W::one() << rhs;
    }
}

impl<W: Word> Sub<Element> for BitSet<W> {
    type Output = Self;

    /// The set with an element removed (if it exists).
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

impl<W: Word> SubAssign<Element> for BitSet<W> {
    /// Remove an element from the set (if it exists).
    #[inline]
    fn sub_assign(&mut self, rhs: Element) {
        self.0 &= !(W::one() << rhs);
    }
}

impl<W: Word> BitOr for BitSet<W> {
    type Output = Self;

    /// The union of two sets.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a | b, set![1..=5]);
    /// ```
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl<W: Word> BitOrAssign for BitSet<W> {
    /// Insert every element from another set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let mut set = set![1..=3];
    /// set |= set![3..=5];
    /// assert_eq!(set, set![1..=5]);
    /// ```
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<W: Word> BitAnd for BitSet<W> {
    type Output = Self;

    /// The intersection of two sets.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a & b, set![3]);
    /// ```
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl<W: Word> BitAndAssign for BitSet<W> {
    /// Remove all elements not present in another set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let mut set = set![1..=5];
    /// set &= set![3..=7];
    /// assert_eq!(set, set![3..=5]);
    /// ```
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl<W: Word> BitXor for BitSet<W> {
    type Output = Self;

    /// The symmetric difference of two sets.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a ^ b, set![1..=2, 4..=5]);
    /// ```
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl<W: Word> BitXorAssign for BitSet<W> {
    /// Toggle all elements present in another set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let mut set = set![1..=3];
    /// set ^= set![3..=5];
    /// assert_eq!(set, set![1..=2, 4..=5]);
    /// ```
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
