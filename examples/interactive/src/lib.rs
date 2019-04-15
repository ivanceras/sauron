//#![deny(warnings)]
use app::App;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Component;
use sauron::DomUpdater;
use sauron::View;
use sauron::Widget;
use wasm_bindgen::prelude::*;

mod app;

#[wasm_bindgen]
pub struct Client {
    app: App,
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
        sauron::log("I see you!");
        let mut app = App::new();
        let body = sauron::body();
        let dom_updater = DomUpdater::new_append_to_mount(app.view(), &body);
        app.subscribe(Box::new(|| {
            global_js.update();
        }));

        Client { app, dom_updater }
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.app.update();
        self.dom_updater.update(self.app.view());
    }
}

#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;
    pub static global_js: GlobalJS;
    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
pub fn initialize() -> Client {
    let client = Client::new();
    client
}
