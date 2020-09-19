#!/bin/bash

set -v

. ./bootstrap.sh


wasm-pack build --target web --dev -- --features "with-lite-markdown with-measure wee_alloc console_log console_error_panic_hook"

basic-http-server ./ -a 0.0.0.0:4001
