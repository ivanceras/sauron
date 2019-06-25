#!/bin/bash

set -v

. ./bootstrap.sh

wasm-pack build --target no-modules --profiling --

sleep 1

mkdir -p ../../../todomvc-perf-comparison/todomvc-benchmark/todomvc/sauron_array/

cp -r index.html style.css pkg ../../../todomvc-perf-comparison/todomvc-benchmark/todomvc/sauron_array/

basic-http-server ./ -a 0.0.0.0:6001
