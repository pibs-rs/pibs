extern crate std;

use crate::*;
use itertools::Itertools;
use std::{collections::HashSet, vec};

#[test]
fn test_bit_combinations() {
    type W = u8;
    type S = BitSet<W>;
    let b = S::BITS;

    // Only extreme cases.
    assert_eq!(S::bit_combinations(b, 0).collect::<Vec<_>>(), vec![0]);
    assert_eq!(
        S::bit_combinations(b, b).collect::<Vec<_>>(),
        vec![(1 as W).wrapping_neg()]
    );

    // Full range.
    for k in 0..=b {
        assert_eq!(
            S::bit_combinations(b, k).collect::<HashSet<_>>(),
            (0..b)
                .combinations(k)
                .map(|subset| subset.into_iter().fold(0 as W, |x, i| x | (1 << i)))
                .collect::<HashSet<_>>()
        );
    }

    // With custom limit.
    let limit = b / 2;
    for k in 0..=limit {
        assert_eq!(
            S::bit_combinations(limit, k).collect::<HashSet<_>>(),
            (0..limit)
                .combinations(k)
                .map(|subset| subset.into_iter().fold(0 as W, |x, i| x | (1 << i)))
                .collect::<HashSet<_>>()
        );
    }
}

#[test]
fn test_iter_all_below() {
    for bound in 0..=10 {
        assert_eq!(
            Set::iter_all_below(bound)
                .map(|set| set.to_vec())
                .collect::<HashSet<_>>(),
            (0..bound).powerset().collect::<HashSet<_>>()
        );
    }
}
