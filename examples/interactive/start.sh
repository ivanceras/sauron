#!/bin/bash

set -v

if ! type wasm-pack > /dev/null; then
    echo "wasm-pack is not installed"
    cargo install wasm-pack
fi

if ! type basic-http-server > /dev/null; then
    echo "basic-http-server is not installed"
    cargo install basic-http-server
fi


wasm-pack build --target no-modules

basic-http-server ./ -a 0.0.0.0:4003
