#!/bin/bash

set -v

./bootstrap.sh

wasm-pack build --target web --release -- --features ""

basic-http-server ./ -a 0.0.0.0:6001
