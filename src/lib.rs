use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use relations::{serialize, XorRelationWithoutInput};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::js_api::{alert, now};

mod js_api;

#[wasm_bindgen]
/// Generate keys for a SNARK relation.
pub fn generate_keys() {
    let start = now();

    let pk = _generate_keys();

    alert(&format!(
        "It took {}ms. Key has length: {}",
        now() - start,
        pk.len()
    ));
}

fn _generate_keys() -> Vec<u8> {
    let circuit = XorRelationWithoutInput::new(2);

    let mut rng = ark_std::test_rng();
    let (pk, _vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

    serialize(&pk)
}
