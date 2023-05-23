use instant::Instant;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasmarking::{ArkRelation, JfRelation};
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

static REPEAT: usize = 3;

fn ark_bench(relation_name: &str) {
    let relation = ArkRelation::from(relation_name);

    let start = Instant::now();
    for _ in 0..REPEAT {
        relation.generate_keys();
    }
    let elapsed = Instant::now().duration_since(start);

    console::log_1(
        &format!(
            "{:?} key generation. Relation performance: {:?}",
            relation_name,
            (elapsed / REPEAT as u32)
        )
        .into(),
    );
}

#[wasm_bindgen_test]
fn ark_bench_xor() {
    ark_bench("xor");
}

#[wasm_bindgen_test]
fn ark_bench_withdraw() {
    ark_bench("withdraw");
}

fn jf_bench(operation: &str, job: impl Fn()) {

    let start = Instant::now();
    for _ in 0..REPEAT {
        job()
    }
    let elapsed = Instant::now().duration_since(start);

    console::log_1(
        &format!(
            "{:?} key generation. Relation performance: {:?}",
            operation,
            (elapsed / REPEAT as u32)
        )
        .into(),
    );
}

#[wasm_bindgen_test]
fn jf_bench_withdraw() {
    let relation = JfRelation::Withdraw;

    let job = || relation.generate_circuit();
    jf_bench("circuit generation", job);

    let job = || {let _ =relation.generate_srs();};
    jf_bench("srs generation", job);

    let srs = relation.generate_srs();

    let job = ||{let _ =relation.generate_keys(&srs);};
    jf_bench("keys generation", job);
}
