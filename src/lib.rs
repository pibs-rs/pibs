//! A **p**rimitive integer b**itset** for high-performance combinatorics involving small numbers.
//!
//! # Purpose
//!
//! The focus of this crate are
//! 1. **minimal overhead** over bitwise operations (no allocation or block management) and
//! 2. a rich interface for **mathematical operations** that involve sets of (small) non-negative
//!    integers.
//!
//! It is best suited when bitset operations are performance critical and all elements naturally lie
//! between 0 and 127 (inclusive).
//!
//! # Alternatives
//!
//! The [`BitSet`] offered by this crate uses a single primitive integer type for storage, and is
//! thus limited to hold numbers up to 127 (using [`u128`]). If your numbers can be larger but you
//! know an upper bound, consider using [fixedbitset](https://docs.rs/fixedbitset) instead.
//! If you don't know your largest number ahead of time, [bit-set](https://docs.rs/bit_set) may
//! be what you are looking for. If you want minimal overhead but only need bit manipulation as
//! opposed to mathematical set abstraction, consider [bittle](https://docs.rs/bittle).
// TODO: Add Examples section.

#![feature(trait_alias)]
#![no_std]

#[cfg(test)]
mod tests;

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
    ops::{Add, AddAssign, BitAndAssign, BitOrAssign, Range, RangeInclusive, Shl, Sub},
};
use num_traits::{CheckedShr, PrimInt, Unsigned, WrappingNeg};

/// A [`BitSet`] using a [`usize`] for highest performance.
///
/// On 64 bit systems, this set can store integers between 0 and 63 (inclusive).
/// For numbers up to 127, use [`BitSet<u128>`] at a potential performance cost.
pub type Set = BitSet<usize>;

/// A [`BitSet`] using a [`u128`] for highest capacity.
///
/// On 64 bit systems, this set can store integers between 0 and 63 (inclusive).
/// For numbers up to 127, use [`BitSet<u128>`] at a potential performance cost.
pub type Set128 = BitSet<u128>;

/// An alias for [`usize`], the default input and output type for numbers stored in a [`BitSet`].
pub type Element = usize;

/// Describes a primitive integer type that [`BitSet`] can use for storage.
pub trait Word = PrimInt
    + Unsigned
    + Debug
    + AddAssign
    + BitAndAssign
    + BitOrAssign
    + Shl<Element, Output = Self>
    + CheckedShr
    + WrappingNeg;

/// A high-performance generic bitset that uses a single primitive integer for storage.
///
/// # Documentation conventions
/// The examples below use the following imports.
/// ```
/// use pitset::{BitSet, Set};
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitSet<W: Word>(W);

impl<W: Word> BitSet<W> {
    /// The number of bits in the [primitive integer type](Word) `W`.
    ///
    /// # Example
    /// ```
    /// # use pitset::BitSet;
    /// assert_eq!(BitSet::<u32>::BITS, 32);
    /// ```
    pub const BITS: usize = size_of::<W>() * 8;

    /// The smallest integer that can be stored in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert_eq!(Set::MIN, 0);
    /// ```
    pub const MIN: Element = 0;

    /// The largest integer that can be stored in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::BitSet;
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

    // -------
    // Queries
    // -------

