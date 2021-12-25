use criterion::{criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    c.bench_function("new", |b| b.iter(|| veb::VebTree::new(1024 * 1024)));
    c.bench_function("insert", |b| {
        let mut tree = veb::VebTree::new(1024 * 1024);
        b.iter(|| tree.insert(65536))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
