//! Miscellaneous foreign trait implementations for [`BitSet`].

use crate::*;
use core::{any::type_name, cmp::Ordering, fmt};

impl<W: Word> Default for BitSet<W> {
    #[inline]
    fn default() -> Self {
        Self(W::zero())
    }
}

impl<W: Word> PartialOrd for BitSet<W> {
    /// Test for a subset relation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// assert!(set![1, 2] <= set![1, 2]);
    /// assert!(!(set![1, 2] < set![1, 2]));
    /// assert!(set![1, 2] <= set![1, 2, 3]);
    /// assert!(set![1, 2] < set![1, 2, 3]);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.is_subset(*other) {
            Some(Ordering::Less)
        } else if self.is_superset(*other) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl<W: Word> IntoIterator for BitSet<W> {
    type Item = Element;
    type IntoIter = BitSetIter<W>;

    /// An iterator over the elements in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![7, 3..=5, 1];
    /// assert!(set.into_iter().eq([1, 3, 4, 5, 7]));
    /// ```
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<W: Word> IntoIterator for &BitSet<W> {
    type Item = Element;
    type IntoIter = BitSetIter<W>;

    /// An iterator over the elements in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![7, 3..=5, 1];
    /// assert!((&set).into_iter().eq([1, 3, 4, 5, 7]));  // Still yields elements by value.
    /// ```
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (*self).iter()
    }
}

impl<W: Word, T> FromIterator<T> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an integer iterator.
    ///
    /// # Preconditions
    ///
    /// The caller must ensure that `e <= Self::MAX` for every element `e` produced by the iterator.
    /// Violating this precondition panics in debug builds and results in unspecified behavior in
    /// release builds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// use core::iter::once;
    /// assert_eq!(Set::from_iter(once(0).chain(once(5))), set![0, 5]);
    /// ```
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut word = W::zero();

        for e in iter {
            let e = match e.try_into() {
                Ok(x) => x,
                Err(_) => panic!("failed to load a bitset element from an iterator"),
            };
            Self::debug_bound_check(e);
            word += W::one() << e;
        }

        Self(word)
    }
}

impl<W: Word> fmt::Display for BitSet<W> {
    /// Pretty-format a bitset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![0, 10, 1, 20];
    /// assert_eq!(format!("{}", set), "{0, 1, 10, 20}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<W: Word> fmt::Debug for BitSet<W> {
    /// Debug-format a bitset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pibs::prelude::*;
    /// let set = set![0, 10, 1, 20];
    /// assert_eq!(format!("{:?}", set), "BitSet<usize>({0, 1, 10, 20})");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().rsplit("::").next().unwrap())
            .field_with(|f| f.debug_set().entries(self.iter()).finish())
            .finish()
    }
}
