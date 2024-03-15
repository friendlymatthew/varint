use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_bit_len(c: &mut Criterion) {

    for i in 0..64 {
        let x = 1u64 << i;
        c.bench_with_input(
            BenchmarkId::new("bit_len", format!("{}", i)),
            &x,
            |b, &v| {
                b.iter(|| {
                    rizz64::Rizz64::size_u64(v);
                })
            }
        );
    }
}

criterion_group!(benches, bench_bit_len);
criterion_main!(benches);
