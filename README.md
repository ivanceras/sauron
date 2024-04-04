![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# sauron


[![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
[![Build Status](https://img.shields.io/github/workflow/status/ivanceras/sauron/Rust)](https://github.com/ivanceras/sauron/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png)

**Sauron** is a versatile web framework and library for building client-side and/or server-side web applications
with strong focus on ergonomics, simplicity and elegance.
This allows you to write least amount of code possible, and focus more on the business logic rather than the inner details of the framework.

Sauron is inspired by elm-lang and is following The Elm Architecture.

#### Features
- server-side rendering
- static site generation
- progressive rendering
- web components / custom-element
- html syntax for writing views
- elegant macro to write styles
- batteries included

### Devoid of unnecessary framework complexities
- **no** framework specific cli needed
- **no** template specific language as everything is in rust.
    - Model and update function is all in rust.
    - view? in rust
    - events handling? rust
    - styling? believe it or not: rust

In a sauron application, there is only the model, view and update.
The model is your application state.
The view describes how to present the model to the user.
The update function describes how to update the model, this uses message which contains the data needed for updating the model.


#### Counter example
In your `src/lib.rs`
```rust
use sauron::{
    html::text, html::units::px, jss, node, wasm_bindgen, Application, Cmd, Node, Program,
};

enum Msg {
    Increment,
    Decrement,
    Reset,
}

struct App {
    count: i32,
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}

impl Application for App {

    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <button class="count" on_click=|_|{Msg::Reset} >{text(self.count)}</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        Cmd::none()
    }

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body":{
                font_family: "verdana, arial, monospace",
            },

            "main":{
                width: px(30),
                height: px(100),
                margin: "auto",
                text_align: "center",
            },

            "input, .count":{
                font_size: px(40),
                padding: px(30),
                margin: px(30),
            }
        }]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}
```

`index.html`
```html
<!doctype html>
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>Counter</title>
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
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
sauron = "0.61.0"
```

#### Prerequisite:

```sh
cargo install wasm-pack
cargo install basic-http-server
```


Build using
```sh
wasm-pack build --target web --release
```
Serve using
```
basic-http-server -a 0.0.0.0:4000
```
Then navigate to http://localhost:4000

Head over to the [`getting-started.md`](docs/getting-started.md) for the full tutorial.

For more details on the commands to build and serve, look on [examples](https://github.com/ivanceras/sauron/tree/master/examples) on this repo, each
has scripts on how to build and run them.


#### Demo examples
- [todomvc](https://ivanceras.github.io/todomvc/) The todomvc example
- [data-viewer](https://ivanceras.github.io/data-viewer/) - A resizable spreadsheet CSV data viewer
- [svg-clock](https://ivanceras.github.io/svg-clock/) - A clock drawn using SVG and window tick event.
- [ultron code-editor](https://ivanceras.github.io/ultron/) - A web-base text-editor with syntax highlighting
- [hackernews-sauron](https://github.com/ivanceras/hackernews-sauron) - A hackernews clone showcasing the feature of sauron to write web applications that can work with or without javascript.



License: MIT
