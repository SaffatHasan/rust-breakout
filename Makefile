run-native:
	cargo run --package native

run-wasm:
	@cd crates/wasm && trunk serve --open
