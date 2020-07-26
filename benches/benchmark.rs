use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::{thread_rng, prelude::*};
use sorting_algorithms::{
    bubble_sort,
    selection_sort,
    insertion_sort,
    shaker_sort,
    gnome_sort
};

pub fn benchmark_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort");
    let mut rng = thread_rng();

    let mut v: Vec<i32> = (0..10000).collect();
    v.shuffle(&mut rng);

    group.bench_with_input(BenchmarkId::new("Bubble sort", 0), &v, |b, v| b.iter(|| bubble_sort::sort(v)));
    group.bench_with_input(BenchmarkId::new("Selection sort", 0), &v, |b, v| b.iter(|| selection_sort::sort(v)));
    group.bench_with_input(BenchmarkId::new("Insertion sort", 0), &v, |b, v| b.iter(|| insertion_sort::sort(v)));
    group.bench_with_input(BenchmarkId::new("Shaker sort", 0), &v, |b, v| b.iter(|| shaker_sort::sort(v)));
    group.bench_with_input(BenchmarkId::new("Gnome sort", 0), &v, |b, v| b.iter(|| gnome_sort::sort(v)));

    group.finish();
}

criterion_group!(benches, benchmark_sort);
criterion_main!(benches);
