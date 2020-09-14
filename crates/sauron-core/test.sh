#!/bin/bash

cargo +stable test

# Install wasm-pack if it isn't installed yet
if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

wasm-pack test --firefox --headless -- +stable
