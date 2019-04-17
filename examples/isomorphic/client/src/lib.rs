//#![deny(warnings)]
#![deny(clippy::all)]
use console_error_panic_hook;
use sauron::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::console;

use app::App;
use app::Msg;

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    dom_updater: DomUpdater<App, Msg>,
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
        let view = app.view();
        let dom_updater =
            DomUpdater::new_replace_mount(Rc::new(RefCell::new(app)), view, root_node);
        let mut client = Client { dom_updater };
        client
    }
}
