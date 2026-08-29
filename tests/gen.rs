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
                .map(|set| set.iter().collect::<Vec<_>>())
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
                    subset.iter().collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>(),
            set.iter().powerset().collect::<HashSet<_>>()
        );
    }
}

#[test]
fn test_subsets_of_size_extremes() {
    let empty = BitSet::<u128>::new();
    assert!(empty.subsets_of_size(0).eq([empty]));
    assert!(empty.subsets_of_size(1).eq([]));

    let full = BitSet::<u128>::full();
    assert!(full.subsets_of_size(0).eq([empty]));
    assert_eq!(full.subsets_of_size(1).count(), 128);
    assert_eq!(full.subsets_of_size(2).count(), 128 * 127 / 2);
    assert_eq!(full.subsets_of_size(126).count(), 128 * 127 / 2);
    assert_eq!(full.subsets_of_size(127).count(), 128);
    assert!(full.subsets_of_size(128).eq([full]));
    assert!(full.subsets_of_size(129).eq([]));
}

#[test]
fn test_subsets_by_size() {
    for mut set in Set::iter_all_below(8) {
        set.insert(Set::MAX); // Test for overflows.
        assert_eq!(
            set.subsets_by_size()
                .map(|subset| {
                    assert!(subset.is_subset(set) && set.is_superset(subset));
                    subset.iter().collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>(),
            set.iter().powerset().collect::<HashSet<_>>()
        );
    }
}
