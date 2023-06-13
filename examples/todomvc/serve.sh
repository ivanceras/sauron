#!/bin/bash

set -v

./bootstrap.sh

wasm-pack build --target web --release -- &&\

basic-http-server ./ -a 0.0.0.0:6001
