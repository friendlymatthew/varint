use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rizz64::Rizz64;
fn bench_read_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rizz64 decoding unsigned integers");

    let edge_cases: [u64; 6] = [0, 1, u64::MAX, u64::MAX / 2, 1 << 63, (1 << 63) - 1];

    // Bench edge cases
    for (i, &value) in edge_cases.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("edge:read_u64", format!("{}{}", i, value)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let n = Rizz64::write_u64(&mut buffer, v).expect("should read");
                b.iter(|| Rizz64::read_u64(&buffer[..n]))
            },
        );
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1u64 << power;
        group.bench_with_input(
            BenchmarkId::new("log:read_u64", format!("{}{}", value, power)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let n = Rizz64::write_u64(&mut buffer, v).expect("should read");
                b.iter(|| Rizz64::read_u64(&buffer[..n]))
            },
        );
    }

    group.finish();
}

fn bench_read_i64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rizz64 decoding signed integers");
    let edge_cases: [i64; 6] = [0, 1, i64::MAX, i64::MAX / 2, 1 << 63, (-1 << 63) + 1];
    // Bench edge cases
    for (i, &value) in edge_cases.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("edge:read_i64", format!("{}{}", value, i)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let n = Rizz64::write_i64(&mut buffer, v).expect("should read");
                b.iter(|| Rizz64::read_i64(&buffer[..n]))
            },
        );
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1i64 << power;
        group.bench_with_input(
            BenchmarkId::new("log:read_i64", format!("{}{}", value, power)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let n = Rizz64::write_i64(&mut buffer, v).expect("should read");
                b.iter(|| Rizz64::read_i64(&buffer[..n]))
            },
        );
    }
}

criterion_group!(benches, bench_read_u64, bench_read_i64);
criterion_main!(benches);
