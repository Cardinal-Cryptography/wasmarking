.PHONY: wasm-setup native-bench wasm-bench

wasm-setup:
	@cargo install wasm-pack --quiet

native-bench:
	@cargo bench --quiet --message-format=short

wasm-bench: wasm-setup
	@wasm-pack test --release --headless --firefox

bench: native-bench wasm-bench
