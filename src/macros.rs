#[allow(unused_imports)]
use crate::*;

/// Create a [`BitSet`] using the given primitive type to store the arguments.
///
/// # Examples
///
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(bitset![u8; 0..5], BitSet::<u8>::from(0..5));
/// assert_eq!(bitset![usize; 1..=23], BitSet::<usize>::from(1..=23));
/// assert_eq!(bitset![u128; 0, 63..=65, 127], BitSet::<u128>::from([0, 63, 64, 65, 127]));
/// ```
///
/// # Compile-time checks
///
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = bitset![u8; 6, 7, 8]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! bitset {
    // Initialize.
    ($ty:ty; $($tt:tt)*) => {
        $crate::bitset!(@accum 0 as $ty; $ty; $($tt)*)
    };

    // Finalize.
    (@accum $word:expr; $ty:ty;) => {
        BitSet::<$ty>::from_word($word)
    };

    // Parse a singleton.
    (@accum $word:expr; $ty:ty; $element:tt $(, $($rest:tt)*)?) => {
        $crate::bitset!(
            @accum $word | ({
                const _: () = assert!($element <= BitSet::<$ty>::MAX);
                (1 as $ty) << $element
            });
            $ty;
            $($($rest)*)?
        )
    };

    // Parse a range.
    (@accum $word:expr; $ty:ty; $start:tt .. $end:tt $(, $($rest:tt)*)?) => {
        $crate::bitset!(
            @accum $word | ({
                const _: () = assert!($end <= BitSet::<$ty>::BITS);
                if $end <= 0 {
                    0 as $ty
                } else {
                    BitSet::<$ty>::interval($start, $end - 1).word()
                }
            });
            $ty;
            $($($rest)*)?
        )
    };

    // Parse an inclusive range.
    (@accum $word:expr; $ty:ty; $start:tt ..= $last:tt $(, $($rest:tt)*)?) => {
        $crate::bitset!(
            @accum $word | ({
                const _: () = assert!($last <= BitSet::<$ty>::MAX);
                BitSet::<$ty>::interval($start, $last).word()
            });
            $ty;
            $($($rest)*)?
        )
    };
}

/// Create a [`Set`] containing the arguments.
///
///  # Examples
///
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(set![], Set::new());
/// assert_eq!(set![5], Set::singleton(5));
/// assert_eq!(set![0, 2, 4], Set::from([0, 2, 4]));
/// assert_eq!(set![0..4], Set::from(0..4));
/// assert_eq!(set![0..=4], Set::from(0..=4));
/// assert_eq!(set![0..2, 2, 10, 4..6, 6], Set::from(0..=6) - 3 + 10);
/// ```
///
/// # Compile-time checks
///
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
/// # Examples
///
/// ```
/// # use pitset::prelude::*;
/// assert_eq!(set128![100..105], Set128::from(100..105));
/// assert_eq!(set128![100..=105], Set128::from(100..=105));
/// assert_eq!(set128![0, 63..=65, 127], Set128::from([0, 63, 64, 65, 127]));
/// ```
///
/// # Compile-time checks
///
/// ```compile_fail
/// # use pitset::prelude::*;
/// let set = set128![0..=128]; // The compiler detects out-of-bounds elements.
/// ```
#[macro_export]
macro_rules! set128 {
    ($($tt:tt)*) => {
        $crate::bitset!(u128; $($tt)*)
    };
}
