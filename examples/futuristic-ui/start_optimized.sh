#!/bin/bash

set -v

. ./bootstrap.sh &&\


./build_optimized.sh &&\

basic-http-server ./ -a 0.0.0.0:4001
