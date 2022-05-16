use crate::web_sys;
use crate::web_sys::Element;
use crate::web_sys::HtmlElement;
use sauron::prelude::*;
use sauron::wasm_bindgen::JsCast;
use std::collections::BTreeMap;

mod app;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    //Program::mount_to_body(App::default());
}
