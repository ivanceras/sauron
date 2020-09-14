#!/bin/bash

cargo test --all --no-default-features
cargo test --all --features "with-dom"

cd crates/sauron-core/
./test.sh
