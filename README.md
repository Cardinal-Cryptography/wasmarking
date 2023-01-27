# wasmarking
Testing how (in)efficient it is to generate SNARK proofs in a browser

# Usage

The wasm module is built with `wasm-pack`. 
You can install it with `cargo install wasm-pack`.

## Building module
```shell
 wasm-pack build --target web --release
```

## Running
Open [`index.html`](index.html) in your favorite browser.
1. The website will **compute keys for `xor` relation** and display in a dialog window how long it took. (~120ms).
2. After clicking `Ok`, the website will **compute proof for `xor` relation** and display in a dialog window how long it took (~240ms).
3. After clicking `Ok`, the website will **compute keys for `withdraw` relation** and display in a dialog window how long it took (~5s).
4. After clicking `Ok`, the website will **compute proof for `withdraw` relation** and display in a dialog window how long it took (~113s / browser begging for abort).

The estimated times were obtained with Firefox on Ubuntu 22 with 12th Gen Intel® Core™ i7-12800H × 20.
