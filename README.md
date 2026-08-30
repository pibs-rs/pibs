# A primitive integer bitset for high-performance combinatorics

The [`pibs`] Rust crate offers a generic [`BitSet<W>`] wrapper struct around a
primitive integer type `W` for representing a set of small numbers. It also
provides the default types [`Set`] (`W = usize`) and [`Set128`] (`W = u128`) and
associated creation macros [`bitset!`], [`set!`], and [`set128!`]. The crate's
main features are

- zero-cost abstraction over bitwise operations without allocation or block
  management and
- a rich interface for mathematics/combinatorics involving integer sets.

The crate is best suited when the bitset should abstract a mathematical set, the
performance of set operations is your primary concern, and the elements
naturally lie in the representable range (`0..128` for [`Set128`]).

[`pibs`]: https://docs.rs/pibs
[`BitSet<W>`]: https://docs.rs/pibs/latest/pibs/struct.BitSet.html
[`Set`]: https://docs.rs/pibs/latest/pibs/type.Set.html
[`Set128`]: https://docs.rs/pibs/latest/pibs/type.Set128.html
[`bitset!`]: https://docs.rs/pibs/latest/pibs/macro.bitset.html
[`set!`]: https://docs.rs/pibs/latest/pibs/macro.set.html
[`set128!`]: https://docs.rs/pibs/latest/pibs/macro.set128.html

## Example

The following computes by brute force a minimum-cardinality set of positive
integers that generate (by taking any subset of the numbers and summing them)
all elements of a target set. This is done using the [`iter_combinations(n, k)`]
generator, which yields all subsets of `0..n` (here shifted to `1..=n`) of size
`k`, and the [`truncating_subset_sums`] operator, which produces all subset sums
that can be represented by the bitset.

```rust
use pibs::prelude::*;

fn min_generating_set(set: Set) -> Set {
    let max: usize = set.max().unwrap_or(0);
    let bit_length = (usize::BITS - max.leading_zeros()) as usize; // ⌈log₂(max + 1)⌉

    // Test all subsets of 1..=max, grouped by increasing cardinality.
    for size in 0..bit_length {
        for generator in Set::iter_combinations(max, size).map(|g| g << 1) {
            if set.is_subset(generator.truncating_subset_sums()) {
                return generator;
            }
        }
    }

    // If no small generator was found, fall back to powers of two.
    Set::from_unchecked((0..bit_length).map(|b| 1 << b))
}

// To generate {0, ..., 9}, we need a generating set of size four.
assert_eq!(min_generating_set(set![0..=9]), set![1, 2, 4, 8]);

// But if we don't need to generate 2 and 7, three numbers suffice.
assert_eq!(min_generating_set(set![0..=9] - set![2, 7]), set![1, 3, 5]);
```

See the [full documentation] for more examples.

[`iter_combinations(n, k)`]: https://docs.rs/pibs/latest/pibs/struct.BitSet.html#method.iter_combinations
[`truncating_subset_sums`]: https://docs.rs/pibs/latest/pibs/struct.BitSet.html#method.truncating_subset_sums
[full documentation]: https://docs.rs/pibs

## Installation

Via one of the following `cargo` commands:

```sh
cargo add pibs                        # with default features
cargo add pibs --no-default-features  # without default features
cargo add pibs -F serde               # with 'serde' feature
```

Or by adding one of the following lines to `Cargo.toml` (replace `0.1` with the
latest version):

```toml
[dependencies]
pibs = "0.1"                                          # with default features
pibs = { version = "0.1", default-features = false }  # without default features
pibs = { version = "0.1", features = ["serde"] }      # with 'serde' feature
```

### Features and dependencies

| feature | default | implements
| ------- | ------- | ----------
| `alloc` | yes     | conversion to and from [`Vec`]
| `serde` | no      | (de)serialization via [`serde`]

The crate is [no_std]-compatible and its only non-optional dependency is
[`num_traits`].

[`Vec`]: https://doc.rust-lang.org/alloc/vec/struct.Vec.html
[`serde`]: https://serde.rs/
[no_std]: https://docs.rust-embedded.org/book/intro/no-std.html
[`num_traits`]: https://docs.rs/num-traits

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
