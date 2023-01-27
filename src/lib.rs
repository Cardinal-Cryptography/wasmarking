use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::test_rng;
use relations::{
    serialize, CanonicalDeserialize, XorRelationWithFullInput, XorRelationWithoutInput,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::js_api::{alert, now};

mod js_api;

/// Generate keys for a SNARK relation.
#[wasm_bindgen]
pub fn generate_keys() -> Vec<u8> {
    let start = now();

    let pk = _generate_keys();

    alert(&format!(
        "It took {}ms. Key has length: {}",
        now() - start,
        pk.len()
    ));

    pk
}

/// Generate a proof for a SNARK relation.
#[wasm_bindgen]
pub fn generate_proof(pk: Vec<u8>) {
    let start = now();

    let proof = _generate_proof(pk);

    alert(&format!(
        "It took {}ms. Proof has length: {}",
        now() - start,
        proof.len()
    ));
}

fn _generate_keys() -> Vec<u8> {
    let circuit = XorRelationWithoutInput::new(2);

    let mut rng = test_rng();
    let (pk, _vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

    serialize(&pk)
}

fn _generate_proof(pk: Vec<u8>) -> Vec<u8> {
    let circuit = XorRelationWithFullInput::new(2, 1, 3);

    let pk = CanonicalDeserialize::deserialize(&*pk).unwrap();

    let mut rng = test_rng();
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

    serialize(&proof)
}
