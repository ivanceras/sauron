
test:
    cargo test --all --no-fail-fast
    cargo test --all --all-features
check:
    cargo check --all

test-no-features:
    cargo test --all --no-default-features  --no-fail-fast

test-with-dom:
    cargo test --all --features "with-dom"  --no-fail-fast


wasm-test:
   RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack test --firefox --headless

wasm-test-with-features:
   RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack test --firefox --headless -- --no-default-features --features "with-dom with-node-macro custom_element"


test-all: test wasm-test

publish-core:
    cargo publish -p sauron-core

publish-html-parser:
    cargo publish -p sauron-html-parser

publish-macro:
    cargo publish -p sauron-macro

publish: publish-core publish-html-parser publish-macro
    cargo publish
