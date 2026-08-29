//! Conversion methods to [`BitSet`] from other types.

use crate::*;
use core::ops::{Range, RangeInclusive};

impl<W: Word> BitSet<W> {
    /// Try to create a [`BitSet`] from an iterator.
    ///
    /// # Errors
    ///
    /// If any item produced by the iterator fails to convert to an [`Element`] or is greater than
    /// [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use core::iter::once;
    /// let iter = once(0).chain(once(5)).chain(once(23));
    /// assert_eq!(Set::try_from_iter(iter), Ok(set![0, 5, 23]));
    /// ```
    #[inline]
    pub fn try_from_iter<I, T>(iter: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        T: TryInto<Element>,
    {
        let mut word = W::zero();

        for item in iter {
            if let Ok(e) = item.try_into()
                && e <= Self::MAX
            {
                word |= W::one() << e;
            } else {
                return Err(Error::Irrepresentable);
            }
        }

        Ok(Self(word))
    }

    /// Create a [`BitSet`] from a collection that implements [`IntoIterator<Item = Element>`].
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every element `e` produced by the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use core::iter::once;
    ///
    /// // Works for iterators.
    /// let iter = once(0).chain(once(5)).chain(once(23));
    /// assert_eq!(Set::from_unchecked(iter), set![0, 5, 23]);
    ///
    /// // Can also consume collections.
    /// let array = [0, 5, 23];
    /// assert_eq!(Set::from_unchecked(array), set![0, 5, 23]);
    /// ```
    #[inline]
    pub fn from_unchecked<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Element>,
    {
        let mut word = W::zero();

        for e in iter {
            Self::debug_bound_check(e);
            word |= W::one() << e;
        }

        Self(word)
    }
}

impl<W: Word, T, const N: usize> TryFrom<[T; N]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    type Error = Error;

    /// Try to create a [`BitSet`] from an array.
    ///
    /// # Errors
    ///
    /// If any array element fails to convert to an [`Element`] or is greater than [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use pibs::Error::Irrepresentable;
    ///
    /// assert_eq!(Set::try_from([2, 4, 6]), Ok(set![2, 4, 6]));
    /// assert_eq!(Set::try_from([-2, 4, 6]), Err(Irrepresentable));
    /// assert_eq!(Set::try_from([10_000]), Err(Irrepresentable));
    /// ```
    #[inline]
    fn try_from(array: [T; N]) -> Result<Self, Self::Error> {
        Self::try_from_iter(array)
    }
}

impl<W: Word, T, const N: usize> TryFrom<&[T; N]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    type Error = Error;

    /// Try to create a [`BitSet`] from an array by reference.
    ///
    /// # Errors
    ///
    /// If any array element fails to convert to an [`Element`] or is greater than [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use pibs::Error::Irrepresentable;
    ///
    /// assert_eq!(Set::try_from(&[2, 4, 6]), Ok(set![2, 4, 6]));
    /// assert_eq!(Set::try_from(&[-2, 4, 6]), Err(Irrepresentable));
    /// assert_eq!(Set::try_from(&[10_000]), Err(Irrepresentable));
    /// ```
    #[inline]
    fn try_from(array: &[T; N]) -> Result<Self, Self::Error> {
        Self::try_from_iter(array.iter().copied())
    }
}

impl<W: Word, T> TryFrom<&[T]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    type Error = Error;

    /// Try to create a [`BitSet`] from a slice.
    ///
    /// # Errors
    ///
    /// If any slice element fails to convert to an [`Element`] or is greater than [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use pibs::Error::Irrepresentable;
    ///
    /// assert_eq!(Set::try_from([2, 4, 6].as_slice()), Ok(set![2, 4, 6]));
    /// assert_eq!(Set::try_from([-2, 4, 6].as_slice()), Err(Irrepresentable));
    /// assert_eq!(Set::try_from([10_000].as_slice()), Err(Irrepresentable));
    /// ```
    #[inline]
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        Self::try_from_iter(slice.iter().copied())
    }
}

impl<W: Word> TryFrom<Range<Element>> for BitSet<W> {
    type Error = Error;

    /// Try to create a [`BitSet`] from an end-exclusive range.
    ///
    /// # Errors
    ///
    /// If `range.end > Self::BITS` for a non-empty range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// for range in [(2..5), (2..3), (2..2), (2..1)] {
    ///     let set = Set::try_from(range.clone());
    ///     assert!(set.is_ok());
    ///     assert!(set.unwrap().iter().eq(range));
    /// }
    /// ```
    #[inline]
    fn try_from(range: Range<Element>) -> Result<Self, Self::Error> {
        if range.is_empty() {
            Ok(Self::new())
        } else if range.end > Self::BITS {
            Err(Error::Irrepresentable)
        } else {
            debug_assert!(range.end >= 1);
            Ok(Self::interval(range.start, range.end - 1))
        }
    }
}

impl<W: Word> TryFrom<RangeInclusive<Element>> for BitSet<W> {
    type Error = Error;

    /// Try to create a [`BitSet`] from an inclusive range.
    ///
    /// # Errors
    ///
    /// If `range.end() > Self::MAX` for a non-empty range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// for range in [(2..=4), (2..=2), (2..=1)] {
    ///     let set = Set::try_from(range.clone());
    ///     assert!(set.is_ok());
    ///     assert!(set.unwrap().iter().eq(range));
    /// }
    /// ```
    #[inline]
    fn try_from(range: RangeInclusive<Element>) -> Result<Self, Self::Error> {
        if range.is_empty() {
            Ok(Self::new())
        } else if *range.end() > Self::MAX {
            Err(Error::Irrepresentable)
        } else {
            Ok(Self::interval(*range.start(), *range.end()))
        }
    }
}
