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
        let view = app.view();
        let rc_app = Rc::new(RefCell::new(app));
        let mut dom_updater = DomUpdater::new(view, body.as_ref());

        let program = Program {
            app: rc_app,
            dom_updater: Rc::new(RefCell::new(dom_updater)),
        };

        let rc_program = Rc::new(program);
        rc_program.dom_updater.borrow_mut().mount(&rc_program);

        Client {
            program: rc_program,
        }
    }

    /*
    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.app.update();
        self.dom_updater.update(self.app.view());
    }
    */
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
