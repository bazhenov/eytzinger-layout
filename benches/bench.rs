use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use eytzinger_layout::{eytzinger, eytzinger_binary_search, generate_data};

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("(1M/1000)");
    let data = generate_data(100_000_000);
    let eytzinger = eytzinger(&data);
    let factor = 1000;
    let elements = data.len() / factor;

    g.throughput(Throughput::Elements(elements as u64));
    g.bench_function("std. binary search", |b| {
        let data = data.clone();
        b.iter(move || {
            for i in 0..elements {
                let el = &data[i * factor];
                let _ = black_box(data.binary_search(el));
            }
        });
    });

    g.bench_function("eytzinger binary search", |b| {
        let data = &data;
        let eytzinger = &eytzinger;
        b.iter(move || {
            for i in 0..elements {
                let el = data[i * factor];
                let _ = black_box(eytzinger_binary_search(&eytzinger, el));
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
