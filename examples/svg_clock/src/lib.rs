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
    Clock,
    Msg,
};

#[macro_use]
extern crate log;

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct SvgClock {
    #[allow(unused)]
    program: Rc<Program<Clock, Msg>>,
}

#[wasm_bindgen]
impl SvgClock {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SvgClock {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        trace!("starting svg clock..");

        let program = Program::mount_to_body(Clock::new());
        let program_clone = Rc::clone(&program);
        let clock: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
            program_clone.dispatch(Msg::Tick);
        }));
        window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                clock.as_ref().unchecked_ref(),
                1000,
            )
            .expect("Unable to start interval");
        clock.forget();
        SvgClock { program }
    }
}
