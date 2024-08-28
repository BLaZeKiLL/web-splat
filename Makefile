### WASM ###
SERVER_PORT = 8000
BUILD_OPTIONS_WASM = --no-default-features --target wasm32-unknown-unknown --lib --features npz --profile web-release
WASM_BINDGEN_OPTIONS = --out-dir public --web target/wasm32-unknown-unknown/web-release/web_splats.wasm --no-typescript

wasm-run: wasm-build wasm-serve

wasm-serve:
	python3 -m http.server -d ./public $(SERVER_PORT)

wasm-build: wasm-build-cargo wasm-bindgen

wasm-build-cargo:
	cargo build $(BUILD_OPTIONS_WASM)

wasm-bindgen:
	wasm-bindgen $(WASM_BINDGEN_OPTIONS)