#!/bin/bash

set -v

./bootstrap.sh

wasm-pack build --target no-modules --release -- --features ""

basic-http-server ./ -a 0.0.0.0:6001
