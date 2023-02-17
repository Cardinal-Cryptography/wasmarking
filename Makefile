.PHONY: wasm-setup native-bench wasm-bench

wasm-setup:
	@cargo install wasm-pack --quiet

native-bench:
	@cargo bench --quiet --message-format=json > .native-bench
	@cat .native-bench | grep time: -A5 && rm .native-bench

wasm-bench: wasm-setup
	@wasm-pack test --release --headless --firefox > .wasm-bench
	@cat .wasm-bench | grep "Relation performance" && rm .wasm-bench

bench: native-bench wasm-bench