    /// Number of elements in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert_eq!(Set::interval(4, 6).len(), 3);
    /// ```
    #[inline]
    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    /// The smallest element in the set, if any.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert_eq!(Set::new().min(), None);
    /// assert_eq!(Set::interval(4, 6).min(), Some(4));
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
    /// # use pitset::Set;
    /// assert_eq!(Set::new().max(), None);
    /// assert_eq!(Set::interval(4, 6).max(), Some(6));
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
    /// # use pitset::Set;
    /// assert!(Set::new().is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == W::zero()
    }

    /// Whether an element is contained in the set.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert!(Set::singleton(5).contains(5));
    /// ```
    ///
    /// # Panics
    /// If the element exceeds [`Self::MAX`].
    /// ```should_panic
    /// # use pitset::BitSet;
    /// BitSet::<u8>::singleton(5).contains(8);
    /// ```
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
    pub fn intersects(&self, other: &Self) -> bool {
        self.0 & other.0 != W::zero()
    }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0 & other.0 == W::zero()
    }

    /// Whether the elements form a contiguous interval.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let mut set = Set::interval(4, 6);
    /// assert!(set.is_interval());
    /// set.remove(5);
    /// assert!(!set.is_interval());
    ///
    /// // Empty sets and singletons are intervals.
    /// assert!(Set::singleton(5).is_interval());
    /// assert!(Set::new().is_interval());
    /// ```
    #[inline]
    pub fn is_interval(&self) -> bool {
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

    /// Ordinal position of an element in the set, counted from zero.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let set: Set = (4..=6).into();
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

    /// Ordinal position of an element assumed to be in the set, counted from zero.
    ///
    /// # Undefined behavior
    ///
    /// If the element is not in the set.
    #[inline]
    pub fn position_unchecked(&self, e: Element) -> usize {
        debug_assert!(self.contains(e));
        (self.0 & ((W::one() << e) - W::one())).count_ones() as usize
    }

    /// Ordinal position of an element in the set, counted from one.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let set: Set = (4..=6).into();
    /// assert_eq!(set.rank(3), None);
    /// assert_eq!(set.rank(4), Some(1));
    /// assert_eq!(set.rank(6), Some(3));
    /// ```
    #[inline]
    pub fn rank(&self, e: Element) -> Option<usize> {
        self.position(e).and_then(|p| Some(p + 1))
    }

    /// Ordinal position of an element assumed to be in the set, counted from one.
    ///
    /// # Undefined behavior
    ///
    /// If the element is not in the set.
    #[inline]
    pub fn rank_unchecked(&self, e: Element) -> usize {
        debug_assert!(self.contains(e));
        self.position_unchecked(e) + 1
    }

    // ------------
    // Constructors
    // ------------

    /// Create an empty set.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert!(Set::new().is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a singleton set.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert_eq!(Set::singleton(5).into_vec(), vec![5]);
    /// ```
    ///
    /// # Panics
    /// If `e` exceeds [`BitSet::MAX`].
    /// ```should_panic
    /// # use pitset::Set;
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
    /// # use pitset::Set;
    /// assert_eq!(Set::interval(1, 3).into_vec(), vec![1, 2, 3]);
    /// assert_eq!(Set::interval(2, 2), Set::singleton(2));
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
    /// # use pitset::Set;
    /// assert_eq!(Set::from_word(1 + 4 + 16).into_vec(), vec![0, 2, 4]);
    /// assert_eq!(Set::from_word(123).word(), 123);
    /// ```
    #[inline]
    pub fn from_word(word: W) -> Self {
        Self(word)
    }

    // -----------
    // Enumerators
    // -----------

    /// Generate all representable sets with the maximum number growing slowly.
    ///
    /// # Example
    /// ```
    /// # use pitset::BitSet;
    /// type S = BitSet<u8>;
    /// let all_sets: Vec<_> = S::iter_all().collect();
    /// assert_eq!(all_sets.len(), 256);
    /// assert_eq!(all_sets[0], S::new());
    /// assert_eq!(all_sets[1], S::singleton(0));
    /// assert_eq!(all_sets[2], S::singleton(1));
    /// assert_eq!(all_sets[3], S::from(0..2));
    /// assert_eq!(all_sets[4], S::singleton(2));
    /// assert_eq!(all_sets[all_sets.len() - 1], S::from(0..8));
    /// ```
    pub fn iter_all() -> impl Iterator<Item = Self> {
        let mut set = Self::new();
        let mut stop = false;

        iter::from_fn(move || {
            if stop {
                None
            } else {
                let next = set;
                if let Some(word) = set.0.checked_add(&W::one()) {
                    set.0 = word;
                } else {
                    stop = true;
                }
                Some(next)
            }
        })
    }

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

    /// Generate all subsets of `0..n` with the cardinality growing slowly.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let subsets: Vec<_> = Set::iter_all_below(3).collect();
    /// let as_vecs: Vec<Vec<_>> = subsets.into_iter().map(|set| set.into_vec()).collect();
    /// assert_eq!(as_vecs, vec![
    ///     vec![],
    ///     vec![0],
    ///     vec![1],
    ///     vec![2],
    ///     vec![0, 1],
    ///     vec![0, 2],
    ///     vec![1, 2],
    ///     vec![0, 1, 2]],
    /// );
    /// ```
    pub fn iter_all_below(n: usize) -> impl Iterator<Item = Self> {
        (0..=n).flat_map(move |k| Self::bit_combinations(n, k).map(|word| Self(word)))
    }

    // ------------------
    // Conversion methods
    // ------------------

    /// A copy of the internal storage word.
    ///
    /// # Example
    /// ```
    /// # use pitset::BitSet;
    /// let set = BitSet::<u8>::from(vec![0, 2, 4]);
    /// assert_eq!(set.word(), 1u8 + 4u8 + 16u8);
    /// ```
    pub fn word(&self) -> W {
        self.0
    }

    /// A writable reference to the internal storage word.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let mut set = Set::new();
    /// *set.word_mut() |= 1 + 4 + 16; // Set bits with index 0, 2, and 4.
    /// assert_eq!(set.into_vec(), vec![0, 2, 4]);
    /// ```
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
    /// # use pitset::Set;
    /// assert_eq!(Set::interval(1, 3).to_vec(), vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn to_vec(&self) -> Vec<Element> {
        self.iter().collect()
    }

    /// The elements as a sorted vector of type [`Vec<Element>`].
    ///
    /// See [`From<BitSet<W>> for Vec<T>`](#impl-From<BitSet<W>>-for-Vec<T>) for  conversion to
    /// vectors storing other primitive integer types.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// assert_eq!(Set::interval(1, 3).into_vec(), vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_vec(self) -> Vec<Element> {
        self.into_iter().collect()
    }
}

