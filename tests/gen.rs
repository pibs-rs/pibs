use itertools::Itertools;
use pibs::*;
use std::collections::HashSet;

#[test]
fn test_iter_combinations() {
    type W = u8;
    type S = BitSet<W>;
    let b = S::BITS;

    // Extreme cases.
    assert!(S::iter_combinations(b, 0).eq([S::new()]));
    assert!(S::iter_combinations(b, b).eq([S::full()]));

    // Full range.
    for k in 0..=b {
        assert_eq!(
            S::iter_combinations(b, k)
                .map(S::word)
                .collect::<HashSet<_>>(),
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
            S::iter_combinations(limit, k)
                .map(S::word)
                .collect::<HashSet<_>>(),
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
            (0..bound)
                .map(|e| e as Element)
                .powerset()
                .collect::<HashSet<_>>()
        );
    }
}

#[test]
fn test_subsets() {
    for mut set in Set::iter_all_below(8) {
        set.insert(Set::MAX); // Test for overflows.
        assert_eq!(
            set.subsets()
                .map(|subset| {
                    assert!(subset.is_subset(set) && set.is_superset(subset));
                    subset.to_vec()
                })
                .collect::<HashSet<_>>(),
            set.iter().powerset().collect::<HashSet<_>>()
        );
    }
}

#[test]
fn test_subsets_by_size() {
    for mut set in Set::iter_all_below(8) {
        set.insert(Set::MAX); // Test for overflows.
        assert_eq!(
            set.subsets_by_size()
                .map(|subset| {
                    assert!(subset.is_subset(set) && set.is_superset(subset));
                    subset.to_vec()
                })
                .collect::<HashSet<_>>(),
            set.iter().powerset().collect::<HashSet<_>>()
        );
    }
}
