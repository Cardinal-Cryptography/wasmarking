use criterion::{criterion_group, criterion_main, Criterion};
use wasmarking::Relation;

fn bench(c: &mut Criterion) {
    let relation = Relation::from("withdraw");

    c.bench_function("withdraw/keygen", |b| b.iter(|| relation.generate_keys()));

    let (pk, vk) = relation.generate_keys();

    c.bench_function("withdraw/prover", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });

    let proof = relation.generate_proof(pk);

    c.bench_function("withdraw/verifier", |b| {
        b.iter(|| relation.verify_proof(&proof, &vk))
    });
}

criterion_group!(withdraw, bench);
criterion_main!(withdraw);
