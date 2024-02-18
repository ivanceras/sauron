wasm-pack build --release --target=web -- --features "sauron/pre-diff" &&\

basic-http-server -a 0.0.0.0:4000
