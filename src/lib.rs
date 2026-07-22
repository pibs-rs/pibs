//! A b**itset** using a single **p**rimitive integer to store small non-negative numbers.
//!
//! The focus of this crate are zero overhead on top of bitwise operations (no heap allocation or
//! block selection) and a rich API for mathematical operations involving small integer sets.
//!
//! If your numbers can exceed the number of bits in a machine word (i.e., larger than 127),
//! use ... instead. If you want zero overhead but only need bit-manipulation but not mathematical
//! set abstraction, consider ... instead.

#![feature(trait_alias)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::{
    any::type_name,
    ops::{Add, AddAssign, BitAndAssign, BitOrAssign, Range, RangeInclusive, Shl, Sub},
};
use num_traits::{PrimInt, Unsigned};

/// The default set, using a usize for internal storage.
///
/// On 64 bit systems, this set can store integers between 0 and 63 (inclusive).
pub type Set = BitSet<usize>;

/// The largest set that this crate offers, storing integers between 0 and 127 (inclusive).
///
/// On 64 bit systems, operations on [`BigSet`] should be slower than on [`Set`].
pub type BigSet = BitSet<u128>;

/// Element stored in a [`BitSet`].
pub type Element = usize;

/// A primitive integer type that [`BitSet`] can use internally.
pub trait Word =
    PrimInt + Unsigned + AddAssign + BitAndAssign + BitOrAssign + Shl<usize, Output = Self>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitSet<W: Word>(W);

impl<W: Word> BitSet<W> {
    pub const BITS: usize = size_of::<W>() * 8;
    pub const MAX: usize = Self::BITS - 1;

    #[inline(always)]
    fn debug_bound_check(e: Element) {
        debug_assert!(
            e <= Self::MAX,
            "element {} out of bounds for {}: maximum is {}",
            e,
            type_name::<Self>(),
            Self::MAX
        )
    }

    #[inline]
    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == W::zero()
    }

    #[inline]
    pub fn insert(&mut self, e: Element) {
        Self::debug_bound_check(e);
        self.0 |= W::one() << e;
    }

    #[inline]
    pub fn remove(&mut self, e: Element) {
        Self::debug_bound_check(e);
        self.0 &= !(W::one() << e);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0 = W::zero();
    }

    #[inline]
    pub fn contains(&self, e: Element) -> bool {
        Self::debug_bound_check(e);
        (self.0 >> e) & W::one() == W::one()
    }

    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        self.0 & !other.0 == W::zero()
    }

    #[inline]
    pub fn is_superset(&self, other: &Self) -> bool {
        !self.0 & other.0 == W::zero()
    }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0 & other.0 == W::zero()
    }

    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub fn difference(&self, other: &Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Ordinal position of an element in the set counted from zero.
    ///
    /// # Example
    /// ```
    /// let set: pitset::Set = (4..=6).into();
    /// assert_eq!(set.position(3), None);
    /// assert_eq!(set.position(4), Some(0));
    /// assert_eq!(set.position(6), Some(2));
    /// ```
    #[inline]
    pub fn position(&self, e: Element) -> Option<usize> {
        if self.contains(e) {
            Some(self.position_unchecked(e))
        } else {
            None
        }
    }

    /// Ordinal position of an element in the set counted from zero, assuming it exists.
    #[inline]
    pub fn position_unchecked(&self, e: Element) -> usize {
        Self::debug_bound_check(e);
        (self.0 & ((W::one() << e) - W::one())).count_ones() as usize
    }

    /// Ordinal position of an element in the set counted from one.
    #[inline]
    pub fn rank(&self, e: Element) -> Option<usize> {
        self.position(e).and_then(|p| Some(p + 1))
    }

    /// Ordinal position of an element in the set from one, assuming it exists.
    #[inline]
    pub fn rank_unchecked(&self, e: Element) -> usize {
        Self::debug_bound_check(e);
        self.position_unchecked(e) + 1
    }

    /// Create an empty set.
    ///
    /// # Example
    /// ```
    /// assert!(pitset::Set::new().is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a singleton set.
    ///
    /// # Example
    /// ```
    /// assert_eq!(pitset::Set::singleton(5).into_vec(), vec![5]);
    /// ```
    ///
    /// # Panics
    /// If `e` exceeds [`BitSet::MAX`].
    #[inline]
    pub fn singleton(e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(W::one() << e)
    }

    #[inline]
    pub fn iter(self) -> BitSetIter<W> {
        BitSetIter::<W>(self.0)
    }

    /// The elements as a sorted vector.
    ///
    /// # Example
    /// ```
    /// assert_eq!(pitset::Set::from(1..=3).to_vec(), vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn to_vec(&self) -> Vec<Element> {
        self.iter().collect()
    }

    /// The elements as a sorted vector.
    ///
    /// # Example
    /// ```
    /// assert_eq!(pitset::Set::from(1..=3).into_vec(), vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_vec(self) -> Vec<Element> {
        self.into_iter().collect()
    }
}

