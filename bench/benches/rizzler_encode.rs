use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rizz64::Rizz64;
fn bench_write_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rizz64 encoding unsigned integers");

    let edge_cases: [u64; 6] = [0, 1, u64::MAX, u64::MAX / 2, 1 << 63, (1 << 63) - 1];
    // Bench edge cases
    for (i, &value) in edge_cases.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("edge:write_u64", format!("{}{}", value, i)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                b.iter(|| Rizz64::write_u64(&mut buffer, v))
            },
        );
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1u64 << power;
        group.bench_with_input(
            BenchmarkId::new("log:write_u64", format!("{}{}", value, power)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                b.iter(|| Rizz64::write_u64(&mut buffer, v))
            },
        );
    }

    group.finish();
}

fn bench_write_i64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rizz64 encoding signed integers");
    let edge_cases: [i64; 6] = [0, 1, i64::MAX, i64::MAX / 2, 1 << 63, (-1 << 63) + 1];
    // Bench edge cases
    for (i, &value) in edge_cases.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("edge:write_i64", format!("{}{}", value, i)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                b.iter(|| Rizz64::write_i64(&mut buffer, v))
            },
        );
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1i64 << power;
        group.bench_with_input(
            BenchmarkId::new("log:write_i64", format!("{}{}", value, power)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                b.iter(|| Rizz64::write_i64(&mut buffer, v))
            },
        );
    }
}

criterion_group!(benches, bench_write_u64, bench_write_i64);
criterion_main!(benches);
