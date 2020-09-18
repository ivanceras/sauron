#!/bin/bash

cargo test --all --no-default-features
cargo test --all --features "with-dom"
wasm-pack test --firefox --headless

cd crates/sauron-core/
./test.sh
