use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    c.bench_function("new", |b| b.iter(|| veb::VebTree::new(1024 * 1024)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
