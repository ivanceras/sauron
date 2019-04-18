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
    program: Rc<Program<App, Msg>>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        sauron::log!("Do something with the initial state: {}", initial_state);

        let root_node = document()
            .get_element_by_id("isomorphic-rust-web-app")
            .unwrap();

        let app = App::new(1);
        let program = Program::new_replace_mount(app, &root_node);
        Client { program }
    }
}
