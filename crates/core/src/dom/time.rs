use crate::dom::{window, Cmd};
use futures::channel::mpsc;
use wasm_bindgen::{prelude::*, JsCast};

/// Provides function related to Time
#[derive(Clone, Copy)]
pub struct Time;

impl Time {

    /// do this task at every `ms` interval
    pub fn every<F, MSG>(interval_ms: i32, cb: F) -> Cmd<MSG>
    where
        F: Fn() -> MSG + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        //The web_sys::Event here is undefined, it is just used here to make storing the closure
        //uniform
        let closure_cb: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |_event| {
            let msg = cb();
            tx.start_send(msg).unwrap();
        });
        window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure_cb.as_ref().unchecked_ref(),
                interval_ms,
            )
            .expect("Unable to start interval");
        Cmd::recurring(rx, closure_cb)
    }
}
