#!/bin/bash

set -v

. ./bootstrap.sh


wasm-pack build --target web --dev -- --features "with-measure wee_alloc console_error_panic_hook"

basic-http-server ./ -a 0.0.0.0:4001
