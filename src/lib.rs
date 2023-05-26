use rayon::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
pub use wasm_bindgen_rayon::init_thread_pool;

use crate::js_api::{alert, now};

mod js_api;
mod relation;

fn sleep(id: i64) -> i64 {
    let n = 10_000;
    let mut x = id;
    let mut z = 1;
    for i in 0i64..n {
        for j in 0i64..n {
            z *= i * j;
        }
        x = id;
    }
    web_sys::console::log_1(&format!("{} {}", x, z).into());
    x
}

#[wasm_bindgen]
pub fn generate_proof() {
    let start = now();
    let v: Vec<i64> = (0..10).collect();
    v.into_par_iter().map(sleep).sum::<i64>();
    let elapsed = now() - start;
    alert(&format!("Slept for {elapsed}ms."));
    sleep(1);

    // let relation = JfRelation::Withdraw;

    // alert("Generating srs");
    // let srs = relation.generate_srs();

    // alert("Generating keys");
    // let (pk, _) = relation.generate_keys(&srs);

    // alert("Generating proof");
    // let start = now();
    // relation.generate_proof(pk.clone());
    // let elapsed = now() - start;

    // alert(&format!("Generating proof for took {elapsed}ms."));
}
