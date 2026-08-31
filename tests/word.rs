use pibs::prelude::{BitSet, bitset};

// TODO: Extend this.
macro_rules! make_word_test {
    (($test_name:ident, $word:ty, $bits:literal)) => {
        #[test]
        fn $test_name() {
            type Set = BitSet<$word>;

            assert_eq!(Set::BITS, $bits);
            assert_eq!(Set::MIN, 0);
            assert_eq!(Set::MAX, $bits - 1);

            let full = Set::full();

            assert_eq!(full, bitset![$word; 0..$bits]);
            assert_eq!(full.subsets_of_size(0).count(), 1);
            assert_eq!(full.subsets_of_size(1).count(), $bits);
        }
    }
}

// TODO: Test with additional crates:
//       - ruint::Uint once it implements num_traits::{ConstZero, ConstOne}
make_word_test!((test_bnum_u256, bnum::types::U256, 256));
make_word_test!((test_bnum_u512, bnum::types::U512, 512));
make_word_test!((test_bnum_u1024, bnum::types::U1024, 1024));
make_word_test!((test_bnum_u2048, bnum::types::U2048, 2048));
