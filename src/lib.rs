//! A primitive integer bitset for high-performance combinatorics involving small numbers.
//!
//! # Scope
//!
//! This crate offers
//! 1. **zero-cost abstraction** over bitwise operations (no allocation or block management) and
//! 2. a rich interface for **combinatorics** involving sets of small non-negative integers.
//!
//! It is best suited when the bitset should abstract a mathematical set, set operations are
//! performance critical, and the numbers stored naturally lie in the representable range `0..=127`.
//! See [Alternatives](#alternatives) if your use case differs.
//!
//! # Usage
//!
//!
//! `# TODO`
//!
//! # Examples
//!
//! `# TODO`
//!
//! # Discussion
//! ## Impact of word size on performance
//!
//! Benchmarking suggests that on a 64 bit system, [`BitSet`] operations are about equally fast for
//! the primitives [`u32`] and [`u64`], while using [`u8`], [`u16`], or [`u128`] for storage makes
//! them slower by a factor of about two. It is thus recommended to use [`Set128`] only when needed
//! for capacity, and [`BitSet<u8>`] to [`<u32>`](BitSet<u32>) only when memory use is a concern or
//! the platform has registers of the corresponding size. The default [`Set`] uses a [`usize`], but
//! pinning to [`u32`] or [`u64`] can make sense to ensure a consistent capacity across platforms.
//!
//! ## Alternatives
//!
//! The obvious limitation of this crate is that [`BitSet`] can only store numbers up to 127. If
//! your numbers can be larger than this but you know an upper bound, consider using
//! [fixedbitset](https://docs.rs/fixedbitset) or [bittle](https://docs.rs/bittle) instead. If you
//! don't know your largest number ahead of time, then [bit-set](https://docs.rs/bit_set) may be
//! what you are looking for.

#![feature(trait_alias)]
#![no_std]

#[cfg(test)]
mod tests;

