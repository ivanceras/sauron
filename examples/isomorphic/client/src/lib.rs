#![deny(warnings)]
#![deny(clippy::all)]
use console_error_panic_hook;
use sauron::Component;
use sauron::View;
use sauron::Widget;
use sauron::*;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::console;

use app::App;

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}

// Expose globals from JS for things such as request animation frame
// that web sys doesn't seem to have yet
//
// TODO: Remove this and use RAF from Rust
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;

    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        console::log_1(&format!("What to do with this initial state: {}", initial_state).into());

        let root_node = document()
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();

        let app = App::new(1);

        let dom_updater = DomUpdater::new_replace_mount(app.view(), root_node);
        let mut client = Client { app, dom_updater };
        client.init_subscrption();
        client
    }

    /// set up the app.store
    /// whenever there is a changes to the store
    /// the app.update function will be called
    pub fn init_subscrption(&mut self) {
        self.app.subscribe(Box::new(|| {
            global_js.update();
        }));
    }

    pub fn render(&mut self) {
        console::log_1(&"in render function".into());
        self.app.update();
        let vdom = self.app.view();
        self.dom_updater.update(vdom);
    }
}
