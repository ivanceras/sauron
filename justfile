
test:
    cargo test --all
check:
    cargo check --all

test-no-features:
    cargo test --all --no-default-features  --no-fail-fast

test-with-dom:
    cargo test --all --features "with-dom"  --no-fail-fast


wasm-test:
    wasm-pack test --firefox --headless

wasm-test-with-features:
    wasm-pack test --firefox --headless -- --no-default-features --features "with-dom with-node-macro custom_element" 


test-all: test wasm-test

publish:
    cd crates/sauron-core && cargo publish && cd - && \
    cd crates/sauron-macro && cargo publish && cd - &&
    cargo publish
