#![feature(lazy_cell)]

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use eytzinger_layout::{generate_data, Eytzinger};
use rand::{thread_rng, Rng};
use std::cell::LazyCell;

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: &[usize] = &[
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
    ];

    {
        let mut g = c.benchmark_group("std");
        for size in SIZE {
            let data = LazyCell::new(|| {
                let data = generate_data(size * 1024);
                let max = *data.last().unwrap();
                (data, max)
            });
            let size_kb = size * 4;
            g.bench_with_input(BenchmarkId::from_parameter(size_kb), size, |b, &_| {
                let (data, max) = &*data;
                let mut rng = thread_rng();
                b.iter_batched(
                    move || rng.gen_range(0..*max),
                    move |i| data.binary_search(&i),
                    BatchSize::SmallInput,
                );
            });
        }
    }

    {
        let mut g = c.benchmark_group("eytzinger branchless");
        for size in SIZE {
            let data = LazyCell::new(|| {
                let data = generate_data(size * 1024);
                let max = data.last().unwrap();
                let eytzinger = Eytzinger::from(&data[..]);
                (eytzinger, *max)
            });
            let size_kb = size * 4;
            g.bench_with_input(BenchmarkId::from_parameter(size_kb), size, |b, &_| {
                let (eytzinger, max) = &*data;
                let mut rng = thread_rng();
                b.iter_batched(
                    move || rng.gen_range(0..*max),
                    move |i| eytzinger.binary_search_branchless(i),
                    BatchSize::SmallInput,
                );
            });
        }
    }

    {
        let mut g = c.benchmark_group("eytzinger");
        for size in SIZE {
            let data = LazyCell::new(|| {
                let data = generate_data(size * 1024);
                let max = data.last().unwrap();
                let eytzinger = Eytzinger::from(&data[..]);
                (eytzinger, *max)
            });
            let size_kb = size * 4;
            g.bench_with_input(BenchmarkId::from_parameter(size_kb), size, |b, &_| {
                let (eytzinger, max) = &*data;
                let mut rng = thread_rng();
                b.iter_batched(
                    move || rng.gen_range(0..*max),
                    move |i| eytzinger.binary_search(i),
                    BatchSize::SmallInput,
                );
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
