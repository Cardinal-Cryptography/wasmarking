use criterion::{criterion_group, criterion_main, Criterion};
use wasmarking::Relation;

fn xor(c: &mut Criterion) {
    let relation = Relation::from("xor");
    let pk = relation.generate_keys();

    c.bench_function("prover/xor", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });
}

fn withdraw(c: &mut Criterion) {
    let relation = Relation::from("withdraw");
    let pk = relation.generate_keys();

    c.bench_function("prover/withdraw", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });
}

criterion_group!(prover, xor, withdraw);
criterion_main!(prover);
