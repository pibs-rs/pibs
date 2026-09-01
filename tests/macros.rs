use pibs::{Word, bitset, set, set128};

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
fn test_macro_expressions() {
    assert_eq!(bitset![u8; 1, 1 + 2, 2 + 3, 3 + 4].word(), 0b10101010);
    assert_eq!(bitset![u8; 2..2 + 2].word(), 0b1100);
    assert_eq!(bitset![u8; 2..=2 + 2].word(), 0b11100);
}

#[test]
fn test_macro_generic_use() {
    fn with_generic_word<W: Word>() {
        assert_eq!(bitset![W; 1, 2, 3].len(), 3);
    }

    fn with_generic_element<const E: usize>() {
        assert!(set![E].contains(E));
    }

    fn with_generic_range<const A: usize, const B: usize>() {
        assert_eq!(set![A..B].len(), B - A);
        assert_eq!(set![A..=B].len(), B + 1 - A);
    }

    with_generic_word::<usize>();
    with_generic_element::<5>();
    with_generic_range::<3, 5>();
}

#[test]
fn const_test_macros() {
    // TODO: Make also range-based construction constant.
    const _: () = assert!(bitset![u8; 1, 2, 3].word() == 0b1110);
    const _: () = assert!(set![1, 2, 3].word() == 0b1110);
    const _: () = assert!(set128![1, 2, 3].word() == 0b1110);
}
