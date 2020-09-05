#!/bin/bash

set -v

dest="../../../ivanceras.github.io/futuristic-ui/"

mkdir -p "$dest"

cp -r index.html sounds img pkg "$dest"

## Remove the ignore file on the pkg directory
rm $dest/pkg/.gitignore
