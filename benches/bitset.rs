use core::{any::type_name, hint::black_box};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use pitset::*;

fn _bench_enumerate<W: Word>(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
) {
    group.throughput(Throughput::Elements(2.pow(BitSet::<W>::BITS as u32)));
    group.bench_function(BenchmarkId::from_parameter("iter_all"), |benchmark| {
        benchmark.iter(|| {
            for set in BitSet::<W>::iter_all() {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("iter_all_below"), |benchmark| {
        benchmark.iter(|| {
            for set in BitSet::<W>::iter_all_below(BitSet::<W>::BITS) {
                black_box(set);
            }
        });
    });
    group.bench_function(BenchmarkId::from_parameter("subsets"), |benchmark| {
        let all = BitSet::<W>::from_word(W::one().wrapping_neg());
        benchmark.iter(|| {
            for set in all.subsets() {
                black_box(set);
            }
        });
    });
    group.bench_function(
        BenchmarkId::from_parameter("subsets_by_size"),
        |benchmark| {
            let all = BitSet::<W>::from_word(W::one().wrapping_neg());
            benchmark.iter(|| {
                for set in all.subsets_by_size() {
                    black_box(set);
                }
            });
        },
    );
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
            |benchmark, &n| {
                benchmark.iter(|| {
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

criterion_group!(benches, bench_enumerate, bench_iter_all_below);
criterion_main!(benches);
