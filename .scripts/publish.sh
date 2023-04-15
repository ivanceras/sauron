#!/bin/sh

# script to publish the crates 

set -ev
cd crates/sauron-core && cargo publish && cd - && \
cd crates/sauron-node-macro && cargo publish && cd - &&
cargo publish
