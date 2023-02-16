# wasmarking

Testing how (in)efficient it is to generate SNARK proofs in a browser

To run all benchmarks:
```shell
make bench
```

## Running wasm benchmarks

```shell
 cargo install wasm-pack
 wasm-pack test --release --headless --firefox
```

## Running native benchmarks

```shell
 cargo bench
```
