#[cfg(feature = "alloc")]
extern crate alloc;

use crate::*;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
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

#[cfg(feature = "alloc")]
impl<W: Word, T> From<Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from a vector.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every vector element `e`. Violating this
    /// precondition panics in debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::from(vec![2, 4, 6]), set![2, 4, 6]);
    /// ```
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<W: Word, T> From<&Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from a vector by reference.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every vector element `e`. Violating this
    /// precondition panics in debug builds and results in unspecified behavior in release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(Set::from(&vec![2, 4, 6]), set![2, 4, 6]);
    /// ```
    #[inline]
    fn from(vec: &Vec<T>) -> Self {
        vec.iter().copied().collect()
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

// ---------------------------------
// Implementations for foreign types
// ---------------------------------

#[cfg(feature = "alloc")]
impl<W: Word, T> From<BitSet<W>> for Vec<T>
where
    T: PrimInt + TryFrom<Element>,
{
    /// Create a sorted [`Vec`] from a [`BitSet`].
    ///
    /// # Exampless
    ///
    /// Any element in a [`BitSet<u128>`] can fit in a [`Vec<i8>`].
    /// ```
    /// # use pibs::prelude::*;
    /// let set = Set128::interval(Set128::MIN, Set128::MAX);
    /// let vec: Vec<i8> = set.into();
    /// assert_eq!(set.len(), u128::BITS as usize);
    /// assert_eq!(set.to_vec(), vec.into_iter().map(|x| x as usize).collect::<Vec<_>>());
    /// ```
    ///
    /// To avoid a type hint, use [`BitSet::to_vec`], which always produces a [`Vec<Element>`].
    /// ```
    /// # use pibs::prelude::*;
    /// let vec = set![1, 2, 3].to_vec();
    /// ```
    #[inline]
    fn from(set: BitSet<W>) -> Self {
        set.into_iter()
            .map(|e| match T::try_from(e) {
                Ok(x) => x,
                Err(_) => {
                    // Even a Vec<i8> can store the largest element in a BitSet<u128>.
                    unreachable!(
                        "any bitset element should be representable by any primitive integer type"
                    )
                }
            })
            .collect()
    }
}
