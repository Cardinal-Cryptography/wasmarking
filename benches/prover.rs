use criterion::{criterion_group, criterion_main, Criterion};
use wasmarking::Relation;

fn xor(c: &mut Criterion) {
    let relation = Relation::from("xor");
    let pk = relation.generate_keys();

    c.bench_function("xor", |b| b.iter(|| relation.generate_proof(pk.clone())));
}

criterion_group!(prover, xor);
criterion_main!(prover);
