use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use eytzinger_layout::{generate_data, Eytzinger};
use rand::{thread_rng, Rng};

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: &[usize] = &[
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
    ];

    {
        let mut g = c.benchmark_group("std");
        for size in SIZE {
            let data = generate_data(size * 1024);
            let max = data.last().unwrap();
            let size_kb = data.len() * 4 / 1024;
            g.bench_function(format!("{size_kb}K"), |b| {
                let data = &data;
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
            let data = generate_data(size * 1024);
            let eytzinger = Eytzinger::from(&data[..]);
            let max = data.last().unwrap();
            let size_kb = data.len() * 4 / 1024;
            g.bench_function(format!("{size_kb}K"), |b| {
                let eytzinger = &eytzinger;
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
            let data = generate_data(size * 1024);
            let eytzinger = Eytzinger::from(&data[..]);
            let max = data.last().unwrap();
            let size_kb = data.len() * 4 / 1024;
            g.bench_function(format!("{size_kb}K"), |b| {
                let eytzinger = &eytzinger;
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
