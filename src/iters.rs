//! Iterators returned by [`BitSet`] methods.

use crate::*;
use core::{iter::ExactSizeIterator, mem::MaybeUninit};

/// Iterator returned by [`BitSet::iter`] and [`BitSet::into_iter`].
pub struct BitSetIter<W: Word>(pub(crate) W);

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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.0.count_ones() as usize;
        (remaining, Some(remaining))
    }
}

impl<W: Word> ExactSizeIterator for BitSetIter<W> {}

/// Iterator returned by [`BitSet::subsets_of_size`].
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
    pub(crate) fn new(set: W, size: usize) -> Self {
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
        assert!(
            size < suffixes.len(),
            "can only generate subsets of size up to {}",
            suffixes.len() - 1
        );
        let mut suffix = W::zero();
        let mut remainder = set;
        suffixes[0].write(suffix);
        for cell in suffixes.iter_mut().skip(1).take(size) {
            let next_bit = W::one() << remainder.trailing_zeros() as usize;
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
            debug_assert!(self.subset & !self.set == W::zero());
            debug_assert!(self.subset.count_ones() as usize == self.size);
            let next = self.subset;
            let bit = self.subset & self.subset.wrapping_neg();
            if bit != W::zero()
                && let Some(x) = (self.subset | !self.set).checked_add(&bit)
            {
                let prefix = x & self.set;
                let lost = (self.subset.count_ones() - prefix.count_ones()) as usize;
                assert!(lost <= self.size); // SAFETY: self.suffixes is initialized up to self.size.
                let suffix = unsafe { self.suffixes.get_unchecked(lost).assume_init() };
                debug_assert!(prefix & suffix == W::zero());
                self.subset = prefix | suffix;
            } else {
                self.stop = true;
            }
            Some(BitSet::<W>(next))
        }
    }
}
