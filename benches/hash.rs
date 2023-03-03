use ark_bls12_381::Fr;
use ark_relations::r1cs::ConstraintSystemRef;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use liminal_ark_poseidon::circuit;
use manta_crypto::{
    arkworks::{
        constraint::{fp::Fp, FpVar},
        ff::field_new,
    },
    hash::ArrayHashFunction,
    rand::{OsRng, Sample},
};
use manta_pay::{
    config::{poseidon::Spec2 as Poseidon2, utxo::InnerHashDomainTag, ConstraintField},
    crypto::poseidon::hash::Hasher,
};
#[inline]
fn poseidon_hash_manta(c: &mut Criterion) {
    let mut group = c.benchmark_group("poseidon");
    let mut rng = OsRng;
    let hasher = black_box(Hasher::<Poseidon2, InnerHashDomainTag, 2>::sample(
        (),
        &mut rng,
    ));
    let inputs = black_box([
        Fp(field_new!(ConstraintField, "1")),
        Fp(field_new!(ConstraintField, "2")),
    ]);
    group.bench_function("manta", |b| {
        b.iter(|| {
            let _ = black_box(hasher.hash([&inputs[0], &inputs[1]], &mut ()));
        })
    });
}

#[inline]
fn poseidon_hash_liminal(c: &mut Criterion) {
    let mut group = c.benchmark_group("poseidon");
    let inputs = black_box([
        FpVar::<Fr>::Constant(1.into()),
        FpVar::<Fr>::Constant(2.into()),
    ]);
    let cs = black_box(ConstraintSystemRef::None);
    group.bench_function("liminal", |b| {
        b.iter(|| {
            let _ = black_box(circuit::two_to_one_hash(
                cs.clone(),
                [inputs[0].clone(), inputs[1].clone()],
            ));
        })
    });
}

criterion_group!(crypto, poseidon_hash_manta, poseidon_hash_liminal);
criterion_main!(crypto);