if wasm-pack build --target web --dev -- --features "console_error_panic_hook"; then
    basic-http-server -a 0.0.0.0:4000
fi
