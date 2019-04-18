//#![deny(warnings)]
use app::App;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::DomUpdater;
use wasm_bindgen::prelude::*;

use crate::app::Msg;
use sauron::Component;
use sauron::Program;
use std::cell::RefCell;
use std::rc::Rc;

mod app;

#[wasm_bindgen]
pub struct Client {
    program: Rc<Program<App, Msg>>,
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
        let program = Program::new_append_mount(app, &body);

        Client { program }
    }
}

#[wasm_bindgen]
pub fn initialize() -> Client {
    let client = Client::new();
    client
}
