#!/bin/bash

cargo +stable test --all --no-default-features
cargo +stable test --all --features "with-dom"

cd crates/sauron-core/
./test.sh
