#![deny(warnings)]
#![deny(clippy::all)]
use console_error_panic_hook;
use sauron::*;
use std::rc::Rc;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use app::App;
use app::Msg;

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main(initial_state: &str) {
    console_error_panic_hook::set_once();
    sauron::log!("Do something with the initial state: {}", initial_state);

    let root_node = document()
        .get_element_by_id("isomorphic-rust-web-app")
        .unwrap();

    let app = App::new(1);
    let program = Program::new_replace_mount(app, &root_node);
    let program_clone = Rc::clone(&program);
    let clock: Closure<Fn()> = Closure::wrap(Box::new(move || {
        sauron::log("is this triggered?");
        program_clone.dispatch(Msg::Clock);
    }));
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            clock.as_ref().unchecked_ref(),
            1000,
        )
        .expect("Unable to start interval");
    clock.forget();
}
