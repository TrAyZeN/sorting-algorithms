use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{prelude::*, thread_rng};
use sorting_algorithms::{bubble_sort, gnome_sort, insertion_sort, selection_sort, shaker_sort};

pub fn benchmark_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort");
    let mut rng = thread_rng();

    for i in 2..7 {
        let size = 10i32.pow(i);
        let mut v: Vec<i32> = (0..size).collect();
        v.shuffle(&mut rng);

        group
            .bench_with_input(BenchmarkId::new("Bubble sort", size), &v, |b, v| {
                b.iter(|| bubble_sort::sort(v))
            })
            .sample_size(10);
        group
            .bench_with_input(BenchmarkId::new("Selection sort", size), &v, |b, v| {
                b.iter(|| selection_sort::sort(v))
            })
            .sample_size(10);
        group
            .bench_with_input(BenchmarkId::new("Insertion sort", size), &v, |b, v| {
                b.iter(|| insertion_sort::sort(v))
            })
            .sample_size(10);
        group
            .bench_with_input(BenchmarkId::new("Shaker sort", size), &v, |b, v| {
                b.iter(|| shaker_sort::sort(v))
            })
            .sample_size(10);
        group
            .bench_with_input(BenchmarkId::new("Gnome sort", size), &v, |b, v| {
                b.iter(|| gnome_sort::sort(v))
            })
            .sample_size(10);
    }

    group.finish();
}

criterion_group!(benches, benchmark_sort);
criterion_main!(benches);
