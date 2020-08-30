#!/bin/bash

set -v

dest="../../../ivanceras.github.io/futuristic-ui/"

mkdir -p "$dest"

cp -r index.html sounds pkg "$dest"
