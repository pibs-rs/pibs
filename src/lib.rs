//! A [primitive integer bitset](BitSet) for high-performance combinatorics involving small numbers.
//!
//! # Scope
//!
//! This crate offers
//! 1. **zero-cost abstraction** over bitwise operations without allocation or block management and
//! 2. a rich interface for **combinatorics** involving sets of small non-negative integers.
//!
//! It is best suited when the bitset should abstract a mathematical set, the performance of set
//! operations is a concern, and the elements naturally lie in the representable range `0..=127`.
//! See [Alternatives](#alternatives) if your use case differs.
//!
//! # Examples
//!
//! ...
//!
//! # Usage
//! ## Onboarding
//!
//! Add the crate to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! pibs = "0.1"                                            # with default features
//! # pibs = { version = "0.1", default-features = false }  # without default features
//! # pibs = { version = "0.1", features = ["serde"] }      # with 'serde' feature
//! ```
//!
//! ## Features and dependencies
//!
//! | feature | default | implements                                         |
//! | ------- | ------- | -------------------------------------------------- |
//! | `alloc` | yes     | conversion from and to [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html) |
//! | `serde` | no      | (de)serialization via [`serde`](https://serde.rs/) |
//!
//! The crate is [no_std](https://docs.rust-embedded.org/book/intro/no-std.html)-compatible and its
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
//! The obvious limitation of this crate is that [`BitSet`] can only store numbers up to 127. If
//! your numbers can be larger than this but you know an upper bound, consider using
//! [fixedbitset](https://docs.rs/fixedbitset) or [bittle](https://docs.rs/bittle) instead. If you
//! don't know your largest number ahead of time, then [bit-set](https://docs.rs/bit_set) may be
//! what you are looking for.

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
