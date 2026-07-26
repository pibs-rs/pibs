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
/// The examples below assume the prelude import:
/// ```
/// use pitset::prelude::*;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitSet<W: Word>(pub(crate) W);

impl<W: Word> BitSet<W> {
    /// The number of bits in the [primitive integer type](Word) `W`.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(BitSet::<u32>::BITS, 32);
    /// ```
    pub const BITS: usize = size_of::<W>() * 8;

    /// The smallest integer that can be stored in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::MIN, 0);
    /// ```
    pub const MIN: Element = 0;

    /// The largest integer that can be stored in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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

    #[inline]
    pub(crate) fn bit_combinations(n: usize, k: usize) -> impl Iterator<Item = W> {
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
                Some(bits)
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
                Some(b)
            }
        })
    }

    // -------
    // Queries
    // -------

    /// Number of elements in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(set![4..=6].len(), 3);
    /// ```
    #[inline]
    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    /// The smallest element in the set, if any.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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

    /// The largest element in the set, if any.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert!(Set::new().is_empty());
    /// ```
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == W::zero()
    }

    /// Whether an element is contained in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert!(set![4, 5, 6].contains(5));
    /// assert!(!set![4, 5, 6].contains(8));
    /// ```
    ///
    /// # Panics
    /// If the element exceeds [`Self::MAX`].
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// bitset![u8; 4, 5, 6].contains(8);
    /// ```
    #[inline]
    pub fn contains(self, e: Element) -> bool {
        Self::debug_bound_check(e);
        self.0 & (W::one() << e) != W::zero()
    }

    #[inline]
    pub fn is_subset(self, other: Self) -> bool {
        self.0 & !other.0 == W::zero()
    }

    #[inline]
    pub fn is_superset(self, other: Self) -> bool {
        !self.0 & other.0 == W::zero()
    }

    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != W::zero()
    }

    #[inline]
    pub fn is_disjoint(self, other: Self) -> bool {
        self.0 & other.0 == W::zero()
    }

    /// Whether the elements form a contiguous interval.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let mut set = set![4..=6];
    /// assert!(set.is_interval());
    /// set.remove(5);
    /// assert!(!set.is_interval());
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
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    pub fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Ordinal position of an element in the set, counted from zero.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Undefined behavior
    ///
    /// If the element is not in the set.
    #[inline]
    pub fn position_unchecked(self, e: Element) -> usize {
        debug_assert!(self.contains(e));
        (self.0 & ((W::one() << e) - W::one())).count_ones() as usize
    }

    /// Ordinal position of an element in the set, counted from one.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Undefined behavior
    ///
    /// If the element is not in the set.
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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

    // ------------
    // Constructors
    // ------------

    /// Create an empty set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert!(Set::new().is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a set containing all representable elements.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(BitSet::<u8>::full(), bitset![u8; 0..8]);
    /// assert_eq!(Set128::full().len(), 128);
    /// ```
    #[inline]
    pub fn full() -> Self {
        Self(W::one().wrapping_neg())
    }

    /// Create a singleton set.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::singleton(5), set![5]);
    /// ```
    ///
    /// # Panics
    /// If `e` exceeds [`BitSet::MAX`].
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::singleton(10_000);
    /// ```
    #[inline]
    pub fn singleton(e: Element) -> Self {
        Self::debug_bound_check(e);
        Self(W::one() << e)
    }

    /// Create a contiguous interval.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::interval(1, 3), set![1..=3]);
    /// assert_eq!(Set::interval(2, 2), set![2]);
    /// assert!(Set::interval(3, 1).is_empty());
    /// ```
    ///
    /// # Panics
    /// If `last` exceeds [`Self::MAX`] in debug builds.
    ///
    /// # Undefined behavior
    /// If `last` exceeds [`Self::MAX`] in release builds.
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    // FIXME: Appears to not panic but hang when n > Self::BITS.
    #[inline]
    pub fn iter_all_below(n: usize) -> impl Iterator<Item = Self> {
        (0..=n).flat_map(move |k| Self::iter_combinations(n, k))
    }

    /// Generate all (n choose k) subsets of `0..n` with cardinality k.
    ///
    /// The maximum number is growing slowly.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
        Self::bit_combinations(n, k).map(Self)
    }

    // ------------------
    // Conversion methods
    // ------------------

    /// A copy of the internal storage word.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(set![0, 2, 4].word(), 1 + 4 + 16);
    /// ```
    #[inline]
    pub fn word(self) -> W {
        self.0
    }

    /// A writable reference to the internal storage word.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(set![1, 2, 3].to_vec(), vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn to_vec(self) -> Vec<Element> {
        self.iter().collect()
    }
}
