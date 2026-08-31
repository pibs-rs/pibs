//! Iterators returned by [`BitSet`] methods.

use crate::*;
use core::{
    iter::{ExactSizeIterator, FusedIterator},
    mem::MaybeUninit,
};

/// Iterator returned by [`BitSet::iter`] and [`BitSet::into_iter`].
pub struct BitSetIter<W: Word>(pub(crate) W);

impl<W: Word> Iterator for BitSetIter<W> {
    type Item = Element;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == W::ZERO {
            return None;
        }
        let item = self.0.trailing_zeros() as Self::Item;
        self.0 &= self.0 - W::ONE;
        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.0.count_ones() as usize;
        (remaining, Some(remaining))
    }
}

impl<W: Word> ExactSizeIterator for BitSetIter<W> {}

impl<W: Word> FusedIterator for BitSetIter<W> {}

const SUFFIX_CACHE_SIZE: usize = u128::BITS as usize + 1;

/// Iterator returned by [`BitSet::subsets_of_size`].
pub struct SubsetsOfSizeIter<W> {
    /// `suffixes[i]` for `i` in `0..=size` stores `subset` with all but the last `i` ones zeroed.
    suffixes: [MaybeUninit<W>; SUFFIX_CACHE_SIZE],
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
    pub(crate) fn new(set: W, size: usize) -> Self {
        let mut suffixes = [const { MaybeUninit::uninit() }; SUFFIX_CACHE_SIZE];
        if size > set.count_ones() as usize {
            return Self {
                suffixes,        // Unused.
                size: 0,         // Unused.
                set: W::ZERO,    // Unused.
                subset: W::ZERO, // Unused.
                stop: true,
            };
        }
        assert!(
            size < SUFFIX_CACHE_SIZE,
            "can only generate subsets of size less than {}",
            SUFFIX_CACHE_SIZE
        );
        let mut suffix = W::ZERO;
        let mut remainder = set;
        suffixes[0].write(suffix);
        for cell in suffixes.iter_mut().skip(1).take(size) {
            let next_bit = W::ONE << remainder.trailing_zeros() as usize;
            suffix |= next_bit;
            remainder &= !next_bit;
            cell.write(suffix);
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
            debug_assert!(self.subset & !self.set == W::ZERO);
            debug_assert!(self.subset.count_ones() as usize == self.size);
            let next = self.subset;
            let bit = self.subset & self.subset.wrapping_neg();
            if bit != W::ZERO
                && let Some(x) = (self.subset | !self.set).checked_add(&bit)
            {
                let prefix = x & self.set;
                let lost = self.size - prefix.count_ones() as usize;
                assert!(lost <= self.size); // SAFETY: self.suffixes is initialized up to self.size.
                let suffix = unsafe { self.suffixes.get_unchecked(lost).assume_init() };
                debug_assert!(prefix & suffix == W::ZERO);
                self.subset = prefix | suffix;
            } else {
                self.stop = true;
            }
            Some(BitSet::<W>(next))
        }
    }
}

impl<W: Word> FusedIterator for SubsetsOfSizeIter<W> {}
