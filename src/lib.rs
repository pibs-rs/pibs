//! A primitive integer bitset for high-performance combinatorics involving small numbers.
//!
//! # Scope
//!
//! This crate offers
//! 1. **zero-cost abstraction** over bitwise operations without allocation or block management and
//! 2. a rich interface for **combinatorics** involving sets of small non-negative integers.
//!
//! It is best suited when the bitset should abstract a mathematical set, set operations are
//! performance critical, and the elements naturally lie in the representable range `0..=127`.
//! See [Alternatives](#alternatives) if your use case differs.
//!
//! # Examples
//!
//! `# TODO`
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! pitset = "0.1"                                            # with default features
//! # pitset = { version = "0.1", default-features = false }  # without default features
//! # pitset = { version = "0.1", features = ["serde"] }      # with 'serde' feature
//! ```
//!
//! ## Features
//!
//! | feature | default | implements                                         |
//! | ------- | ------- | -------------------------------------------------- |
//! | `alloc` | yes     | conversion from and to [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html) |
//! | `serde` | no      | (de)serialization via [`serde`](https://serde.rs/) |
//!
//! ## Dependencies
//!
//! The crate is [no_std](https://docs.rust-embedded.org/book/intro/no-std.html)-compatible and its
//! only dependency is [`num_traits`].
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
use num_traits::{CheckedShr, PrimInt, Unsigned, WrappingNeg};

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
    + CheckedShr
    + WrappingNeg;
