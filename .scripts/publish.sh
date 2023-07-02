#!/bin/sh

# script to publish the crates 

set -ev
cd crates/sauron-core && cargo publish && cd - && \
cd crates/sauron-macro && cargo publish && cd - &&
cargo publish
