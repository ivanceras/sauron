#!/bin/bash



# Install cargo-readme if it isn't installed yet
if ! type cargo-readme > /dev/null; then
    cargo install cargo-readme
fi

cargo readme > README.md

