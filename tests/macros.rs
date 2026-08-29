use pibs::{bitset, set, set128};

/// A local struct defined for testing macro hygiene.
#[allow(unused)]
struct BitSet {}

/// A local type defined for testing macro hygiene.
#[allow(unused)]
type Set = usize;

#[test]
fn test_macro_ranges() {
    let two = 2; // Range starts may be non-constant.

    assert_eq!(bitset![u8; two..=3].word(), 0b1100);
    assert_eq!(bitset![u8; two..=2].word(), 0b100);
    assert_eq!(bitset![u8; two..=1].word(), 0);
    assert_eq!(bitset![u8; two..=0].word(), 0);
    assert_eq!(bitset![u8; two..4].word(), 0b1100);
    assert_eq!(bitset![u8; two..3].word(), 0b100);
    assert_eq!(bitset![u8; two..2].word(), 0);
    assert_eq!(bitset![u8; two..1].word(), 0);

    assert_eq!(set![two..=3].word(), 0b1100);
    assert_eq!(set![two..=2].word(), 0b100);
    assert_eq!(set![two..=1].word(), 0);
    assert_eq!(set![two..=0].word(), 0);
    assert_eq!(set![two..4].word(), 0b1100);
    assert_eq!(set![two..3].word(), 0b100);
    assert_eq!(set![two..2].word(), 0);
    assert_eq!(set![two..1].word(), 0);

    assert_eq!(set128![two..=3].word(), 0b1100);
    assert_eq!(set128![two..=2].word(), 0b100);
    assert_eq!(set128![two..=1].word(), 0);
    assert_eq!(set128![two..=0].word(), 0);
    assert_eq!(set128![two..4].word(), 0b1100);
    assert_eq!(set128![two..3].word(), 0b100);
    assert_eq!(set128![two..2].word(), 0);
    assert_eq!(set128![two..1].word(), 0);
}

#[test]
fn const_test_macros() {
    // TODO: Make also range-based construction constant.
    const _: () = assert!(bitset![u8; 1, 2, 3].word() == 0b1110);
    const _: () = assert!(set![1, 2, 3].word() == 0b1110);
    const _: () = assert!(set128![1, 2, 3].word() == 0b1110);
}
