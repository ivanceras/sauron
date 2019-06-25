#!/bin/bash

set -v

. ./bootstrap.sh

wasm-pack build --target no-modules --release -- --features "wee_alloc"


cp -r index.html style.css pkg ../../../todomvc-perf-comparison/todomvc-benchmark/todomvc/sauron_array/

basic-http-server ./ -a 0.0.0.0:6001
