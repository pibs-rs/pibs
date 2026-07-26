use crate::*;

impl<W: Word, T, const N: usize> From<[T; N]> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an array.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from([2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([10_000]);
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from(&[2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&[-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&[10_000]);
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from([2, 4, 6].as_slice()), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([-1].as_slice());
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from([10_000].as_slice());
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from(vec![2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(vec![-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(vec![10_000]);
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// assert_eq!(Set::from(&vec![2, 4, 6]), set![2, 4, 6]);
    /// ```
    ///
    /// # Panics
    ///
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&vec![-1]);
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// Set::from(&vec![10_000]);
    #[inline]
    fn from(vec: &Vec<T>) -> Self {
        vec.iter().copied().collect()
    }
}

impl<W: Word> From<Range<Element>> for BitSet<W> {
    /// Create a [`BitSet`] from an end-exclusive range.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
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
    /// # Examples
    /// Any element in a [`BitSet<u128>`] can fit in a [`Vec<i8>`].
    /// ```
    /// # use pitset::prelude::*;
    /// let set = Set128::interval(Set128::MIN, Set128::MAX);
    /// let vec: Vec<i8> = set.into();
    /// assert_eq!(set.len(), u128::BITS as usize);
    /// assert_eq!(set.to_vec(), vec.into_iter().map(|x| x as usize).collect::<Vec<_>>());
    /// ```
    /// To avoid a type hint, use [`BitSet::to_vec`], which always produces a [`Vec<Element>`].
    /// ```
    /// # use pitset::prelude::*;
    /// let vec = set![1, 2, 3].to_vec();
    /// ```
    ///
    /// # Panics
    ///
    /// If an element of the bitset cannot be represented by `T`.
    ///
    /// Note that even the extreme combination of [`BitSet<u128>`] and [`Vec<i8>`] is safe as the
    /// largest possible element in the former (127) can still be represented by the latter.
    /// Therefore, this implementation could only panic if additional primitive integer types are
    /// introduced in the future.
    #[inline]
    fn from(set: BitSet<W>) -> Self {
        set.into_iter()
            .map(|e| match T::try_from(e) {
                Ok(x) => x,
                Err(_) => panic!("bitset element cannot be represented by target integer type"),
            })
            .collect()
    }
}
