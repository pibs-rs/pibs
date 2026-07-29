//! A [primitive integer bitset](BitSet) for high-performance combinatorics involving small numbers.
//!
//! # Scope
//!
//! This crate offers
//!
//! - zero-cost abstraction over bitwise operations without allocation or block management and
//! - a rich interface for mathematics/combinatorics involving sets of small non-negative integers.
//!
//! *pibs* is best suited when the bitset should abstract a mathematical set, the performance of set
//! operations is your primary concern, and the elements naturally lie in the representable range
//! `0..=127`. See [Alternatives](#alternatives) if this is not the case.
//!
//! The ambition of *pibs* is that you can't write faster code for any of its operations. If you
//! can, please report this as an issue!
//!
//! # Examples
//!
//! ...
//!
//! # Usage
//! ## Cheat Sheet
//!
//! M denotes the largest representable element [`BitSet::MAX`].
//!
//! ### Creation
//!
//! | set             | short form      | long form
//! | --------------- | --------------- | ---------
//! | ∅               | `set![]`        | [`Set::new()`]
//! | {0}             | `set![0]`       | [`Set::singleton(0)`](BitSet::singleton)
//! | {1, ..., n}     | `set![1..=n]`   | [`Set::interval(1, n)`](BitSet::interval)
//! | {0, ..., n - 1} | `set![0..n]`    | `Set::from(0..n)`
//! | {0, ..., M}     |                 | [`Set::full()`]
//! | {2, 3, 5}       | `set![2, 3, 5]` | `Set::from([2, 3, 5])`
//!
//! ### Queries
//!
//! | set          | short form | long form
//! | ------------ | ---------- | ---------
//! | \|A\|        |            | [`a.len()`](BitSet::len)
//! | min A, max A |            | [`a.min()`](BitSet::min), [`a.max()`](BitSet::max)
//! | ∑(x : x ∈ A) |            | [`a.sum()`](BitSet::sum)
//! | A = ∅        |            | [`a.is_empty()`](BitSet::is_empty)
//! | A ⊆ B        | `a <= b`   | [`a.is_subset(b)`](BitSet::is_subset)
//! | A ⊇ B        | `a >= b`   | [`a.is_superset(b)`](BitSet::is_superset)
//! | A ⊂ B        | `a < b`    | [`a.is_strict_subset(b)`](BitSet::is_strict_subset)
//! | A ⊃ B        | `a > b`    | [`a.is_strict_superset(b)`](BitSet::is_strict_superset)
//! | A ∩ B ≠ ∅    |            | [`a.intersects(b)`](BitSet::intersects)
//! | A ∩ B = ∅    |            | [`a.is_disjoint(b)`](BitSet::is_disjoint)
//! | x ∈ A        |            | [`a.contains(x)`](BitSet::contains)
//! | \|A\| = max A - min A + 1 | | [`a.is_interval()`](BitSet::is_interval)
//!
//! ### Set operations
//!
//! | operation | short form | long form
//! | --------- | ---------- | ---------
//! | A ∪ B     | `a \| b`   | [`a.union(b)`](BitSet::union)
//! | A ∩ B     | `a & b`    | [`a.intersection(b)`](BitSet::intersection)
//! | A ∖ B     | `a - b`    | [`a.difference(b)`](BitSet::difference)
//! | A ∆ B     | `a ^ b`    | [`a.symmetric_difference(b)`](BitSet::symmetric_difference)
//! | A ∪ {x}   | `a + x`    | [`a.with(x)`](BitSet::with)
//! | A ∖ {x}   | `a - x`    | [`a.without(x)`](BitSet::without)
//!
//! ### Arithmetic operations
//!
//! Default long forms are checked and return `None` if an output element cannot be represented;
//! truncating variants instead drop any values `< 0` or `> M`. The short forms are truncating.
//!
//! | operation | definition | short form   | checked variant | truncating variant
//! | --------- | ---------- | ------------ | --------------- | ------------------
//! | A + B     | {x + y \| x ∈ A ∧ y ∈ B} | `a + b`  | [`a.minkowski_sum(b)`](BitSet::minkowski_sum) | [`a.truncating_minkowski_sum(b)`](BitSet::minkowski_sum)
//! | A + A     | {x + y \| x, y ∈ A}      | `a + a`  | [`a.sumset()`](BitSet::sumset)                | [`a.truncating_sumset()`](BitSet::sumset)
//! | A + {x}   | {x + y \| y ∈ A}         | `a << x` | [`a.add_to_all(x)`](BitSet::add_to_all)       | [`a.truncating_add_to_all(x)`](BitSet::add_to_all)
//! | A - {x}   | {x - y \| y ∈ A}         | `a >> x` | [`a.sub_from_all(x)`](BitSet::sub_from_all)   | [`a.truncating_sub_from_all(x)`](BitSet::sub_from_all)
//! |           | {∑(x : x ∈ X) \| X ⊆ A}  |          | [`a.subset_sum()`](BitSet::subset_sum)        | [`a.truncating_subset_sum()`](BitSet::subset_sum)
//!
//! ### Generation
//!
//! Generation ordered "by size" (cardinality grows slowly) is slower by a factor of about 10.
//!
//! | example                                                    | iterator yields
//! | ---------------------------------------------------------- | ---------------
//! | [`Set::iter_all()`]                                        | ∅, {0}, {1}, {0, 1}, {2}, {0, 2}, {1, 2}, {0, 1, 2}, {3}, ...
//! | [`Set::iter_all_by_size()`]                                | ∅, {0}, {1}, {2}, ..., {M}, {0, 1}, {0, 2}, {1, 2}, ..., {M, M - 1}, {0, 1, 2}, ...
//! | [`Set::iter_all_below(3)`](Set::iter_all_below)            | ∅, {0}, {1}, {2}, {0, 1}, {0, 2}, {1, 2}, {0, 1, 2}
//! | [`Set::iter_combinations(4, 2)`](Set::iter_combinations)   | {0, 1}, {0, 2}, {1, 2}, {0, 3}, {1, 3}, {2, 3}
//! | [`set![2, 4, 6].subsets()`](Set::subsets)                  | ∅, {2}, {4}, {2, 4}, {6}, {2, 6}, {4, 6}, {2, 4, 6}
//! | [`set![2, 4, 6].subsets_by_size()`](Set::subsets_by_size)  | ∅, {2}, {4}, {6}, {2, 4}, {2, 6}, {4, 6}, {2, 4, 6}
//! | [`set![2, 4, 6].subsets_of_size(2)`](Set::subsets_of_size) | {2, 4}, {2, 6}, {4, 6}
//!
//! ## Onboarding
//!
//! Add *pibs* to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! pibs = "0.1"                                            # with default features
//! # pibs = { version = "0.1", default-features = false }  # without default features
//! # pibs = { version = "0.1", features = ["serde"] }      # with 'serde' feature
//! ```
//!
//! ## Features and dependencies
//!
//! | feature | default | implements
//! | ------- | ------- | ----------
//! | `alloc` | yes     | conversion from and to [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)
//! | `serde` | no      | (de)serialization via [`serde`](https://serde.rs/)
//!
//! *pibs* is [no_std](https://docs.rust-embedded.org/book/intro/no-std.html)-compatible and its
//! only non-optional dependency is [`num_traits`].
//!
//! # Discussion
//! ## Checks and preconditions
//!
//! Almost all methods that take an [`Element`] as an argument require the caller to ensure that it
//! does not exceed [`BitSet::MAX`]. This condition is checked in debug builds. In release builds,
//! the outcome of providing out-of-bounds elements is unspecified.
//!
//! On the other hand, creation macros such as [`set!`] will check the numbers provided at compile
//! time.
//!
//! ## Impact of word size on performance
//!
//! Benchmarking suggests that on a 64 bit system, [`BitSet`] operations are often equally fast for
//! the primitives [`u32`] and [`u64`], while using [`u8`], [`u16`], or [`u128`] for storage can
//! make them slower by a factor of about two. It is thus recommended to use [`Set128`] only when
//! needed for capacity, and [`BitSet<u8>`] to [`<u32>`](BitSet<u32>) only when memory use is a
//! concern or the platform has registers of the corresponding size. The default [`Set`] uses a
//! [`usize`], but pinning to [`u32`] or [`u64`] can make sense to ensure a consistent capacity
//! across platforms.
//!
//! ## Alternatives
//!
//! The obvious limitation of *pibs* is that its [`BitSet`] can only store numbers up to 127. If
//! your numbers can be larger than this but you know an upper bound, consider using
//! [fixedbitset](https://docs.rs/fixedbitset) (SIMD-optimized set abstraction) or
//! [bittle](https://docs.rs/bittle) (low-level bit manipulation) instead. If you don't know your
//! largest number ahead of time, then [bit-set](https://docs.rs/bit_set) (based on
//! [bit-vec](https://docs.rs/bit_set)) or [roaring](https://docs.rs/bit_set) (compressed
//! representation) may be what you are looking for.

#![feature(trait_alias)]
#![no_std]

// -------
// Modules
// -------

mod bitset;
mod from;
mod impls;
mod iters;
mod macros;
mod ops;
#[cfg(test)]
mod tests;

// -------
// Imports
// -------

use core::{
    fmt::Debug,
    ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, Shl},
};
use num_traits::{CheckedShl, CheckedShr, PrimInt, Unsigned, WrappingNeg};

// -------
// Exports
// -------

/// Re-exports [`BitSet`], [`Set`], [`Set128`], and their creation macros.
pub mod prelude {
    pub use crate::BitSet;
    pub use crate::Set;
    pub use crate::Set128;
    pub use crate::bitset;
    pub use crate::set;
    pub use crate::set128;
}

pub use bitset::BitSet;
pub use iters::{BitSetIter, SubsetsOfSizeIter};

/// Alias for [`BitSet<usize>`]; the set offering the best performance.
///
/// On 64 bit systems, this set can store integers between 0 and 63 (inclusive).
/// For numbers up to 127, use [`Set128`] at a possible performance cost.
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
    + BitXorAssign
    + Shl<Element, Output = Self>
    + CheckedShl
    + CheckedShr
    + WrappingNeg;
