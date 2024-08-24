#!/bin/sh

set -ex

# RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
#   rustup run stable \
#   wasm-pack build --release --target web

# RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --release --target web

# Run the `wasm2js` tool from `binaryen`
wasm2js pkg/wasm_lib_bg.wasm -o pkg/wasm_lib_bg.wasm.js

cp pkg/wasm_lib.js ../web
cp pkg/wasm_lib_bg.wasm ../web

# Update our JS shim to require the JS file instead
# sed -i 's/wasm2js_bg.wasm/wasm2js_bg.wasm.js/' pkg/wasm_lib.js
# sed -i 's/wasm2js_bg.wasm/wasm2js_bg.wasm.js/' pkg/wasm_lib_bg.js
