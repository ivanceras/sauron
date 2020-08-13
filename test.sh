#!/bin/bash

cargo test --all --no-default-features
cargo test --all --all-features

cd crates/sauron-core/
./test.sh
