use core::{any::type_name, hint::black_box, ops::BitOr};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pibs::*;

/// A word with eight evenly distributed bits set.
///
/// The leftmost bit is set. No [`black_box`] is applied.
///
/// type    | output
/// ------- | ------
/// [`u8`]  | `11111111`
/// [`u16`] | `1010101010101010`
/// [`u32`] | `10001000100010001000100010001000`
/// [`u64`] | `1000000010000000100000001000000010000000100000001000000010000000`
#[inline(always)]
fn _evenly_spaced_bitmask<W: Word>() -> W {
    (0..8)
        .map(|e| e * size_of::<W>() + size_of::<W>() - 1)
        .map(|e| W::ONE << e)
        .reduce(BitOr::bitor)
        .unwrap()
}

/// A black-boxed set containing eight evenly distributed elements.
///
/// Elements correspond to the bits in [`_evenly_spaced_bitmask`], so that the maximum representable
/// element is present.
#[inline(always)]
fn _evenly_spaced_set<W: Word>() -> BitSet<W> {
    black_box(BitSet::<W>::from_word(_evenly_spaced_bitmask::<W>()))
}

/// Yield eight black-boxed elements with evenly distributed bit positions in the target primitive.
///
/// The bits are the same as are set in [`_evenly_spaced_bitmask`].
#[inline(always)]
fn _evenly_spaced_elements<W: Word>() -> impl Iterator<Item = Element> {
    (0..8)
        .map(|i| i * size_of::<W>() + size_of::<W>() - 1)
        .map(|e| e as Element)
        .map(black_box)
}

fn _bench_mutation<W: Word>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    group.throughput(Throughput::Bits(3 * 8));
    group.bench_function(BenchmarkId::from_parameter(type_name::<W>()), |b| {
        b.iter(|| {
            let mut set = BitSet::<W>::new();
            for e in _evenly_spaced_elements::<W>() {
                set.insert(e);
                black_box(&mut set);
                set.remove(e);
                black_box(&mut set);
                set.toggle(e);
                black_box(&mut set);
            }
        });
    });
}

fn bench_mutation(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("mutation");
    _bench_mutation::<u8>(&mut group);
    _bench_mutation::<u16>(&mut group);
    _bench_mutation::<u32>(&mut group);
    _bench_mutation::<u64>(&mut group);
    _bench_mutation::<u128>(&mut group);
    group.finish();
}

fn _bench_enumerate<W: Word>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    group.throughput(Throughput::Elements(2u64.pow(BitSet::<W>::BITS as u32)));
    group.bench_function(BenchmarkId::from_parameter("iter_all"), |b| {
        b.iter(|| {
            for set in BitSet::<W>::iter_all() {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("iter_all_by_size"), |b| {
        b.iter(|| {
            for set in BitSet::<W>::iter_all_by_size() {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("subsets"), |b| {
        b.iter(|| {
            for set in BitSet::<W>::full().subsets() {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("subsets_by_size"), |b| {
        b.iter(|| {
            for set in BitSet::<W>::full().subsets_by_size() {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("bb_subsets"), |b| {
        b.iter(|| {
            for set in black_box(BitSet::<W>::full()).subsets() {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("bb_subsets_by_size"), |b| {
        b.iter(|| {
            for set in black_box(BitSet::<W>::full()).subsets_by_size() {
                black_box(set);
            }
        });
    });
}

fn bench_enumerate(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("enumerate_u8");
    _bench_enumerate::<u8>(&mut group);
    group.finish();

    let mut group = criterion.benchmark_group("enumerate_u16");
    _bench_enumerate::<u16>(&mut group);
    group.finish();
}

fn _bench_iter_all_below<W: Word>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    n: usize,
) {
    if n <= BitSet::<W>::BITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(type_name::<W>()),
            &n,
            |b, &n| {
                b.iter(|| {
                    for set in BitSet::<W>::iter_all_below(n) {
                        black_box(set);
                    }
                });
            },
        );
    }
}

fn bench_iter_all_below(criterion: &mut Criterion) {
    for n in [8, 16] {
        let mut group = criterion.benchmark_group(format!("iter_all_below_{n}"));
        group.throughput(Throughput::Elements(2u64.pow(n as u32)));
        _bench_iter_all_below::<u8>(&mut group, n);
        _bench_iter_all_below::<u16>(&mut group, n);
        _bench_iter_all_below::<u32>(&mut group, n);
        _bench_iter_all_below::<u64>(&mut group, n);
        _bench_iter_all_below::<u128>(&mut group, n);
        group.finish();
    }
}

fn _bench_subsets<W: Word>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    group.throughput(Throughput::Elements(2u64.pow(8)));
    group.bench_function(
        BenchmarkId::from_parameter(format!("subsets/{}", type_name::<W>())),
        |b| {
            b.iter(|| {
                for subset in _evenly_spaced_set::<W>().subsets() {
                    black_box(subset);
                }
            });
        },
    );
    group.bench_function(
        BenchmarkId::from_parameter(format!("subsets_by_size/{}", type_name::<W>())),
        |b| {
            b.iter(|| {
                for subset in _evenly_spaced_set::<W>().subsets_by_size() {
                    black_box(subset);
                }
            });
        },
    );
}

fn bench_subsets(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("subsets");
    _bench_subsets::<u8>(&mut group);
    _bench_subsets::<u16>(&mut group);
    _bench_subsets::<u32>(&mut group);
    _bench_subsets::<u64>(&mut group);
    _bench_subsets::<u128>(&mut group);
    group.finish();
}

criterion_group!(
    benches,
    bench_mutation,
    bench_enumerate,
    bench_iter_all_below,
    bench_subsets
);
criterion_main!(benches);
