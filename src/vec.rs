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
    #[doc(cfg(feature = "alloc"))]
    fn from(vec: Vec<T>) -> Self {
        vec.into_iter().collect()
    }
}

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
    #[doc(cfg(feature = "alloc"))]
    fn from(vec: &Vec<T>) -> Self {
        vec.iter().copied().collect()
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
