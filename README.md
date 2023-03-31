# wasmarking

Testing performance of generating SNARK proofs in native and browser (WASM) environments.

To run all benchmarks:

```shell
make bench
```

## Running wasm benchmarks

```shell
 cargo install wasm-pack
 wasm-pack test --release --headless --firefox
```

or

```shell
make wasm-bench
```

## Running native benchmarks

```shell
cargo install cargo-criterion
cargo criterion
```

or

```shell
make native-bench
```
