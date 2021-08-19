#!/bin/sh

set -ev
cd crates/sauron-core && cargo publish && cd - && \
cd crates/sauron-node-macro && sleep 5 && cargo publish && cd - &&
cargo publish
