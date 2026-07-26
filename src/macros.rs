#[allow(unused_imports)]
use crate::*;

/// Create a [`BitSet`] using the given primitive type to store the arguments.
///
/// # Example
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(bitset![u8; 0..5], BitSet::<u8>::from(0..5));
/// assert_eq!(bitset![usize; 1..=23], BitSet::<usize>::from(1..=23));
/// assert_eq!(bitset![u128; 0, 64, 127], BitSet::<u128>::from([0, 64, 127]));
/// ```
///
/// # Compile-time checks
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = bitset![u8; 6, 7, 8]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! bitset {
    ($word:ty; $start:literal .. $end:literal) => {{
        const _: () = assert!($end <= BitSet::<$word>::BITS);
        if $end <= 0 {
            BitSet::<$word>::new()
        } else {
            BitSet::<$word>::interval($start, $end - 1)
        }
    }};
    ($word:ty; $start:literal ..= $last:literal) => {{
        const _: () = assert!($last <= BitSet::<$word>::MAX);
        BitSet::<$word>::interval($start, $last)
    }};
    ($word:ty; $($element:expr),* $(,)?) => {{
        $(const _: () = assert!($element < BitSet::<$word>::BITS);)*
        BitSet::<$word>::from_word(0 as $word $(| ((1 as $word) << $element))*)
    }};
}

/// Create a [`Set`] containing the arguments.
///
///  # Example
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(set![], Set::new());
/// assert_eq!(set![5], Set::singleton(5));
/// assert_eq!(set![0, 2, 4], Set::from([0, 2, 4]));
/// assert_eq!(set![0..4], Set::from(0..4));
/// assert_eq!(set![0..=4], Set::from(0..=4));
/// ```
///
/// # Compile-time checks
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = set![10_000]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! set {
    ($($tt:tt)*) => {
        $crate::bitset!(usize; $($tt)*)
    };
}

/// Create a [`Set128`] containing the arguments.
///
/// # Example
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(set128![100..105], Set128::from(100..105));
/// assert_eq!(set128![100..=105], Set128::from(100..=105));
/// assert_eq!(set128![0, 64, 127], Set128::from([0, 64, 127]));
/// ```
///
/// # Compile-time checks
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = set![0..=128]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! set128 {
    ($($tt:tt)*) => {
        $crate::bitset!(u128; $($tt)*)
    };
}
