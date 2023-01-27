use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    js_api::{alert, now},
    relation::Relation,
};

mod js_api;
mod relation;

/// Generate keys for a SNARK relation.
#[wasm_bindgen]
pub fn generate_keys(relation_name: &str) -> Vec<u8> {
    let start = now();

    let relation = Relation::from(relation_name);
    let pk = relation.generate_keys();

    let elapsed = now() - start;

    alert(&format!(
        "Generating keys for `{relation_name}` took {elapsed}ms. Key has length: {}",
        pk.len()
    ));

    pk
}

/// Generate a proof for a SNARK relation.
#[wasm_bindgen]
pub fn generate_proof(relation_name: &str, pk: Vec<u8>) {
    let start = now();

    let relation = Relation::from(relation_name);
    let proof = relation.generate_proof(pk);

    let elapsed = now() - start;

    alert(&format!(
        "Generating proof for `{relation_name}` took {elapsed}ms. Proof has length: {}",
        proof.len()
    ));
}
