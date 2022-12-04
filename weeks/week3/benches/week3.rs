use sha3::Digest;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const MESSAGE: &[u8] = b"Hello World";
fn benchmark_blake3(c: &mut Criterion) {
    c.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(black_box(MESSAGE));
        })
    });
}

fn benchmark_sha3(c: &mut Criterion) {
    c.bench_function("sha3", |b| {
        b.iter(|| {
            let mut hasher = sha3::Sha3_256::new();

            // write input message
            hasher.update(black_box(MESSAGE));

            // read hash digest
            hasher.finalize();
        })
    });
}

fn benchmark_sha2(c: &mut Criterion) {
    c.bench_function("sha2", |b| {
        b.iter(|| {
            let mut hasher = sha2::Sha256::new();

            // write input message
            hasher.update(black_box(MESSAGE));

            // read hash digest
            hasher.finalize();
        })
    });
}

criterion_group! {
    name= benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = benchmark_blake3, benchmark_sha3, benchmark_sha2
}
criterion_main!(benches);
