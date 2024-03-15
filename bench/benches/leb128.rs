use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};


fn bench_leb128(c: &mut Criterion) {
    let edge_cases = [0, 1, u64::MAX, u64::MAX / 2, 1 << 63, (1 << 63) - 1];


    // Bench edge cases
    for &value in edge_cases.iter() {
        c.bench_with_input(BenchmarkId::new("Edge", format!("{} {}", "leb128::write", value)), &value, |b, &v| {
            let mut buffer = [0u8; 10];
            b.iter(|| {
                let mut writable = &mut buffer[..];
                leb128::write::unsigned(&mut writable, v).expect("Should write number");
            });
        });
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1u64 << power;
        c.bench_with_input(BenchmarkId::new("Log", format!("{} {}", "leb128::write", value)), &value, |b, &v| {
            let mut buffer = [0u8; 10];
            b.iter(|| {
                let mut writable = &mut buffer[..];
                leb128::write::unsigned(&mut writable, v).expect("Should write number");
            });
        });
    }


}

criterion_group!(benches, bench_leb128);
criterion_main!(benches);
