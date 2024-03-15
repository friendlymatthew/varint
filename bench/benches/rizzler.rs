/// the Rizzler
use rizz64::{
    Rizz64,
};

use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};

fn bench_new(c: &mut Criterion) {
    let edge_cases = [0, 1, u64::MAX, u64::MAX / 2, 1 << 63, (1 << 63) - 1];

    // Bench edge cases
    for &value in edge_cases.iter() {
        c.bench_with_input(BenchmarkId::new("Edge", format!("{} {}", "put_u64", value)), &value, |b, &v| {
            let mut buffer = [0u8; 10];
            b.iter(|| Rizz64::append_u64(&mut buffer, v))
        });
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1u64 << power;
        c.bench_with_input(BenchmarkId::new("Log", format!("{} {}", "put_u64", value)), &value, |b, &v| {
            let mut buffer = [0u8; 10];
            b.iter(|| Rizz64::append_u64(&mut buffer, v))
        });
    }

}


criterion_group!(benches, bench_new);
criterion_main!(benches);

