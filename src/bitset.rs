//! Declaration and low-level implementation of [`BitSet`].

use crate::*;
use core::any::type_name;
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
#[repr(transparent)]
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
    // FIXME: This need not work for any implementation of Word.
    //        As soon as num_traits supports querying the bit length at compile time, use that.
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
    pub const MAX: Element = Self::BITS as Element - 1;

    /// A copy of the internal storage word.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![0, 2, 4].word(), 1 + 4 + 16);
    /// ```
    #[inline]
    pub const fn word(self) -> W {
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
    pub const fn word_mut(&mut self) -> &mut W {
        &mut self.0
    }

    #[inline(always)]
    pub(crate) fn debug_bound_check(e: Element) {
        debug_assert_eq!(
            Self::BITS,
            W::zero().count_zeros() as usize,
            "calculated bit length of {} does not match the bit length of its zero",
            type_name::<W>()
        );
        debug_assert!(
            e <= Self::MAX,
            "element {} out of bounds for {}: maximum is {}",
            e,
            type_name::<Self>(),
            Self::MAX
        )
    }
}
