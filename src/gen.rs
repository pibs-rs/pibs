//! Generators: methods that return iterators which yield [`BitSet`].

use crate::*;
use core::iter;

impl<W: Word> BitSet<W> {
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
        let mut word = W::ZERO;
        let mut stop = false;

        iter::from_fn(move || {
            if stop {
                None
            } else {
                let next = word;
                if let Some(next_word) = word.checked_add(&W::ONE) {
                    word = next_word;
                } else {
                    stop = true;
                }
                Some(Self(next))
            }
        })
    }

    /// Generate all representable sets, with the cardinality growing slowly.
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
    /// The caller must ensure that `n <= Self::BITS`.
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
    /// The caller must ensure that `k <= n <= Self::BITS`.
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

        // TODO: Avoid cases via an unbounded shift once num_traits::UnboundedShl exists.
        let mut bits: W = if k == Self::BITS {
            !W::ZERO
        } else {
            (W::ONE << k) - W::ONE
        };

        // TODO: Avoid cases via an unbounded shift once num_traits::UnboundedShl exists.
        let last: W = if k == 0 {
            W::ZERO
        } else {
            (!W::ZERO << (Self::BITS - k)) >> (Self::BITS - n)
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
                    .unwrap_or(W::ZERO)
                    | r;
                Some(Self(b))
            }
        })
    }

    /// Generate all subsets, with the maximum number growing slowly.
    ///
    /// See [`Self::subsets_by_size`] for a different iteration order.
    /// To generate all subsets of `0..=Self::MAX`, use the faster [`Self::iter_all`].
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
        let mut word = W::ZERO;
        let mut stop = false;

        iter::from_fn(move || {
            if stop {
                None
            } else {
                let next = word;
                if let Some(x) = (word | !self.0).checked_add(&W::ONE) {
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
    ///
    /// # Panics
    ///
    /// If `size <= self.len()` and `size > 128`. Note that this requires the storage word `W` to
    /// have capacity for more than 128 elements; builtin primitives up to [`u128`] are infallible.
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
}