// ---------------------
// Trait implementations
// ---------------------

impl<W: Word> fmt::Display for BitSet<W> {
    /// Pretty-format a bitset.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let set = Set::from(vec![0, 10, 1, 20]);
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
    /// # use pitset::Set;
    /// let set = Set::from(vec![0, 10, 1, 20]);
    /// assert_eq!(format!("{:?}", set), "BitSet<usize>(1049603)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().rsplit("::").next().unwrap())
            .field(&self.0)
            .finish()
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
    /// # use pitset::Set;
    /// use core::iter::once;
    /// let iter = once(0).chain(once(5));
    /// let set = Set::from_iter(iter);
    /// assert_eq!(set.into_vec(), vec![0, 5]);
    /// ```
    ///
    /// # Panics
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::Set;
    /// use core::iter::once;
    /// Set::from_iter(once(-1));
    /// ```
    /// ```should_panic
    /// # use pitset::Set;
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

// --------------------
// From implementations
// --------------------

impl<W: Word, T> From<Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an integer vector.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let set: Set = vec![2, 4, 6].into();
    /// assert_eq!(set.into_vec(), vec![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::Set;
    /// Set::from(vec![-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::Set;
    /// Set::from(vec![10_000]);
    /// ```
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

impl<W: Word, T> From<&Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an integer vector by reference.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// let vec = vec![2, 4, 6];
    /// let set = Set::from(&vec);
    /// assert_eq!(set.into_vec(), vec);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::Set;
    /// Set::from(&vec![-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::Set;
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
    /// # use pitset::Set;
    /// for range in [(2..5), (2..3), (2..2), (2..1)] {
    ///     let set: Set = range.clone().into();
    ///     let vec: Vec<_> = range.collect();
    ///     assert_eq!(set.into_vec(), vec);
    /// }
    /// ```
    #[inline]
    fn from(range: Range<Element>) -> Self {
        Self::interval(range.start, range.end - 1)
    }
}

impl<W: Word> From<RangeInclusive<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an inclusive range.
    ///
    /// # Example
    /// ```
    /// # use pitset::Set;
    /// for range in [(2..=4), (2..=2), (2..=1)] {
    ///     let set: Set = range.clone().into();
    ///     let vec: Vec<_> = range.collect();
    ///     assert_eq!(set.into_vec(), vec);
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

/// Iterator over the [elements](Element) of a [`BitSet`].
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
    /// # use pitset::BitSet;
    /// type S = BitSet<u128>;
    /// let set = S::from(vec![S::MIN, S::MAX]);
    /// let v: Vec<i8> = set.into();
    /// assert_eq!(v, vec![0, 127]);
    /// ```
    /// To avoid a type hint, use [`BitSet::to_vec`] or [`BitSet::into_vec`], which always produce a
    /// [`Vec<Element>`].
    /// ```
    /// # use pitset::Set;
    /// let set = Set::interval(1, 3);
    /// let vec = set.to_vec(); // does not consume the set
    /// let vec = set.into_vec(); // consumes the set
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
    fn from(set: BitSet<W>) -> Self {
        set.into_iter()
            .map(|e| match T::try_from(e) {
                Ok(x) => x,
                Err(_) => panic!("bitset element cannot be represented by target integer type"),
            })
            .collect()
    }
}
