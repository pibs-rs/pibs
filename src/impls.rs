use crate::*;

impl<W: Word> Default for BitSet<W> {
    #[inline]
    fn default() -> Self {
        Self(W::zero())
    }
}

impl<W: Word> IntoIterator for BitSet<W> {
    type Item = usize;
    type IntoIter = BitSetIter<W>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<W: Word, T> FromIterator<T> for BitSet<W>
where
    T: PrimInt + TryInto<Element>,
{
    /// Create a [`BitSet`] from an integer iterator.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// use core::iter::once;
    /// assert_eq!(Set::from_iter(once(0).chain(once(5))), set![0, 5]);
    /// ```
    ///
    /// # Panics
    /// If an element cannot be represented in the bitset.
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// use core::iter::once;
    /// Set::from_iter(once(-1));
    /// ```
    /// ```should_panic
    /// # use pitset::prelude::*;
    /// use core::iter::once;
    /// Set::from_iter(once(10_000));
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
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let set = set![0, 10, 1, 20];
    /// assert_eq!(format!("{}", set), "{0, 1, 10, 20}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for e in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", e)?;
            first = false;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<W: Word> fmt::Debug for BitSet<W> {
    /// Debug-format a bitset.
    ///
    /// # Example
    /// ```
    /// # use pitset::prelude::*;
    /// let set = set![0, 10, 1, 20];
    /// assert_eq!(format!("{:?}", set), "BitSet<usize>(1049603)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>().rsplit("::").next().unwrap())
            .field(&self.0)
            .finish()
    }
}
