use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eytzinger_layout::generate_data;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("binary search (1M/1000)", |b| {
        let data = generate_data(1_0000_000);
        b.iter(move || {
            iterate(&data[..], 1_000);
        });
    });
}

/// Iterate each `factor`'th element in a `data` and search it using standart
/// binary_search algorithm.
#[inline]
fn iterate(data: &[u32], factor: usize) {
    assert!(factor > 0);
    for i in 0..data.len() / factor {
        let el = &data[i];
        let _ = black_box(data.binary_search(el));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
