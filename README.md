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
use sauron::prelude::*;

#[derive(Debug)]
enum Msg {
    Click,
}

struct App {
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>"Minimal example"</h1>
                <div class="some-class" id="some-id" {attr("data-id", 1)}>
                    <input class="client"
                            type="button"
                            value="Click me!"
                            key=1
                            on_click={|_| {
                                log::trace!("Button is clicked");
                                Msg::Click
                            }}
                    />
                    <div>{text(format!("Clicked: {}", self.click_count))}</div>
                    <input type="text" value={self.click_count}/>
                </div>
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        log::trace!("App is updating with msg: {:?}", msg);
        match msg {
            Msg::Click => self.click_count += 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
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
sauron = "0.50.0"
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
