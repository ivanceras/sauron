# Notes

## Publishing to crates.io

Publish the crates in the following order:
 1. sauron-core
 2. sauron-node-macro
 3. sauron

All of the crates must bump to the version number whenever any of the crate changed.


## Limitations of wasm_bindgen macro.
 - Does not support generics yet.
 - methods can not be simultaneously a getter and static
