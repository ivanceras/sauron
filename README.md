![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# sauron


[![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
[![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png)

[Guide](https://sauron-rs.github.io/)

 **Sauron** is an HTML web framework for building web-apps with the goal of
 closely adhering to [The Elm Architecture](https://guide.elm-lang.org/architecture/), a paragon of elegant design.

 Sauron follow Elm's simplistic design of writing view code.

#### Example
```rust
use log::trace;
use sauron::html::attributes::attr;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};

#[derive(Debug)]
pub enum Msg {
    Click,
}

pub struct App {
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Component<Msg> for App {
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
                                trace!("Button is clicked");
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
        trace!("App is updating with msg: {:?}", msg);
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

index.html
```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>Minimal sauron app</title>
  </head>
  <body>
    <script type=module>
        import init from './pkg/minimal.js';
        init().catch(console.error);
    </script>
  </body>
</html>
```
In Cargo.toml, specify the crate-type to be `cdylib`
```toml

[package]
name = "minimal"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]


[dependencies]
sauron = "0.32"
console_error_panic_hook = "0.1"
log = "0.4"
console_log = "0.2"
```


Build using
```sh
$> wasm-pack build --target web --release
```
Look at the [examples](https://github.com/ivanceras/sauron/tree/master/examples)
and the build script for the details.


#### Demo examples
- [todomvc](https://ivanceras.github.io/todomvc/) The todomvc example
- [futuristic-ui](https://ivanceras.github.io/futuristic-ui/) - A demo of futuristic-ui
showcasing animation, transition and timed Component update.
- [data-viewer](https://ivanceras.github.io/data-viewer/) - A resizable spreadsheet CSV data viewer
- [svg-clock](https://ivanceras.github.io/svg-clock/) - A clock drawn using SVG and window tick event.
- [svg-graph](https://ivanceras.github.io/svg-graph/) - A simple graph using SVG
- [code-editor](https://ivanceras.github.io/code-editor/) - A WIP web-base code-editor

#### Converting HTML into Sauron's syntax

[html2sauron](https://ivanceras.github.io/html2sauron/) - A tool to easily convert html into
sauron node tree for your views.

#### Prerequisite:

```sh
cargo install wasm-pack
cargo install basic-http-server
```

#### Performance:
Sauron is one of the fastest.

![Benchmark](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/alt-sauron-0.28.png)
![Benchmark](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron-0.27.png)

#### Run the benchmark yourself:
[Benchmark 1](https://ivanceras.github.io/todo-mvc-bench/)
[Benchmark 2](https://ivanceras.github.io/todomvc-benchmark/)

#### Please support this project:
 [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)





License: MIT
