#!/bin/bash

set -v

. ./bootstrap.sh


wasm-pack build --target web --dev -- --no-default-features --features "sauron/with-dom sauron/with-measure"

basic-http-server ./ -a 0.0.0.0:4001
