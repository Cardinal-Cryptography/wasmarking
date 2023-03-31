use criterion::{criterion_group, criterion_main, Criterion};
use wasmarking::Relation;

fn xor(c: &mut Criterion) {
    let relation = Relation::from("xor");

    c.bench_function("keygen/xor", |b| b.iter(|| relation.generate_keys()));
}

fn withdraw(c: &mut Criterion) {
    let relation = Relation::from("withdraw");

    c.bench_function("keygen/withdraw", |b| b.iter(|| relation.generate_keys()));
}

criterion_group!(key_generation, xor, withdraw);
criterion_main!(key_generation);
