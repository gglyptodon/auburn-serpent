cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/serpent.wasm --out-dir ./wasm_out --no-modules --no-typescript
echo "serve via:   python3 -m http.server --directory wasm_out/"
