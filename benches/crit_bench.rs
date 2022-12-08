use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exper_some::add;

#[allow(unused)]
fn bench_add(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| black_box(add(2, 2)));
    });
}

criterion_group!(benches, bench_add,);
criterion_main!(benches);
