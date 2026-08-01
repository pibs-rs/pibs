//! [`From`] trait implementations for [`BitSet`].

use crate::*;
use core::ops::{Range, RangeInclusive};

impl<W: Word, T, const N: usize> From<[T; N]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an array.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every array element `e`. Violating this
    /// precondition panics in debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::from([2, 4, 6]), set![2, 4, 6]);
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
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every array element `e`. Violating this
    /// precondition panics in debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::from(&[2, 4, 6]), set![2, 4, 6]);
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
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every slice element `e`. Violating this
    /// precondition panics in debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::from([2, 4, 6].as_slice()), set![2, 4, 6]);
    /// ```
    #[inline]
    fn from(slice: &[T]) -> Self {
        slice.iter().copied().collect()
    }
}

impl<W: Word> From<Range<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an end-exclusive range.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `range.end <= Self::BITS`. Violating this precondition panics in
    /// debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
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
    /// # Preconditions
    ///
    /// The caller must ensure that `range.end() <= Self::MAX`. Violating this precondition panics
    /// in debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
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
