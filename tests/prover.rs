use instant::Instant;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasmarking::Relation;
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

static REPEAT: usize = 3;

fn bench(relation_name: &str) {
    let relation = Relation::from(relation_name);
    let pk = relation.generate_keys();

    let start = Instant::now();
    for _ in 0..REPEAT {
        relation.generate_proof(pk.clone());
    }
    let elapsed = Instant::now().duration_since(start);

    console::log_1(
        &format!(
            "{:?} Performance: {:?}",
            relation_name,
            (elapsed / REPEAT as u32)
        )
        .into(),
    );
}

#[wasm_bindgen_test]
fn bench_xor() {
    bench("xor");
}
