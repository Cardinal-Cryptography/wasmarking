# wasmarking
Testing how (in)efficient it is to generate SNARK proofs in a browser

# Usage

The wasm module is built with `wasm-pack`. 
You can install it with `cargo install wasm-pack`.

## Building module
```shell
 wasm-pack build --target web --release
```

## Running in browser
Open [`index.html`](index.html) in your favorite browser.
1. The website will **compute keys for `xor` relation** and display in a dialog window how long it took. (~120ms).
2. After clicking `Ok`, the website will **compute proof for `xor` relation** and display in a dialog window how long it took (~240ms).
3. After clicking `Ok`, the website will **compute keys for `withdraw` relation** and display in a dialog window how long it took (~5s).
4. After clicking `Ok`, the website will **compute proof for `withdraw` relation** and display in a dialog window how long it took (~113s / browser begging for abort).

The estimated times were obtained with Firefox on Ubuntu 22 with 12th Gen Intel® Core™ i7-12800H × 20.

## Running native
In the main folder of the project, run `cargo run --release`.

The following results were produced on i9-10885H 2.4GHz:
```
Generating keys for "xor" took 18.6374ms. Key has length: 9504
Generating proof for `"xor"` took 54.1048ms. Proof has length: 192
Generating keys for "withdraw" took 10.3467s. Key has length: 19737168
Generating proof for `"withdraw"` took 109.2391212s. Proof has length: 192
```
