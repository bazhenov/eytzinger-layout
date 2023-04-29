use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use eytzinger_layout::{eytzinger, eytzinger_binary_search, generate_data};
use rand::{thread_rng, RngCore};

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: &[usize] = &[
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
    ];

    {
        let mut g = c.benchmark_group("std. binary search");
        g.throughput(Throughput::Elements(1));
        for size in SIZE {
            let data = generate_data(size * 1024);
            let rng = thread_rng();
            let size_kb = data.len() * 4 / 1024;
            g.bench_function(format!("{size_kb}K"), |b| {
                let data = &data;
                let mut rng = rng.clone();
                b.iter_batched(
                    move || rng.next_u32(),
                    move |i| data.binary_search(&i),
                    BatchSize::SmallInput,
                );
            });
        }
    }

    {
        let mut g = c.benchmark_group("eytzinger binary search");
        for size in SIZE {
            let data = generate_data(size * 1024);
            let eytzinger = eytzinger(&data);
            let rng = thread_rng();
            let size_kb = data.len() * 4 / 1024;
            g.bench_function(format!("{size_kb}K"), |b| {
                let eytzinger = &eytzinger;
                let mut rng = rng.clone();
                b.iter_batched(
                    move || rng.next_u32(),
                    move |i| eytzinger_binary_search(eytzinger, i),
                    BatchSize::SmallInput,
                );
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
