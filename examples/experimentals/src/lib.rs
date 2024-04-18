#![deny(warnings)]
#![deny(clippy::all)]
use app::App;
use sauron::*;

#[macro_use]
extern crate log;

mod app;
mod button;
mod datebox;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let root_node = document().get_element_by_id("web-app").unwrap();
    Program::clear_append_to_mount(App::new(0), &root_node);
}
