#![deny(warnings)]
use app::Model;
use sauron::Program;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

mod app;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
    }
    trace!("in main!");
    Program::mount_to_body(Model::new());
}
