use crate::*;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, Shr, Sub,
    SubAssign,
};

impl<W: Word> Add<Element> for BitSet<W> {
    type Output = Self;

    /// The set with an element added (or left in).
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `rhs <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![4..=6];
    /// assert_eq!(set + 6, set);
    /// assert_eq!(set + 7, set![4..=7]);
    /// assert_eq!(set + 8, set.union(set![8]));
    /// ```
    #[inline]
    fn add(self, rhs: Element) -> Self {
        self.with(rhs)
    }
}

impl<W: Word> Add<BitSet<W>> for BitSet<W> {
    type Output = Self;

    /// The sumset obtained by adding each element in `rhs` to each element in `self`.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), they are discarded.
    ///
    /// This operation is also known as a Minkowski sum.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![0, 10, 20];
    /// let b = set![1, 2];
    /// assert_eq!(a + b, set![1, 2, 11, 12, 21, 22]);
    /// assert_eq!(a + a, set![0, 10, 20, 30, 40]);
    /// assert_eq!(b + b, set![2, 3, 4]);
    /// ```
    ///
    /// # Pitfalls
    ///
    /// The operation `a - b` denotes set difference of the sets `a` and `b`, which is not a
    /// counterpart to the sumset `a + b`.
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.truncating_sumset(rhs)
    }
}

impl<W: Word> AddAssign<Element> for BitSet<W> {
    /// Add an element to the set (or leave it in).
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `rhs <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![4..=6];
    /// set += 7;
    /// assert_eq!(set, set![4..=7]);
    /// set += 7; // Does nothing.
    /// ```
    #[inline]
    fn add_assign(&mut self, rhs: Element) {
        self.insert(rhs);
    }
}

impl<W: Word> Sub<Element> for BitSet<W> {
    type Output = Self;

    /// The set with an element removed (if it exists).
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `rhs <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![4..=6];
    /// assert_eq!(set - 5, set.difference(set![5]));
    /// assert_eq!(set - 6, set![4..=5]);
    /// assert_eq!(set - 7, set);
    /// ```
    #[inline]
    fn sub(self, rhs: Element) -> Self {
        self.without(rhs)
    }
}

impl<W: Word> Sub<BitSet<W>> for BitSet<W> {
    type Output = Self;

    /// The set with every element also present in another set removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a - b, set![1..=2]);
    /// ```
    ///
    /// # Pitfalls
    ///
    /// While `a - b` denotes the difference of sets `a` and `b`, the counterpart of set union is
    /// written as `a | b` and not `a + b`.
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.difference(rhs)
    }
}

impl<W: Word> SubAssign<Element> for BitSet<W> {
    /// Remove an element from the set (if it exists).
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `rhs <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![4..=7];
    /// set -= 7;
    /// assert_eq!(set, set![4..=6]);
    /// set -= 7; // Does nothing.
    /// ```
    #[inline]
    fn sub_assign(&mut self, rhs: Element) {
        self.remove(rhs);
    }
}

impl<W: Word> BitOr for BitSet<W> {
    type Output = Self;

    /// The union of two sets.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a | b, set![1..=5]);
    /// ```
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

// TODO: Implement a long form method for this.
impl<W: Word> BitOrAssign for BitSet<W> {
    /// Insert every element from another set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
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
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a & b, set![3]);
    /// ```
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        self.intersection(rhs)
    }
}

// TODO: Implement a long form method for this.
impl<W: Word> BitAndAssign for BitSet<W> {
    /// Remove all elements not present in another set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
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
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a ^ b, set![1..=2, 4..=5]);
    /// ```
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        self.symmetric_difference(rhs)
    }
}

// TODO: Implement a long form method for this.
impl<W: Word> BitXorAssign for BitSet<W> {
    /// Toggle all elements present in another set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![1..=3];
    /// set ^= set![3..=5];
    /// assert_eq!(set, set![1..=2, 4..=5]);
    /// ```
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl<W: Word> Shl<usize> for BitSet<W> {
    type Output = Self;

    /// Add a number to each element in the set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), they are discarded.
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
    /// assert_eq!(set << 2, bitset![u8; 3..=5, 7]);
    /// assert_eq!(set << 3, bitset![u8; 4..=6]); // Truncates.
    /// ```
    #[inline]
    fn shl(self, rhs: usize) -> Self::Output {
        self.truncating_add_to_all(rhs)
    }
}

impl<W: Word> Shr<usize> for BitSet<W> {
    type Output = Self;

    /// Subtract a number from each element in the set.
    ///
    /// If resulting elements are not representable (below zero), they are discarded.
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
    /// assert_eq!(set >> 1, set![0..=2, 4]);
    /// assert_eq!(set >> 3, set![0, 2]); // Truncates.
    /// ```
    #[inline]
    fn shr(self, rhs: usize) -> Self::Output {
        self.truncating_sub_from_all(rhs)
    }
}
