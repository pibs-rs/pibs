//! Conversion methods between [`BitSet`] and [`Vec`].

extern crate alloc;

use crate::*;
use alloc::vec::Vec;

impl<W: Word> BitSet<W> {
    /// The elements as a sorted vector of type [`Vec<Element>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert_eq!(set![1, 2, 3].to_vec(), vec![1, 2, 3]);
    /// ```
    #[inline]
    #[doc(cfg(feature = "alloc"))]
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
    #[doc(cfg(feature = "alloc"))]
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
    #[doc(cfg(feature = "alloc"))]
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
