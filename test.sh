#!/bin/bash

cargo test --all --no-default-features  --no-fail-fast &&\
cargo test --all --features "with-dom"  --no-fail-fast &&\
cargo test --all --all-features --no-fail-fast &&\
wasm-pack test --firefox --headless -- --no-default-features --features "with-dom with-node-macro" &&\
wasm-pack test --firefox --headless -- 
