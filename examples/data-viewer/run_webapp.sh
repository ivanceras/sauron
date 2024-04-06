#!/bin/bash
set -v
wasm-pack build --target web --release -- --features "console_error_panic_hook" &&\
basic-http-server -a 0.0.0.0:4000
