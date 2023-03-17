use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use auth_service::security::PasswordHasher;

fn sha2(hasher: &PasswordHasher, password: &str, salt: &str) -> String {
    hasher.hash_password(password, salt)
}

fn ring(hasher: &PasswordHasher, password: &str, salt: &str) -> String {
    hasher.hash_password(password, salt)
}

fn criterion_benchmark(c: &mut Criterion) {
    let sha2_hasher = PasswordHasher::new_sha2("pepper");
    let ring_hasher = PasswordHasher::new_ring("pepper");

    c.bench_with_input(
        BenchmarkId::new("sha2", &sha2_hasher),
        &sha2_hasher,
        |b, s| {
            b.iter(|| sha2(s, black_box("password"), black_box("salt")));
        },
    )
    .bench_with_input(
        BenchmarkId::new("ring", &ring_hasher),
        &ring_hasher,
        |b, s| {
            b.iter(|| ring(s, black_box("password"), black_box("salt")));
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
