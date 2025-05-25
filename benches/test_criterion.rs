use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use std::hint::black_box;

const SETUP: i32 = 1_000_000;
fn test_criterion(c: &mut Criterion) {
    let setup = || {
        let mut i = 0;
        let mut x = 1;
        while i < SETUP {
            x += black_box(i * x);
            i += 1;
        }
        black_box(i)
    };

    let f = |i| {
        let mut j = 0;
        let mut x = 1;
        while j < 2 {
            x += black_box(i * x + j);
            j += black_box(1);
        }
    };

    c.bench_function("test_criterion", move |b| b.iter_batched(setup, f, BatchSize::SmallInput));
}

criterion_group!(
    benches,
    test_criterion,
);
criterion_main!(benches);
