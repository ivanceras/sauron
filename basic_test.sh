#!/bin/bash

cargo test --all

cd crates/sauron-core/
./basic_test.sh
