use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rizz64::Rizz128;

fn bench_ilog2(c: &mut Criterion) {
    for i in 0..64 {
        let x = 1u64 << i;
        c.bench_with_input(BenchmarkId::new("ilog2", format!("{}", i)), &x, |b, &v| {
            b.iter(|| Rizz128::size_u64(x))
        });
    }
}

criterion_group!(benches, bench_ilog2);
criterion_main!(benches);
