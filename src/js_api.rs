use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// For PoC we will display results in a dialog window.
    pub fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/js-extension.js")]
extern "C" {
    /// We cannot use `std::time::Instant` in wasm environment. Instead, JS will provide us with
    /// current time in milliseconds.
    pub fn now() -> u32;
}