/// Re-exports [`BitSet`], [`Set`], [`Set128`], and their creation macros.
pub mod prelude {
    pub use crate::BitSet;
    pub use crate::Set;
    pub use crate::Set128;
    pub use crate::bitset;
    pub use crate::set;
    pub use crate::set128;
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use core::{
    any::type_name,
    fmt::{self, Debug},
    iter,
    mem::MaybeUninit,
    ops::{Add, AddAssign, BitAndAssign, BitOrAssign, Range, RangeInclusive, Shl, Sub},
};
use num_traits::{CheckedShr, PrimInt, Unsigned, WrappingNeg};

/// Alias for [`BitSet<usize>`]; the set offering the best performance.
///
/// On 64 bit systems, this set can store integers between 0 and 63 (inclusive).
/// For numbers up to 127, use [`Set128`] at a potential performance cost.
pub type Set = BitSet<usize>;

/// Alias for [`BitSet<u128>`]; the set with the highest capacity.
///
/// This set can store integers between 0 and 127 (inclusive).
/// For numbers smaller than the number of bits in a usize, use [`Set`] for best performance.
pub type Set128 = BitSet<u128>;

/// Alias for [`usize`], the default input and output type for the numbers stored in a [`BitSet`].
pub type Element = usize;

/// A primitive integer that [`BitSet`] can use for storage.
pub trait Word = PrimInt
    + Unsigned
    + Debug
    + AddAssign
    + BitAndAssign
    + BitOrAssign
    + Shl<Element, Output = Self>
    + CheckedShr
    + WrappingNeg;

/// A high-performance generic bitset that wraps a single primitive integer for storage.
///
/// # Documentation conventions
/// The examples below assume the prelude import:
/// ```
/// use pitset::prelude::*;
/// ```
/// which gives access to the generic [`BitSet`], its variants [`Set`] (using [`usize`]) and
/// [`Set128`] (using [`u128`]), and the associated creation macros.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitSet<W: Word>(W);

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

// ----------------------------
// Common trait implementations
// ----------------------------

impl<W: Word> Default for BitSet<W> {
    #[inline]
    fn default() -> Self {
        Self(W::zero())
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

impl<W: Word, T> FromIterator<T> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an integer iterator.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// use core::iter::once;
    /// assert_eq!(Set::from_iter(once(0).chain(once(5))), set![0, 5]);
    /// ```
    ///
    /// # Panics
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// use core::iter::once;
    /// Set::from_iter(once(-1));
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// use core::iter::once;
    /// Set::from_iter(once(10_000));
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut word = W::zero();

        for e in iter {
            let e = match e.try_into() {
                Ok(x) => x,
                Err(_) => panic!("failed to load a bitset element from an iterator"),
            };
            Self::debug_bound_check(e);
            word += W::one() << e;
        }

        Self(word)
    }
}

impl<W: Word> fmt::Display for BitSet<W> {
    /// Pretty-format a bitset.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let set = set![0, 10, 1, 20];
    /// assert_eq!(format!("{}", set), "{0, 1, 10, 20}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for e in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", e)?;
            first = false;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<W: Word> fmt::Debug for BitSet<W> {
    /// Debug-format a bitset.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let set = set![0, 10, 1, 20];
    /// assert_eq!(format!("{:?}", set), "BitSet<usize>(1049603)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().rsplit("::").next().unwrap())
            .field(&self.0)
            .finish()
    }
}

// ------------------------------
// Operator trait implementations
// ------------------------------

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

// --------------------------
// From trait implementations
// --------------------------

impl<W: Word, T, const N: usize> From<[T; N]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an array.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from([2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([10_000]);
    /// ```
    #[inline]
    fn from(arr: [T; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<W: Word, T, const N: usize> From<&[T; N]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an array by reference.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from(&[2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&[-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&[10_000]);
    /// ```
    #[inline]
    fn from(arr: &[T; N]) -> Self {
        arr.iter().copied().collect()
    }
}

impl<W: Word, T> From<&[T]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from a slice.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from([2, 4, 6].as_slice()), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([-1].as_slice());
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([10_000].as_slice());
    /// ```
    #[inline]
    fn from(slice: &[T]) -> Self {
        slice.iter().copied().collect()
    }
}

#[cfg(feature = "alloc")]
impl<W: Word, T> From<Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from a vector.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from(vec![2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(vec![-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(vec![10_000]);
    /// ```
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<W: Word, T> From<&Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from a vector by reference.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from(&vec![2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&vec![-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&vec![10_000]);
    #[inline]
    fn from(vec: &Vec<T>) -> Self {
        vec.iter().copied().collect()
    }
}

impl<W: Word> From<Range<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an end-exclusive range.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// for range in [(2..5), (2..3), (2..2), (2..1)] {
    ///     let set: Set = range.clone().into();
    ///     assert!(set.iter().eq(range));
    /// }
    /// ```
    #[inline]
    fn from(range: Range<Element>) -> Self {
        if range.end == 0 {
            Self::new()
        } else {
            Self::interval(range.start, range.end - 1)
        }
    }
}

impl<W: Word> From<RangeInclusive<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an inclusive range.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// for range in [(2..=4), (2..=2), (2..=1)] {
    ///     let set: Set = range.clone().into();
    ///     assert!(set.iter().eq(range));
    /// }
    /// ```
    #[inline]
    fn from(range: RangeInclusive<Element>) -> Self {
        Self::interval(*range.start(), *range.end())
    }
}

// --------------------
// Associated iterators
// --------------------

/// Iterator returned by [`BitSet::iter`] and [`BitSet::into_iter`].
#[doc(hidden)]
pub struct BitSetIter<W: Word>(W);

impl<W: Word> Iterator for BitSetIter<W> {
    type Item = Element;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == W::zero() {
            return None;
        }
        let item = self.0.trailing_zeros() as Self::Item;
        self.0 &= self.0 - W::one();
        Some(item)
    }
}

/// Iterator returned by [`BitSet::subsets_of_size`].
#[doc(hidden)]
pub struct SubsetsOfSizeIter<W> {
    /// `suffixes[i]` for `i` in `0..=size` stores `subset` with all but the last `i`` ones zeroed.
    suffixes: [MaybeUninit<W>; u128::BITS as usize + 1],
    /// Cardinality of the subsets to generate.
    size: usize,
    /// The base set.
    set: W,
    /// The current subset.
    subset: W,
    /// Whether to yield [`None`] next.
    stop: bool,
}

impl<W: Word> SubsetsOfSizeIter<W> {
    #[inline]
    fn new(set: W, size: usize) -> Self {
        let mut suffixes = [const { MaybeUninit::uninit() }; _];
        if size > set.count_ones() as usize {
            return Self {
                suffixes,          // Unused.
                size: 0,           // Unused.
                set: W::zero(),    // Unused.
                subset: W::zero(), // Unused.
                stop: true,
            };
        }
        debug_assert!(size < suffixes.len());
        let mut suffix = W::zero();
        let mut remainder = set;
        suffixes[0].write(suffix);
        for i in 1..=size {
            let next_bit = W::one() << remainder.trailing_zeros() as usize;
            suffix |= next_bit;
            remainder &= !next_bit;
            suffixes[i].write(suffix);
        }
        debug_assert_eq!(suffix.count_ones() as usize, size);
        Self {
            suffixes,
            size,
            set,
            subset: suffix,
            stop: false,
        }
    }
}

impl<W: Word> Iterator for SubsetsOfSizeIter<W> {
    type Item = BitSet<W>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            None
        } else {
            debug_assert!(self.subset & !self.set == W::zero());
            debug_assert!(self.subset.count_ones() as usize == self.size);
            let next = self.subset;
            let bit = self.subset & self.subset.wrapping_neg();
            if bit != W::zero()
                && let Some(x) = (self.subset | !self.set).checked_add(&bit)
            {
                let prefix = x & self.set;
                let lost = (self.subset.count_ones() - prefix.count_ones()) as usize;
                let suffix = unsafe { self.suffixes[lost].assume_init() };
                debug_assert!(prefix & suffix == W::zero());
                self.subset = prefix | suffix;
            } else {
                self.stop = true;
            }
            Some(BitSet::<W>(next))
        }
    }
}

// ---------------------------------
// Implementations for foreign types
// ---------------------------------

#[cfg(feature = "alloc")]
impl<W: Word, T> From<BitSet<W>> for Vec<T>
where
    T: PrimInt + TryFrom<Element>,
{
    /// Create a sorted [`Vec`] from a [`BitSet`].
    ///
    /// # Examples
    /// Any element in a [`BitSet<u128>`] can fit in a [`Vec<i8>`].
    /// ```
    /// # use pitset::prelude::*;
    /// let set = Set128::interval(Set128::MIN, Set128::MAX);
    /// let vec: Vec<i8> = set.into();
    /// assert_eq!(set.len(), u128::BITS as usize);
    /// assert_eq!(set.to_vec(), vec.into_iter().map(|x| x as usize).collect::<Vec<_>>());
    /// ```
    /// To avoid a type hint, use [`BitSet::to_vec`], which always produces a [`Vec<Element>`].
    /// ```
    /// # use pitset::prelude::*;
    /// let vec = set![1, 2, 3].to_vec();
    /// ```
    ///
    /// # Panics
    ///
    /// If an element of the bitset cannot be represented by `T`.
    ///
    /// Note that even the extreme combination of [`BitSet<u128>`] and [`Vec<i8>`] is safe as the
    /// largest possible element in the former (127) can still be represented by the latter.
    /// Therefore, this implementation could only panic if additional primitive integer types are
    /// introduced in the future.
    #[inline]
    fn from(set: BitSet<W>) -> Self {
        set.into_iter()
            .map(|e| match T::try_from(e) {
                Ok(x) => x,
                Err(_) => panic!("bitset element cannot be represented by target integer type"),
            })
            .collect()
    }
}

// ------
// Macros
// ------

/// Create a [`BitSet`] using the given primitive type to store the arguments.
///
/// # Example
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(bitset![u8; 0..5], BitSet::<u8>::from(0..5));
/// assert_eq!(bitset![usize; 1..=23], BitSet::<usize>::from(1..=23));
/// assert_eq!(bitset![u128; 0, 64, 127], BitSet::<u128>::from([0, 64, 127]));
/// ```
///
/// # Compile-time checks
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = bitset![u8; 6, 7, 8]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! bitset {
    ($word:ty; $start:literal .. $end:literal) => {{
        const _: () = assert!($end <= BitSet::<$word>::BITS);
        if $end <= 0 {
            BitSet::<$word>::new()
        } else {
            BitSet::<$word>::interval($start, $end - 1)
        }
    }};
    ($word:ty; $start:literal ..= $last:literal) => {{
        const _: () = assert!($last <= BitSet::<$word>::MAX);
        BitSet::<$word>::interval($start, $last)
    }};
    ($word:ty; $($element:expr),* $(,)?) => {{
        $(const _: () = assert!($element < BitSet::<$word>::BITS);)*
        BitSet::<$word>::from_word(0 as $word $(| ((1 as $word) << $element))*)
    }};
}

/// Create a [`Set`] containing the arguments.
///
///  # Example
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(set![], Set::new());
/// assert_eq!(set![5], Set::singleton(5));
/// assert_eq!(set![0, 2, 4], Set::from([0, 2, 4]));
/// assert_eq!(set![0..4], Set::from(0..4));
/// assert_eq!(set![0..=4], Set::from(0..=4));
/// ```
///
/// # Compile-time checks
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = set![10_000]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! set {
    ($($tt:tt)*) => {
        $crate::bitset!(usize; $($tt)*)
    };
}

/// Create a [`Set128`] containing the arguments.
///
/// # Example
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(set128![100..105], Set128::from(100..105));
/// assert_eq!(set128![100..=105], Set128::from(100..=105));
/// assert_eq!(set128![0, 64, 127], Set128::from([0, 64, 127]));
/// ```
///
/// # Compile-time checks
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = set![0..=128]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! set128 {
    ($($tt:tt)*) => {
        $crate::bitset!(u128; $($tt)*)
    };
}
