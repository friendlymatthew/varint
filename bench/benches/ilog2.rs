use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_ilog2(c: &mut Criterion) {

    for i in 0..64 {
        let x = 1u64 << i;
        c.bench_with_input(
            BenchmarkId::new("ilog2", format!("{}", i)),
            &x,
            |b, &v| {
                b.iter(|| {
                    if x == 0 { 0 } else { v.ilog2() as usize + 1 }
                })
            }
        );
    }
}

criterion_group!(benches, bench_ilog2);
criterion_main!(benches);
