#[cfg(feature = "alloc")]
extern crate alloc;

use crate::*;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::{any::type_name, iter};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A high-performance generic bitset that wraps a single primitive integer for storage.
///
/// # Example conventions
///
/// The examples below assume the prelude import:
/// ```
/// use pibs::prelude::*;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitSet<W: Word>(pub(crate) W);

impl<W: Word> BitSet<W> {
    /// The number of bits in the [primitive integer type](Word) `W`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(BitSet::<u32>::BITS, 32);
    /// ```
    pub const BITS: usize = size_of::<W>() * 8;

    /// The smallest integer that can be stored in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::MIN, 0);
    /// ```
    pub const MIN: Element = 0;

    /// The largest integer that can be stored in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(BitSet::<u64>::MAX, 63);
    /// assert_eq!(BitSet::<u128>::MAX, 127);
    /// ```
    pub const MAX: Element = Self::BITS - 1;

    // -------
    // Helpers
    // -------

    #[inline(always)]
    pub(crate) fn debug_bound_check(e: Element) {
        debug_assert!(
            e <= Self::MAX,
            "element {} out of bounds for {}: maximum is {}",
            e,
            type_name::<Self>(),
            Self::MAX
        )
    }

    // -------
    // Queries
    // -------

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

    // --------------
    // Set operations
    // --------------

    /// Insert an element into the set (or leave it in).
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

