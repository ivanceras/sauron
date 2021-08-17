![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# sauron


[![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
[![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png)

**Sauron** is a versatile web framework and library for building client-side and/or server-side web applications
with strong focus on simplicity. It is suited for developing web application which uses progressive rendering.


#### Counter example
In your `src/lib.rs`
```rust
use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};

#[derive(Debug)]
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    count: i32,
}

impl App {
    pub fn new() -> Self {
        App { count: 0 }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    key="inc"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <div class="count">{text(self.count)}</div>
                <input type="button"
                    value="-"
                    key="dec"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App::new());
}
```
`index.html`
```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>Counter</title>
    <style type="text/css">
        body { font-family: verdana, arial, monospace; }
        main {
            width:30px;
            height: 100px;
            margin:auto;
            text-align: center;
        }
        input, .count{
            font-size: 40px;
            padding: 30px;
        }
    </style>
    <script type=module>
        import init from './pkg/counter.js';
        await init().catch(console.error);
    </script>
  </head>
  <body>
  </body>
</html>
```
In `Cargo.toml`, specify the crate-type to be `cdylib`

```toml
[package]
name = "counter"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
sauron = "0.40"
```


Build using
```sh
$> wasm-pack build --target web --release
```
Explore some other [examples](https://github.com/ivanceras/sauron/tree/master/examples)
on this repo.


#### Demo examples
- [todomvc](https://ivanceras.github.io/todomvc/) The todomvc example
- [data-viewer](https://ivanceras.github.io/data-viewer/) - A resizable spreadsheet CSV data viewer
- [svg-clock](https://ivanceras.github.io/svg-clock/) - A clock drawn using SVG and window tick event.
- [ultron code-editor](https://ivanceras.github.io/ultron/) - A web-base text-editor with syntax highlighting
- [hackernews-sauron](https://github.com/ivanceras/hackernews-sauron) - A hackernews clone showcasing the feature of sauron to write web applications that can work with our without javascript.


#### Prerequisite:

```sh
cargo install wasm-pack
cargo install basic-http-server
```


License: MIT
