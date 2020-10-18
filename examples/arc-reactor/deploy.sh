#!/bin/bash

set -v

./build_optimized.sh

dest="../../../ivanceras.github.io/arc-reactor/"

mkdir -p "$dest"

cp -r index.html pkg "$dest"

## Remove the ignore file on the pkg directory
rm $dest/pkg/.gitignore
