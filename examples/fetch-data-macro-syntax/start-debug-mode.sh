#!/bin/bash

set -v

. ./bootstrap.sh


wasm-pack build --target web --dev
basic-http-server ./ -a 0.0.0.0:4007
