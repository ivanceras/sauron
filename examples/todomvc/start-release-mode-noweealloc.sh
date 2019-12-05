#!/bin/bash

set -v

. ./bootstrap.sh

wasm-pack build --target no-modules --release


cp -r index.html style.css pkg ../../../todomvc-perf-comparison/todomvc-benchmark/todomvc/sauron/

cp -r index.html style.css pkg $HOME/playground/mogwai-perf/todomvc/sauron/

basic-http-server ./ -a 0.0.0.0:6001
