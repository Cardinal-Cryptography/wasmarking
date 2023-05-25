use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    js_api::{alert, now},
    relation::JfRelation,
};

mod js_api;
mod relation;

#[wasm_bindgen]
pub fn generate_proof() {
    let relation = JfRelation::Withdraw;

    alert("Generating srs");
    let srs = relation.generate_srs();

    alert("Generating keys");
    let (pk, _) = relation.generate_keys(&srs);

    alert("Generating proof");
    let start = now();
    for _ in 0..1 {
        relation.generate_proof(pk.clone());
    }
    let elapsed = now() - start;

    alert(&format!("Generating proof for took {elapsed}ms."));
}
