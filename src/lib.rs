//! A [primitive integer bitset](BitSet) for high-performance combinatorics involving small numbers.
//!
//! # Scope
//!
//! This crate offers
//!
//! - zero-cost abstraction over bitwise operations without allocation or block management and
//! - a rich interface for mathematics/combinatorics involving integer sets.
//!
//! *pibs* is best suited when the bitset should abstract a mathematical set, the performance of set
//! operations is your primary concern, and the elements naturally lie in the representable range
//! `0..128`. See [Alternatives](#alternatives) if this is not the case.
//!
//! The ambition of *pibs* is that you can't write faster code for any of its operations. If you
//! can, please report this as an issue!
//!
//! # Examples
//!
//! All examples assume the following import, which gives access to [`Set`] = [`BitSet<usize>`] and
//! [`set!`].
//!
//! ```
//! use pibs::prelude::*;
//! ```
//!
//! ## Subset sum problem
//!
//! The [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) asks: does a given
//! set of integers have a subset with a particular sum?
//!
//! ### By bruteforce
//!
//! The following solves the problem by bruteforce. This is not efficient, but showcases the
//! [`subsets`](BitSet::subsets) iterator.
//!
//! ```
//! # use pibs::prelude::*;
//! fn subset_with_sum(set: Set, sum: usize) -> Option<Set> {
//!     set.subsets().find(|subset| subset.sum() == sum)
//! }
//!
//! let set = set![4, 7, 10, 13, 18, 22, 27];
//! let solution = subset_with_sum(set, 30).unwrap();
//! assert_eq!(solution, set![7, 10, 13]);
//! ```
//!
//! ### Fast check for small sums
//!
//! For deciding the problem without recovering the subset, a faster implementation uses
//! [`truncating_subset_sums`](BitSet::truncating_subset_sums), which produces the set of
//! representable sums of all subsets of a set.
//!
//! ```
//! # use pibs::prelude::*;
//! fn has_subset_with_sum(set: Set, sum: usize) -> bool {
//!     if sum > Set::MAX { unimplemented!("this only works for representable numbers") }
//!     set.truncating_subset_sums().contains(sum)
//! }
//!
//! let set = set![4, 7, 10, 13, 18, 22, 27];
//! assert_eq!(has_subset_with_sum(set, 30), true);
//! assert_eq!(has_subset_with_sum(set, 15), false);
//! ```
//!
//! ## Testing Sidon sets
//!
//! A set is a [Sidon set](https://en.wikipedia.org/wiki/Sidon_sequence) if every pair of elements
//! (repetition allowed) has a unique sum. The following makes use of the
//! [`sumset`](https://en.wikipedia.org/wiki/Sumset) operator (`+`, aka Minkowski sum) to check if
//! this is the case.
//!
//! ```
//! # use pibs::prelude::*;
//! fn is_sidon(set: Set) -> bool {
//!     let n = set.len();
//!     (set + set).len() == n * (n + 1) / 2
//! }
//!
//! assert_eq!(is_sidon(set![1, 2, 4]), true);
//! assert_eq!(is_sidon(set![1, 2, 3]), false); // 2 + 2 = 1 + 3
//! ```
//!
//! ## Minimum generating set
//!
//! The following computes by bruteforce a minimum-cardinality set of positive integers that
//! generate (by taking any subset of the numbers and summing them) all elements of a target set.
//! This is done using the [`iter_combinations(n, k)`](Set::iter_combinations) generator, which
//! yields all subsets of `0..n` of size `k`, and the
//! [`truncating_add_to_all`](BitSet::truncating_add_to_all) operation (`<<`), to shift these
//! subsets to `1..=n`.
//!
//! ```
//! # use pibs::prelude::*;
//! fn min_generating_set(set: Set) -> Set {
//!     let max = set.max().unwrap_or(0);
//!     let bit_length = (usize::BITS - max.leading_zeros()) as usize; // ⌈log₂(max + 1)⌉
//!
//!     // Test all subsets of 1..=max, grouped by increasing cardinality.
//!     for size in 0..bit_length {
//!         for generator in Set::iter_combinations(max, size).map(|set| set << 1) {
//!             if set.is_subset(generator.truncating_subset_sums()) {
//!                 return generator;
//!             }
//!         }
//!     }
//!
//!     // If no small generator was found, fall back to powers of two.
//!     Set::from_iter((0..bit_length).map(|b| 1 << b))
//! }
//!
//! // To generate {0, ..., 9}, we need a generating set of size four.
//! assert_eq!(min_generating_set(set![0..=9]), set![1, 2, 4, 8]);
//!
//! // But if we don't need to generate 2 and 7, three numbers suffice.
//! assert_eq!(min_generating_set(set![0..=9] - set![2, 7]), set![1, 3, 5]);
//! ```
//!
//! # Usage
//! ## Cheat Sheet
//!
//! M denotes the largest representable element [`BitSet::MAX`].
//!
//! ### Creation
//!
//! For using [`u128`]/`W` instead of [`usize`], replace [`Set`] with [`Set128`]/[`BitSet<W>`] and [`set!`] with
//! [`set128!`]/[`bitset![W; ...]`](bitset!).
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
//! | operation | short form | mutating | long form | mutating long form
//! | --------- | ---------- | -------- | --------- | ------------------
//! | A ∪ B     | `a \| b`   | a \|= b  | [`a.union(b)`](BitSet::union) | [`a.union_update(b)`](BitSet::union_update)
//! | A ∩ B     | `a & b`    | a &= b   | [`a.intersection(b)`](BitSet::intersection) | [`a.intersection_update(b)`](BitSet::intersection_update)
//! | A ∖ B     | `a - b`    | a -= b   | [`a.difference(b)`](BitSet::difference) | [`a.difference_update(b)`](BitSet::difference_update)
//! | A ∆ B     | `a ^ b`    | a ^= b   | [`a.symmetric_difference(b)`](BitSet::symmetric_difference) | [`a.symmetric_difference_update(b)`](BitSet::symmetric_difference_update)
//! | A ∪ {x}   | `a + x`    | a += x   | [`a.with(x)`](BitSet::with) | [`a.insert(x)`](BitSet::insert)
//! | A ∖ {x}   | `a - x`    | a -= x   | [`a.without(x)`](BitSet::without) | [`a.remove(x)`](BitSet::remove)
//!
//! ### Arithmetic operations
//!
//! Default long forms are checked and return `None` if an output element cannot be represented;
//! truncating variants instead drop any values `< 0` or `> M`. The short forms are truncating.
//!
//! | operation | definition | short form   | checked variant | truncating variant
//! | --------- | ---------- | ------------ | --------------- | ------------------
//! | A + B     | {x + y \| x ∈ A ∧ y ∈ B} | `a + b`  | [`a.sumset(b)`](BitSet::sumset)             | [`a.truncating_sumset(b)`](BitSet::truncating_sumset)
//! | A + {x}   | {x + y \| y ∈ A}         | `a << x` | [`a.add_to_all(x)`](BitSet::add_to_all)     | [`a.truncating_add_to_all(x)`](BitSet::truncating_add_to_all)
//! | A - {x}   | {x - y \| y ∈ A}         | `a >> x` | [`a.sub_from_all(x)`](BitSet::sub_from_all) | [`a.truncating_sub_from_all(x)`](BitSet::truncating_sub_from_all)
//! |           | {∑(x : x ∈ X) \| X ⊆ A}  |          | [`a.subset_sums()`](BitSet::subset_sums)    | [`a.truncating_subset_sums()`](BitSet::truncating_subset_sums)
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
//! ## Performance
//! ### Checks and preconditions
//!
//! As *pibs* aims for zero-cost abstraction, it prefers preconditions over runtime checks. The
//! latter are still performed
//!
//! 1. in debug builds,
//! 1. if a foreign trait requires it, or
//! 2. whenever a precondition on a method's arguments alone is not sufficient to ensure correct
//!    operation.
//!
//! An example of (2) is [`try_from`](TryFrom::try_from), which needs to ensure that every number
//! obtained from the source collection is at most [`BitSet::MAX`]. An instance of (3) is
//! [`add_to_all`](BitSet::add_to_all): testing whether the outcome is representable requires
//! knowledge of the set's largest element in addition to the number being added, so the method
//! takes care of this check and returns an [`Option<BitSet>`]. On the other hand, a successful
//! [`insert`](BitSet::insert) only requires the argument to be within bounds, which is a
//! precondition on its use.
//!
//! Where checks are performed in release builds, *pibs* still offers alternative methods that omit
//! them. For example, [`truncating_add_to_all`](BitSet::truncating_add_to_all) discards
//! irrepresentable sums, which is a zero-cost side effect of the shift used to compute them.
//!
//! Creation macros such as [`set!`] check their arguments at compile time, which introduces no
//! runtime cost.
//!
//! ### Impact of word size
//!
//! Benchmarking suggests that on a 64 bit system, [`BitSet`] operations are often equally fast for
//! the primitives [`u32`] and [`u64`], while using [`u8`], [`u16`], or [`u128`] for storage can
//! make them slower by a factor of about two. It is thus recommended to use [`Set128`] only when
//! needed for capacity, and [`BitSet<u8>`] to [`<u32>`](BitSet<u32>) only when memory use is a
//! concern or the platform has registers of the corresponding size. The default [`Set`] uses a
//! [`usize`] for best performance.
//!
//! ## Alternatives
//!
//! The obvious limitation of *pibs* is that its [`BitSet`] can only store numbers up to 127. If
//! your numbers can be larger than this but you know a bound, consider using
//! [fixedbitset](https://docs.rs/fixedbitset) (SIMD-optimized set abstraction) or
//! [bittle](https://docs.rs/bittle) (low-level bit manipulation) instead. If you don't know your
//! largest number ahead of time, then [bit-set](https://docs.rs/bit_set) (based on
//! [bit-vec](https://docs.rs/bit_set)) or [roaring](https://docs.rs/bit_set) (compressed
//! representation) may suit you.

#![feature(trait_alias)]
#![feature(debug_closure_helpers)]
#![feature(doc_cfg)]
#![no_std]

// -------
// Modules
// -------

mod bitset;
mod create;
mod foreign;
mod from;
mod r#gen;
mod iters;
mod macros;
mod math;
mod mutate;
mod ops;
mod query;
mod set;
#[cfg(feature = "alloc")]
mod vec;

// -------
// Imports
// -------

use core::ops::{AddAssign, BitAndAssign, BitOrAssign, BitXorAssign};
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
    + AddAssign
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + CheckedShl
    + CheckedShr
    + WrappingNeg;
