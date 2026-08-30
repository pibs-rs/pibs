//! Errors returned for [`BitSet`] operations.

#[allow(unused)]
use crate::BitSet; // Needed for documentation links.
use core::{error, fmt};

/// Errors returned for [`BitSet`] operations.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum Error {
    /// A number cannot be represented as a [`BitSet`] element.
    ///
    /// This occurs when the number is negative or exceeds [`BitSet::MAX`].
    Irrepresentable,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Irrepresentable => {
                f.write_str("a number is outside the bitset's representable range")
            }
        }
    }
}
