# sauron


 Sauron is an html web framework for building web-apps.
 It is heavily inspired by elm.

## Example
```rust
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::DomUpdater;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Client {
    #[allow(unused)]
    dom_updater: DomUpdater,
}

/// Build using
/// ```sh
/// $ wasm-pack build --target no-modules
/// ```
///
#[wasm_bindgen]
impl Client {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Client {
        let html = div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [input(
                [
                    class("client"),
                    r#type("button"),
                    value("Click me!"),
                    onclick(|_| {
                        sauron::log("i've been clicked");
                    }),
                ],
                [],
            )],
        );
        sauron::log("hello from here!");
        let body = sauron::body();
        let dom_updater = DomUpdater::new_append_to_mount(html, &body);
        Client { dom_updater }
    }
}

```


License: MIT