impl<W: Word> Default for BitSet<W> {
    fn default() -> Self {
        Self(W::zero())
    }
}

impl<W: Word> Add<Element> for BitSet<W> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Element) -> Self {
        Self(self.0 | (W::one() << rhs))
    }
}

impl<W: Word> Sub<Element> for BitSet<W> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Element) -> Self {
        Self(self.0 & !(W::one() << rhs))
    }
}

impl<W: Word> From<Range<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an exclusive range.
    ///
    /// # Example
    /// ```
    /// for range in [(2..5), (2..3), (2..2), (2..1)] {
    ///     let set: pitset::Set = range.clone().into();
    ///     let vec: Vec<_> = range.collect();
    ///     assert_eq!(set.into_vec(), vec);
    /// }
    /// ```
    #[inline]
    fn from(range: Range<Element>) -> Self {
        if range.is_empty() {
            return Self(W::zero());
        }
        Self::debug_bound_check(range.end - 1);
        Self(((W::one() << (range.end - range.start)) - W::one()) << range.start)
    }
}

impl<W: Word> From<RangeInclusive<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an inclusive range.
    ///
    /// # Example
    /// ```
    /// for range in [(2..=4), (2..=2), (2..=1)] {
    ///     let set: pitset::Set = range.clone().into();
    ///     let vec: Vec<_> = range.collect();
    ///     assert_eq!(set.into_vec(), vec);
    /// }
    /// ```
    #[inline]
    fn from(range: RangeInclusive<Element>) -> Self {
        if range.is_empty() {
            return Self(W::zero());
        }
        let start = *range.start();
        let end = range.last().unwrap() + 1;
        Self::debug_bound_check(end - 1);
        Self(((W::one() << (end - start)) - W::one()) << start)
    }
}

impl<W: Word> FromIterator<Element> for BitSet<W> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        let mut word = W::zero();
        for e in iter {
            word += W::one() << e;
        }
        Self(word)
    }
}

impl<W: Word> IntoIterator for BitSet<W> {
    type Item = usize;
    type IntoIter = BitSetIter<W>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct BitSetIter<W: Word>(W);

impl<W: Word> Iterator for BitSetIter<W> {
    type Item = Element;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == W::zero() {
            return None;
        }
        let item = self.0.trailing_zeros() as Element;
        self.0 &= self.0 - W::one();
        Some(item)
    }
}

#[cfg(feature = "alloc")]
impl<W: Word> From<BitSet<W>> for Vec<Element> {
    /// Create a [`Vec`] from a [`BitSet`].
    ///
    /// # Example
    /// ```
    /// let v: Vec<_> = pitset::Set::from(1..=3).into();
    /// assert_eq!(v, vec![1, 2, 3]);
    /// ```
    fn from(value: BitSet<W>) -> Self {
        value.into_vec()
    }
}
