#![deny(warnings)]
use app::Model;
use sauron::Program;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

mod app;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
    }
    trace!("in main!");

    #[cfg(feature = "with-storage")]
    let model = Model::get_from_storage();
    #[cfg(not(feature = "with-storage"))]
    let model = Model::new();

    Program::mount_to_body(model);
}
