#!/bin/sh

# script to publish the crates 
# we added a 5s sleep in between publish to give time for dependency crate to propagate to crates.io
#

set -ev
cd crates/sauron-component-macro && cargo publish && cd - && \
echo "sleeping" && sleep 20 &&\
cd crates/sauron-core && cargo publish && cd - && \
echo "sleeping" && sleep 20 &&\
cd crates/sauron-node-macro && cargo publish && cd - &&
echo "sleeping" && sleep 20 &&\
cargo publish
