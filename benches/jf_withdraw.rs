use criterion::{criterion_group, criterion_main, Criterion};
use wasmarking::JfRelation;

fn bench(c: &mut Criterion) {
    let relation = JfRelation::Withdraw;

    c.bench_function("jf/withdraw/gen_circuit", |b| {
        b.iter(|| relation.generate_circuit())
    });

    c.bench_function("jf/withdraw/srsgen", |b| b.iter(|| relation.generate_srs()));

    let srs = relation.generate_srs();

    c.bench_function("jf/withdraw/keygen", |b| {
        b.iter(|| relation.generate_keys(&srs))
    });

    let (pk, vk) = relation.generate_keys(&srs);

    c.bench_function("jf/withdraw/prover", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });

    let proof = relation.generate_proof(pk);

    c.bench_function("jf/withdraw/verifier", |b| {
        b.iter(|| relation.verify_proof(&proof, &vk))
    });
}

criterion_group!(jf_withdraw, bench);
criterion_main!(jf_withdraw);
