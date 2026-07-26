use core::{any::type_name, hint::black_box};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pitset::*;

fn _bench_enumerate<W: Word>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    group.throughput(Throughput::Elements(2.pow(BitSet::<W>::BITS as u32)));
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
        group.throughput(Throughput::Elements(2.pow(n as u32)));
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
    /// Generate all sets containing the largest representable element and some subset of 0..7.
    fn sets<W: Word>() -> impl Iterator<Item = BitSet<W>> {
        BitSet::<W>::iter_all_below(7).map(|set| black_box(set + BitSet::<W>::MAX))
    }

    group.bench_function(
        BenchmarkId::from_parameter(format!("subsets/{}", type_name::<W>())),
        |b| {
            b.iter(|| {
                for set in sets::<W>() {
                    for subset in set.subsets() {
                        black_box(subset);
                    }
                }
            });
        },
    );
    group.bench_function(
        BenchmarkId::from_parameter(format!("subsets_by_size/{}", type_name::<W>())),
        |b| {
            b.iter(|| {
                for set in sets::<W>() {
                    for subset in set.subsets_by_size() {
                        black_box(subset);
                    }
                }
            });
        },
    );
}

fn bench_subsets(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group(format!("subsets"));
    _bench_subsets::<u8>(&mut group);
    _bench_subsets::<u16>(&mut group);
    _bench_subsets::<u32>(&mut group);
    _bench_subsets::<u64>(&mut group);
    _bench_subsets::<u128>(&mut group);
    group.finish();
}

criterion_group!(
    benches,
    bench_enumerate,
    bench_iter_all_below,
    bench_subsets
);
criterion_main!(benches);
