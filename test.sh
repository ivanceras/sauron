#!/bin/bash

cargo test --all --no-default-features  &&\
cargo test --all --features "with-dom"  &&\
cargo test --all --all-features &&\
wasm-pack test --firefox --headless -- --no-default-features --features "with-dom with-node-macro" &&\
wasm-pack test --firefox --headless -- 
