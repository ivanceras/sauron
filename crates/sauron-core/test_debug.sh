#!/bin/bash

cargo test

# Install wasm-pack if it isn't installed yet
if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

RUST_LOG=wasm_bindgen_test_runner wasm-pack test --firefox --headless -- +stable
