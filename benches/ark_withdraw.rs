use ark_serialize::CanonicalSerialize;
use criterion::{criterion_group, criterion_main, Criterion};
use wasmarking::ArkRelation;

fn bench(c: &mut Criterion) {
    let relation = ArkRelation::from("withdraw");

    c.bench_function("ark/withdraw/keygen", |b| b.iter(|| relation.generate_keys()));

    let (pk, vk) = relation.generate_keys();
    print_sizes("verify key", &vk);

    c.bench_function("ark/withdraw/prover", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });

    let proof = relation.generate_proof(pk);
    print_sizes("proof", &proof);

    c.bench_function("ark/withdraw/verifier", |b| {
        b.iter(|| relation.verify_proof(&proof, &vk))
    });
}

fn print_sizes<T: CanonicalSerialize>(name: &str, obj: &T) {
    let mut buf_compressed = vec![0; obj.serialized_size()];
    obj.serialize(&mut buf_compressed[..]).unwrap();
    let mut buf_uncompressed = vec![0; obj.uncompressed_size()];
    obj.serialize_uncompressed(&mut buf_uncompressed[..])
        .unwrap();
    println!(
        "{} size is: {} {}",
        name,
        buf_uncompressed.len(),
        buf_compressed.len()
    );
}

criterion_group!(ark_withdraw, bench);
criterion_main!(ark_withdraw);
