cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/main.wasm --out-dir ./wasm_help --no-modules --no-typescript
echo "serve via:   python3 -m http.server --directory wasm_help/"
#wasm-bindgen target/wasm32-unknown-unknown/release/main.wasm --out-dir ./wasm_help --no-modules --no-typescript
