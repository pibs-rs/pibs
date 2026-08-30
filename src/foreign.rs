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
    /// assert_eq!(set![1, 2] <= set![1, 2], true);
    /// assert_eq!(set![1, 2] < set![1, 2], false);
    /// assert_eq!(set![1, 2] <= set![1, 2, 3], true);
    /// assert_eq!(set![1, 2] < set![1, 2, 3], true);
    /// ```
    #[inline]
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

impl<W: Word> Extend<Element> for BitSet<W> {
    /// Extend the set with elements from an iterator.
    ///
    /// # Panics
    ///
    /// If any element is greater than [`Self::MAX`].
    #[inline]
    fn extend<I: IntoIterator<Item = Element>>(&mut self, iter: I) {
        self.union_update(
            Self::try_from_iter(iter).expect("cannot extend with an irrepresentable element"),
        );
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

struct DebugElements<W: Word>(BitSet<W>);

impl<W: Word> fmt::Debug for DebugElements<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
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
            // TODO: Once stable, use field_with instead of the helper struct:
            // .field_with(|f| f.debug_set().entries(self.iter()).finish())
            .field(&DebugElements(*self))
            .finish()
    }
}
