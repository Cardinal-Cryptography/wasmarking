use ark_bls12_381::{Fr, FrParameters};
use ark_crypto_primitives::SNARK;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use manta_crypto::{
    arkworks::{
        constraint::{fp::Fp, FpVar, R1CS},
        ff::Fp256,
    },
    constraint::ProofSystem,
    eclair::{
        alloc::{
            mode::{Public, Secret},
            Allocate, Allocator,
        },
        bool::AssertEq,
    },
    hash::ArrayHashFunction,
    rand::{OsRng, Sample},
};
use manta_pay::{
    config::{
        poseidon::Spec2 as Poseidon2, utxo::InnerHashDomainTag, Compiler, ProofSystem as mG16,
    },
    crypto::poseidon::hash::Hasher,
};
use relations::{
    Groth16, LinearEquationRelationWithFullInput, LinearEquationRelationWithoutInput,
    PreimageMantaRelationWithFullInput, PreimageRelationWithFullInput,
    PreimageRelationWithoutInput,
};
use wasmarking::Relation;

#[inline]
fn linear(c: &mut Criterion) {
    let a = 2;
    let b = 1;
    let y = 3;
    let circuit_withouth_input = LinearEquationRelationWithoutInput::new(a, b, y);
    let x = 1;

    let mut rng = ark_std::test_rng();
    let (pk, _) = Groth16::circuit_specific_setup(circuit_withouth_input, &mut rng).unwrap();

    c.bench_function("linear/liminal", |f| {
        f.iter(|| {
            // #constraints = 99
            let full_circuit = LinearEquationRelationWithFullInput::new(a, b, y, x);

            let _ = <Groth16 as SNARK<Fr>>::prove(&pk, full_circuit, &mut rng);
        })
    });
}

#[inline]
fn linear_manta(c: &mut Criterion) {
    // num constraints 99
    let mut rng = OsRng;
    let a = Fp(Fr::from(2));
    let b = Fp(Fr::from(1));
    let y = Fp(Fr::from(3));
    c.bench_function("linear/liminal", |f| {
        f.iter(|| {
            // 241 constraints
            let mut compiler = Compiler::for_proofs();
            // let x: FpVar<Fp256<FrParameters>> = x.as_known::<Secret, _>(&mut compiler);
            // let y: FpVar<Fp256<FrParameters>> = y.as_known::<Secret, _>(&mut compiler);
            // let image: FpVar<Fp256<FrParameters>> = image.as_known::<Public, _>(&mut compiler);
            // let hash_var = hasher_circuit.hash([&x, &y], &mut compiler);
            // compiler.assert_eq(&hash_var, &image);
        })
    });
}

// TODO blockbox
#[inline]
fn preimage_manta(c: &mut Criterion) {
    let mut group = c.benchmark_group("preimage");
    let mut rng = OsRng;
    let hasher = black_box(Hasher::<Poseidon2, InnerHashDomainTag, 2>::sample(
        (),
        &mut rng,
    ));
    let x = Fp(Fr::from(1));
    let y = Fp(Fr::from(2));
    let image = hasher.hash([&x, &y], &mut ());
    let hasher_circuit = Hasher::<Poseidon2, InnerHashDomainTag, 2, R1CS<_>>::sample((), &mut rng);
    let mut compiler = mG16::context_compiler();
    let xuk: FpVar<Fp256<FrParameters>> = compiler.allocate_unknown::<Secret, _>();
    let yuk: FpVar<Fp256<FrParameters>> = compiler.allocate_unknown::<Secret, _>();
    let iamgeuk: FpVar<Fp256<FrParameters>> = compiler.allocate_unknown::<Public, _>();
    let hash_var_uk = hasher_circuit.hash([&xuk, &yuk], &mut compiler);
    compiler.assert_eq(&hash_var_uk, &iamgeuk);
    let (pk, _) = mG16::compile(&(), compiler, &mut rng).unwrap();
    group.bench_function("manta", |b| {
        b.iter(|| {
            // 241 constraints
            let mut compiler = mG16::proof_compiler();
            let x: FpVar<Fp256<FrParameters>> = x.as_known::<Secret, _>(&mut compiler);
            let y: FpVar<Fp256<FrParameters>> = y.as_known::<Secret, _>(&mut compiler);
            let image: FpVar<Fp256<FrParameters>> = image.as_known::<Public, _>(&mut compiler);
            let hash_var = hasher_circuit.hash([&x, &y], &mut compiler);
            compiler.assert_eq(&hash_var, &image);
            mG16::prove(&pk, compiler, &mut rng);
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
            // #constraints = 238
            let full_circuit =
                PreimageRelationWithFullInput::new(frontend_image, preimage.0 .0, preimage1.0 .0);

            let _ = <Groth16 as SNARK<Fr>>::prove(&pk, full_circuit, &mut rng);
        })
    });
}

#[inline]
fn preimage_poseidon_manta(c: &mut Criterion) {
    let circuit_withouth_input = PreimageRelationWithoutInput::new();

    let preimage = Fr::from(7u64);
    let preimage1 = Fr::from(13u64);
    let image = liminal_ark_poseidon::hash::two_to_one_hash([preimage, preimage1]);
    let frontend_image: [u64; 4] = image.0 .0;

    let mut rng = ark_std::test_rng();
    let (pk, _) = Groth16::circuit_specific_setup(circuit_withouth_input, &mut rng).unwrap();

    c.bench_function("preimage/liminal_poseidon_manta", |b| {
        b.iter(|| {
            // #constraints = 238
            let full_circuit = PreimageMantaRelationWithFullInput::new(
                frontend_image,
                preimage.0 .0,
                preimage1.0 .0,
            );

            let _ = <Groth16 as SNARK<Fr>>::prove(&pk, full_circuit, &mut rng);
        })
    });
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
        // #constraints = 1534
        b.iter(|| relation.generate_proof(pk.clone()))
    });
    group.finish();
}

criterion_group!(
    prover,
    xor,
    withdraw,
    preimage,
    preimage_manta,
    preimage_poseidon_manta,
    linear,
    linear_manta
);
criterion_main!(prover);
