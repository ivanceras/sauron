#!/bin/bash

set -v

./bootstrap.sh

wasm-pack build --target web --profiling --


./copy_files_to_benchmark_directory.sh

basic-http-server ./ -a 0.0.0.0:6001
