#![deny(warnings)]
#![deny(clippy::all)]
use console_error_panic_hook;
use sauron::*;
use std::rc::Rc;
use wasm_bindgen::{
    self,
    prelude::*,
    JsCast,
};

use app::{
    App,
    Msg,
};

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    #[allow(unused)]
    program: Rc<Program<App, Msg>>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();
        sauron::log!("Do something with the initial state: {}", initial_state);

        let root_node = document().get_element_by_id("web-app").unwrap();

        let app = App::new(0);
        let program = Program::new_replace_mount(app, &root_node);
        let program_clone = Rc::clone(&program);
        let clock: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
            program_clone.dispatch(Msg::Clock);
        }));
        window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                clock.as_ref().unchecked_ref(),
                1000,
            )
            .expect("Unable to start interval");
        clock.forget();
        Client { program }
    }
}
