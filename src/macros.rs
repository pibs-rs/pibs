//! Macros to create a [`BitSet`].

#[allow(unused_imports)]
use crate::*;

/// Create a [`BitSet`] using the given primitive type to store the arguments.
///
/// Explicit elements and range ends must be constant expressions and are bounds-checked at compile
/// time. Range starts may be runtime values.
///
/// # Examples
///
/// ```
/// # use pibs::prelude::*;
/// assert_eq!(bitset![u8; 0..5], BitSet::<u8>::try_from(0..5).unwrap());
/// assert_eq!(bitset![usize; 1..=23], BitSet::<usize>::try_from(1..=23).unwrap());
/// assert_eq!(
///     bitset![u128; 0, 63..=65, 127],
///     BitSet::<u128>::try_from([0, 63, 64, 65, 127]).unwrap()
/// );
/// ```
///
/// The compiler performs out-of-bounds checks.
///
/// ```compile_fail
/// # use pibs::prelude::*;
/// let set = bitset![u32; 32]; // Cannot be represented.
/// ```
///
/// Explicit elements and range ends must be constant expressions.
///
/// ```compile_fail
/// # use pibs::prelude::*;
/// let x = 5;
/// let set = bitset![u8; x]; // Not a constant expression.
/// ```
#[macro_export]
macro_rules! bitset {
    // Initialize.
    ($ty:ty; $($tt:tt)*) => {
        $crate::bitset!(@accum <$ty as num_traits::ConstZero>::ZERO; $ty; $($tt)*)
    };

    // Finalize.
    (@accum $word:expr; $ty:ty;) => {
        $crate::BitSet::<$ty>::from_word($word)
    };

    // Parse a singleton.
    (@accum $word:expr; $ty:ty; $element:tt $(, $($rest:tt)*)?) => {
        $crate::bitset!(
            @accum $word | {
                // Bounds-check the element at compile time.
                let element: $crate::Element = const {
                    let element: $crate::Element = $element;
                    assert!(element <= $crate::BitSet::<$ty>::MAX);
                    element
                };

                <$ty as num_traits::ConstOne>::ONE << element
            };
            $ty;
            $($($rest)*)?
        )
    };

    // Parse a range.
    (@accum $word:expr; $ty:ty; $start:tt .. $end:tt $(, $($rest:tt)*)?) => {
        $crate::bitset!(
            @accum $word | {
                // Bounds-check the range end at compile time.
                let end: $crate::Element = const {
                    let end: $crate::Element = $end;
                    assert!(end <= $crate::BitSet::<$ty>::BITS);
                    end
                };

                if end <= 0 {
                    <$ty as num_traits::ConstZero>::ZERO
                } else {
                    $crate::BitSet::<$ty>::interval($start, end - 1).word()
                }
            };
            $ty;
            $($($rest)*)?
        )
    };

    // Parse an inclusive range.
    (@accum $word:expr; $ty:ty; $start:tt ..= $last:tt $(, $($rest:tt)*)?) => {
        $crate::bitset!(
            @accum $word | {
                // Bounds-check the last element at compile time.
                let last: $crate::Element = const {
                    let last: $crate::Element = $last;
                    assert!(last <= $crate::BitSet::<$ty>::MAX);
                    last
                };

                $crate::BitSet::<$ty>::interval($start, last).word()
            };
            $ty;
            $($($rest)*)?
        )
    };
}

/// Create a [`Set`] containing the arguments.
///
/// Explicit elements and range ends must be constant expressions and are bounds-checked at compile
/// time. Range starts may be runtime values.
///
/// # Examples
///
/// ```
/// # use pibs::prelude::*;
/// assert_eq!(set![], Set::new());
/// assert_eq!(set![5], Set::singleton(5));
/// assert_eq!(set![0, 2, 4], Set::try_from([0, 2, 4]).unwrap());
/// assert_eq!(set![0..4], Set::try_from(0..4).unwrap());
/// assert_eq!(set![0..=4], Set::try_from(0..=4).unwrap());
/// assert_eq!(set![0..2, 2, 10, 4..6, 6], Set::try_from(0..=6).unwrap() - 3 + 10);
/// ```
///
/// The compiler performs out-of-bounds checks.
///
/// ```compile_fail
/// # use pibs::prelude::*;
/// let set = set![10_000]; // Cannot be represented.
/// ```
///
/// Explicit elements and range ends must be constant expressions.
///
/// ```compile_fail
/// # use pibs::prelude::*;
/// let x = 5;
/// let set = set![x]; // Not a constant expression.
/// ```
#[macro_export]
macro_rules! set {
    ($($tt:tt)*) => {
        $crate::bitset!(usize; $($tt)*)
    };
}

/// Create a [`Set128`] containing the arguments.
///
/// Explicit elements and range ends must be constant expressions and are bounds-checked at compile
/// time. Range starts may be runtime values.
///
/// # Examples
///
/// ```
/// # use pibs::prelude::*;
/// assert_eq!(set128![100..105], Set128::try_from(100..105).unwrap());
/// assert_eq!(set128![100..=105], Set128::try_from(100..=105).unwrap());
/// assert_eq!(set128![0, 63..=65, 127], Set128::try_from([0, 63, 64, 65, 127]).unwrap());
/// ```
///
/// The compiler performs out-of-bounds checks.
///
/// ```compile_fail
/// # use pibs::prelude::*;
/// let set = set128![0..=128]; // Cannot be represented.
/// ```
///
/// Explicit elements and range ends must be constant expressions.
///
/// ```compile_fail
/// # use pibs::prelude::*;
/// let x = 5;
/// let set = set128![x]; // Not a constant expression.
/// ```
#[macro_export]
macro_rules! set128 {
    ($($tt:tt)*) => {
        $crate::bitset!(u128; $($tt)*)
    };
}