    /// The set with an element added to it (or left in).
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
    /// Alternative notation for the same operation is `self | other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a.union(b), set![1..=5]);
    /// ```
    #[inline]
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// The intersection of two sets.
    ///
    /// Alternative notation for the same operation is `self & other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a.intersection(b), set![3]);
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
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a.difference(b), set![1..=2]);
    /// ```
    #[inline]
    pub fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// The symmetric difference of two sets.
    ///
    /// Alternative notation for the same operation is `self ^ other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let a = set![1..=3];
    /// let b = set![3..=5];
    /// assert_eq!(a.symmetric_difference(b), set![1..=2, 4..=5]);
    /// ```
    #[inline]
    pub fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Ordinal position of an element in the set, counted from zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set: Set = (4..=6).into();
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
        self.position(e).and_then(|p| Some(p + 1))
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

    /// Generate all subsets, with the maximum number growing slowly.
    ///
    /// See [`Self::subsets_by_size`] for a different iteration order.
    /// To generate all subsets of `0..=Self::BITS`, use the faster [`Self::iter_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![0, 5, 23];
    /// assert!(
    ///     set.subsets().eq([
    ///         set![],
    ///         set![0],
    ///         set![5],
    ///         set![0, 5],
    ///         set![23],
    ///         set![0, 23],
    ///         set![5, 23],
    ///         set![0, 5, 23]
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn subsets(self) -> impl Iterator<Item = Self> {
        let mut word = W::zero();
        let mut stop = false;

        iter::from_fn(move || {
            if stop {
                None
            } else {
                let next = word;
                if let Some(x) = (word | !self.0).checked_add(&W::one()) {
                    word = x & self.0;
                } else {
                    stop = true;
                }
                Some(Self(next))
            }
        })
    }

    /// Generate all subsets of a given cardinality.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![0, 5, 23];
    /// assert!(
    ///     set.subsets_of_size(2).eq([
    ///         set![0, 5],
    ///         set![0, 23],
    ///         set![5, 23]
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn subsets_of_size(self, size: usize) -> SubsetsOfSizeIter<W> {
        SubsetsOfSizeIter::<W>::new(self.0, size)
    }

    /// Generate all subsets, with the cardinality growing slowly.
    ///
    /// If the iteration order is not important, use the faster [`Self::subsets`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![0, 5, 23];
    /// assert!(
    ///     set.subsets_by_size().eq([
    ///         set![],
    ///         set![0],
    ///         set![5],
    ///         set![23],
    ///         set![0, 5],
    ///         set![0, 23],
    ///         set![5, 23],
    ///         set![0, 5, 23]
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn subsets_by_size(self) -> impl Iterator<Item = Self> {
        (0..=self.len()).flat_map(move |k| self.subsets_of_size(k))
    }

    // ---------------------
    // Arithmetic operations
    // ---------------------

    /// Try to add a number to each element in the set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), returns [`None`].
    ///
    /// See [`Self::truncating_add_to_all`] for a variant that drops irrepresentable elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = bitset![u8; 1..=3, 5];
    /// assert_eq!(set.add_to_all(2), Some(bitset![u8; 3..=5, 7]));
    /// assert_eq!(set.add_to_all(3), None);
    /// assert_eq!(set.add_to_all(10_000), None);
    /// ```
    #[inline]
    pub fn add_to_all(self, e: Element) -> Option<Self> {
        self.0.checked_shl(e as u32).and_then(|word| {
            if word.count_ones() as usize == self.len() {
                Some(Self(word))
            } else {
                None
            }
        })
    }

    /// Try to subtract a number from each element in the set.
    ///
    /// If resulting elements are not representable (below zero), returns [`None`].
    ///
    /// See [`Self::truncating_sub_from_all`] for a variant that drops irrepresentable elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![1..=3, 5];
    /// assert_eq!(set.sub_from_all(1), Some(set![0..=2, 4]));
    /// assert_eq!(set.sub_from_all(3), None);
    /// assert_eq!(set.sub_from_all(10_000), None);
    /// ```
    #[inline]
    pub fn sub_from_all(self, e: Element) -> Option<Self> {
        self.0.checked_shr(e as u32).and_then(|word| {
            if word.count_ones() as usize == self.len() {
                Some(Self(word))
            } else {
                None
            }
        })
    }

    /// Add a number to each element in the set.
    ///
    /// If resulting elements are not representable (above [`Self::MAX`]), they are discarded.
    ///
    /// See [`Self::add_to_all`] for a checked variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = bitset![u8; 1..=3, 5];
    /// assert_eq!(set.truncating_add_to_all(2), bitset![u8; 3..=5, 7]);
    /// assert_eq!(set.truncating_add_to_all(3), bitset![u8; 4..=6]);
    /// assert_eq!(set.truncating_add_to_all(10_000), bitset![u8;]);
    /// ```
    #[inline]
    pub fn truncating_add_to_all(self, e: Element) -> Self {
        Self(self.0.checked_shl(e as u32).unwrap_or(W::zero()))
    }

    /// Subtract a number from each element in the set.
    ///
    /// If resulting elements are not representable (below zero), they are discarded.
    ///
    /// See [`Self::sub_from_all`] for a checked variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![1..=3, 5];
    /// assert_eq!(set.truncating_sub_from_all(1), set![0..=2, 4]);
    /// assert_eq!(set.truncating_sub_from_all(3), set![0, 2]);
    /// assert_eq!(set.truncating_sub_from_all(10_000), set![]);
    /// ```
    #[inline]
    pub fn truncating_sub_from_all(self, e: Element) -> Self {
        Self(self.0.checked_shr(e as u32).unwrap_or(W::zero()))
    }

    // ------------
    // Constructors
    // ------------

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
    /// The caller must ensure that `e <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
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
    /// The caller must ensure that `e <= Self::MAX`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
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

    // -----------
    // Enumerators
    // -----------

    /// Generate all representable sets, with the maximum number growing slowly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(
    ///     Set::iter_all().take(8).eq([
    ///         set![],
    ///         set![0],
    ///         set![1],
    ///         set![0, 1],
    ///         set![2],
    ///         set![0, 2],
    ///         set![1, 2],
    ///         set![0, 1, 2]
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn iter_all() -> impl Iterator<Item = Self> {
        let mut word = W::zero();
        let mut stop = false;

        iter::from_fn(move || {
            if stop {
                None
            } else {
                let next = word;
                if let Some(next_word) = word.checked_add(&W::one()) {
                    word = next_word;
                } else {
                    stop = true;
                }
                Some(Self(next))
            }
        })
    }

    /// Generate all representable subsets, with the cardinality growing slowly.
    ///
    /// This is a shorthand for `Self::iter_all_below(Self::BITS)`.
    ///
    /// If you do not care about the iteration order, use the faster [`Self::iter_all`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(
    ///     Set::iter_all_by_size().take(8).eq([
    ///         set![],
    ///         set![0],
    ///         set![1],
    ///         set![2],
    ///         set![3],
    ///         set![4],
    ///         set![5],
    ///         set![6]
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn iter_all_by_size() -> impl Iterator<Item = Self> {
        Self::iter_all_below(Self::BITS)
    }

    /// Generate all 2^n subsets of `0..n`, with the cardinality growing slowly.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `n <= Self::BITS`. Violating this precondition panics in debug
    /// builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(
    ///     Set::iter_all_below(3).eq([
    ///         set![],
    ///         set![0],
    ///         set![1],
    ///         set![2],
    ///         set![0, 1],
    ///         set![0, 2],
    ///         set![1, 2],
    ///         set![0, 1, 2]
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn iter_all_below(n: usize) -> impl Iterator<Item = Self> {
        (0..=n).flat_map(move |k| Self::iter_combinations(n, k))
    }

    /// Generate all (n choose k) subsets of `0..n` with cardinality k.
    ///
    /// The maximum number is growing slowly.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `k <= n <= Self::BITS`. Violating this precondition panics in
    /// debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(
    ///     Set::iter_combinations(4, 2).eq([
    ///         set![0, 1],
    ///         set![0, 2],
    ///         set![1, 2],
    ///         set![0, 3],
    ///         set![1, 3],
    ///         set![2, 3],
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn iter_combinations(n: usize, k: usize) -> impl Iterator<Item = Self> {
        debug_assert!(k <= n);
        debug_assert!(n <= Self::BITS);

        // TODO: Avoid cases below via unbounded shift once there is trait support for it.
        // IDEA: Use checked shift with fallback behavior?
        let mut bits: W = if k == Self::BITS {
            !W::zero()
        } else {
            (W::one() << k) - W::one()
        };

        let last: W = if k == 0 {
            W::zero()
        } else {
            (!W::zero() << Self::BITS - k) >> Self::BITS - n
        };

        let mut stop: bool = false;

        iter::from_fn(move || {
            if stop {
                None
            } else if bits == last {
                stop = true;
                Some(Self(bits))
            } else {
                // Gosper's hack.
                let b = bits;
                let c = b & b.wrapping_neg();
                let r = b + c;
                debug_assert_eq!(c.count_ones(), 1);
                // The following equals the standard `(((r ^ b) >> 2) / c) | r` and might be faster.
                bits = (r ^ b)
                    .checked_shr(2 + c.trailing_zeros())
                    .unwrap_or(W::zero())
                    | r;
                Some(Self(b))
            }
        })
    }

    // ------------------
    // Conversion methods
    // ------------------

    /// A copy of the internal storage word.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![0, 2, 4].word(), 1 + 4 + 16);
    /// ```
    #[inline]
    pub fn word(self) -> W {
        self.0
    }

    /// A writable reference to the internal storage word.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let mut set = set![];
    /// *set.word_mut() |= 1 + 4 + 16; // Set bits with index 0, 2, and 4.
    /// assert_eq!(set, set![0, 2, 4]);
    /// ```
    #[inline]
    pub fn word_mut(&mut self) -> &mut W {
        &mut self.0
    }

    #[inline]
    pub fn iter(self) -> BitSetIter<W> {
        BitSetIter::<W>(self.0)
    }

    /// The elements as a sorted vector of type [`Vec<Element>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2, 3].to_vec(), vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn to_vec(self) -> Vec<Element> {
        self.iter().collect()
    }
}
