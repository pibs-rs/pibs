//! Conversion methods between [`BitSet`] and [`Vec`].

extern crate alloc;

use crate::*;
use alloc::vec::Vec;

impl<W: Word> BitSet<W> {
    /// The elements as a sorted vector of type [`Vec<Element>`].
    ///
    /// Available with the `alloc` feature, which is enabled by default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![3, 2, 1].to_vec(), vec![1, 2, 3]);
    /// ```
    ///
    /// To produce a [`Vec<T>`] for a different primitive integer type `T`, use `into` or `from`
    /// with a type hint.
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set128![127, 5, 23];
    ///
    /// // Via BitSet::into.
    /// let vec: Vec<i8> = set.into();
    /// assert_eq!(vec, vec![5, 23, 127]);
    ///
    /// // Via Vec::from.
    /// let vec = Vec::<i8>::from(set);
    /// assert_eq!(vec, vec![5, 23, 127]);
    /// ```
    #[inline]
    pub fn to_vec(self) -> Vec<Element> {
        self.iter().collect()
    }
}

impl<W: Word, T> TryFrom<Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    type Error = Error;

    /// Try to create a [`BitSet`] from a vector.
    ///
    /// Available with the `alloc` feature, which is enabled by default.
    ///
    /// # Errors
    ///
    /// If any vector element fails to convert to an [`Element`] or is greater than [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use pibs::Error::Irrepresentable;
    ///
    /// assert_eq!(Set::try_from(vec![2, 4, 6]), Ok(set![2, 4, 6]));
    /// assert_eq!(Set::try_from(vec![-2, 4, 6]), Err(Irrepresentable));
    /// assert_eq!(Set::try_from(vec![10_000]), Err(Irrepresentable));
    /// ```
    #[inline]
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from_iter(vec)
    }
}

impl<W: Word, T> TryFrom<&Vec<T>> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    type Error = Error;

    /// Try to create a [`BitSet`] from a vector by reference.
    ///
    /// Available with the `alloc` feature, which is enabled by default.
    ///
    /// # Errors
    ///
    /// If any vector element fails to convert to an [`Element`] or is greater than [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use pibs::Error::Irrepresentable;
    ///
    /// assert_eq!(Set::try_from(&vec![2, 4, 6]), Ok(set![2, 4, 6]));
    /// assert_eq!(Set::try_from(&vec![-2, 4, 6]), Err(Irrepresentable));
    /// assert_eq!(Set::try_from(&vec![10_000]), Err(Irrepresentable));
    /// ```
    #[inline]
    fn try_from(vec: &Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from_iter(vec.iter().copied())
    }
}

macro_rules! impl_vec_from_bitset {
    (@to $word:ty; $($target:ty),+) => {
        $(
            #[doc(hidden)]
            impl From<BitSet<$word>> for Vec<$target> {
                /// Create a sorted [`Vec`] from a [`BitSet`].
                #[inline]
                fn from(set: BitSet<$word>) -> Self {
                    // This is lossless as any element in a BitSet<u128> can fit in a i8.
                    set.into_iter().map(|e| e as $target).collect()
                }
            }
        )+
    };

    (@from $word:ty) => {
        impl_vec_from_bitset!(
            @to $word;
            u8, u16, u32, u64, u128, usize,
            i8, i16, i32, i64, i128, isize
        );
    };

    ($($word:ty),+) => {
        $(impl_vec_from_bitset!(@from $word);)+
    };
}

impl_vec_from_bitset!(u8, u16, u32, u64, u128, usize);
