use instant::Instant;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasmarking::{ArkRelation, JfRelation};
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

static REPEAT: usize = 3;

fn bench_ark(relation_name: &str) {
    let relation = ArkRelation::from(relation_name);
    let (pk, _) = relation.generate_keys();

    let start = Instant::now();
    for _ in 0..REPEAT {
        relation.generate_proof(pk.clone());
    }
    let elapsed = Instant::now().duration_since(start);

    console::log_1(
        &format!(
            "{:?} prover. Relation performance: {:?}",
            relation_name,
            (elapsed / REPEAT as u32)
        )
        .into(),
    );
}

#[wasm_bindgen_test]
fn bench_ark_xor() {
    bench_ark("xor");
}

#[wasm_bindgen_test]
fn bench_ark_withdraw() {
    bench_ark("withdraw");
}

#[wasm_bindgen_test]
fn bench_jf_withdraw() {
    let relation = JfRelation::Withdraw;
    let (pk, _) = relation.generate_keys();

    let start = Instant::now();
    for _ in 0..REPEAT {
        relation.generate_proof(pk.clone());
    }
    let elapsed = Instant::now().duration_since(start);

    console::log_1(
        &format!(
            "{:?} prover. Relation performance: {:?}",
            "jf_withdraw",
            (elapsed / REPEAT as u32)
        )
        .into(),
    );
}
