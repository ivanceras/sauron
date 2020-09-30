#!/bin/bash

set -v

./bootstrap.sh


wasm-pack build --target web --dev -- --features "console_error_panic_hook with-measure with-storage sauron/with-nodeidx-debug"

basic-http-server ./ -a 0.0.0.0:6001
