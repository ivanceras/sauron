# Now, you see me

- This demonstrate dispatching delayed message to toggle flip a flag in the model.
- The view node is flashed in between 2 second interval
- This also demonstrate the use of fragment node which is used in placed for top-level view that returns 0 or more nodes, but not 1.

## Pre-requisite
- rust
- wasm-pack
- basic-http-server
- just

If you have come to learn rust and wasm, we assumed you have rust installed and the `wasm32` toolchain target.
In addition wasm-pack must have already been installed, otherwise go to the installation instruction for [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

Utility binary crates that are written in rust, comes in handy to facilitate our development workflow.
`basic-http-server` is used to serve static files, and `just` command is a nice way to run our build scripts using `justfile`.

```sh
cargo install basic-http-server
cargo install just
```

## Running

```sh
just serve
```
