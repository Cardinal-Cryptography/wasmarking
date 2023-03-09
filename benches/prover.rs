use ark_bls12_381::Fr;
use ark_crypto_primitives::SNARK;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use manta_crypto::{
    arkworks::constraint::fp::Fp,
    hash::ArrayHashFunction,
    rand::{OsRng, Sample},
};
use manta_pay::{
    config::{poseidon::Spec2 as Poseidon2, utxo::InnerHashDomainTag},
    crypto::poseidon::hash::Hasher,
};
use relations::{
    preimage_proving, ConstraintSystemRef, Groth16, PreimageRelationWithFullInput,
    PreimageRelationWithoutInput,
};
use wasmarking::Relation;

#[inline]
fn preimage_manta(c: &mut Criterion) {
    let mut group = c.benchmark_group("preimage");
    let mut rng = OsRng;
    let hasher = black_box(Hasher::<Poseidon2, InnerHashDomainTag, 2>::sample(
        (),
        &mut rng,
    ));
    let inputs = black_box([Fp(Fr::from(1)), Fp(Fr::from(2))]);
    let _hash = black_box(hasher.hash([&inputs[0], &inputs[1]], &mut ()));
    group.bench_function("manta", |b| {
        b.iter(|| {
            let _ = black_box(hasher.hash([&inputs[0], &inputs[1]], &mut ()));
        })
    });
}

#[inline]
fn preimage(c: &mut Criterion) {
    let circuit_withouth_input = PreimageRelationWithoutInput::new();

    let preimage = Fr::from(7u64);
    let preimage1 = Fr::from(13u64);
    let image = liminal_ark_poseidon::hash::two_to_one_hash([preimage, preimage1]);
    let frontend_image: [u64; 4] = image.0 .0;

    let mut rng = ark_std::test_rng();
    let (pk, _) = Groth16::circuit_specific_setup(circuit_withouth_input, &mut rng).unwrap();

    c.bench_function("preimage/liminal", |b| {
        b.iter(|| {
            let full_circuit =
                PreimageRelationWithFullInput::new(frontend_image, preimage.0 .0, preimage1.0 .0);

            match full_circuit {
                ConstraintSystemRef::None => panic!(""),
                ConstraintSystemRef::CS(cs) => println!("Number of constraints: {:?}", cs),
            }
            let _ = <Groth16 as SNARK<Fr>>::prove(&pk, full_circuit, &mut rng);
        })
    });
}

#[inline]
fn preimage_2(c: &mut Criterion) {
    c.bench_function("preimage/liminal_simple", |b| b.iter(preimage_proving));
}

#[inline]
fn xor(c: &mut Criterion) {
    let relation = Relation::from("xor");
    let pk = relation.generate_keys();

    c.bench_function("prover/xor", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });
}

#[inline]
fn withdraw(c: &mut Criterion) {
    let relation = Relation::from("withdraw");
    let pk = relation.generate_keys();

    let mut group = c.benchmark_group("prover");
    group.sample_size(10);
    group.bench_function("withdraw", |b| {
        b.iter(|| relation.generate_proof(pk.clone()))
    });
    group.finish();
}

criterion_group!(prover, xor, withdraw, preimage, preimage_2);
criterion_main!(prover);
