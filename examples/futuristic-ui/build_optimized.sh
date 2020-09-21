#!/bin/bash

wasm-pack build --target web --release -- --features "wee_alloc"
