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
    /// assert_eq!(set![1, 2, 3].to_vec(), vec![1, 2, 3]);
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

impl<W: Word, T> From<BitSet<W>> for Vec<T>
where
    T: PrimInt + TryFrom<Element>,
{
    /// Create a sorted [`Vec`] from a [`BitSet`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let vec: Vec<usize> = set![7, 2, 5].into();
    /// assert_eq!(vec, vec![2, 5, 7]);
    /// ```
    ///
    /// A full type hint is needed, as this method can produce any primitive integer vector.
    ///
    /// ```compile_fail
    /// # use pibs::prelude::*;
    /// let vec: Vec<_> = set![7, 2, 5].into(); // Does not compile.
    /// ```
    ///
    /// Using [`BitSet::to_vec`] avoids the type hint, as it always produces a [`Vec<Element>`].
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let vec = set![7, 2, 5].to_vec();
    /// assert_eq!(vec, vec![2, 5, 7]);
    /// ```
    ///
    /// This method is infallible, as any element in a [`BitSet<u128>`] can still fit in a
    /// [`Vec<i8>`].
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = Set128::full();
    /// let vec: Vec<i8> = set.into();
    /// assert!(vec.iter().map(|&x| x as usize).eq(set.iter()));
    /// ```
    #[inline]
    #[doc(cfg(feature = "alloc"))]
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
